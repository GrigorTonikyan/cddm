#![forbid(unsafe_code)]

use super::types::ExtractResult;
use std::fs;
use std::path::Path;

/// Applies the generated extraction files, manifest updates, and caller rewrites to the workspace filesystem.
pub fn apply_extraction_to_workspace(
    workspace_root: &Path,
    result: &ExtractResult,
    dry_run: bool,
) -> Result<usize, String> {
    if dry_run {
        return Ok(0);
    }

    let mut total_files_written = 0;

    // 1. Write newly generated target files (e.g. Cargo.toml, src/lib.rs, index.ts)
    for gen_file in &result.generated_files {
        let abs_path = workspace_root.join(&gen_file.file_path);
        if let Some(parent) = abs_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory '{}': {}", parent.display(), e))?;
        }
        fs::write(&abs_path, &gen_file.content)
            .map_err(|e| format!("Failed to write file '{}': {}", abs_path.display(), e))?;
        total_files_written += 1;
    }

    // 1b. Write synthesized unit test files
    for test_file in &result.test_files {
        let abs_path = workspace_root.join(&test_file.file_path);
        if let Some(parent) = abs_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory '{}': {}", parent.display(), e))?;
        }
        fs::write(&abs_path, &test_file.content)
            .map_err(|e| format!("Failed to write test file '{}': {}", abs_path.display(), e))?;
        total_files_written += 1;
    }

    // 1c. Write synthesized performance micro-benchmark files
    for bench_file in &result.benchmark_files {
        let abs_path = workspace_root.join(&bench_file.file_path);
        if let Some(parent) = abs_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory '{}': {}", parent.display(), e))?;
        }
        fs::write(&abs_path, &bench_file.content).map_err(|e| {
            format!(
                "Failed to write benchmark file '{}': {}",
                abs_path.display(),
                e
            )
        })?;
        total_files_written += 1;
    }

    // 2. Apply manifest updates
    for update in &result.manifest_updates {
        let abs_path = workspace_root.join(&update.manifest_path);
        if let Some(parent) = abs_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Failed to create manifest directory '{}': {}",
                    parent.display(),
                    e
                )
            })?;
        }
        fs::write(&abs_path, &update.updated_content)
            .map_err(|e| format!("Failed to update manifest '{}': {}", abs_path.display(), e))?;
        total_files_written += 1;
    }

    // 3. Apply caller file rewrites
    for rewrite in &result.caller_rewrites {
        let abs_path = workspace_root.join(&rewrite.file_path);
        fs::write(&abs_path, &rewrite.rewritten_content).map_err(|e| {
            format!(
                "Failed to rewrite caller file '{}': {}",
                abs_path.display(),
                e
            )
        })?;
        total_files_written += 1;
    }

    Ok(total_files_written)
}
