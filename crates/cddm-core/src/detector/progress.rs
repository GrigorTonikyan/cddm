#![forbid(unsafe_code)]

use crate::types::ScanPhase;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize};

/// Tracks granular progress and execution state across all detection phases.
pub(crate) struct ProgressTracker {
    pub(crate) scan_id: String,
    pub(crate) phase: RwLock<ScanPhase>,
    pub(crate) files_processed: AtomicUsize,
    pub(crate) total_files: AtomicUsize,
    pub(crate) progress_scaled: AtomicU64,
    pub(crate) message: RwLock<String>,
    pub(crate) done: AtomicBool,
}

impl ProgressTracker {
    pub(crate) fn new(scan_id: String) -> Self {
        Self {
            scan_id,
            phase: RwLock::new(ScanPhase::Discovery),
            files_processed: AtomicUsize::new(0),
            total_files: AtomicUsize::new(0),
            progress_scaled: AtomicU64::new(0),
            message: RwLock::new("Discovering files...".to_string()),
            done: AtomicBool::new(false),
        }
    }
}

/// Executes a closure within a dedicated Rayon thread pool if specified, or current thread pool.
pub(crate) fn execute_in_thread_pool<F, R>(threads: Option<usize>, f: F) -> R
where
    F: FnOnce() -> R + Send,
    R: Send,
{
    if let Some(num_threads) = threads.filter(|&n| n > 0)
        && let Ok(pool) = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
    {
        return pool.install(f);
    }
    f()
}
