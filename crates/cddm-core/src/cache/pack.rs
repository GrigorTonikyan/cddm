#![forbid(unsafe_code)]

use crate::cache::{CachedFileEntry, DiskFingerprintCache, TABLE_FINGERPRINTS};
use redb::{ReadableDatabase, ReadableTable};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

/// Schema format version for cache pack archive files.
pub const CACHE_PACK_VERSION: u32 = 1;

/// Manifest header describing the contents of an exported .cddmpack archive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CachePackManifest {
    pub pack_version: u32,
    pub cddm_version: String,
    pub created_at: String,
    pub entry_count: usize,
    pub payload_checksum_sha256: String,
}

/// Detailed summary of a cache pack export or import operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CachePackSummary {
    pub success: bool,
    pub pack_file: String,
    pub entry_count: usize,
    pub checksum: String,
    pub message: String,
}

/// Archive container wrapping manifest and raw file entry payloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachePackContainer {
    manifest: CachePackManifest,
    entries: Vec<(String, CachedFileEntry)>,
}

/// Exports an active persistent cache database into a portable `.cddmpack` archive.
pub fn export_cache_pack(
    cache_db_path: &Path,
    output_pack_path: &Path,
) -> Result<CachePackSummary, String> {
    if !cache_db_path.exists() {
        return Err(format!(
            "Cache database not found at '{}'",
            cache_db_path.display()
        ));
    }

    let cache = DiskFingerprintCache::open_or_create(cache_db_path)?;
    let mut entries = Vec::new();

    if let Some(db) = &cache.db {
        let read_txn = db
            .begin_read()
            .map_err(|e| format!("Failed to read cache: {}", e))?;
        let table = read_txn
            .open_table(TABLE_FINGERPRINTS)
            .map_err(|e| format!("Failed to open table: {}", e))?;

        let iter = table
            .iter()
            .map_err(|e| format!("Failed to iterate cache entries: {}", e))?;

        for (key_guard, val_guard) in iter.flatten() {
            let key = key_guard.value().to_string();
            let bytes = val_guard.value();
            if let Ok(entry) = serde_json::from_slice::<CachedFileEntry>(bytes) {
                entries.push((key, entry));
            }
        }
    }

    let entries_json = serde_json::to_vec(&entries)
        .map_err(|e| format!("Failed to serialize cache entries: {}", e))?;

    let mut hasher = Sha256::new();
    hasher.update(&entries_json);
    let checksum: String = hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();

    let manifest = CachePackManifest {
        pack_version: CACHE_PACK_VERSION,
        cddm_version: env!("CARGO_PKG_VERSION").to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        entry_count: entries.len(),
        payload_checksum_sha256: checksum.clone(),
    };

    let container = CachePackContainer { manifest, entries };

    if let Some(parent) = output_pack_path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create export pack directory: {}", e))?;
    }

    let pack_bytes = serde_json::to_vec_pretty(&container)
        .map_err(|e| format!("Failed to serialize cache pack container: {}", e))?;

    fs::write(output_pack_path, pack_bytes)
        .map_err(|e| format!("Failed to write cache pack file: {}", e))?;

    Ok(CachePackSummary {
        success: true,
        pack_file: output_pack_path.display().to_string(),
        entry_count: container.entries.len(),
        checksum,
        message: format!(
            "Exported {} cache entries to '{}'",
            container.entries.len(),
            output_pack_path.display()
        ),
    })
}

/// Imports a portable `.cddmpack` archive into the target persistent cache database.
pub fn import_cache_pack(
    pack_path: &Path,
    target_cache_dir: &Path,
) -> Result<CachePackSummary, String> {
    if !pack_path.exists() {
        return Err(format!("Cache pack not found at '{}'", pack_path.display()));
    }

    let pack_bytes =
        fs::read(pack_path).map_err(|e| format!("Failed to read cache pack file: {}", e))?;

    let container: CachePackContainer = serde_json::from_slice(&pack_bytes)
        .map_err(|e| format!("Corrupted or incompatible cache pack file: {}", e))?;

    let entries_json = serde_json::to_vec(&container.entries)
        .map_err(|e| format!("Failed to serialize entries for verification: {}", e))?;

    let mut hasher = Sha256::new();
    hasher.update(&entries_json);
    let computed_checksum: String = hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();

    if computed_checksum != container.manifest.payload_checksum_sha256 {
        return Err(format!(
            "Checksum mismatch! Pack checksum: {}, computed: {}",
            container.manifest.payload_checksum_sha256, computed_checksum
        ));
    }

    let db_path = target_cache_dir.join("cache.db");
    let cache = DiskFingerprintCache::open_or_create(&db_path)?;
    let saved = cache.batch_save_entries(&container.entries)?;

    Ok(CachePackSummary {
        success: true,
        pack_file: pack_path.display().to_string(),
        entry_count: saved,
        checksum: computed_checksum,
        message: format!(
            "Successfully imported {} entries from '{}' into cache",
            saved,
            pack_path.display()
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::CachedFileEntry;
    use tempfile::tempdir;

    #[test]
    fn test_export_and_import_cache_pack() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("source_cache.redb");
        let pack_path = dir.path().join("export.cddmpack");
        let import_dir = dir.path().join("imported");

        let cache = DiskFingerprintCache::open_or_create(&db_path).unwrap();
        let entry = CachedFileEntry {
            schema_version: 1,
            content_hash: "hash_abc".to_string(),
            mtime_secs: 1700000000,
            file_size: 512,
            language: "Rust".to_string(),
            token_count: 42,
            token_spans: vec![],
            fingerprints: vec![],
        };

        cache
            .batch_save_entries(&[("src/main.rs".to_string(), entry.clone())])
            .unwrap();
        drop(cache);

        let export_res = export_cache_pack(&db_path, &pack_path).unwrap();
        assert!(export_res.success);
        assert_eq!(export_res.entry_count, 1);

        let import_res = import_cache_pack(&pack_path, &import_dir).unwrap();
        assert!(import_res.success);
        assert_eq!(import_res.entry_count, 1);

        let imported_cache =
            DiskFingerprintCache::open_or_create(&import_dir.join("cache.db")).unwrap();
        let fetched = imported_cache.get_entry("src/main.rs").unwrap();
        assert_eq!(fetched.content_hash, "hash_abc");
    }
}
