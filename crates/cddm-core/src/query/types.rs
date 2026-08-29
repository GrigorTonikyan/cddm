#![forbid(unsafe_code)]

use crate::types::{LineSpan, NormalizedToken};
use serde::{Deserialize, Serialize};

/// 32-byte content hash representing a unique immutable version of a source file.
pub type ContentHash = [u8; 32];

/// Cached tokenization result for a specific file version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedTokenization {
    pub language: String,
    pub tokens: Vec<NormalizedToken>,
    pub spans: Vec<LineSpan>,
    pub content_hash: ContentHash,
}

/// Key used to look up cached computations in the incremental query engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QueryKey {
    pub file_path: String,
    pub content_hash: ContentHash,
}

/// Statistics on incremental query cache hit/miss efficiency.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryCacheStats {
    pub hits: usize,
    pub misses: usize,
    pub entries: usize,
}

impl QueryCacheStats {
    /// Returns the cache hit ratio as a percentage (0.0 to 100.0).
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            (self.hits as f64 / total as f64) * 100.0
        }
    }
}

/// Delta report summarizing incremental change detection across repository snapshots.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IncrementalDeltaReport {
    pub unmodified_files: usize,
    pub modified_files: usize,
    pub added_files: usize,
    pub removed_files: usize,
    pub short_circuited_count: usize,
}
