use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{Receiver, channel};

/// Real-time file system change watcher built on `notify`.
pub struct CddmWatcher {
    _watcher: RecommendedWatcher,
    pub rx: Receiver<notify::Result<Event>>,
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
}
