use crate::grammar::get_grammar_for_path;
use crate::types::ScanResult;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, channel};

/// Default folder names ignored during real-time watch scanning.
pub const DEFAULT_WATCH_IGNORES: &[&str] = &[
    "target",
    "node_modules",
    ".git",
    ".cddm",
    "dist",
    ".turbo",
    ".next",
    ".output",
    "build",
];

/// Detailed change event for an observed file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WatchFileEvent {
    pub path: String,
    pub timestamp_millis: u64,
}

/// Comparative delta metrics between successive incremental scans.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WatchDeltaReport {
    /// List of file paths modified in this watch batch
    pub changed_files: Vec<String>,
    /// Previous DRY health score (0.0 - 100.0)
    pub previous_health_score: f64,
    /// New DRY health score (0.0 - 100.0)
    pub new_health_score: f64,
    /// Shift in DRY health score
    pub score_delta: f64,
    /// Previous total clone pair count
    pub previous_clones: usize,
    /// New total clone pair count
    pub new_clones: usize,
    /// Shift in clone count (+ means more clones, - means fewer)
    pub clone_count_delta: i64,
    /// Previous total clone cluster count
    pub previous_clusters: usize,
    /// New total clone cluster count
    pub new_clusters: usize,
    /// Scan execution time in milliseconds
    pub duration_ms: u128,
    /// Timestamp in milliseconds since Unix epoch
    pub timestamp_millis: u64,
}

impl WatchDeltaReport {
    /// Computes delta metrics between an optional previous scan result and a new result.
    pub fn compute(
        prev: Option<&ScanResult>,
        new_res: &ScanResult,
        changed_paths: &[PathBuf],
        duration_ms: u128,
    ) -> Self {
        let changed_files = changed_paths
            .iter()
            .map(|p| p.to_string_lossy().to_string().replace('\\', "/"))
            .collect();

        let previous_health_score = prev
            .map(|p| p.dry_health_score)
            .unwrap_or(new_res.dry_health_score);
        let previous_clones = prev.map(|p| p.total_clones).unwrap_or(new_res.total_clones);
        let previous_clusters = prev
            .map(|p| p.total_clusters)
            .unwrap_or(new_res.total_clusters);

        let raw_delta = new_res.dry_health_score - previous_health_score;
        let score_delta = (raw_delta * 10.0).round() / 10.0;
        let clone_count_delta = new_res.total_clones as i64 - previous_clones as i64;

        let timestamp_millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            changed_files,
            previous_health_score,
            new_health_score: new_res.dry_health_score,
            score_delta,
            previous_clones,
            new_clones: new_res.total_clones,
            clone_count_delta,
            previous_clusters,
            new_clusters: new_res.total_clusters,
            duration_ms,
            timestamp_millis,
        }
    }
}

/// Real-time file system change watcher built on `notify`.
pub struct CddmWatcher {
    _watcher: RecommendedWatcher,
    pub rx: Receiver<notify::Result<Event>>,
}

impl std::fmt::Debug for CddmWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CddmWatcher").finish_non_exhaustive()
    }
}

impl CddmWatcher {
    /// Creates a new directory watcher on `dir_path`.
    pub fn watch_directory<P: AsRef<Path>>(dir_path: P) -> Result<Self, String> {
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default(),
        )
        .map_err(|e| e.to_string())?;

        watcher
            .watch(dir_path.as_ref(), RecursiveMode::Recursive)
            .map_err(|e| e.to_string())?;

        Ok(Self {
            _watcher: watcher,
            rx,
        })
    }

    /// Determines if a file path is a relevant source file that warrants a re-scan.
    pub fn is_relevant_path(path: &Path, ignore_patterns: &[String]) -> bool {
        let path_str = path.to_string_lossy();

        for comp in path.components() {
            if let Some(comp_str) = comp.as_os_str().to_str()
                && DEFAULT_WATCH_IGNORES.contains(&comp_str)
            {
                return false;
            }
        }

        for pat in ignore_patterns {
            if !pat.is_empty() && path_str.contains(pat) {
                return false;
            }
        }

        get_grammar_for_path(path).is_some()
    }

    /// Drains available watcher events and returns unique, relevant file paths.
    pub fn collect_changed_paths(&self, ignore_patterns: &[String]) -> Vec<PathBuf> {
        let mut changed = HashSet::new();

        while let Ok(res) = self.rx.try_recv() {
            if let Ok(event) = res {
                for path in event.paths {
                    if Self::is_relevant_path(&path, ignore_patterns) {
                        changed.insert(path);
                    }
                }
            }
        }

        changed.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watcher_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let watcher = CddmWatcher::watch_directory(temp_dir.path());
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_is_relevant_path_filtering() {
        let rust_src = Path::new("src/main.rs");
        assert!(CddmWatcher::is_relevant_path(rust_src, &[]));

        let ts_src = Path::new("webui/src/App.tsx");
        assert!(CddmWatcher::is_relevant_path(ts_src, &[]));

        let node_modules = Path::new("node_modules/react/index.js");
        assert!(!CddmWatcher::is_relevant_path(node_modules, &[]));

        let target_build = Path::new("target/debug/build/something.rs");
        assert!(!CddmWatcher::is_relevant_path(target_build, &[]));

        let ignored_file = Path::new("src/generated/types.ts");
        assert!(!CddmWatcher::is_relevant_path(
            ignored_file,
            &["generated".to_string()]
        ));

        let non_source = Path::new("README.md");
        assert!(!CddmWatcher::is_relevant_path(non_source, &[]));
    }

    #[test]
    fn test_watch_delta_report_computation() {
        let res1 = ScanResult {
            dry_health_score: 90.0,
            total_clones: 5,
            total_clusters: 2,
            ..Default::default()
        };

        let res2 = ScanResult {
            dry_health_score: 95.0,
            total_clones: 3,
            total_clusters: 1,
            ..Default::default()
        };

        let changed_paths = vec![PathBuf::from("src/lib.rs")];
        let delta = WatchDeltaReport::compute(Some(&res1), &res2, &changed_paths, 42);

        assert_eq!(delta.changed_files, vec!["src/lib.rs".to_string()]);
        assert_eq!(delta.previous_health_score, 90.0);
        assert_eq!(delta.new_health_score, 95.0);
        assert_eq!(delta.score_delta, 5.0);
        assert_eq!(delta.clone_count_delta, -2);
        assert_eq!(delta.previous_clones, 5);
        assert_eq!(delta.new_clones, 3);
        assert_eq!(delta.duration_ms, 42);
    }
}
