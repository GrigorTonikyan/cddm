#![forbid(unsafe_code)]

use super::types::*;
use crate::types::{CloneLocation, RefactorSandboxResult};
use std::fs;
use std::path::Path;

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
pub fn compute_lcs_matrix(a: &[String], b: &[String]) -> Vec<Vec<usize>> {
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

pub type AlignedSlice<'a> = (
    Vec<Option<(usize, &'a String)>>,
    Vec<Option<(usize, &'a String)>>,
);

pub fn backtrack_lcs_alignment<'a>(
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

/// Generates an interactive refactoring preview for the sandbox with customized function naming.
pub fn preview_cluster_refactor(
    occurrences: &[CloneLocation],
    custom_function_name: Option<&str>,
    target_module_path: Option<&str>,
    _custom_parameter_names: Option<&[String]>,
) -> Result<RefactorSandboxResult, String> {
    if occurrences.is_empty() {
        return Err("No occurrences provided for refactoring preview".to_string());
    }

    let mut site_snippets = Vec::new();
    for occ in occurrences {
        let path = Path::new(&occ.file);
        if !path.exists() {
            return Err(format!("Occurrence file '{}' does not exist", occ.file));
        }

        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read occurrence file '{}': {}", occ.file, e))?;
        let all_lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        let start_idx = occ.start_line.saturating_sub(1);
        let end_idx = occ.end_line.min(all_lines.len());
        if start_idx < end_idx {
            let snippet = all_lines[start_idx..end_idx].to_vec();
            site_snippets.push((occ, snippet));
        }
    }

    if site_snippets.is_empty() {
        return Err("Failed to extract code snippets from occurrences".to_string());
    }

    let fn_name = custom_function_name.unwrap_or(DEFAULT_HELPER_PREFIX);
    let target_path = target_module_path.unwrap_or(&occurrences[0].file);

    let occ_pairs: Vec<(&CloneLocation, &[String])> = site_snippets
        .iter()
        .map(|(occ, snip)| (*occ, snip.as_slice()))
        .collect();

    let mut cluster_refactor = analyze_cluster_snippets_refactoring("custom", &occ_pairs);
    if let Some(name) = custom_function_name {
        cluster_refactor.suggested_function_name = name.to_string();
        cluster_refactor.unified_patch = cluster_refactor
            .unified_patch
            .replace(DEFAULT_HELPER_PREFIX, name);
    }

    let mut affected_files: Vec<String> = occurrences.iter().map(|o| o.file.clone()).collect();
    affected_files.sort();
    affected_files.dedup();

    Ok(RefactorSandboxResult {
        cluster_id: None,
        function_name: fn_name.to_string(),
        target_module_path: target_path.to_string(),
        unified_patch: cluster_refactor.unified_patch,
        total_lines_saved: cluster_refactor.total_lines_saved,
        sites_count: cluster_refactor.sites.len(),
        affected_files,
    })
}
