use crate::grammar::get_grammar_for_path;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
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
}
