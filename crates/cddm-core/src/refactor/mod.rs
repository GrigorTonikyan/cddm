#![forbid(unsafe_code)]

pub mod ast;
pub mod branch;
pub mod consensus;
pub mod patch;
pub mod types;
pub mod verify;

pub use ast::generate_ast_cluster_refactor;
pub use branch::apply_cluster_refactor_branch;
pub use consensus::{
    AlignedSlice, analyze_clone_refactoring, analyze_cluster_refactoring,
    analyze_cluster_snippets_refactoring, analyze_snippets_refactoring, backtrack_lcs_alignment,
    compute_lcs_matrix, preview_cluster_refactor, read_file_lines_range,
};
pub use patch::{apply_patch_to_workspace, apply_patch_to_workspace_dir, parse_unified_patch};
pub use types::*;
pub use verify::verify_refactor_test_suite;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CloneLocation;
    use std::fs;
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

    fn create_temp_file_with_content(content: &str) -> (NamedTempFile, String) {
        let mut file = NamedTempFile::new().unwrap();
        let path_str = file.path().to_str().unwrap().to_string();
        file.write_all(content.as_bytes()).unwrap();
        file.flush().unwrap();
        (file, path_str)
    }

    #[test]
    fn test_apply_patch_single_file_success() {
        let (file_a, path_str) = create_temp_file_with_content(
            "fn compute() {\n    let a = 10;\n    let b = 20;\n    println!(\"{}\", a + b);\n}\n",
        );

        let patch = format!(
            "--- a/{}\n+++ b/{}\n@@ -2,3 +2,1 @@\n-    let a = 10;\n-    let b = 20;\n-    \
             println!(\"{{}}\", a + b);\n+    extracted_shared_helper();\n",
            path_str, path_str
        );

        let res = apply_patch_to_workspace(&patch, false);
        assert!(res.is_ok(), "{:?}", res.err());
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
        let original_code =
            "fn compute() {\n    let a = 10;\n    let b = 20;\n    println!(\"{}\", a + b);\n}\n";
        let (file_a, path_str) = create_temp_file_with_content(original_code);

        let patch = format!(
            "--- a/{}\n+++ b/{}\n@@ -2,3 +2,1 @@\n-    let a = 10;\n-    let b = 20;\n-    \
             println!(\"{{}}\", a + b);\n+    extracted_shared_helper();\n",
            path_str, path_str
        );

        let res = apply_patch_to_workspace(&patch, true);
        assert!(res.is_ok(), "{:?}", res.err());
        let result = res.unwrap();
        assert!(result.success);
        assert!(result.message.contains("dry run"));

        let current_content = fs::read_to_string(file_a.path()).unwrap();
        assert_eq!(current_content, original_code);
    }

    #[test]
    fn test_apply_patch_multi_file_cluster() {
        let (file_a, path_a) = create_temp_file_with_content("fn one() {\n    let v = 42;\n}\n");
        let (file_b, path_b) = create_temp_file_with_content("fn two() {\n    let v = 42;\n}\n");

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
        let (_file_a, path_str) =
            create_temp_file_with_content("fn compute() {\n    let x = 999;\n}\n");

        let patch = format!(
            "--- a/{}\n+++ b/{}\n@@ -2,1 +2,1 @@\n-    let a = 10;\n+    helper();\n",
            path_str, path_str
        );

        let res = apply_patch_to_workspace(&patch, false);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Hunk mismatch"));
    }

    fn create_test_pair_files(
        suffix: &str,
        content_a: &str,
        content_b: &str,
        start_line: usize,
        end_line: usize,
    ) -> (NamedTempFile, NamedTempFile, String, Vec<CloneLocation>) {
        let mut file_a = NamedTempFile::with_suffix(suffix).unwrap();
        let mut file_b = NamedTempFile::with_suffix(suffix).unwrap();
        let path_a = file_a.path().to_str().unwrap().to_string();
        let path_b = file_b.path().to_str().unwrap().to_string();

        writeln!(file_a, "{}", content_a).unwrap();
        writeln!(file_b, "{}", content_b).unwrap();
        file_a.flush().unwrap();
        file_b.flush().unwrap();

        let occurrences = vec![
            CloneLocation {
                file: path_a.clone(),
                start_line,
                end_line,
                author: None,
            },
            CloneLocation {
                file: path_b,
                start_line,
                end_line,
                author: None,
            },
        ];
        (file_a, file_b, path_a, occurrences)
    }

    #[test]
    fn test_generate_ast_cluster_refactor() {
        let (_fa, _fb, path_a, occurrences) = create_test_pair_files(
            ".rs",
            "fn compute_a() {\n    let x = 10;\n    let y = 20;\n    println!(\"{}\", x + y);\n}",
            "fn compute_b() {\n    let x = 10;\n    let y = 20;\n    println!(\"{}\", x + y);\n}",
            2,
            4,
        );

        let result = generate_ast_cluster_refactor(
            &occurrences,
            Some("calculate_shared_sum"),
            Some(&path_a),
            None,
        );

        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.function_name, "calculate_shared_sum");
        assert!(
            res.helper_function_code
                .contains("pub fn calculate_shared_sum()")
        );
        assert_eq!(res.rewritten_files.len(), 2);
    }

    #[test]
    fn test_generate_ast_cluster_refactor_go() {
        let (_fa, _fb, path_a, occurrences) = create_test_pair_files(
            ".go",
            "package main\n\nfunc RunA() {\n\tval := 100\n\t_ = val * 2\n}",
            "package main\n\nfunc RunB() {\n\tval := 100\n\t_ = val * 2\n}",
            4,
            5,
        );

        let result = generate_ast_cluster_refactor(
            &occurrences,
            Some("process_values"),
            Some(&path_a),
            None,
        );

        assert!(result.is_ok());
        let res = result.unwrap();
        assert!(res.helper_function_code.contains("func ProcessValues() {"));
        assert_eq!(res.rewritten_files.len(), 2);
        assert!(
            res.rewritten_files[0]
                .rewritten_source
                .contains("ProcessValues()")
        );
    }

    #[test]
    fn test_generate_ast_cluster_refactor_python() {
        let (_fa, _fb, path_a, occurrences) = create_test_pair_files(
            ".py",
            "def handle_a():\n    score = 10\n    print(score)\n",
            "def handle_b():\n    score = 10\n    print(score)\n",
            2,
            3,
        );

        let result =
            generate_ast_cluster_refactor(&occurrences, Some("log_score"), Some(&path_a), None);

        assert!(result.is_ok());
        let res = result.unwrap();
        assert!(
            res.helper_function_code
                .contains("def log_score() -> None:")
        );
        assert_eq!(res.rewritten_files.len(), 2);
        assert!(
            res.rewritten_files[0]
                .rewritten_source
                .contains("log_score()")
        );
    }
}
