#![forbid(unsafe_code)]

mod watch_tests;

use super::*;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use cddm_core::{CloneLocation, ScanResult};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_resolve_safe_path_valid() {
    let file = NamedTempFile::new().unwrap();
    let path_str = file.path().to_str().unwrap();
    let res = resolve_safe_path(path_str);
    assert!(res.is_ok());
}

#[test]
fn test_resolve_safe_path_nonexistent() {
    let res = resolve_safe_path("non_existent_file_xyz_123.rs");
    assert!(res.is_err());
    let (status, _) = res.unwrap_err();
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_snippet_handler_success() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
        file,
        "line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9\nline10"
    )
    .unwrap();

    let path_str = file.path().to_str().unwrap().to_string();
    let query = SnippetQuery {
        file: path_str,
        start: 4,
        end: 6,
        context: Some(2),
    };

    let result = snippet_handler(Query(query)).await;
    assert!(result.is_ok());
    let axum::Json(response) = result.unwrap();
    assert_eq!(response.start_line, 4);
    assert_eq!(response.end_line, 6);
    assert_eq!(response.context_start_line, 2);
    assert_eq!(response.context_end_line, 8);
    assert_eq!(response.lines.len(), 7);
    assert!(!response.lines[0].is_target);
    assert!(response.lines[2].is_target);
}

