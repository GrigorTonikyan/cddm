#![forbid(unsafe_code)]

use crate::fingerprint::{MIN_K_GRAM, WINDOW_OFFSET, winnow};
use crate::grammar::get_grammar_for_path;
use crate::tokenizer::tokenize;
use crate::types::{ClonePair, CloneType, LineSpan, MAX_HEALTH_SCORE, MIN_HEALTH_SCORE};
use std::collections::HashMap;
use std::path::Path;

/// Determines whether a directory entry should be ignored during tree extraction.
pub fn is_ignored_dir(name: &str) -> bool {
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
pub fn extract_files_from_tree(
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

pub fn count_tokens_in_line_span(spans: &[LineSpan], start_line: usize, end_line: usize) -> usize {
    spans
        .iter()
        .filter(|s| s.line_start >= start_line && s.line_end <= end_line)
        .count()
}

/// Evaluates duplication metrics for a set of file contents in memory.
pub fn evaluate_in_memory_duplication(
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

    let merged_pairs =
        crate::detector::indexer::merge_overlapping_clone_pairs(raw_pairs, min_tokens, k, |path| {
            parsed_files
                .iter()
                .find(|(p, _, _, _)| p == path)
                .map(|(_, spans, _, _)| spans.as_slice())
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
