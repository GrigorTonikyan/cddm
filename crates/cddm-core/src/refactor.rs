use crate::types::CloneLocation;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Refactoring strategies supported by the CDDM refactoring engine.
pub mod refactor_strategies {
    pub const EXTRACT_FUNCTION: &str = "extract_function";
    pub const PARAMETERIZE: &str = "parameterize";
}

/// Default function name prefix for synthesized helper abstractions.
pub const DEFAULT_HELPER_PREFIX: &str = "extracted_shared_helper";

/// Result of applying a synthesized refactoring patch to workspace files.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApplyPatchResult {
    pub success: bool,
    pub modified_files: Vec<String>,
    pub hunks_applied: usize,
    pub message: String,
}

/// A parsed hunk within a unified diff patch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub deleted_lines: Vec<String>,
    pub added_lines: Vec<String>,
}

/// A parsed file patch containing target path and list of hunks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedFilePatch {
    pub file_path: String,
    pub hunks: Vec<ParsedHunk>,
}

/// Represents a variable difference between two clone fragments.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParameterDifference {
    pub line_number_a: usize,
    pub line_number_b: usize,
    pub fragment_a_code: String,
    pub fragment_b_code: String,
}

/// Comprehensive deduplication and refactoring recommendation for a clone pair.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefactorSuggestion {
    pub suggested_function_name: String,
    pub strategy: String,
    pub common_body_lines: Vec<String>,
    pub parameter_differences: Vec<ParameterDifference>,
    pub target_module_hint: String,
    pub unified_patch: String,
    pub lines_saved: usize,
}

/// Describes refactoring transformation at a specific file site within a clone cluster.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClusterSiteRefactor {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub parameter_differences: Vec<ParameterDifference>,
    pub call_site_replacement: String,
}

/// Comprehensive deduplication and multi-site refactoring recommendation for an N-way clone cluster.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClusterRefactorSuggestion {
    pub cluster_id: String,
    pub suggested_function_name: String,
    pub strategy: String,
    pub common_body_lines: Vec<String>,
    pub target_module_hint: String,
    pub sites: Vec<ClusterSiteRefactor>,
    pub unified_patch: String,
    pub total_lines_saved: usize,
}

/// Extracts source lines for a given 1-based [start_line, end_line] range.
pub fn read_file_lines_range(
    file_path: &Path,
    start_line: usize,
    end_line: usize,
) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path.display(), e))?;

    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    if start_line == 0 || start_line > lines.len() {
        return Err(format!(
            "Start line {} out of bounds (file has {} lines)",
            start_line,
            lines.len()
        ));
    }

    let end_idx = end_line.min(lines.len());
    let start_idx = start_line - 1;

    if start_idx > end_idx {
        return Err(format!("Invalid line range: {}-{}", start_line, end_line));
    }

    Ok(lines[start_idx..end_idx].to_vec())
}

/// Computes the Longest Common Subsequence matrix between two string slices.
fn compute_lcs_matrix(a: &[String], b: &[String]) -> Vec<Vec<usize>> {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1].trim() == b[j - 1].trim() {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp
}

type AlignedSlice<'a> = (
    Vec<Option<(usize, &'a String)>>,
    Vec<Option<(usize, &'a String)>>,
);

fn backtrack_lcs_alignment<'a>(
    lines_a: &'a [String],
    lines_b: &'a [String],
    dp: &[Vec<usize>],
) -> AlignedSlice<'a> {
    let mut i = lines_a.len();
    let mut j = lines_b.len();

    let mut aligned_a = Vec::new();
    let mut aligned_b = Vec::new();

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && lines_a[i - 1].trim() == lines_b[j - 1].trim() {
            aligned_a.push(Some((i - 1, &lines_a[i - 1])));
            aligned_b.push(Some((j - 1, &lines_b[j - 1])));
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || dp[i][j - 1] >= dp[i - 1][j]) {
            aligned_a.push(None);
            aligned_b.push(Some((j - 1, &lines_b[j - 1])));
            j -= 1;
        } else if i > 0 && (j == 0 || dp[i][j - 1] < dp[i - 1][j]) {
            aligned_a.push(Some((i - 1, &lines_a[i - 1])));
            aligned_b.push(None);
            i -= 1;
        }
    }

    aligned_a.reverse();
    aligned_b.reverse();
    (aligned_a, aligned_b)
}

