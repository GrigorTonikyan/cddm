#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use super::detector::run_dead_code_detection;
use super::types::{
    DeadClonePruneConfig, DeadClonePruneResult, DeadCodeConfig, DeadCodeItem, DeadCodeKind,
    PruneActionStatus, PrunedItem,
};
use crate::error::CddmError;
use crate::io::read_file_source;

/// Prunes dead clone clusters and unreferenced dead code entities across the workspace.
pub async fn prune_dead_clone_clusters(
    config: DeadClonePruneConfig,
) -> Result<DeadClonePruneResult, CddmError> {
    tracing::info!(
        directory = %config.directory,
        dry_run = config.dry_run,
        safe_only = config.safe_only,
        min_tokens = config.min_tokens,
        "Executing dead clone cluster pruning & safe deletion synthesizer"
    );

    let detect_config = DeadCodeConfig {
        directory: config.directory.clone(),
        min_tokens: config.min_tokens,
        static_only: false,
        report_path: None,
        report_content: None,
        languages: config.languages.clone(),
        ignore: config.ignore.clone(),
    };

    let summary = run_dead_code_detection(detect_config).await?;

    let candidate_items: Vec<DeadCodeItem> = summary
        .items
        .into_iter()
        .filter(|item| {
            if let Some(ref selected_ids) = config.item_ids {
                selected_ids.contains(&item.id)
            } else {
                matches!(
                    item.kind,
                    DeadCodeKind::DeadClone
                        | DeadCodeKind::UnreferencedFunction
                        | DeadCodeKind::UnreachableBlock
                )
            }
        })
        .collect();

    let total_candidates = candidate_items.len();
    let mut pruned_items = Vec::new();
    let mut files_to_modify: HashMap<String, Vec<DeadCodeItem>> = HashMap::new();
    let mut skipped_count = 0;

    for item in candidate_items {
        let is_safe = item.confidence >= config.confidence_threshold;
        if config.safe_only && !is_safe {
            tracing::warn!(
                id = item.id,
                symbol = %item.symbol_name,
                confidence = item.confidence,
                threshold = config.confidence_threshold,
                "Skipping item due to safety threshold constraint"
            );
            pruned_items.push(PrunedItem {
                id: item.id,
                file_path: item.file_path,
                symbol_name: item.symbol_name,
                line_start: item.line_start,
                line_end: item.line_end,
                lines_removed: 0,
                status: PruneActionStatus::SkippedUnsafe,
                confidence: item.confidence,
                reason: format!(
                    "Confidence {:.2} < safety threshold {:.2}",
                    item.confidence, config.confidence_threshold
                ),
                diff_preview: None,
            });
            skipped_count += 1;
        } else {
            files_to_modify
                .entry(item.file_path.clone())
                .or_default()
                .push(item);
        }
    }

    let mut affected_files_set = HashSet::new();
    let mut total_lines_removed = 0;

    for (file_rel, mut items_in_file) in files_to_modify {
        // Sort descending by line_start so removals from bottom to top do not invalidate line numbers
        items_in_file.sort_by_key(|b| std::cmp::Reverse(b.line_start));

        let file_path = if Path::new(&file_rel).is_absolute() {
            Path::new(&file_rel).to_path_buf()
        } else {
            Path::new(&config.directory).join(&file_rel)
        };

        let file_content = match read_file_source(&file_path) {
            Ok(src) => src.as_str().to_string(),
            Err(e) => {
                tracing::error!(file = %file_path.display(), error = %e, "Could not read file for pruning");
                for item in items_in_file {
                    pruned_items.push(PrunedItem {
                        id: item.id,
                        file_path: item.file_path,
                        symbol_name: item.symbol_name,
                        line_start: item.line_start,
                        line_end: item.line_end,
                        lines_removed: 0,
                        status: PruneActionStatus::Failed,
                        confidence: item.confidence,
                        reason: format!("File read error: {e}"),
                        diff_preview: None,
                    });
                    skipped_count += 1;
                }
                continue;
            }
        };

        let mut current_lines: Vec<String> = file_content.lines().map(|s| s.to_string()).collect();
        let mut file_changed = false;

        for item in items_in_file {
            let start_idx = item.line_start.saturating_sub(1);
            let end_idx = item.line_end.min(current_lines.len());

            if start_idx >= current_lines.len() || start_idx >= end_idx {
                pruned_items.push(PrunedItem {
                    id: item.id,
                    file_path: item.file_path,
                    symbol_name: item.symbol_name,
                    line_start: item.line_start,
                    line_end: item.line_end,
                    lines_removed: 0,
                    status: PruneActionStatus::Failed,
                    confidence: item.confidence,
                    reason: "Line boundaries out of file range".to_string(),
                    diff_preview: None,
                });
                skipped_count += 1;
                continue;
            }

            let lines_to_remove = end_idx - start_idx;
            let removed_slice = &current_lines[start_idx..end_idx];
            let diff_preview =
                generate_diff_snippet(&item.file_path, item.line_start, removed_slice);

            if !config.dry_run {
                current_lines.drain(start_idx..end_idx);
                file_changed = true;
            }

            total_lines_removed += lines_to_remove;
            affected_files_set.insert(item.file_path.clone());

            pruned_items.push(PrunedItem {
                id: item.id,
                file_path: item.file_path,
                symbol_name: item.symbol_name,
                line_start: item.line_start,
                line_end: item.line_end,
                lines_removed: lines_to_remove,
                status: if config.dry_run {
                    PruneActionStatus::DryRunPruned
                } else {
                    PruneActionStatus::Pruned
                },
                confidence: item.confidence,
                reason: item.reason,
                diff_preview: Some(diff_preview),
            });
        }

        if !config.dry_run && file_changed {
            let new_content = if current_lines.is_empty() {
                String::new()
            } else {
                format!("{}\n", current_lines.join("\n"))
            };

            if let Err(e) = fs::write(&file_path, new_content) {
                tracing::error!(file = %file_path.display(), error = %e, "Failed to write pruned file");
            }
        }
    }

    let pruned_count = pruned_items
        .iter()
        .filter(|p| {
            matches!(
                p.status,
                PruneActionStatus::Pruned | PruneActionStatus::DryRunPruned
            )
        })
        .count();

    let mut affected_files: Vec<String> = affected_files_set.into_iter().collect();
    affected_files.sort();

    Ok(DeadClonePruneResult {
        total_candidates,
        pruned_items: pruned_count,
        skipped_items: skipped_count,
        total_lines_removed,
        dry_run: config.dry_run,
        files_affected: affected_files,
        items: pruned_items,
    })
}

