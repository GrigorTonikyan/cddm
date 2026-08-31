use crate::fingerprint::Fingerprint;
use crate::types::{LineSpan, ScanResult};
use redb::{Database, ReadableDatabase, TableDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Current schema version for cache database compatibility.
pub const CACHE_SCHEMA_VERSION: u32 = 1;

/// Table name identifier for persistent file fingerprints.
pub const TABLE_NAME_FINGERPRINTS: &str = "fingerprints_v1";

/// redb table definition mapping relative file path to serialized cache payload.
pub const TABLE_FINGERPRINTS: TableDefinition<&str, &[u8]> =
    TableDefinition::new(TABLE_NAME_FINGERPRINTS);

/// Cached payload storing precomputed tokenization, spans, and winnowed fingerprints.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CachedFileEntry {
    /// Schema format version
    pub schema_version: u32,
    /// Blake3 content hash
    pub content_hash: String,
    /// Modification timestamp in seconds since UNIX epoch
    pub mtime_secs: i64,
    /// Size of the file in bytes
    pub file_size: u64,
    /// Detected language name
    pub language: String,
    /// Total token count
    pub token_count: usize,
    /// Line spans for parsed tokens
    pub token_spans: Vec<LineSpan>,
    /// Precomputed winnowed fingerprints
    pub fingerprints: Vec<Fingerprint>,
}

/// In-Memory and File Fingerprint Cache for incremental scans.
#[derive(Default, Debug, Clone)]
pub struct FingerprintCache {
    /// File path -> Content hash
    file_hashes: HashMap<String, String>,
    /// Last scan results cache
    pub last_result: Option<ScanResult>,
}

