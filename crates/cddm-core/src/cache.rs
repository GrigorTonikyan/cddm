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

    #[test]
    fn test_compute_file_hash_real_file() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "hello world").unwrap();

        let hash = FingerprintCache::compute_file_hash(file.path()).unwrap();
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_compute_file_hash_nonexistent() {
        let path = Path::new("does_not_exist_abc123.txt");
        let hash = FingerprintCache::compute_file_hash(path);
        assert!(hash.is_none());
    }

    #[test]
    fn test_cache_multiple_files() {
        let mut cache = FingerprintCache::new();
        cache.update_file("f1".to_string(), "hash1".to_string());
        cache.update_file("f2".to_string(), "hash2".to_string());

        assert!(!cache.is_file_modified("f1", "hash1"));
        assert!(cache.is_file_modified("f1", "hash2"));
        assert!(!cache.is_file_modified("f2", "hash2"));
        assert!(cache.is_file_modified("f3", "hash1"));
    }
}
