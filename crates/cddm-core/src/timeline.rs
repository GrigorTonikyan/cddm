use crate::fingerprint::{MIN_K_GRAM, WINDOW_OFFSET, winnow};
use crate::grammar::get_grammar_for_path;
use crate::tokenizer::tokenize;
use crate::types::{
    ClonePair, CloneType, FileChurnMetric, LineSpan, MAX_HEALTH_SCORE, MIN_HEALTH_SCORE,
    TimelineSnapshot, TimelineTrend,
};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Determines whether a directory entry should be ignored during tree extraction.
fn is_ignored_dir(name: &str) -> bool {
    matches!(
        name,
        "node_modules"
            | "target"
            | "target_test"
            | "dist"
            | "build"
            | ".git"
            | ".logs"
            | ".cddm"
            | ".vite-hooks"
            | "fixtures"
    )
}

/// Recursively extracts all supported code file paths and UTF-8 contents from a Git Tree.
fn extract_files_from_tree(
    repo: &gix::Repository,
    tree: &gix::Tree,
    prefix: &str,
    files: &mut Vec<(String, String)>,
) -> Result<(), String> {
    for entry_ref in tree.iter() {
        let entry = entry_ref.map_err(|e| format!("Failed to read tree entry: {e}"))?;
        let name = entry.filename().to_string();

        if is_ignored_dir(&name) {
            continue;
        }

        let relative_path = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };

        let mode = entry.mode();
        if mode.is_tree() {
            let sub_tree = repo
                .find_object(entry.oid())
                .map_err(|e| format!("Failed to find sub-tree object: {e}"))?
                .peel_to_tree()
                .map_err(|e| format!("Failed to peel object to tree: {e}"))?;
            extract_files_from_tree(repo, &sub_tree, &relative_path, files)?;
        } else if (mode.is_blob() || mode.is_executable())
            && get_grammar_for_path(Path::new(&relative_path)).is_some()
            && let Ok(blob) = repo.find_object(entry.oid())
            && let Ok(text) = std::str::from_utf8(&blob.data)
        {
            files.push((relative_path, text.to_string()));
        }
    }
    Ok(())
}

fn count_tokens_in_line_span(spans: &[LineSpan], start_line: usize, end_line: usize) -> usize {
    spans
        .iter()
        .filter(|s| s.line_start >= start_line && s.line_end <= end_line)
        .count()
}

