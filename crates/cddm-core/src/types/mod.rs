#![forbid(unsafe_code)]

pub mod clone;
pub mod diff;
pub mod policy;
pub mod refactor;
pub mod scan;
pub mod suppression;
pub mod timeline;
pub mod workflow;

pub use clone::{CloneCluster, CloneLocation, ClonePair, CloneType, deduplicate_clone_pairs};
pub use diff::{CloneStatus, DiffClonePair, DiffScanResult, DiffSummary};
pub use policy::{
    BoundaryRule, LimitRule, PolicyConfig, PolicyEvaluationResult, PolicySeverity, PolicyViolation,
    ZeroDuplicationRule,
};
pub use refactor::{
    ApplyRefactorBranchRequest, ApplyRefactorBranchResult, AstRewriteResult, AstRewrittenFile,
    InferredParameter, RefactorSandboxRequest, RefactorSandboxResult, VerifyRefactorRequest,
    VerifyRefactorResult,
};
pub use scan::{
    DEFAULT_CACHE_FILE, DEFAULT_DIRECTORY, DEFAULT_FAIL_THRESHOLD, DEFAULT_IGNORE_PATTERNS,
    DEFAULT_MIN_TOKENS, DEFAULT_RULES_FILE, LanguageStats, LineSpan, MAX_HEALTH_SCORE,
    MIN_HEALTH_SCORE, NormalizedToken, ScanConfig, ScanPhase, ScanProgress, ScanResult,
};
pub use suppression::{SuppressionConfig, SuppressionDirective, SuppressionRule};
pub use timeline::{FileChurnMetric, TimelineSnapshot, TimelineTrend};
pub use workflow::{HookStatus, WorkflowPlatform};

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_scan_config_default() {
        let config = ScanConfig::default();
        assert_eq!(config.directory, DEFAULT_DIRECTORY);
        assert_eq!(config.min_tokens, DEFAULT_MIN_TOKENS);
        assert!(config.languages.is_empty());
        assert_eq!(config.ignore_patterns.len(), DEFAULT_IGNORE_PATTERNS.len());
        assert!(config.detect_type2);
        assert!(config.detect_type3);
        assert!(config.scan_self);
        assert!(!config.enable_git_blame);
        assert!(config.cross_language);
        assert!(config.ignore_tests);
        assert!(config.ignore_mocks);
    }

    fn assert_serde_roundtrip<
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    >(
        val: &T,
    ) {
        let json = serde_json::to_string(val).expect("Serialization failed");
        let recovered: T = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(val, &recovered);
    }

    #[test]
    fn test_scan_phase_serde() {
        let phases = [
            ScanPhase::Discovery,
            ScanPhase::Tokenization,
            ScanPhase::AstAnalysis,
            ScanPhase::Indexing,
            ScanPhase::Merging,
            ScanPhase::Scoring,
            ScanPhase::Complete,
            ScanPhase::Cancelled,
            ScanPhase::Failed,
        ];
        for phase in phases {
            assert_serde_roundtrip(&phase);
            assert_eq!(phase.to_string(), phase.as_ref());
        }
    }

    #[test]
    fn test_scan_config_serde_roundtrip() {
        assert_serde_roundtrip(&ScanConfig::default());
    }

    #[test]
    fn test_clone_type_serde_variants() {
        let variants = [
            CloneType::Exact,
            CloneType::Renamed,
            CloneType::NearMiss,
            CloneType::Semantic,
        ];
        for variant in variants {
            assert_serde_roundtrip(&variant);
        }
    }

    #[test]
    fn test_scan_result_serde_roundtrip() {
        let result = ScanResult {
            scan_id: "test-id".to_string(),
            total_files: 10,
            total_tokens: 1000,
            total_clones: 5,
            total_clusters: 1,
            duplication_percentage: 2.5,
            dry_health_score: 95.0,
            clone_pairs: vec![ClonePair {
                file_a: "a.rs".to_string(),
                start_line_a: 1,
                end_line_a: 10,
                file_b: "b.rs".to_string(),
                start_line_b: 2,
                end_line_b: 11,
                token_count: 50,
                similarity: 1.0,
                fragment_hash: "hash".to_string(),
                clone_type: CloneType::Exact,
                author_a: None,
                author_b: None,
            }],
            clone_clusters: vec![CloneCluster {
                id: 1,
                clone_type: CloneType::Exact,
                token_count: 50,
                similarity: 1.0,
                fragment_hash: "hash".to_string(),
                occurrences: vec![
                    CloneLocation {
                        file: "a.rs".to_string(),
                        start_line: 1,
                        end_line: 10,
                        author: None,
                    },
                    CloneLocation {
                        file: "b.rs".to_string(),
                        start_line: 2,
                        end_line: 11,
                        author: None,
                    },
                ],
            }],
            duration_ms: 100,
            language_breakdown: vec![LanguageStats {
                language: "Rust".to_string(),
                files: 10,
                tokens: 1000,
                clones: 5,
            }],
            policy_violations: Vec::new(),
        };
        assert_serde_roundtrip(&result);
    }

    #[test]
    fn test_line_span_equality() {
        let span1 = LineSpan {
            line_start: 1,
            line_end: 2,
            byte_offset: 0,
        };
        let span2 = LineSpan {
            line_start: 1,
            line_end: 2,
            byte_offset: 0,
        };
        let span3 = LineSpan {
            line_start: 1,
            line_end: 3,
            byte_offset: 0,
        };
        assert_eq!(span1, span2);
        assert_ne!(span1, span3);
    }

    #[test]
    fn test_clone_status_display_and_serde() {
        let statuses = [CloneStatus::New, CloneStatus::Legacy, CloneStatus::Resolved];
        for status in statuses {
            assert_serde_roundtrip(&status);
            assert_eq!(status.to_string(), status.as_ref());
        }
    }

    #[test]
    fn test_diff_scan_result_serde_roundtrip() {
        let diff_result = DiffScanResult {
            scan_id: "diff-test-id".to_string(),
            summary: DiffSummary {
                base_ref: "main".to_string(),
                target_ref: "feature/refactor".to_string(),
                base_dry_score: 92.5,
                target_dry_score: 96.0,
                net_dry_delta: 3.5,
                total_changed_files: 3,
                new_clones: 0,
                legacy_clones: 2,
                resolved_clones: 1,
            },
            diff_clones: vec![DiffClonePair {
                clone_pair: ClonePair {
                    file_a: "src/a.rs".to_string(),
                    start_line_a: 10,
                    end_line_a: 20,
                    file_b: "src/b.rs".to_string(),
                    start_line_b: 30,
                    end_line_b: 40,
                    token_count: 60,
                    similarity: 1.0,
                    fragment_hash: "hash_diff".to_string(),
                    clone_type: CloneType::Exact,
                    author_a: None,
                    author_b: None,
                },
                status: CloneStatus::Legacy,
            }],
            duration_ms: 45,
        };
        assert_serde_roundtrip(&diff_result);
    }

    #[test]
    fn test_timeline_serde_roundtrip() {
        let snapshot = TimelineSnapshot {
            commit_hash: "0123456789abcdef0123456789abcdef01234567".to_string(),
            short_hash: "0123456".to_string(),
            author: "Grigor Tonikyan".to_string(),
            commit_time: 1700000000,
            formatted_date: "2026-08-24 12:00:00".to_string(),
            message: "feat: add timeline".to_string(),
            tag: Some("v1.3.0".to_string()),
            total_files: 100,
            total_tokens: 50000,
            total_clones: 12,
            total_clusters: 4,
            duplication_percentage: 5.2,
            dry_health_score: 92.4,
        };
        assert_serde_roundtrip(&snapshot);

        let trend = TimelineTrend {
            snapshots: vec![snapshot],
            initial_score: 88.0,
            current_score: 92.4,
            score_delta: 4.4,
            duplication_delta: -2.1,
            churn_hotspots: vec![FileChurnMetric {
                file_path: "src/main.rs".to_string(),
                commit_count: 5,
                clone_count: 2,
            }],
        };
        assert_serde_roundtrip(&trend);
    }

    #[test]
    fn test_workflow_platform_display_and_serde() {
        let platforms = [
            WorkflowPlatform::Gitea,
            WorkflowPlatform::GitHub,
            WorkflowPlatform::GitLab,
            WorkflowPlatform::Azure,
        ];
        for platform in platforms {
            assert_serde_roundtrip(&platform);
        }
        assert_eq!(WorkflowPlatform::Gitea.to_string(), "Gitea Actions");
        assert_eq!(WorkflowPlatform::GitHub.to_string(), "GitHub Actions");
        assert_eq!(WorkflowPlatform::GitLab.to_string(), "GitLab CI");
        assert_eq!(WorkflowPlatform::Azure.to_string(), "Azure Pipelines");
    }

    #[test]
    fn test_hook_status_serde() {
        let hook_status = HookStatus {
            pre_commit_installed: true,
            pre_push_installed: false,
            hooks_dir: ".git/hooks".to_string(),
        };
        assert_serde_roundtrip(&hook_status);
    }

    #[test]
    fn test_suppression_types_serde() {
        let rule = SuppressionRule {
            pattern: "tests/**".to_string(),
            rule_type: "threshold".to_string(),
            min_tokens: Some(100),
            ignored_clone_types: vec![CloneType::Exact],
            line_number: 12,
        };
        assert_serde_roundtrip(&rule);

        let directive = SuppressionDirective {
            file_path: "src/auth.rs".to_string(),
            start_line: 10,
            end_line: 25,
            directive_type: "ignore_block".to_string(),
            reason: Some("Intentional duplicate API mock".to_string()),
        };
        assert_serde_roundtrip(&directive);

        let config = SuppressionConfig {
            rules: vec![rule],
            ignore_tests: true,
            ignore_mocks: true,
            ignore_generated: true,
            raw_cddmignore: Some("# .cddmignore\ntests/**\n".to_string()),
        };
        assert_serde_roundtrip(&config);
    }

    #[test]
    fn test_refactor_sandbox_types_serde() {
        let req = RefactorSandboxRequest {
            cluster_id: Some(1),
            occurrences: vec![CloneLocation {
                file: "src/a.rs".to_string(),
                start_line: 10,
                end_line: 20,
                author: None,
            }],
            custom_function_name: Some("custom_validate".to_string()),
            target_module_path: Some("src/utils.rs".to_string()),
            custom_parameter_names: Some(vec!["arg1".to_string(), "arg2".to_string()]),
        };
        assert_serde_roundtrip(&req);

        let res = RefactorSandboxResult {
            cluster_id: Some(1),
            function_name: "custom_validate".to_string(),
            target_module_path: "src/utils.rs".to_string(),
            unified_patch: "--- a/src/a.rs\n+++ b/src/a.rs\n...".to_string(),
            total_lines_saved: 25,
            sites_count: 2,
            affected_files: vec!["src/a.rs".to_string(), "src/b.rs".to_string()],
        };
        assert_serde_roundtrip(&res);

        let branch_res = ApplyRefactorBranchResult {
            success: true,
            branch_created: Some("cddm/refactor-cluster-1".to_string()),
            modified_files: vec!["src/a.rs".to_string(), "src/utils.rs".to_string()],
            hunks_applied: 2,
            message: "Refactor applied to branch cddm/refactor-cluster-1".to_string(),
        };
        assert_serde_roundtrip(&branch_res);
    }

    #[test]
    fn test_policy_types_serde() {
        let severities = [
            PolicySeverity::Error,
            PolicySeverity::Warning,
            PolicySeverity::Info,
        ];
        for s in severities {
            assert_serde_roundtrip(&s);
            assert_eq!(s.to_string(), s.as_ref());
        }

        let boundary = BoundaryRule {
            name: "domain-isolation".to_string(),
            description: Some("Prevent domain duplicates in presentation".to_string()),
            source: "src/domain/**".to_string(),
            forbidden_targets: vec!["src/presentation/**".to_string()],
            severity: PolicySeverity::Error,
        };
        assert_serde_roundtrip(&boundary);

        let zero_dup = ZeroDuplicationRule {
            name: "auth-zero-dup".to_string(),
            description: Some("Zero duplication in auth package".to_string()),
            pattern: "src/auth/**".to_string(),
            severity: PolicySeverity::Error,
        };
        assert_serde_roundtrip(&zero_dup);

        let limit = LimitRule {
            name: "max-api-clone".to_string(),
            description: None,
            pattern: "src/api/**".to_string(),
            max_tokens: Some(100),
            max_occurrences: Some(3),
            severity: PolicySeverity::Warning,
        };
        assert_serde_roundtrip(&limit);

        let config = PolicyConfig {
            boundaries: vec![boundary],
            zero_duplication: vec![zero_dup],
            limits: vec![limit],
            raw_toml: Some("[[boundaries]]\nname = \"domain-isolation\"\n".to_string()),
        };
        assert_serde_roundtrip(&config);

        let violation = PolicyViolation {
            rule_name: "domain-isolation".to_string(),
            rule_type: "boundary".to_string(),
            severity: PolicySeverity::Error,
            message: "Boundary violation detected".to_string(),
            file_a: "src/domain/user.rs".to_string(),
            start_line_a: 10,
            end_line_a: 25,
            file_b: Some("src/presentation/user.rs".to_string()),
            start_line_b: Some(30),
            end_line_b: Some(45),
            cluster_id: Some(1),
            token_count: 75,
        };
        assert_serde_roundtrip(&violation);

        let eval = PolicyEvaluationResult {
            passed: false,
            total_violations: 1,
            error_count: 1,
            warning_count: 0,
            info_count: 0,
            violations: vec![violation],
        };
        assert_serde_roundtrip(&eval);
    }
}
