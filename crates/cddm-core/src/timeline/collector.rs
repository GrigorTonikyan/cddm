#![forbid(unsafe_code)]

use super::eval::{evaluate_in_memory_duplication, extract_files_from_tree};
use crate::types::{FileChurnMetric, TimelineSnapshot, TimelineTrend};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

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