/// Evaluates duplication metrics for a set of file contents in memory.
fn evaluate_in_memory_duplication(
    files: &[(String, String)],
    min_tokens: usize,
) -> (usize, usize, usize, usize, f64, f64) {
    if files.is_empty() {
        return (0, 0, 0, 0, 0.0, 100.0);
    }

    let k = std::cmp::max(MIN_K_GRAM, min_tokens / 2);
    let w = k + WINDOW_OFFSET;

    let mut total_tokens = 0usize;
    let mut parsed_files = Vec::with_capacity(files.len());

    for (file_path, content) in files {
        if let Some(grammar) = get_grammar_for_path(Path::new(file_path)) {
            let tokens = tokenize(content, grammar, true);
            let count = tokens.len();
            total_tokens += count;

            let token_spans: Vec<LineSpan> = tokens.iter().map(|(_, span)| span.clone()).collect();
            let fingerprints = if count >= k {
                winnow(&tokens, k, w)
            } else {
                Vec::new()
            };

            parsed_files.push((file_path.clone(), token_spans, fingerprints, count));
        }
    }

    if total_tokens == 0 {
        return (files.len(), 0, 0, 0, 0.0, 100.0);
    }

    // Invert fingerprints: hash -> list of (file_idx, LineSpan)
    let mut hash_index: HashMap<(u64, u64), Vec<(usize, LineSpan)>> = HashMap::new();
    for (file_idx, (_, _, fps, _)) in parsed_files.iter().enumerate() {
        for fp in fps {
            hash_index
                .entry(fp.hash)
                .or_default()
                .push((file_idx, fp.span.clone()));
        }
    }

    // Generate raw clone pairs
    let mut raw_pairs = Vec::new();
    for (&hash, locs) in &hash_index {
        if locs.len() > 1 {
            for i in 0..locs.len() {
                for j in (i + 1)..locs.len() {
                    let (idx_a, span_a) = &locs[i];
                    let (idx_b, span_b) = &locs[j];

                    if idx_a == idx_b && span_a.line_start == span_b.line_start {
                        continue;
                    }

                    let (loc_first, loc_second) = if idx_a < idx_b
                        || (idx_a == idx_b && span_a.line_start <= span_b.line_start)
                    {
                        ((*idx_a, span_a), (*idx_b, span_b))
                    } else {
                        ((*idx_b, span_b), (*idx_a, span_a))
                    };

                    raw_pairs.push(ClonePair {
                        file_a: parsed_files[loc_first.0].0.clone(),
                        start_line_a: loc_first.1.line_start,
                        end_line_a: loc_first.1.line_end,
                        file_b: parsed_files[loc_second.0].0.clone(),
                        start_line_b: loc_second.1.line_start,
                        end_line_b: loc_second.1.line_end,
                        token_count: k,
                        similarity: 1.0,
                        fragment_hash: format!("{:x}-{:x}", hash.0, hash.1),
                        clone_type: CloneType::Exact,
                        author_a: None,
                        author_b: None,
                    });
                }
            }
        }
    }

    // Sort raw pairs for merging
    raw_pairs.sort_by(|a, b| {
        a.file_a
            .cmp(&b.file_a)
            .then(a.file_b.cmp(&b.file_b))
            .then(a.start_line_a.cmp(&b.start_line_a))
            .then(a.start_line_b.cmp(&b.start_line_b))
    });

    let mut merged_pairs = Vec::new();
    if !raw_pairs.is_empty() {
        let mut push_pair_if_valid = |mut pair: ClonePair, f_a_idx: usize, f_b_idx: usize| {
            let count_a = count_tokens_in_line_span(
                &parsed_files[f_a_idx].1,
                pair.start_line_a,
                pair.end_line_a,
            );
            let count_b = count_tokens_in_line_span(
                &parsed_files[f_b_idx].1,
                pair.start_line_b,
                pair.end_line_b,
            );
            pair.token_count = std::cmp::max(k, std::cmp::min(count_a, count_b));
            if pair.token_count >= min_tokens {
                merged_pairs.push(pair);
            }
        };

        let mut current = raw_pairs[0].clone();
        let mut curr_f_a_idx = parsed_files
            .iter()
            .position(|f| f.0 == current.file_a)
            .unwrap_or(0);
        let mut curr_f_b_idx = parsed_files
            .iter()
            .position(|f| f.0 == current.file_b)
            .unwrap_or(0);

        for next in raw_pairs.into_iter().skip(1) {
            let is_same_file = current.file_a == current.file_b;
            let candidate_end_a = std::cmp::max(current.end_line_a, next.end_line_a);
            let candidate_end_b = std::cmp::max(current.end_line_b, next.end_line_b);
            let (first_end, second_start) = if current.start_line_a <= current.start_line_b {
                (candidate_end_a, current.start_line_b)
            } else {
                (candidate_end_b, current.start_line_a)
            };
            let would_overlap = is_same_file && (first_end >= second_start);

            if current.file_a == next.file_a
                && current.file_b == next.file_b
                && next.start_line_a <= current.end_line_a + 3
                && next.start_line_b <= current.end_line_b + 3
                && !would_overlap
            {
                current.end_line_a = candidate_end_a;
                current.end_line_b = candidate_end_b;
            } else {
                push_pair_if_valid(current, curr_f_a_idx, curr_f_b_idx);
                current = next;
                curr_f_a_idx = parsed_files
                    .iter()
                    .position(|f| f.0 == current.file_a)
                    .unwrap_or(0);
                curr_f_b_idx = parsed_files
                    .iter()
                    .position(|f| f.0 == current.file_b)
                    .unwrap_or(0);
            }
        }

        push_pair_if_valid(current, curr_f_a_idx, curr_f_b_idx);
    }

    // Dedup identical clone pairs
    merged_pairs.sort_by(|a, b| {
        a.file_a
            .cmp(&b.file_a)
            .then(a.file_b.cmp(&b.file_b))
            .then(a.start_line_a.cmp(&b.start_line_a))
            .then(a.start_line_b.cmp(&b.start_line_b))
            .then(a.end_line_a.cmp(&b.end_line_a))
            .then(a.end_line_b.cmp(&b.end_line_b))
    });
    merged_pairs.dedup_by(|a, b| {
        a.file_a == b.file_a
            && a.file_b == b.file_b
            && a.start_line_a == b.start_line_a
            && a.end_line_a == b.end_line_a
            && a.start_line_b == b.start_line_b
            && a.end_line_b == b.end_line_b
    });

    let total_clones = merged_pairs.len();
    let total_duplicated_tokens: usize = merged_pairs.iter().map(|p| p.token_count).sum();
    let duplication_percentage = if total_tokens > 0 {
        ((total_duplicated_tokens as f64) / (total_tokens as f64) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    // Compute cross-module ratio
    let mut cross_module_clones = 0usize;
    for pair in &merged_pairs {
        let norm_a = pair.file_a.replace('\\', "/");
        let norm_b = pair.file_b.replace('\\', "/");
        let dir_a = Path::new(&norm_a).parent().unwrap_or(Path::new(""));
        let dir_b = Path::new(&norm_b).parent().unwrap_or(Path::new(""));
        if dir_a != dir_b {
            cross_module_clones += 1;
        }
    }

    let cross_module_ratio = if total_clones > 0 {
        cross_module_clones as f64 / total_clones as f64
    } else {
        0.0
    };

    let duplication_weight = 1.5 * (1.0 + 0.3 * cross_module_ratio);
    let dry_health_score = (MAX_HEALTH_SCORE - duplication_percentage * duplication_weight)
        .clamp(MIN_HEALTH_SCORE, MAX_HEALTH_SCORE);

    // Group into clusters via simple disjoint set
    let clusters = crate::cluster::cluster_clone_pairs(&merged_pairs);

    (
        files.len(),
        total_tokens,
        total_clones,
        clusters.len(),
        duplication_percentage,
        dry_health_score,
    )
}

/// Collects historical duplication metrics and DRY Health score trajectory across Git history.
pub fn collect_git_timeline(
    repo_root: &Path,
    max_samples: usize,
    min_tokens: usize,
    cancel_flag: Arc<AtomicBool>,
) -> Result<TimelineTrend, String> {
    let repo = gix::discover_with_environment_overrides(repo_root).map_err(|e| {
        format!(
            "Failed to discover Git repository at '{}': {}",
            repo_root.display(),
            e
        )
    })?;

    let head_id = repo
        .head_id()
        .map_err(|e| format!("Failed to resolve HEAD: {e}"))?;

    // Build map of commit_hash -> tag_name
    let mut tag_map: HashMap<String, String> = HashMap::new();
    if let Ok(references) = repo.references()
        && let Ok(tags) = references.tags()
    {
        for tag_ref in tags.flatten() {
            let name = tag_ref.name().shorten().to_string();
            if let Some(target_id) = tag_ref.target().try_id() {
                tag_map.insert(target_id.to_string(), name);
            }
        }
    }

    // Traverse commit history using rev_walk
    let walk = repo
        .rev_walk([head_id])
        .all()
        .map_err(|e| format!("Failed to initialize revision walk: {e}"))?;

    let mut commit_ids = Vec::new();
    for commit_res in walk {
        if cancel_flag.load(Ordering::Relaxed) {
            return Err("Timeline analysis cancelled by user".to_string());
        }
        let commit_info = commit_res.map_err(|e| format!("Error during commit traversal: {e}"))?;
        commit_ids.push(commit_info.id);
    }

    if commit_ids.is_empty() {
        return Err("No commits found in Git repository history".to_string());
    }

    // Reverse so commit_ids is oldest -> newest
    commit_ids.reverse();

    // Sample up to max_samples evenly
    let samples_count = max_samples.clamp(1, 50).min(commit_ids.len());
    let sampled_indices: Vec<usize> = if commit_ids.len() <= samples_count {
        (0..commit_ids.len()).collect()
    } else {
        let step = (commit_ids.len() - 1) as f64 / (samples_count - 1) as f64;
        let mut idxs = Vec::with_capacity(samples_count);
        for i in 0..samples_count {
            let idx = (i as f64 * step).round() as usize;
            if !idxs.contains(&idx) && idx < commit_ids.len() {
                idxs.push(idx);
            }
        }
        if !idxs.contains(&(commit_ids.len() - 1)) {
            idxs.push(commit_ids.len() - 1);
        }
        idxs
    };

    let mut snapshots = Vec::with_capacity(sampled_indices.len());
    let mut file_commit_counts: HashMap<String, usize> = HashMap::new();

    for &idx in &sampled_indices {
        if cancel_flag.load(Ordering::Relaxed) {
            return Err("Timeline analysis cancelled by user".to_string());
        }

        let cid = commit_ids[idx];
        let commit = repo
            .find_object(cid)
            .map_err(|e| format!("Failed to read commit {cid}: {e}"))?
            .peel_to_commit()
            .map_err(|e| format!("Failed to peel to commit {cid}: {e}"))?;

        let commit_hash = cid.to_string();
        let short_hash = if commit_hash.len() >= 7 {
            commit_hash[..7].to_string()
        } else {
            commit_hash.clone()
        };

        let author_name = commit
            .author()
            .map(|a| a.name.to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

        let commit_time = commit.time().map(|t| t.seconds).unwrap_or(0);
        let formatted_date = chrono::DateTime::from_timestamp(commit_time, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "1970-01-01 00:00:00".to_string());

        let message = commit
            .message()
            .map(|m| m.title.to_string())
            .unwrap_or_else(|_| "No message".to_string());

        let tag = tag_map.get(&commit_hash).cloned();

        let tree = commit
            .tree()
            .map_err(|e| format!("Failed to read tree for commit {cid}: {e}"))?;

        let mut tree_files = Vec::new();
        let _ = extract_files_from_tree(&repo, &tree, "", &mut tree_files);

        for (fp, _) in &tree_files {
            *file_commit_counts.entry(fp.clone()).or_insert(0) += 1;
        }

        let (total_files, total_tokens, total_clones, total_clusters, dup_pct, dry_score) =
            evaluate_in_memory_duplication(&tree_files, min_tokens);

        snapshots.push(TimelineSnapshot {
            commit_hash,
            short_hash,
            author: author_name,
            commit_time,
            formatted_date,
            message,
            tag,
            total_files,
            total_tokens,
            total_clones,
            total_clusters,
            duplication_percentage: (dup_pct * 100.0).round() / 100.0,
            dry_health_score: (dry_score * 10.0).round() / 10.0,
        });
    }

    let initial_score = snapshots
        .first()
        .map(|s| s.dry_health_score)
        .unwrap_or(100.0);
    let current_score = snapshots
        .last()
        .map(|s| s.dry_health_score)
        .unwrap_or(100.0);
    let initial_dup = snapshots
        .first()
        .map(|s| s.duplication_percentage)
        .unwrap_or(0.0);
    let current_dup = snapshots
        .last()
        .map(|s| s.duplication_percentage)
        .unwrap_or(0.0);

    let score_delta = ((current_score - initial_score) * 10.0).round() / 10.0;
    let duplication_delta = ((current_dup - initial_dup) * 100.0).round() / 100.0;

    let mut churn_hotspots: Vec<FileChurnMetric> = file_commit_counts
        .into_iter()
        .map(|(file_path, commit_count)| FileChurnMetric {
            file_path,
            commit_count,
            clone_count: 0,
        })
        .collect();

    churn_hotspots.sort_by_key(|b| std::cmp::Reverse(b.commit_count));
    churn_hotspots.truncate(10);

    Ok(TimelineTrend {
        snapshots,
        initial_score,
        current_score,
        score_delta,
        duplication_delta,
        churn_hotspots,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_evaluate_in_memory_duplication_empty() {
        let (files, tokens, clones, clusters, dup, score) = evaluate_in_memory_duplication(&[], 50);
        assert_eq!(files, 0);
        assert_eq!(tokens, 0);
        assert_eq!(clones, 0);
        assert_eq!(clusters, 0);
        assert_eq!(dup, 0.0);
        assert_eq!(score, 100.0);
    }

    #[test]
    fn test_evaluate_in_memory_duplication_duplicate_files() {
        let code = r#"
            fn calculate_sum(a: i32, b: i32) -> i32 {
                let temp1 = a * 2;
                let temp2 = b * 3;
                let result = temp1 + temp2;
                println!("Result is {}", result);
                result + 100
            }
        "#;

        let files = vec![
            ("src/module_a.rs".to_string(), code.to_string()),
            ("src/module_b.rs".to_string(), code.to_string()),
        ];

        let (file_count, token_count, clone_count, cluster_count, dup_pct, dry_score) =
            evaluate_in_memory_duplication(&files, 20);

        assert_eq!(file_count, 2);
        assert!(token_count > 0);
        assert!(clone_count >= 1);
        assert!(cluster_count >= 1);
        assert!(dup_pct > 0.0);
        assert!(dry_score < 100.0);
    }

    #[test]
    fn test_collect_git_timeline_real_workspace() {
        let repo_root = Path::new(".");
        let cancel_flag = Arc::new(AtomicBool::new(false));

        let trend_result = collect_git_timeline(repo_root, 5, 50, cancel_flag);
        assert!(trend_result.is_ok(), "Expected git timeline to succeed");

        let trend = trend_result.expect("trend result");
        assert!(!trend.snapshots.is_empty());
        assert!(trend.snapshots.len() <= 6);
        assert!(trend.current_score >= 0.0 && trend.current_score <= 100.0);
    }

    #[test]
    fn test_collect_git_timeline_non_git_dir() {
        let temp = tempdir().expect("tempdir");
        let cancel_flag = Arc::new(AtomicBool::new(false));

        let trend_result = collect_git_timeline(temp.path(), 5, 50, cancel_flag);
        assert!(trend_result.is_err());
    }
}