/// Analyzes two line buffers and identifies common invariant lines and parameter differences.
pub fn analyze_snippets_refactoring(
    file_a: &str,
    range_a: (usize, usize),
    lines_a: &[String],
    file_b: &str,
    range_b: (usize, usize),
    lines_b: &[String],
) -> RefactorSuggestion {
    let dp = compute_lcs_matrix(lines_a, lines_b);
    let mut common_body_lines = Vec::new();
    let mut parameter_differences = Vec::new();

    let (aligned_a, aligned_b) = backtrack_lcs_alignment(lines_a, lines_b, &dp);

    for (item_a, item_b) in aligned_a.iter().zip(aligned_b.iter()) {
        match (item_a, item_b) {
            (Some((idx_a, line_a)), Some((idx_b, line_b))) => {
                if line_a.trim() == line_b.trim() {
                    common_body_lines.push((*line_a).clone());
                } else {
                    parameter_differences.push(ParameterDifference {
                        line_number_a: range_a.0 + idx_a,
                        line_number_b: range_b.0 + idx_b,
                        fragment_a_code: (*line_a).clone(),
                        fragment_b_code: (*line_b).clone(),
                    });
                }
            }
            (Some((idx_a, line_a)), None) => {
                parameter_differences.push(ParameterDifference {
                    line_number_a: range_a.0 + idx_a,
                    line_number_b: range_b.0,
                    fragment_a_code: (*line_a).clone(),
                    fragment_b_code: String::new(),
                });
            }
            (None, Some((idx_b, line_b))) => {
                parameter_differences.push(ParameterDifference {
                    line_number_a: range_a.0,
                    line_number_b: range_b.0 + idx_b,
                    fragment_a_code: String::new(),
                    fragment_b_code: (*line_b).clone(),
                });
            }
            (None, None) => {}
        }
    }

    let strategy = if parameter_differences.is_empty() {
        refactor_strategies::EXTRACT_FUNCTION.to_string()
    } else {
        refactor_strategies::PARAMETERIZE.to_string()
    };

    let target_module_hint = if file_a == file_b {
        format!("Local private helper inside {}", file_a)
    } else {
        "Shared utility module or common crate".to_string()
    };

    let helper_name = DEFAULT_HELPER_PREFIX.to_string();

    // Synthesize unified patch demonstrating substitution of Fragment A with helper invocation
    let mut patch = String::new();
    patch.push_str(&format!("--- a/{}\n", file_a));
    patch.push_str(&format!("+++ b/{}\n", file_a));
    patch.push_str(&format!(
        "@@ -{},{} +{},1 @@\n",
        range_a.0,
        lines_a.len(),
        range_a.0
    ));
    for line in lines_a {
        patch.push_str(&format!("-{}\n", line));
    }
    patch.push_str(&format!("+    {}();\n", helper_name));

    let total_lines = lines_a.len() + lines_b.len();
    let shared_lines = common_body_lines.len();
    let lines_saved = total_lines.saturating_sub(shared_lines + 3);

    RefactorSuggestion {
        suggested_function_name: helper_name,
        strategy,
        common_body_lines,
        parameter_differences,
        target_module_hint,
        unified_patch: patch,
        lines_saved,
    }
}

/// Analyzes duplicate code fragments directly from file paths and line ranges on disk.
pub fn analyze_clone_refactoring(
    file_a: &str,
    range_a: (usize, usize),
    file_b: &str,
    range_b: (usize, usize),
) -> Result<RefactorSuggestion, String> {
    let path_a = Path::new(file_a);
    let path_b = Path::new(file_b);

    let lines_a = read_file_lines_range(path_a, range_a.0, range_a.1)?;
    let lines_b = read_file_lines_range(path_b, range_b.0, range_b.1)?;

    Ok(analyze_snippets_refactoring(
        file_a, range_a, &lines_a, file_b, range_b, &lines_b,
    ))
}