impl FingerprintCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes Blake3 content hash of file.
    pub fn compute_file_hash(path: &Path) -> Option<String> {
        let content = fs::read(path).ok()?;
        Some(blake3::hash(&content).to_hex().to_string())
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

pub mod pack;

/// Persistent, disk-backed ACID cache for file tokens and fingerprints powered by `redb`.
#[derive(Debug, Clone)]
pub struct DiskFingerprintCache {
    pub(crate) db: Option<Arc<Database>>,
    cache_path: PathBuf,
}

impl DiskFingerprintCache {
    /// Opens or creates a persistent `redb` database at the specified path.
    ///
    /// If the database file is corrupted or cannot be opened, it automatically self-heals
    /// by creating a fresh instance.
    pub fn open_or_create(db_path: &Path) -> Result<Self, String> {
        if let Some(parent) = db_path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create cache directory: {}", e))?;
        }

        let db = match Database::create(db_path) {
            Ok(database) => database,
            Err(_) => {
                // Attempt to open existing if create failed (e.g. file exists)
                match Database::open(db_path) {
                    Ok(database) => database,
                    Err(err) => {
                        // Auto-healing recovery: remove corrupted DB and recreate
                        tracing::warn!(
                            "Cache database at '{}' is corrupt or incompatible ({}); recreating...",
                            db_path.display(),
                            err
                        );
                        let _ = fs::remove_file(db_path);
                        Database::create(db_path).map_err(|e| {
                            format!("Failed to recreate cache database after corruption: {}", e)
                        })?
                    }
                }
            }
        };

        // Initialize table if not present
        {
            let write_txn = db
                .begin_write()
                .map_err(|e| format!("Failed to begin write transaction: {}", e))?;
            {
                let _ = write_txn
                    .open_table(TABLE_FINGERPRINTS)
                    .map_err(|e| format!("Failed to open table: {}", e))?;
            }
            write_txn
                .commit()
                .map_err(|e| format!("Failed to commit table creation: {}", e))?;
        }

        Ok(Self {
            db: Some(Arc::new(db)),
            cache_path: db_path.to_path_buf(),
        })
    }

    /// Returns a disabled / no-op disk cache.
    pub fn disabled() -> Self {
        Self {
            db: None,
            cache_path: PathBuf::new(),
        }
    }

    /// Returns true if the persistent cache is active.
    pub fn is_enabled(&self) -> bool {
        self.db.is_some()
    }

    /// Retrieves a cached entry by relative file path.
    pub fn get_entry(&self, relative_path: &str) -> Option<CachedFileEntry> {
        let db = self.db.as_ref()?;
        let read_txn = db.begin_read().ok()?;
        let table = read_txn.open_table(TABLE_FINGERPRINTS).ok()?;
        let guard = table.get(relative_path).ok()??;
        let bytes = guard.value();
        let entry: CachedFileEntry = serde_json::from_slice(bytes).ok()?;

        if entry.schema_version == CACHE_SCHEMA_VERSION {
            Some(entry)
        } else {
            None
        }
    }

    /// Validates file against cached metadata (fast mtime and size check).
    pub fn is_file_valid(
        &self,
        relative_path: &str,
        mtime_secs: i64,
        file_size: u64,
    ) -> Option<CachedFileEntry> {
        let entry = self.get_entry(relative_path)?;
        if entry.mtime_secs == mtime_secs && entry.file_size == file_size {
            Some(entry)
        } else {
            None
        }
    }

    fn execute_write_transaction<F>(&self, op: F) -> Result<usize, String>
    where
        F: FnOnce(&mut redb::Table<&str, &[u8]>) -> usize,
    {
        let db = match &self.db {
            Some(database) => database,
            None => return Ok(0),
        };

        let write_txn = db
            .begin_write()
            .map_err(|e| format!("Failed to begin write transaction: {}", e))?;

        let count = {
            let mut table = write_txn
                .open_table(TABLE_FINGERPRINTS)
                .map_err(|e| format!("Failed to open table: {}", e))?;
            op(&mut table)
        };

        write_txn
            .commit()
            .map_err(|e| format!("Failed to commit cache transaction: {}", e))?;

        Ok(count)
    }

    /// Persists a batch of cached file entries in a single atomic ACID transaction.
    pub fn batch_save_entries(
        &self,
        entries: &[(String, CachedFileEntry)],
    ) -> Result<usize, String> {
        if entries.is_empty() {
            return Ok(0);
        }

        self.execute_write_transaction(|table| {
            let mut saved_count = 0;
            for (path, entry) in entries {
                if let Ok(serialized) = serde_json::to_vec(entry)
                    && table.insert(path.as_str(), serialized.as_slice()).is_ok()
                {
                    saved_count += 1;
                }
            }
            saved_count
        })
    }

    /// Removes deleted files from the persistent cache in a single write transaction.
    pub fn remove_entries(&self, paths: &[String]) -> Result<usize, String> {
        if paths.is_empty() {
            return Ok(0);
        }

        self.execute_write_transaction(|table| {
            let mut removed_count = 0;
            for path in paths {
                if table.remove(path.as_str()).is_ok() {
                    removed_count += 1;
                }
            }
            removed_count
        })
    }

    /// Clears all entries from the persistent cache database.
    pub fn clear(&self) -> Result<(), String> {
        let db = match &self.db {
            Some(database) => database,
            None => return Ok(()),
        };

        let write_txn = db
            .begin_write()
            .map_err(|e| format!("Failed to begin write transaction: {}", e))?;

        {
            let mut table = write_txn
                .open_table(TABLE_FINGERPRINTS)
                .map_err(|e| format!("Failed to open table: {}", e))?;

            // redb does not have table.clear(), so we retain until empty
            table
                .retain(|_, _| false)
                .map_err(|e| format!("Failed to clear table: {}", e))?;
        }

        write_txn
            .commit()
            .map_err(|e| format!("Failed to commit table clear: {}", e))?;

        Ok(())
    }

    /// Returns the filesystem path to the underlying cache database file.
    pub fn cache_path(&self) -> &Path {
        &self.cache_path
    }
}

/// Traverses directory ancestors from `start_path` to find the root of the workspace.
///
/// Discovers repository markers (`.git`), workspace descriptors (`Cargo.toml` with `[workspace]`,
/// `pnpm-workspace.yaml`, `.cddm`), or falls back to `start_path` or current working directory.
pub fn find_workspace_root(start_path: &Path) -> PathBuf {
    let canonical = if start_path.is_relative() {
        std::env::current_dir()
            .map(|cwd| cwd.join(start_path))
            .unwrap_or_else(|_| start_path.to_path_buf())
    } else {
        start_path.to_path_buf()
    };

    let mut current = if canonical.is_file() {
        canonical.parent().map(|p| p.to_path_buf())
    } else {
        Some(canonical)
    };

    let mut best_root = None;

    while let Some(dir) = current {
        if dir.join(".git").exists() {
            return dir;
        }
        if dir.join(".cddm").exists() {
            best_root = Some(dir.clone());
        }
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.exists()
            && let Ok(content) = fs::read_to_string(&cargo_toml)
            && content.contains("[workspace]")
        {
            return dir;
        }
        if dir.join("pnpm-workspace.yaml").exists() {
            return dir;
        }
        current = dir.parent().map(|p| p.to_path_buf());
    }

    best_root.unwrap_or_else(|| {
        if start_path.as_os_str().is_empty() || start_path == Path::new(".") {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        } else {
            start_path.to_path_buf()
        }
    })
}

