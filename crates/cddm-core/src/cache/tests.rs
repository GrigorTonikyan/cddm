#![forbid(unsafe_code)]

use super::path::{
    find_workspace_root, resolve_cache_path, resolve_default_cache_path, resolve_user_cache_dir,
};
use super::*;
use std::fs;
use std::io::Write;
use std::path::Path;
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
}

#[test]
fn test_find_workspace_root_fallback() {
    let temp_dir = tempfile::tempdir().unwrap();
    let sub = temp_dir.path().join("custom");
    fs::create_dir_all(&sub).unwrap();

    let root = find_workspace_root(&sub);
    assert_eq!(root, sub);
}

#[test]
fn test_resolve_user_cache_dir_not_empty() {
    let user_cache = resolve_user_cache_dir();
    assert!(!user_cache.as_os_str().is_empty());
}

#[test]
fn test_resolve_default_cache_path_does_not_dirty_workspace() {
    let temp_dir = tempfile::tempdir().unwrap();
    let sub = temp_dir.path().join("my-project");
    fs::create_dir_all(&sub).unwrap();
    fs::create_dir_all(sub.join(".git")).unwrap();

    // Default resolution should NOT return in-tree .cddm/cache.db if it doesn't already exist
    let default_path = resolve_default_cache_path(&sub);
    assert_ne!(default_path, sub.join(crate::types::DEFAULT_CACHE_FILE));
    assert!(default_path.to_string_lossy().ends_with(".db"));

    // In-tree resolution explicitly requested should return in-tree .cddm/cache.db
    let in_tree_path = resolve_cache_path(&sub, true);
    assert_eq!(in_tree_path, sub.join(crate::types::DEFAULT_CACHE_FILE));

    // If in-tree .cddm/cache.db ALREADY exists, default resolution should use it for backwards compatibility
    let cddm_dir = sub.join(".cddm");
    fs::create_dir_all(&cddm_dir).unwrap();
    let existing_db = cddm_dir.join("cache.db");
    fs::write(&existing_db, b"test").unwrap();

    let backwards_compat_path = resolve_default_cache_path(&sub);
    assert_eq!(backwards_compat_path, existing_db);
}