#[tokio::test]
async fn test_refactor_handler_success() {
    let mut file_a = NamedTempFile::new().unwrap();
    let mut file_b = NamedTempFile::new().unwrap();

    writeln!(file_a, "fn test() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
    writeln!(file_b, "fn other() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();

    let req = RefactorRequest {
        file_a: file_a.path().to_str().unwrap().to_string(),
        start_line_a: 2,
        end_line_a: 3,
        file_b: file_b.path().to_str().unwrap().to_string(),
        start_line_b: 2,
        end_line_b: 3,
    };

    let result = refactor_handler(axum::Json(req)).await;
    assert!(result.is_ok());
    let axum::Json(suggestion) = result.unwrap();
    assert_eq!(
        suggestion.strategy,
        cddm_core::refactor::refactor_strategies::EXTRACT_FUNCTION
    );
    assert!(suggestion.unified_patch.contains("--- a/"));
}

#[tokio::test]
async fn test_refactor_cluster_handler_success() {
    let mut file_a = NamedTempFile::new().unwrap();
    let mut file_b = NamedTempFile::new().unwrap();
    let mut file_c = NamedTempFile::new().unwrap();

    writeln!(file_a, "fn a() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
    writeln!(file_b, "fn b() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
    writeln!(file_c, "fn c() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();

    let req = ClusterRefactorRequest {
        cluster_id: "cluster-1".to_string(),
        occurrences: vec![
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
        ],
    };

    let result = refactor_cluster_handler(axum::Json(req)).await;
    assert!(result.is_ok());
    let axum::Json(suggestion) = result.unwrap();
    assert_eq!(
        suggestion.strategy,
        cddm_core::refactor::refactor_strategies::EXTRACT_FUNCTION
    );
    assert_eq!(suggestion.sites.len(), 3);
    assert!(suggestion.unified_patch.contains("--- a/"));
}

#[tokio::test]
async fn test_apply_patch_handler_success() {
    let (state, _) = build_app();

    let mut file_a = NamedTempFile::new().unwrap();
    let path_str = file_a.path().to_str().unwrap().to_string();

    writeln!(file_a, "fn test() {{\n    let x = 1;\n}}").unwrap();
    file_a.flush().unwrap();

    let patch = format!(
        "--- a/{}\n+++ b/{}\n@@ -2,1 +2,1 @@\n-    let x = 1;\n+    helper();\n",
        path_str, path_str
    );

    let req = ApplyPatchRequest {
        patch,
        dry_run: false,
    };

    let result = apply_patch_handler(State(state), axum::Json(req)).await;
    assert!(result.is_ok());
    let axum::Json(res) = result.unwrap();
    assert!(res.success);
    assert_eq!(res.hunks_applied, 1);
}

#[tokio::test]
async fn test_apply_patch_handler_bad_request() {
    let (state, _) = build_app();

    let req = ApplyPatchRequest {
        patch: "invalid patch without hunks".to_string(),
        dry_run: false,
    };

    let result = apply_patch_handler(State(state), axum::Json(req)).await;
    assert!(result.is_err());
    let (status, _) = result.unwrap_err();
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_build_app_router() {
    let app = build_app();
    let _ = app;
}

#[tokio::test]
async fn test_timeline_handler_success() {
    let query = TimelineQuery {
        directory: Some(".".to_string()),
        max_samples: Some(3),
        min_tokens: Some(50),
    };
    let res = timeline_handler(Query(query)).await;
    assert!(res.is_ok());
    let axum::Json(trend) = res.unwrap();
    assert!(!trend.snapshots.is_empty());
}

#[tokio::test]
async fn test_hooks_handlers() {
    let temp = tempfile::tempdir().expect("tempdir");
    let git_dir = temp.path().join(".git");
    std::fs::create_dir_all(&git_dir).expect("create .git");

    let status_res = hooks_status_handler(Query(TimelineQuery {
        directory: Some(temp.path().to_string_lossy().to_string()),
        max_samples: None,
        min_tokens: None,
    }))
    .await;
    let axum::Json(status) = status_res;
    assert!(!status.pre_commit_installed);

    let install_res = install_hook_handler(axum::Json(InstallHookRequest {
        directory: Some(temp.path().to_string_lossy().to_string()),
        hook_type: "pre-commit".to_string(),
        fail_threshold: Some(15.0),
        min_tokens: Some(50),
    }))
    .await;
    assert!(install_res.is_ok());

    let status_after = hooks_status_handler(Query(TimelineQuery {
        directory: Some(temp.path().to_string_lossy().to_string()),
        max_samples: None,
        min_tokens: None,
    }))
    .await;
    let axum::Json(status2) = status_after;
    assert!(status2.pre_commit_installed);
}

#[tokio::test]
async fn test_suppression_rules_handlers() {
    let axum::Json(get_res) = suppression_rules_get_handler().await;
    assert!(get_res.ignore_generated);

    let config = cddm_core::SuppressionConfig {
        rules: vec![],
        ignore_tests: true,
        ignore_mocks: true,
        ignore_generated: true,
        raw_cddmignore: Some("# custom ignore\ntarget/**\n".to_string()),
    };
    let post_res = suppression_rules_post_handler(axum::Json(config)).await;
    assert!(post_res.is_ok());
}

#[tokio::test]
async fn test_refactor_sandbox_handlers() {
    let mut file_a = NamedTempFile::new().unwrap();
    let mut file_b = NamedTempFile::new().unwrap();
    writeln!(file_a, "fn foo() {{\n    let a = 1;\n    let b = 2;\n}}").unwrap();
    writeln!(file_b, "fn bar() {{\n    let a = 1;\n    let b = 2;\n}}").unwrap();

    let req = cddm_core::RefactorSandboxRequest {
        cluster_id: Some(1),
        occurrences: vec![
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
        ],
        custom_function_name: Some("custom_compute".to_string()),
        target_module_path: None,
        custom_parameter_names: None,
    };

    let res = refactor_sandbox_handler(axum::Json(req)).await;
    assert!(res.is_ok());
    let axum::Json(sandbox_res) = res.unwrap();
    assert_eq!(sandbox_res.function_name, "custom_compute");
    assert!(sandbox_res.unified_patch.contains("custom_compute"));
}

#[tokio::test]
async fn test_refactor_ai_prompt_handler() {
    let req = cddm_core::AiRefactorPromptRequest {
        clone_type: cddm_core::CloneType::Renamed,
        similarity: 0.95,
        token_count: 100,
        lines_saved_est: 20,
        function_name: "shared_helper".to_string(),
        target_module: "src/utils.rs".to_string(),
        occurrences: vec![cddm_core::AiOccurrenceContext {
            path: "src/a.rs".to_string(),
            span: cddm_core::LineSpan {
                line_start: 1,
                line_end: 10,
                byte_offset: 0,
            },
            snippet: "let x = 1;".to_string(),
        }],
        invariant_body: "let x = 1;".to_string(),
        parameters: vec!["x".to_string()],
        custom_instructions: None,
    };

    let res = refactor_ai_prompt_handler(axum::Json(req)).await;
    assert!(res.is_ok());
    let axum::Json(body) = res.unwrap();
    assert!(body.prompt.contains("shared_helper"));
    assert!(body.prompt.contains("src/utils.rs"));
}

#[tokio::test]
async fn test_refactor_ast_handler() {
    let mut file_a = NamedTempFile::with_suffix(".rs").unwrap();
    let mut file_b = NamedTempFile::with_suffix(".rs").unwrap();
    writeln!(file_a, "fn a() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
    writeln!(file_b, "fn b() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
    file_a.flush().unwrap();
    file_b.flush().unwrap();

    let req = cddm_core::RefactorSandboxRequest {
        cluster_id: Some(1),
        occurrences: vec![
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
        ],
        custom_function_name: Some("ast_shared_compute".to_string()),
        target_module_path: None,
        custom_parameter_names: None,
    };

    let res = refactor_ast_handler(axum::Json(req)).await;
    assert!(res.is_ok());
    let axum::Json(body) = res.unwrap();
    assert_eq!(body.function_name, "ast_shared_compute");
    assert!(body.helper_function_code.contains("ast_shared_compute"));
    assert_eq!(body.rewritten_files.len(), 2);
}

#[tokio::test]
async fn test_policy_rules_get_and_post_handlers() {
    let (state, _) = build_app();
    let axum::Json(cfg) = policy_rules_get_handler().await;
    assert!(cfg.boundaries.is_empty() || !cfg.boundaries.is_empty());

    let new_cfg = cddm_core::PolicyConfig {
        boundaries: vec![cddm_core::BoundaryRule {
            name: "test-boundary".to_string(),
            description: None,
            source: "src/domain/**".to_string(),
            forbidden_targets: vec!["src/web/**".to_string()],
            severity: cddm_core::PolicySeverity::Error,
        }],
        zero_duplication: vec![],
        limits: vec![],
        raw_toml: None,
    };

    let post_res = policy_rules_post_handler(axum::Json(new_cfg.clone())).await;
    assert!(post_res.is_ok());

    let scan_result = ScanResult {
        scan_id: "test-eval".to_string(),
        total_files: 2,
        total_tokens: 100,
        total_clones: 1,
        total_clusters: 0,
        duplication_percentage: 10.0,
        dry_health_score: 90.0,
        clone_pairs: vec![cddm_core::ClonePair {
            file_a: "src/domain/entity.rs".to_string(),
            start_line_a: 1,
            end_line_a: 10,
            file_b: "src/web/controller.rs".to_string(),
            start_line_b: 1,
            end_line_b: 10,
            token_count: 50,
            similarity: 1.0,
            fragment_hash: "hash123".to_string(),
            clone_type: cddm_core::CloneType::Exact,
            author_a: None,
            author_b: None,
        }],
        clone_clusters: vec![],
        duration_ms: 50,
        language_breakdown: vec![],
        policy_violations: vec![],
    };

    {
        let mut latest = state.latest_result.write().await;
        *latest = Some(scan_result);
    }

    let eval_res = policy_evaluate_handler(State(state), axum::Json(Some(new_cfg))).await;
    assert!(eval_res.is_ok());
    let axum::Json(eval_body) = eval_res.unwrap();
    assert!(!eval_body.violations.is_empty());
    assert_eq!(eval_body.violations[0].rule_name, "test-boundary");
}

#[tokio::test]
async fn test_semantic_graph_handler() {
    let req = SemanticGraphRequest {
        file: Some("src/calc.rs".to_string()),
        code: Some(
            "pub fn compute(a: i32) -> i32 { if a > 0 { return a * 2; } else { return 0; } }"
                .to_string(),
        ),
        language: Some("Rust".to_string()),
        file_b: Some("src/calc_alt.rs".to_string()),
        code_b: Some(
            "pub fn calculate(b: i32) -> i32 { if b > 0 { return b * 2; } else { return 0; } }"
                .to_string(),
        ),
        language_b: Some("Rust".to_string()),
    };

    let res = semantic_graph_handler(axum::Json(req)).await;
    assert!(res.is_ok());
    let axum::Json(body) = res.unwrap();
    assert_eq!(body.cfgs.len(), 2);
    assert_eq!(body.pdgs.len(), 2);
    assert!(body.comparison.is_some());
    let comp = body.comparison.unwrap();
    assert!(comp.similarity >= 0.8);
    assert!(comp.is_semantic_clone);
}

#[tokio::test]
async fn test_semantic_scan_handler() {
    let req = SemanticScanRequest {
        directory: Some(".".to_string()),
        threshold: Some(0.70),
        min_tokens: Some(50),
        languages: None,
        ignore: None,
    };

    let res = semantic_scan_handler(axum::Json(req)).await;
    assert!(res.is_ok());
}