fn generate_diff_snippet(file_path: &str, start_line: usize, removed_lines: &[String]) -> String {
    let mut diff = format!(
        "--- a/{file_path}\n+++ /dev/null\n@@ -{},{} +0,0 @@\n",
        start_line,
        removed_lines.len()
    );
    for line in removed_lines {
        diff.push('-');
        diff.push_str(line);
        diff.push('\n');
    }
    diff
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_prune_dead_clones_dry_run() {
        let temp = tempdir().unwrap();
        let file_a = temp.path().join("dead_func.rs");
        let code = r#"fn main() {
    active_caller();
}

fn active_caller() {
    println!("hello");
}

fn unreferenced_dummy_unused_dead_function() {
    let x = 10;
    let y = 20;
    let z = x + y;
    println!("dead: {}", z);
}
"#;
        fs::write(&file_a, code).unwrap();

        let config = DeadClonePruneConfig {
            directory: temp.path().to_string_lossy().to_string(),
            min_tokens: 5,
            dry_run: true,
            safe_only: false,
            confidence_threshold: 0.5,
            ..Default::default()
        };

        let result = prune_dead_clone_clusters(config).await.unwrap();
        assert!(result.dry_run);
        assert_eq!(result.skipped_items, 0);

        // File should not be modified in dry-run
        let current_content = fs::read_to_string(&file_a).unwrap();
        assert_eq!(current_content, code);
    }

    #[tokio::test]
    async fn test_prune_dead_clones_actual_write() {
        let temp = tempdir().unwrap();
        let file_a = temp.path().join("dead_target.rs");
        let code = r#"fn main() {
    active_code();
}

fn active_code() {
    println!("active");
}

fn dead_unused_helper_to_prune() {
    let a = 1;
    let b = 2;
    let c = a + b;
    println!("unused: {}", c);
}
"#;
        fs::write(&file_a, code).unwrap();

        let config = DeadClonePruneConfig {
            directory: temp.path().to_string_lossy().to_string(),
            min_tokens: 5,
            dry_run: false,
            safe_only: true,
            confidence_threshold: 0.7,
            ..Default::default()
        };

        let result = prune_dead_clone_clusters(config).await.unwrap();
        assert!(!result.dry_run);
        assert!(result.pruned_items > 0);
        let modified = fs::read_to_string(&file_a).unwrap();
        assert!(!modified.contains("dead_unused_helper_to_prune"));
        assert!(modified.contains("active_code"));
        assert!(modified.contains("fn main()"));
    }
}