/// Analyzes N-way duplicate code snippets across multiple occurrences and synthesizes a multi-file refactoring patch.
pub fn analyze_cluster_snippets_refactoring(
    cluster_id: &str,
    occurrences_with_lines: &[(&CloneLocation, &[String])],
) -> ClusterRefactorSuggestion {
    if occurrences_with_lines.is_empty() {
        return ClusterRefactorSuggestion {
            cluster_id: cluster_id.to_string(),
            suggested_function_name: DEFAULT_HELPER_PREFIX.to_string(),
            strategy: refactor_strategies::EXTRACT_FUNCTION.to_string(),
            common_body_lines: Vec::new(),
            target_module_hint: "Shared utility module or common crate".to_string(),
            sites: Vec::new(),
            unified_patch: String::new(),
            total_lines_saved: 0,
        };
    }

    // Step 1: Compute iterative common invariant lines across all occurrences
    let mut common_body_lines: Vec<String> = occurrences_with_lines[0].1.to_vec();
    for (_, lines) in occurrences_with_lines.iter().skip(1) {
        let dp = compute_lcs_matrix(&common_body_lines, lines);
        let mut i = common_body_lines.len();
        let mut j = lines.len();
        let mut next_common = Vec::new();

        while i > 0 && j > 0 {
            if common_body_lines[i - 1].trim() == lines[j - 1].trim() {
                next_common.push(common_body_lines[i - 1].clone());
                i -= 1;
                j -= 1;
            } else if dp[i][j - 1] >= dp[i - 1][j] {
                j -= 1;
            } else {
                i -= 1;
            }
        }
        next_common.reverse();
        common_body_lines = next_common;
    }

    let helper_name = DEFAULT_HELPER_PREFIX.to_string();
    let mut sites = Vec::new();
    let mut unified_patch = String::new();
    let mut has_parameters = false;
    let mut total_original_lines = 0;

    // Step 2: For each site, compute differences against common invariant and generate patch hunk
    for (loc, lines) in occurrences_with_lines {
        total_original_lines += lines.len();
        let mut param_diffs = Vec::new();

        let dp = compute_lcs_matrix(lines, &common_body_lines);
        let (aligned_site, aligned_common) =
            backtrack_lcs_alignment(lines, &common_body_lines, &dp);

        for (item_s, item_c) in aligned_site.iter().zip(aligned_common.iter()) {
            match (item_s, item_c) {
                (Some((idx_s, line_s)), Some((_, line_c))) if line_s.trim() != line_c.trim() => {
                    param_diffs.push(ParameterDifference {
                        line_number_a: loc.start_line + idx_s,
                        line_number_b: 0,
                        fragment_a_code: (*line_s).clone(),
                        fragment_b_code: (*line_c).clone(),
                    });
                }
                (Some((idx_s, line_s)), None) => {
                    param_diffs.push(ParameterDifference {
                        line_number_a: loc.start_line + idx_s,
                        line_number_b: 0,
                        fragment_a_code: (*line_s).clone(),
                        fragment_b_code: String::new(),
                    });
                }
                _ => {}
            }
        }

        if !param_diffs.is_empty() {
            has_parameters = true;
        }

        let call_site_replacement = format!("    {}();", helper_name);

        // Generate unified diff section for this site
        unified_patch.push_str(&format!("--- a/{}\n", loc.file));
        unified_patch.push_str(&format!("+++ b/{}\n", loc.file));
        unified_patch.push_str(&format!(
            "@@ -{},{} +{},1 @@\n",
            loc.start_line,
            lines.len(),
            loc.start_line
        ));
        for line in *lines {
            unified_patch.push_str(&format!("-{}\n", line));
        }
        unified_patch.push_str(&format!("+{}\n", call_site_replacement));

        sites.push(ClusterSiteRefactor {
            file: loc.file.clone(),
            start_line: loc.start_line,
            end_line: loc.end_line,
            parameter_differences: param_diffs,
            call_site_replacement,
        });
    }

    let strategy = if has_parameters {
        refactor_strategies::PARAMETERIZE.to_string()
    } else {
        refactor_strategies::EXTRACT_FUNCTION.to_string()
    };

    let all_same_file = occurrences_with_lines
        .windows(2)
        .all(|w| w[0].0.file == w[1].0.file);

    let target_module_hint = if all_same_file && !occurrences_with_lines.is_empty() {
        format!(
            "Local private helper inside {}",
            occurrences_with_lines[0].0.file
        )
    } else {
        "Shared utility module or common crate".to_string()
    };

    let helper_overhead = common_body_lines.len() + 3;
    let call_sites_overhead = occurrences_with_lines.len();
    let total_lines_saved =
        total_original_lines.saturating_sub(helper_overhead + call_sites_overhead);

    ClusterRefactorSuggestion {
        cluster_id: cluster_id.to_string(),
        suggested_function_name: helper_name,
        strategy,
        common_body_lines,
        target_module_hint,
        sites,
        unified_patch,
        total_lines_saved,
    }
}

