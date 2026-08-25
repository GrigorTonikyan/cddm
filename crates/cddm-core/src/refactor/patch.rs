#![forbid(unsafe_code)]

use super::types::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Parses a standard unified diff string into structured file patches and hunks.
pub fn parse_unified_patch(patch_content: &str) -> Result<Vec<ParsedFilePatch>, String> {
    let mut file_patches = Vec::new();
    let lines: Vec<&str> = patch_content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("--- ") {
            let raw_old = line.trim_start_matches("--- ").trim();
            let mut file_path = raw_old;
            if let Some(stripped) = file_path.strip_prefix("a/") {
                file_path = stripped;
            } else if let Some(stripped) = file_path.strip_prefix("a\\") {
                file_path = stripped;
            }

            i += 1;
            if i < lines.len() && lines[i].starts_with("+++ ") {
                let raw_new = lines[i].trim_start_matches("+++ ").trim();
                let mut target_path = raw_new;
                if let Some(stripped) = target_path.strip_prefix("b/") {
                    target_path = stripped;
                } else if let Some(stripped) = target_path.strip_prefix("b\\") {
                    target_path = stripped;
                }
                if !target_path.is_empty() && target_path != "/dev/null" {
                    file_path = target_path;
                }
                i += 1;
            }

            let mut hunks = Vec::new();
            while i < lines.len() && !lines[i].starts_with("--- ") {
                let hunk_line = lines[i];
                if hunk_line.starts_with("@@ ") {
                    let parts: Vec<&str> = hunk_line.split("@@").collect();
                    if parts.len() >= 3 {
                        let header = parts[1].trim();
                        let ranges: Vec<&str> = header.split_whitespace().collect();
                        let mut old_start = 1;
                        let mut old_count = 1;
                        let mut new_start = 1;
                        let mut new_count = 1;

                        for r in ranges {
                            if let Some(old_r) = r.strip_prefix('-') {
                                let nums: Vec<&str> = old_r.split(',').collect();
                                old_start = nums[0].parse().unwrap_or(1);
                                if nums.len() > 1 {
                                    old_count = nums[1].parse().unwrap_or(1);
                                }
                            } else if let Some(new_r) = r.strip_prefix('+') {
                                let nums: Vec<&str> = new_r.split(',').collect();
                                new_start = nums[0].parse().unwrap_or(1);
                                if nums.len() > 1 {
                                    new_count = nums[1].parse().unwrap_or(1);
                                }
                            }
                        }

                        i += 1;
                        let mut deleted_lines = Vec::new();
                        let mut added_lines = Vec::new();

                        while i < lines.len()
                            && !lines[i].starts_with("@@ ")
                            && !lines[i].starts_with("--- ")
                        {
                            let l = lines[i];
                            if let Some(del) = l.strip_prefix('-') {
                                deleted_lines.push(del.to_string());
                            } else if let Some(add) = l.strip_prefix('+') {
                                added_lines.push(add.to_string());
                            } else if let Some(ctx) = l.strip_prefix(' ') {
                                deleted_lines.push(ctx.to_string());
                                added_lines.push(ctx.to_string());
                            }
                            i += 1;
                        }

                        hunks.push(ParsedHunk {
                            old_start,
                            old_count,
                            new_start,
                            new_count,
                            deleted_lines,
                            added_lines,
                        });
                        continue;
                    }
                }
                i += 1;
            }

            if !file_path.is_empty() && !hunks.is_empty() {
                file_patches.push(ParsedFilePatch {
                    file_path: file_path.to_string(),
                    hunks,
                });
            }
        } else {
            i += 1;
        }
    }

    if file_patches.is_empty() {
        return Err("No valid unified diff hunks found in patch content".to_string());
    }

    Ok(file_patches)
}

/// Applies a synthesized unified refactoring patch directly to workspace files on disk.
pub fn apply_patch_to_workspace(
    patch_content: &str,
    dry_run: bool,
) -> Result<ApplyPatchResult, String> {
    let file_patches = parse_unified_patch(patch_content)?;
    let mut modified_files = Vec::new();
    let mut total_hunks = 0;

    let mut file_modifications: Vec<(PathBuf, String)> = Vec::new();

    for file_patch in &file_patches {
        let file_path = Path::new(&file_patch.file_path);
        if !file_path.exists() {
            return Err(format!(
                "Target file '{}' specified in patch does not exist",
                file_patch.file_path
            ));
        }

        let raw_content = fs::read_to_string(file_path).map_err(|e| {
            format!(
                "Failed to read target file '{}': {}",
                file_patch.file_path, e
            )
        })?;

        let has_crlf = raw_content.contains("\r\n");
        let original_lines: Vec<String> = raw_content
            .lines()
            .map(|l| l.trim_end_matches('\r').to_string())
            .collect();

        let mut current_lines = original_lines.clone();

        let mut sorted_hunks = file_patch.hunks.clone();
        sorted_hunks.sort_by_key(|b| std::cmp::Reverse(b.old_start));

        for hunk in &sorted_hunks {
            if hunk.old_start == 0 {
                return Err(format!(
                    "Invalid hunk old_start line 0 in patch for '{}'",
                    file_patch.file_path
                ));
            }

            let start_idx = hunk.old_start.saturating_sub(1);
            let del_count = hunk.deleted_lines.len();

            if start_idx + del_count > current_lines.len() {
                return Err(format!(
                    "Hunk range [{}, {}] exceeds file line count ({}) for '{}'",
                    hunk.old_start,
                    hunk.old_start + del_count,
                    current_lines.len(),
                    file_patch.file_path
                ));
            }

            for (idx, expected_del) in hunk.deleted_lines.iter().enumerate() {
                let actual = &current_lines[start_idx + idx];
                if actual != expected_del {
                    return Err(format!(
                        "Hunk mismatch in '{}' at line {}: expected '{}', found '{}'",
                        file_patch.file_path,
                        start_idx + idx + 1,
                        expected_del,
                        actual
                    ));
                }
            }

            current_lines.splice(start_idx..start_idx + del_count, hunk.added_lines.clone());
            total_hunks += 1;
        }

        let line_ending = if has_crlf { "\r\n" } else { "\n" };
        let mut new_content = current_lines.join(line_ending);
        if raw_content.ends_with('\n') || raw_content.ends_with("\r\n") {
            new_content.push_str(line_ending);
        }

        file_modifications.push((file_path.to_path_buf(), new_content));
        modified_files.push(file_patch.file_path.clone());
    }

    if !dry_run {
        for (path, content) in &file_modifications {
            fs::write(path, content)
                .map_err(|e| format!("Failed to write patched file '{}': {}", path.display(), e))?;
        }
    }

    let mode_str = if dry_run { " (dry run)" } else { "" };
    Ok(ApplyPatchResult {
        success: true,
        modified_files,
        hunks_applied: total_hunks,
        message: format!(
            "Successfully applied {} hunk(s) across {} file(s){}",
            total_hunks,
            file_modifications.len(),
            mode_str
        ),
    })
}
