#![forbid(unsafe_code)]

pub mod budgets;
pub mod compiled;
pub mod emoji;
pub mod engine;
pub mod eval;
pub mod forbidden_files;
pub mod patterns;
pub mod route_docs;
pub mod sync;

pub use budgets::{CompiledTokenBudget, count_file_tokens, evaluate_token_budgets};
pub use compiled::{
    CompiledBoundary, CompiledLimit, CompiledProhibitedPattern, CompiledZeroDuplication,
    path_matches_glob,
};
pub use emoji::{evaluate_zero_emoji, is_emoji_char};
pub use engine::PolicyEngine;
pub use eval::evaluate_scan_policies;
pub use forbidden_files::{CompiledForbiddenFile, evaluate_forbidden_files};
pub use patterns::{CompiledProhibitedPattern as ProhibitedPattern, evaluate_prohibited_patterns};
pub use route_docs::evaluate_route_docs_policies;
pub use sync::evaluate_sync_rules;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CloneLocation, ClonePair, CloneType, ScanResult};

    #[test]
    fn test_policy_engine_parse_and_evaluate_boundary() {
        let toml_content = r#"
[[boundaries]]
name = "domain-isolation"
description = "No domain clones in presentation"
source = "src/domain/**"
forbidden_targets = ["src/presentation/**"]
severity = "error"
"#;
        let engine = toml_content
            .parse::<PolicyEngine>()
            .expect("Failed to parse policy TOML");

        let scan_result = ScanResult {
            scan_id: "test".to_string(),
            total_files: 2,
            total_tokens: 500,
            total_clones: 1,
            total_clusters: 0,
            duplication_percentage: 10.0,
            dry_health_score: 90.0,
            clone_pairs: vec![ClonePair {
                file_a: "src/domain/user.rs".to_string(),
                start_line_a: 10,
                end_line_a: 30,
                file_b: "src/presentation/user.rs".to_string(),
                start_line_b: 15,
                end_line_b: 35,
                token_count: 65,
                similarity: 1.0,
                fragment_hash: "hash1".to_string(),
                clone_type: CloneType::Exact,
                author_a: None,
                author_b: None,
            }],
            clone_clusters: Vec::new(),
            duration_ms: 50,
            language_breakdown: Vec::new(),
            policy_violations: Vec::new(),
            target_directory: None,
        };

        let eval = engine.evaluate(&scan_result);
        assert!(!eval.passed);
        assert_eq!(eval.error_count, 1);
        assert_eq!(eval.violations.len(), 1);
        assert_eq!(eval.violations[0].rule_name, "domain-isolation");
        assert_eq!(eval.violations[0].rule_type, "boundary");
    }

    #[test]
    fn test_policy_engine_zero_duplication_rule() {
        let toml_content = r#"
[[zero_duplication]]
name = "auth-clean"
pattern = "src/auth/**"
severity = "error"
"#;
        let engine = toml_content
            .parse::<PolicyEngine>()
            .expect("Failed to parse policy TOML");

        let scan_result = ScanResult {
            scan_id: "test-zd".to_string(),
            total_files: 2,
            total_tokens: 500,
            total_clones: 1,
            total_clusters: 0,
            duplication_percentage: 5.0,
            dry_health_score: 95.0,
            clone_pairs: vec![ClonePair {
                file_a: "src/auth/token.rs".to_string(),
                start_line_a: 1,
                end_line_a: 20,
                file_b: "src/utils/token.rs".to_string(),
                start_line_b: 1,
                end_line_b: 20,
                token_count: 55,
                similarity: 1.0,
                fragment_hash: "hash2".to_string(),
                clone_type: CloneType::Exact,
                author_a: None,
                author_b: None,
            }],
            clone_clusters: Vec::new(),
            duration_ms: 20,
            language_breakdown: Vec::new(),
            policy_violations: Vec::new(),
            target_directory: None,
        };

        let eval = engine.evaluate(&scan_result);
        assert!(!eval.passed);
        assert_eq!(eval.error_count, 1);
        assert_eq!(eval.violations[0].rule_name, "auth-clean");
    }

    #[test]
    fn test_policy_engine_limit_rule() {
        let toml_content = r#"
[[limits]]
name = "max-api-tokens"
pattern = "src/api/**"
max_tokens = 60
max_occurrences = 2
severity = "warning"
"#;
        let engine = toml_content
            .parse::<PolicyEngine>()
            .expect("Failed to parse policy TOML");

        let scan_result = ScanResult {
            scan_id: "test-limit".to_string(),
            total_files: 3,
            total_tokens: 1000,
            total_clones: 1,
            total_clusters: 1,
            duplication_percentage: 8.0,
            dry_health_score: 92.0,
            clone_pairs: vec![ClonePair {
                file_a: "src/api/handler1.rs".to_string(),
                start_line_a: 10,
                end_line_a: 50,
                file_b: "src/api/handler2.rs".to_string(),
                start_line_b: 10,
                end_line_b: 50,
                token_count: 80, // Exceeds 60
                similarity: 1.0,
                fragment_hash: "hash3".to_string(),
                clone_type: CloneType::Exact,
                author_a: None,
                author_b: None,
            }],
            clone_clusters: vec![crate::types::CloneCluster {
                id: 1,
                clone_type: CloneType::Exact,
                token_count: 80,
                similarity: 1.0,
                fragment_hash: "hash3".to_string(),
                occurrences: vec![
                    CloneLocation {
                        file: "src/api/handler1.rs".to_string(),
                        start_line: 10,
                        end_line: 50,
                        author: None,
                    },
                    CloneLocation {
                        file: "src/api/handler2.rs".to_string(),
                        start_line: 10,
                        end_line: 50,
                        author: None,
                    },
                    CloneLocation {
                        file: "src/api/handler3.rs".to_string(),
                        start_line: 10,
                        end_line: 50,
                        author: None,
                    },
                ],
            }],
            duration_ms: 30,
            language_breakdown: Vec::new(),
            policy_violations: Vec::new(),
            target_directory: None,
        };

        let eval = engine.evaluate(&scan_result);
        assert!(eval.passed); // Warnings only
        assert_eq!(eval.warning_count, 2); // 1 for pair token_count, 1 for cluster max_occurrences
        assert_eq!(eval.error_count, 0);
    }

    #[test]
    fn test_policy_engine_prohibited_patterns_and_budgets() {
        let temp_dir = tempfile::tempdir().expect("Failed to create tempdir");
        let root = temp_dir.path();

        let source_file = root.join("bad_file.rs");
        std::fs::write(
            &source_file,
            "fn dangerous() { let _x = 1; println!(\"alert\"); }",
        )
        .expect("Failed to write test file");

        let toml_content = r#"
[[prohibited_patterns]]
name = "no-dangerous-fn"
description = "Do not use dangerous functions"
pattern = "fn dangerous"
pattern_type = "regex"
severity = "error"
remediation = "Refactor into safe module"

[[token_budgets]]
name = "max-source-tokens"
pattern = "*.rs"
max_file_tokens = 5
severity = "warning"
"#;
        let engine = toml_content
            .parse::<PolicyEngine>()
            .expect("Failed to parse policy TOML");

        let scan_result = ScanResult {
            scan_id: "test-patterns".to_string(),
            total_files: 1,
            target_directory: Some(root.to_string_lossy().to_string()),
            ..Default::default()
        };

        let eval = engine.evaluate(&scan_result);
        assert!(!eval.passed);
        assert_eq!(eval.error_count, 1); // prohibited pattern error
        assert_eq!(eval.warning_count, 1); // token budget warning
        assert_eq!(eval.violations.len(), 2);

        let p_violation = eval
            .violations
            .iter()
            .find(|v| v.rule_name == "no-dangerous-fn")
            .expect("Expected prohibited pattern violation");
        assert_eq!(p_violation.rule_type, "prohibited_pattern");
        assert!(p_violation.snippet.is_some());
        assert_eq!(
            p_violation.remediation.as_deref(),
            Some("Refactor into safe module")
        );
    }

    #[test]
    fn test_policy_engine_zero_emoji_detection() {
        let temp_dir = tempfile::tempdir().expect("Failed to create tempdir");
        let root = temp_dir.path();

        let doc_file = root.join("README.md");
        std::fs::write(&doc_file, "# Project\nThis is great \u{1F680} rockets!\n")
            .expect("Failed to write doc file");

        let toml_content = "zero_emojis = true\n";
        let engine = toml_content
            .parse::<PolicyEngine>()
            .expect("Failed to parse policy TOML");

        let scan_result = ScanResult {
            scan_id: "test-emoji".to_string(),
            total_files: 1,
            target_directory: Some(root.to_string_lossy().to_string()),
            ..Default::default()
        };

        let eval = engine.evaluate(&scan_result);
        assert!(!eval.passed);
        assert_eq!(eval.error_count, 1);
        assert_eq!(eval.violations[0].rule_type, "zero_emojis");
        assert_eq!(eval.violations[0].rule_name, "zero-emoji");
    }

    #[test]
    fn test_policy_engine_forbidden_files_and_sync() {
        let temp_dir = tempfile::tempdir().expect("Failed to create tempdir");
        let root = temp_dir.path();

        let yarn_lock = root.join("yarn.lock");
        std::fs::write(&yarn_lock, "# lock").expect("write yarn.lock");

        let pkg_json = root.join("package.json");
        std::fs::write(&pkg_json, r#"{"version": "3.0.0"}"#).expect("write package.json");

        let const_file = root.join("constants.ts");
        std::fs::write(&const_file, r#"export const VERSION = "2.9.0";"#)
            .expect("write constants.ts");

        let toml_content = r#"
[[forbidden_files]]
name = "no-yarn"
patterns = ["yarn.lock"]
severity = "error"
remediation = "Use bun.lock"

[[sync_rules]]
name = "version-sync"
source_file = "package.json"
source_key = "version"
target_files = ["constants.ts"]
target_regex = 'VERSION\s*=\s*"([^"]+)"'
severity = "error"
"#;
        let engine = toml_content
            .parse::<PolicyEngine>()
            .expect("parse policy TOML");

        let scan_result = ScanResult {
            scan_id: "test-forbidden-sync".to_string(),
            total_files: 3,
            target_directory: Some(root.to_string_lossy().to_string()),
            ..Default::default()
        };

        let eval = engine.evaluate(&scan_result);
        assert!(!eval.passed);
        assert_eq!(eval.error_count, 2);
        assert!(
            eval.violations
                .iter()
                .any(|v| v.rule_type == "forbidden_file")
        );
        assert!(eval.violations.iter().any(|v| v.rule_type == "sync_drift"));
    }

    #[test]
    fn test_policy_engine_route_docs_sync() {
        let temp_dir = tempfile::tempdir().expect("Failed to create tempdir");
        let root = temp_dir.path();

        let app_dir = root.join("src").join("app");
        std::fs::create_dir_all(app_dir.join("analytics")).expect("create app dir");
        std::fs::write(
            app_dir.join("analytics").join("page.tsx"),
            "export default function Analytics() { return null; }",
        )
        .expect("write page");

        let readme = root.join("README.md");
        std::fs::write(&readme, "# App Docs\nNo routes listed yet.\n").expect("write readme");

        let toml_content = r#"
[route_docs_sync]
name = "route-sync"
framework = "nextjs"
routes_dir = "src/app"
doc_file = "README.md"
severity = "warning"
"#;
        let engine = toml_content
            .parse::<PolicyEngine>()
            .expect("parse policy TOML");

        let scan_result = ScanResult {
            scan_id: "test-route-docs".to_string(),
            total_files: 2,
            target_directory: Some(root.to_string_lossy().to_string()),
            ..Default::default()
        };

        let eval = engine.evaluate(&scan_result);
        assert!(eval.passed); // Warning severity
        assert_eq!(eval.warning_count, 1);
        assert_eq!(eval.violations[0].rule_type, "route_docs");
        assert!(eval.violations[0].message.contains("/analytics"));
    }
}
