#![forbid(unsafe_code)]

use super::scan_handlers::resolve_safe_path;
use super::types::*;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use cddm_core::grammar::get_grammar_for_path;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

/// Resolves a file path securely for writing/mutation, ensuring workspace root containment.
pub fn resolve_safe_write_path(file_str: &str) -> Result<PathBuf, (StatusCode, String)> {
    if file_str.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "File path cannot be empty".to_string(),
        ));
    }
    if file_str.contains('\0') {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid characters in file path".to_string(),
        ));
    }

    let workspace_root = std::env::current_dir().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to determine workspace root: {}", e),
        )
    })?;
    let canonical_root = workspace_root.canonicalize().unwrap_or(workspace_root);

    let requested = Path::new(file_str);
    let target = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        canonical_root.join(requested)
    };

    if target.exists() {
        let canonical = target.canonicalize().map_err(|e| {
            (
                StatusCode::NOT_FOUND,
                format!("Failed to resolve file '{}': {}", file_str, e),
            )
        })?;

        if !canonical.starts_with(&canonical_root) {
            return Err((
                StatusCode::FORBIDDEN,
                format!(
                    "Access denied: Path '{}' traverses outside the workspace root",
                    file_str
                ),
            ));
        }

        if !canonical.is_file() {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Path '{}' is not a regular file", file_str),
            ));
        }

        Ok(canonical)
    } else {
        let parent = target.parent().ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                format!("Path '{}' has no valid parent directory", file_str),
            )
        })?;

        let canonical_parent = parent.canonicalize().map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!(
                    "Parent directory for '{}' does not exist or is invalid: {}",
                    file_str, e
                ),
            )
        })?;

        if !canonical_parent.starts_with(&canonical_root) {
            return Err((
                StatusCode::FORBIDDEN,
                format!(
                    "Access denied: Parent directory of '{}' is outside the workspace root",
                    file_str
                ),
            ));
        }

        let file_name = target.file_name().ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                format!("Path '{}' has no valid file name", file_str),
            )
        })?;

        Ok(canonical_parent.join(file_name))
    }
}

/// Helper to format a display path relative to the workspace root when possible.
pub fn get_relative_display_path(path: &Path) -> String {
    let rel_opt = std::env::current_dir().ok().and_then(|root| {
        root.canonicalize()
            .ok()
            .and_then(|c| path.strip_prefix(c).ok().map(|p| p.to_path_buf()))
            .or_else(|| path.strip_prefix(&root).ok().map(|p| p.to_path_buf()))
    });
    if let Some(rel) = rel_opt {
        return rel.to_string_lossy().replace('\\', "/");
    }
    path.to_string_lossy().replace('\\', "/")
}

/// Handler for reading workspace files securely into the WebUI code editor.
pub async fn file_read_handler(
    Query(query): Query<FileReadQuery>,
) -> Result<Json<FileReadResponse>, (StatusCode, String)> {
    let canonical = resolve_safe_path(&query.path)?;
    let content = fs::read_to_string(&canonical).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read file '{}': {}", query.path, e),
        )
    })?;

    let modified_timestamp_ms = read_modified_timestamp(&canonical, &query.path)?;

    let language = get_grammar_for_path(&canonical)
        .map(|g| g.name.to_string())
        .unwrap_or_else(|| "text".to_string());

    let total_lines = content.lines().count();
    let size_bytes = content.len();
    let display_path = get_relative_display_path(&canonical);

    Ok(Json(FileReadResponse {
        path: display_path,
        content,
        language,
        size_bytes,
        total_lines,
        modified_timestamp_ms,
    }))
}

/// Handler for saving/mutating workspace files from the WebUI code editor.
pub async fn file_save_handler(
    State(state): State<AppState>,
    Json(req): Json<FileWriteRequest>,
) -> Result<Json<FileWriteResponse>, (StatusCode, String)> {
    let canonical = resolve_safe_write_path(&req.path)?;

    fs::write(&canonical, &req.content).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to write file '{}': {}", req.path, e),
        )
    })?;

    let modified_timestamp_ms = read_modified_timestamp(&canonical, &req.path)?;

    let display_path = get_relative_display_path(&canonical);

    let _ = state.broadcast_tx.send(ServerEvent::FileSaved {
        path: display_path.clone(),
        timestamp: modified_timestamp_ms,
    });

    Ok(Json(FileWriteResponse {
        path: display_path,
        bytes_written: req.content.len(),
        success: true,
        modified_timestamp_ms,
    }))
}

fn read_modified_timestamp(
    canonical: &Path,
    display_path: &str,
) -> Result<u64, (StatusCode, String)> {
    let meta = fs::metadata(canonical).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read metadata for '{}': {}", display_path, e),
        )
    })?;

    Ok(meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0))
}

/// Recursively scans workspace directories for editable files, ignoring build artifacts.
fn scan_directory_for_files(current: &Path, files: &mut Vec<WorkspaceFileEntry>, depth: usize) {
    if depth > 10 || files.len() >= 2000 {
        return;
    }

    let entries = match fs::read_dir(current) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden and build artifact folders
        if name.starts_with('.')
            || name == "target"
            || name == "node_modules"
            || name == "dist"
            || name == "build"
            || name == "out"
            || name == "coverage"
            || name == "brain"
            || name == "packaging"
        {
            continue;
        }

        if let Ok(ft) = entry.file_type() {
            if ft.is_dir() {
                scan_directory_for_files(&path, files, depth + 1);
            } else if ft.is_file() {
                let size_bytes = entry.metadata().map(|m| m.len()).unwrap_or(0);
                let language = get_grammar_for_path(&path).map(|g| g.name.to_string());
                let rel_path = get_relative_display_path(&path);

                files.push(WorkspaceFileEntry {
                    path: rel_path,
                    name,
                    is_dir: false,
                    size_bytes,
                    language,
                });
            }
        }
    }
}

/// Handler for listing workspace files for the WebUI code editor quick open.
pub async fn file_tree_handler() -> Result<Json<FileTreeResponse>, (StatusCode, String)> {
    let workspace_root = std::env::current_dir().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to determine workspace root: {}", e),
        )
    })?;
    let canonical_root = workspace_root.canonicalize().unwrap_or(workspace_root);

    let mut files = Vec::new();
    scan_directory_for_files(&canonical_root, &mut files, 0);
    files.sort_by(|a, b| a.path.cmp(&b.path));

    let root_display = canonical_root.to_string_lossy().replace('\\', "/");

    Ok(Json(FileTreeResponse {
        root: root_display,
        files,
    }))
}
