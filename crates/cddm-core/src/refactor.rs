use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Refactoring strategies supported by the CDDM refactoring engine.
pub mod refactor_strategies {
    pub const EXTRACT_FUNCTION: &str = "extract_function";
    pub const PARAMETERIZE: &str = "parameterize";
}

/// Default function name prefix for synthesized helper abstractions.
pub const DEFAULT_HELPER_PREFIX: &str = "extracted_shared_helper";

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
}