/// Resolves the canonical cache database path (`.cddm/cache.db`) anchored at the workspace root.
pub fn resolve_default_cache_path(target_path: &Path) -> PathBuf {
    let root = find_workspace_root(target_path);
    root.join(crate::types::DEFAULT_CACHE_FILE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

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
    fn test_disk_cache_lifecycle() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("cache.redb");

        let cache = DiskFingerprintCache::open_or_create(&db_path).unwrap();
        assert!(cache.is_enabled());

        let entry = CachedFileEntry {
            schema_version: CACHE_SCHEMA_VERSION,
            content_hash: "abcd1234".to_string(),
            mtime_secs: 1700000000,
            file_size: 1024,
            language: "Rust".to_string(),
            token_count: 50,
            token_spans: vec![LineSpan {
                line_start: 1,
                line_end: 5,
                byte_offset: 0,
            }],
            fingerprints: vec![Fingerprint {
                hash: (42, 42),
                span: LineSpan {
                    line_start: 1,
                    line_end: 5,
                    byte_offset: 0,
                },
            }],
        };

        // Batch save
        let saved = cache
            .batch_save_entries(&[("src/main.rs".to_string(), entry.clone())])
            .unwrap();
        assert_eq!(saved, 1);

        // Fetch
        let fetched = cache.get_entry("src/main.rs").unwrap();
        assert_eq!(fetched, entry);

        // Fast metadata check
        let valid = cache.is_file_valid("src/main.rs", 1700000000, 1024);
        assert_eq!(valid, Some(entry.clone()));

        let invalid_mtime = cache.is_file_valid("src/main.rs", 1700000001, 1024);
        assert!(invalid_mtime.is_none());

        let invalid_size = cache.is_file_valid("src/main.rs", 1700000000, 2048);
        assert!(invalid_size.is_none());

        // Remove
        let removed = cache.remove_entries(&["src/main.rs".to_string()]).unwrap();
        assert_eq!(removed, 1);
        assert!(cache.get_entry("src/main.rs").is_none());

        // Re-insert and clear
        cache
            .batch_save_entries(&[("src/lib.rs".to_string(), entry)])
            .unwrap();
        assert!(cache.get_entry("src/lib.rs").is_some());
        cache.clear().unwrap();
        assert!(cache.get_entry("src/lib.rs").is_none());
    }

    #[test]
    fn test_disk_cache_disabled() {
        let cache = DiskFingerprintCache::disabled();
        assert!(!cache.is_enabled());
        assert!(cache.get_entry("any.rs").is_none());
        assert_eq!(cache.batch_save_entries(&[]).unwrap(), 0);
        assert_eq!(cache.remove_entries(&[]).unwrap(), 0);
        assert!(cache.clear().is_ok());
    }

    #[test]
    fn test_disk_cache_auto_healing_corrupted_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("corrupted.redb");

        // Write garbage bytes
        fs::write(&db_path, b"not a valid redb file header garbage").unwrap();

        // open_or_create should self-heal and succeed
        let cache = DiskFingerprintCache::open_or_create(&db_path).unwrap();
        assert!(cache.is_enabled());
    }

    #[test]
    fn test_find_workspace_root_git() {
        let temp_dir = tempfile::tempdir().unwrap();
        let sub = temp_dir.path().join("a").join("b").join("c");
        fs::create_dir_all(&sub).unwrap();
        fs::create_dir_all(temp_dir.path().join(".git")).unwrap();

        let root = find_workspace_root(&sub);
        assert_eq!(root, temp_dir.path());
        let cache_path = resolve_default_cache_path(&sub);
        assert_eq!(
            cache_path,
            temp_dir.path().join(crate::types::DEFAULT_CACHE_FILE)
        );
    }

    #[test]
    fn test_find_workspace_root_fallback() {
        let temp_dir = tempfile::tempdir().unwrap();
        let sub = temp_dir.path().join("custom");
        fs::create_dir_all(&sub).unwrap();

        let root = find_workspace_root(&sub);
        assert_eq!(root, sub);
    }
}
