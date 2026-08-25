#![forbid(unsafe_code)]

pub mod compiled;
pub mod engine;
pub mod eval;

pub use compiled::{CompiledBoundary, CompiledLimit, CompiledZeroDuplication, path_matches_glob};
pub use engine::PolicyEngine;
pub use eval::evaluate_scan_policies;

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
        };

        let eval = engine.evaluate(&scan_result);
        assert!(eval.passed); // Warnings only
        assert_eq!(eval.warning_count, 2); // 1 for pair token_count, 1 for cluster max_occurrences
        assert_eq!(eval.error_count, 0);
    }
}
