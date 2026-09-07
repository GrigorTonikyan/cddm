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
pub mod path;

#[cfg(test)]
mod tests;

pub use path::{
    find_workspace_root, resolve_cache_path, resolve_default_cache_path, resolve_user_cache_dir,
};

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
