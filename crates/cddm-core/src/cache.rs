use crate::types::ScanResult;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// In-Memory and File Fingerprint Cache for incremental scans.
#[derive(Default, Debug, Clone)]
pub struct FingerprintCache {
    /// File path -> Sha256 content hash
    file_hashes: HashMap<String, String>,
    /// Last scan results cache
    pub last_result: Option<ScanResult>,
}

impl FingerprintCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes Sha256 content hash of file.
    pub fn compute_file_hash(path: &Path) -> Option<String> {
        let content = fs::read(path).ok()?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        Some(format!("{:x}", hasher.finalize()))
    }

    /// Checks if file has been modified since last scan.
    pub fn is_file_modified(&self, path_str: &str, current_hash: &str) -> bool {
        match self.file_hashes.get(path_str) {
            Some(prev_hash) => prev_hash != current_hash,
            None => true,
        }
    }

    /// Updates cache entry for file.
    pub fn update_file(&mut self, path_str: String, hash: String) {
        self.file_hashes.insert(path_str, hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_cache() {
        let mut cache = FingerprintCache::new();
        let path = "src/lib.rs";
        let hash1 = "hash_v1";
        let hash2 = "hash_v2";

        assert!(cache.is_file_modified(path, hash1));
        cache.update_file(path.to_string(), hash1.to_string());
        assert!(!cache.is_file_modified(path, hash1));
        assert!(cache.is_file_modified(path, hash2));
    }
}
