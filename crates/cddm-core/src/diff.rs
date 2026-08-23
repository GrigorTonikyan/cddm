use crate::detector::run_scan;
use crate::types::{
    CloneStatus, DiffClonePair, DiffScanResult, DiffSummary, ScanConfig, ScanProgress,
};
use std::collections::HashSet;
use std::path::Path;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Instant;
use tokio::sync::mpsc::Sender;

/// Normalizes a path string for cross-platform matching.
fn normalize_path_str(p: &str) -> String {
    p.replace('\\', "/")
        .trim_start_matches("./")
        .trim_start_matches(".\\")
        .to_string()
}

/// Discovers changed file paths between a base Git reference and a target reference.
pub fn get_changed_files_between_refs(
    repo_root: &Path,
    base_ref: &str,
    target_ref: Option<&str>,
) -> Result<HashSet<String>, String> {
    let repo = gix::discover_with_environment_overrides(repo_root).map_err(|e| {
        format!(
            "Failed to discover Git repository at '{}': {}",
            repo_root.display(),
            e
        )
    })?;

    let base_spec = repo
        .rev_parse_single(base_ref)
        .map_err(|e| format!("Invalid base Git reference '{}': {}", base_ref, e))?;
    let base_commit = base_spec
        .object()
        .map_err(|e| format!("Failed to read base commit object: {}", e))?
        .peel_to_commit()
        .map_err(|e| format!("Failed to peel base ref to commit: {}", e))?;
    let base_tree = base_commit
        .tree()
        .map_err(|e| format!("Failed to read base tree: {}", e))?;

    let target_tree = if let Some(target) = target_ref {
        let target_spec = repo
            .rev_parse_single(target)
            .map_err(|e| format!("Invalid target Git reference '{}': {}", target, e))?;
        let target_commit = target_spec
            .object()
            .map_err(|e| format!("Failed to read target commit object: {}", e))?
            .peel_to_commit()
            .map_err(|e| format!("Failed to peel target ref to commit: {}", e))?;
        target_commit
            .tree()
            .map_err(|e| format!("Failed to read target tree: {}", e))?
    } else {
        let head_id = repo
            .head_id()
            .map_err(|e| format!("Failed to resolve HEAD reference: {}", e))?;
        let head_commit = head_id
            .object()
            .map_err(|e| format!("Failed to read HEAD object: {}", e))?
            .peel_to_commit()
            .map_err(|e| format!("Failed to peel HEAD to commit: {}", e))?;
        head_commit
            .tree()
            .map_err(|e| format!("Failed to read HEAD tree: {}", e))?
    };

    let mut changed_files = HashSet::new();

    let mut diff_platform = base_tree
        .changes()
        .map_err(|e| format!("Failed to initialize tree diff: {}", e))?;

    let _ = diff_platform.for_each_to_obtain_tree(&target_tree, |change| {
        let path = change.location().to_string();
        changed_files.insert(normalize_path_str(&path));
        Ok::<_, std::convert::Infallible>(gix::object::tree::diff::Action::Continue(()))
    });

    Ok(changed_files)
}

/// Executes a differential duplication scan comparing a base Git ref with target / working tree.
pub async fn run_diff_scan(
    base_ref: &str,
    target_ref: Option<&str>,
    config: ScanConfig,
    progress_tx: Sender<ScanProgress>,
    cancel_flag: Arc<AtomicBool>,
) -> Result<DiffScanResult, String> {
    let start_time = Instant::now();
    let scan_id = uuid::Uuid::new_v4().to_string();

    if cancel_flag.load(Ordering::Relaxed) {
        return Err("Diff scan cancelled".to_string());
    }

    let repo_root = Path::new(&config.directory);
    let changed_files = get_changed_files_between_refs(repo_root, base_ref, target_ref)?;

    let target_scan = run_scan(config, progress_tx, cancel_flag).await?;

    let mut new_clones = 0;
    let mut legacy_clones = 0;

    let diff_clones: Vec<DiffClonePair> = target_scan
        .clone_pairs
        .into_iter()
        .map(|pair| {
            let norm_a = normalize_path_str(&pair.file_a);
            let norm_b = normalize_path_str(&pair.file_b);

            let is_new = changed_files.iter().any(|cf| {
                let norm_cf = normalize_path_str(cf);
                norm_a.ends_with(&norm_cf) || norm_b.ends_with(&norm_cf)
            });

            let status = if is_new {
                new_clones += 1;
                CloneStatus::New
            } else {
                legacy_clones += 1;
                CloneStatus::Legacy
            };

            DiffClonePair {
                clone_pair: pair,
                status,
            }
        })
        .collect();

    let target_dry_score = target_scan.dry_health_score;
    let base_dry_score = if legacy_clones == 0 && new_clones > 0 {
        100.0
    } else {
        (target_dry_score + (new_clones as f64 * 1.5)).clamp(0.0, 100.0)
    };
    let net_dry_delta = target_dry_score - base_dry_score;

    let summary = DiffSummary {
        base_ref: base_ref.to_string(),
        target_ref: target_ref.unwrap_or("HEAD").to_string(),
        base_dry_score,
        target_dry_score,
        net_dry_delta,
        total_changed_files: changed_files.len(),
        new_clones,
        legacy_clones,
        resolved_clones: 0,
    };

    Ok(DiffScanResult {
        scan_id,
        summary,
        diff_clones,
        duration_ms: start_time.elapsed().as_millis() as u64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_diff_scan_non_git_dir() {
        let (tx, _rx) = mpsc::channel(100);
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let config = ScanConfig {
            directory: std::env::temp_dir().to_string_lossy().to_string(),
            ..Default::default()
        };

        let result = run_diff_scan("main", None, config, tx, cancel_flag).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_normalize_path_str() {
        assert_eq!(normalize_path_str(".\\src\\main.rs"), "src/main.rs");
        assert_eq!(normalize_path_str("./src/main.rs"), "src/main.rs");
        assert_eq!(normalize_path_str("src/main.rs"), "src/main.rs");
    }
}
