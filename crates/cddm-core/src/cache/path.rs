#![forbid(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};

/// Resolves the platform-standard OS user cache directory for CDDM.
///
/// Hierarchy:
/// 1. `CDDM_CACHE_DIR` environment variable (if set and non-empty).
/// 2. Windows: `%LOCALAPPDATA%\cddm\cache` or `%APPDATA%\cddm\cache`.
/// 3. macOS: `$HOME/Library/Caches/cddm`.
/// 4. Linux / Unix: `$XDG_CACHE_HOME/cddm` or `$HOME/.cache/cddm`.
/// 5. Fallback: OS temporary directory (`std::env::temp_dir().join("cddm_cache")`).
pub fn resolve_user_cache_dir() -> PathBuf {
    if let Ok(cddm_cache) = std::env::var("CDDM_CACHE_DIR") {
        let trimmed = cddm_cache.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let trimmed = local_app_data.trim();
            if !trimmed.is_empty() {
                return PathBuf::from(trimmed).join("cddm").join("cache");
            }
        }
        if let Ok(app_data) = std::env::var("APPDATA") {
            let trimmed = app_data.trim();
            if !trimmed.is_empty() {
                return PathBuf::from(trimmed).join("cddm").join("cache");
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            let trimmed = home.trim();
            if !trimmed.is_empty() {
                return PathBuf::from(trimmed)
                    .join("Library")
                    .join("Caches")
                    .join("cddm");
            }
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Ok(xdg) = std::env::var("XDG_CACHE_HOME") {
            let trimmed = xdg.trim();
            if !trimmed.is_empty() {
                return PathBuf::from(trimmed).join("cddm");
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            let trimmed = home.trim();
            if !trimmed.is_empty() {
                return PathBuf::from(trimmed).join(".cache").join("cddm");
            }
        }
    }

    std::env::temp_dir().join("cddm_cache")
}

/// Discovers the nearest workspace root by traversing upwards looking for manifest anchors.
pub fn find_workspace_root(start_path: &Path) -> PathBuf {
    let mut current = if start_path.is_file() {
        start_path.parent().map(|p| p.to_path_buf())
    } else {
        Some(start_path.to_path_buf())
    };

    let mut best_root = None;

    while let Some(dir) = current {
        if dir.join(".git").exists()
            || dir.join(".cddmrules.toml").exists()
            || dir.join(".cddmignore").exists()
        {
            return dir;
        }

        if (dir.join("Cargo.toml").exists() || dir.join("package.json").exists())
            && best_root.is_none()
        {
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

/// Resolves the cache database path for a workspace.
///
/// Behavior:
/// 1. If `in_tree` is true, writes to `.cddm/cache.db` anchored at the workspace root.
/// 2. If `.cddm/cache.db` already exists on disk in the workspace, uses it for backwards compatibility.
/// 3. Otherwise, writes to the OS user cache directory (`resolve_user_cache_dir()`),
///    namespaced by the first 16 characters of the Blake3 hash of the canonical project path.
pub fn resolve_cache_path(target_path: &Path, in_tree: bool) -> PathBuf {
    let root = find_workspace_root(target_path);
    let in_tree_path = root.join(crate::types::DEFAULT_CACHE_FILE);

    if in_tree || in_tree_path.exists() {
        return in_tree_path;
    }

    let canonical = root.canonicalize().unwrap_or_else(|_| root.clone());
    let path_str = canonical.to_string_lossy();
    let hash = &blake3::hash(path_str.as_bytes()).to_hex()[..16];
    let user_cache = resolve_user_cache_dir();
    user_cache.join(format!("{hash}.db"))
}

/// Resolves the default cache database path without dirtying the VCS working tree.
pub fn resolve_default_cache_path(target_path: &Path) -> PathBuf {
    resolve_cache_path(target_path, false)
}