/// Analyzes an N-way clone cluster from disk files and synthesizes a multi-file refactoring recommendation.
pub fn analyze_cluster_refactoring(
    cluster_id: &str,
    occurrences: &[CloneLocation],
) -> Result<ClusterRefactorSuggestion, String> {
    let mut lines_storage = Vec::with_capacity(occurrences.len());
    for occ in occurrences {
        let lines = read_file_lines_range(Path::new(&occ.file), occ.start_line, occ.end_line)?;
        lines_storage.push(lines);
    }

    let mut occurrences_with_lines = Vec::with_capacity(occurrences.len());
    for (idx, occ) in occurrences.iter().enumerate() {
        occurrences_with_lines.push((occ, lines_storage[idx].as_slice()));
    }

    Ok(analyze_cluster_snippets_refactoring(
        cluster_id,
        &occurrences_with_lines,
    ))
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_identical_snippet_refactoring() {
        let lines_a = vec![
            "let x = 10;".to_string(),
            "let y = 20;".to_string(),
            "return x + y;".to_string(),
        ];
        let lines_b = lines_a.clone();

        let suggestion = analyze_snippets_refactoring(
            "src/a.rs",
            (10, 12),
            &lines_a,
            "src/b.rs",
            (20, 22),
            &lines_b,
        );

        assert_eq!(suggestion.strategy, refactor_strategies::EXTRACT_FUNCTION);
        assert_eq!(suggestion.common_body_lines.len(), 3);
        assert!(suggestion.parameter_differences.is_empty());
        assert!(suggestion.unified_patch.contains("--- a/src/a.rs"));
        assert!(
            suggestion
                .unified_patch
                .contains("+    extracted_shared_helper();")
        );
    }

    #[test]
    fn test_renamed_parameter_refactoring() {
        let lines_a = vec![
            "let user = get_user(id);".to_string(),
            "validate_session(user);".to_string(),
            "log_access(user);".to_string(),
        ];
        let lines_b = vec![
            "let admin = get_admin(id);".to_string(),
            "validate_session(admin);".to_string(),
            "log_access(admin);".to_string(),
        ];

        let suggestion = analyze_snippets_refactoring(
            "src/auth.rs",
            (5, 7),
            &lines_a,
            "src/admin.rs",
            (15, 17),
            &lines_b,
        );

        assert_eq!(suggestion.strategy, refactor_strategies::PARAMETERIZE);
        assert!(!suggestion.parameter_differences.is_empty());
        assert_eq!(suggestion.parameter_differences[0].line_number_a, 5);
        assert_eq!(suggestion.parameter_differences[0].line_number_b, 15);
    }

    #[test]
    fn test_real_file_clone_refactoring() {
        let mut file_a = NamedTempFile::new().unwrap();
        let mut file_b = NamedTempFile::new().unwrap();

        writeln!(
            file_a,
            "line 1\nlet val = 42;\nprintln!(\"val: {{}}\", val);"
        )
        .unwrap();
        writeln!(
            file_b,
            "header\nlet val = 42;\nprintln!(\"val: {{}}\", val);"
        )
        .unwrap();

        let res = analyze_clone_refactoring(
            file_a.path().to_str().unwrap(),
            (2, 3),
            file_b.path().to_str().unwrap(),
            (2, 3),
        );

        assert!(res.is_ok());
        let suggestion = res.unwrap();
        assert_eq!(suggestion.common_body_lines.len(), 2);
        assert_eq!(suggestion.strategy, refactor_strategies::EXTRACT_FUNCTION);
    }

    #[test]
    fn test_invalid_line_range() {
        let file = NamedTempFile::new().unwrap();
        let path_str = file.path().to_str().unwrap();
        let res = analyze_clone_refactoring(path_str, (10, 20), path_str, (1, 2));
        assert!(res.is_err());
    }

    #[test]
    fn test_identical_cluster_refactoring_three_sites() {
        let loc1 = CloneLocation {
            file: "src/alpha.rs".to_string(),
            start_line: 10,
            end_line: 12,
            author: None,
        };
        let loc2 = CloneLocation {
            file: "src/beta.rs".to_string(),
            start_line: 20,
            end_line: 22,
            author: None,
        };
        let loc3 = CloneLocation {
            file: "src/gamma.rs".to_string(),
            start_line: 30,
            end_line: 32,
            author: None,
        };

        let lines = vec![
            "let x = 10;".to_string(),
            "let y = 20;".to_string(),
            "return x + y;".to_string(),
        ];

        let occurrences = vec![
            (&loc1, lines.as_slice()),
            (&loc2, lines.as_slice()),
            (&loc3, lines.as_slice()),
        ];

        let suggestion = analyze_cluster_snippets_refactoring("cluster-1", &occurrences);

        assert_eq!(suggestion.strategy, refactor_strategies::EXTRACT_FUNCTION);
        assert_eq!(suggestion.common_body_lines.len(), 3);
        assert_eq!(suggestion.sites.len(), 3);
        assert!(suggestion.unified_patch.contains("--- a/src/alpha.rs"));
        assert!(suggestion.unified_patch.contains("--- a/src/beta.rs"));
        assert!(suggestion.unified_patch.contains("--- a/src/gamma.rs"));
    }

    #[test]
    fn test_real_file_cluster_refactoring() {
        let mut file_a = NamedTempFile::new().unwrap();
        let mut file_b = NamedTempFile::new().unwrap();
        let mut file_c = NamedTempFile::new().unwrap();

        writeln!(
            file_a,
            "fn foo() {{\nlet v = 100;\nprintln!(\"{{}}\", v);\n}}"
        )
        .unwrap();
        writeln!(
            file_b,
            "fn bar() {{\nlet v = 100;\nprintln!(\"{{}}\", v);\n}}"
        )
        .unwrap();
        writeln!(
            file_c,
            "fn baz() {{\nlet v = 100;\nprintln!(\"{{}}\", v);\n}}"
        )
        .unwrap();

        let occurrences = vec![
            CloneLocation {
                file: file_a.path().to_str().unwrap().to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
            CloneLocation {
                file: file_b.path().to_str().unwrap().to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
            CloneLocation {
                file: file_c.path().to_str().unwrap().to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
        ];

        let res = analyze_cluster_refactoring("cluster-test", &occurrences);
        assert!(res.is_ok());
        let suggestion = res.unwrap();
        assert_eq!(suggestion.common_body_lines.len(), 2);
        assert_eq!(suggestion.sites.len(), 3);
        assert_eq!(suggestion.strategy, refactor_strategies::EXTRACT_FUNCTION);
    }

    #[test]
    fn test_apply_patch_single_file_success() {
        let mut file_a = NamedTempFile::new().unwrap();
        let path_str = file_a.path().to_str().unwrap().to_string();

        writeln!(
            file_a,
            "fn compute() {{\n    let a = 10;\n    let b = 20;\n    println!(\"{{}}\", a + b);\n}}"
        )
        .unwrap();
        file_a.flush().unwrap();

        let patch = format!(
            "--- a/{}\n+++ b/{}\n@@ -2,3 +2,1 @@\n-    let a = 10;\n-    let b = 20;\n-    \
             println!(\"{{}}\", a + b);\n+    extracted_shared_helper();\n",
            path_str, path_str
        );

        let res = apply_patch_to_workspace(&patch, false);
        assert!(res.is_ok());
        let result = res.unwrap();
        assert!(result.success);
        assert_eq!(result.hunks_applied, 1);
        assert_eq!(result.modified_files.len(), 1);

        let modified_content = fs::read_to_string(file_a.path()).unwrap();
        assert!(modified_content.contains("extracted_shared_helper();"));
        assert!(!modified_content.contains("let a = 10;"));
    }

    #[test]
    fn test_apply_patch_dry_run_preserves_file() {
        let mut file_a = NamedTempFile::new().unwrap();
        let path_str = file_a.path().to_str().unwrap().to_string();

        let original_code =
            "fn compute() {\n    let a = 10;\n    let b = 20;\n    println!(\"{}\", a + b);\n}\n";
        file_a.write_all(original_code.as_bytes()).unwrap();
        file_a.flush().unwrap();

        let patch = format!(
            "--- a/{}\n+++ b/{}\n@@ -2,3 +2,1 @@\n-    let a = 10;\n-    let b = 20;\n-    \
             println!(\"{{}}\", a + b);\n+    extracted_shared_helper();\n",
            path_str, path_str
        );

        let res = apply_patch_to_workspace(&patch, true);
        assert!(res.is_ok());
        let result = res.unwrap();
        assert!(result.success);
        assert!(result.message.contains("dry run"));

        let current_content = fs::read_to_string(file_a.path()).unwrap();
        assert_eq!(current_content, original_code);
    }

    #[test]
    fn test_apply_patch_multi_file_cluster() {
        let mut file_a = NamedTempFile::new().unwrap();
        let mut file_b = NamedTempFile::new().unwrap();
        let path_a = file_a.path().to_str().unwrap().to_string();
        let path_b = file_b.path().to_str().unwrap().to_string();

        writeln!(file_a, "fn one() {{\n    let v = 42;\n}}").unwrap();
        writeln!(file_b, "fn two() {{\n    let v = 42;\n}}").unwrap();
        file_a.flush().unwrap();
        file_b.flush().unwrap();

        let patch = format!(
            "--- a/{}\n+++ b/{}\n@@ -2,1 +2,1 @@\n-    let v = 42;\n+    helper();\n--- a/{}\n+++ \
             b/{}\n@@ -2,1 +2,1 @@\n-    let v = 42;\n+    helper();\n",
            path_a, path_a, path_b, path_b
        );

        let res = apply_patch_to_workspace(&patch, false);
        assert!(res.is_ok());
        let result = res.unwrap();
        assert_eq!(result.hunks_applied, 2);
        assert_eq!(result.modified_files.len(), 2);

        let content_a = fs::read_to_string(file_a.path()).unwrap();
        let content_b = fs::read_to_string(file_b.path()).unwrap();
        assert!(content_a.contains("helper();"));
        assert!(content_b.contains("helper();"));
    }

    #[test]
    fn test_apply_patch_mismatch_fails() {
        let mut file_a = NamedTempFile::new().unwrap();
        let path_str = file_a.path().to_str().unwrap().to_string();

        writeln!(file_a, "fn compute() {{\n    let x = 999;\n}}").unwrap();
        file_a.flush().unwrap();

        let patch = format!(
            "--- a/{}\n+++ b/{}\n@@ -2,1 +2,1 @@\n-    let a = 10;\n+    helper();\n",
            path_str, path_str
        );

        let res = apply_patch_to_workspace(&patch, false);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Hunk mismatch"));
    }
}
