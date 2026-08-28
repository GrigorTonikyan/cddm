#![forbid(unsafe_code)]

use super::make_test_req;
use crate::protocol::{mcp_methods, mcp_tools};
use crate::server::handle_mcp_request;
use serde_json::json;

#[tokio::test]
async fn test_mcp_generate_ai_prompt_tool() {
    let resp = handle_mcp_request(make_test_req(
        16,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::GENERATE_AI_PROMPT,
            "arguments": {
                "function_name": "compute_total",
                "target_module": "src/calc.rs",
                "occurrences": [
                    { "path": "src/a.rs", "start_line": 1, "end_line": 5, "snippet": "let x = a + b;" }
                ],
                "invariant_body": "let x = a + b;",
                "parameters": ["a", "b"]
            }
        })),
    ))
    .await
    .expect("Expected response");

    assert!(resp.error.is_none());
    let content = resp.result.unwrap()["content"][0]["text"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(content.contains("compute_total"));
    assert!(content.contains("src/calc.rs"));
}

#[tokio::test]
async fn test_mcp_diff_scan_missing_params() {
    let resp = handle_mcp_request(make_test_req(
        15,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::DIFF_SCAN,
            "arguments": {}
        })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.error.is_some());
}

#[tokio::test]
async fn test_mcp_check_suppression_tool() {
    let resp = handle_mcp_request(make_test_req(
        35,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::CHECK_SUPPRESSION,
            "arguments": { "path": "test_file.rs" }
        })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.error.is_none());
}

#[tokio::test]
async fn test_mcp_get_timeline_tool() {
    let resp = handle_mcp_request(make_test_req(
        24,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::GET_TIMELINE,
            "arguments": { "max_samples": 3, "min_tokens": 50 }
        })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.error.is_none());
}

#[tokio::test]
async fn test_mcp_cluster_refactor_explicit_occurrences() {
    use std::io::Write;
    let mut file_a = tempfile::NamedTempFile::new().unwrap();
    let mut file_b = tempfile::NamedTempFile::new().unwrap();
    let mut file_c = tempfile::NamedTempFile::new().unwrap();

    writeln!(file_a, "fn a() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
    writeln!(file_b, "fn b() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
    writeln!(file_c, "fn c() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();

    let resp = handle_mcp_request(make_test_req(
        30,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::SUGGEST_CLUSTER_REFACTOR,
            "arguments": {
                "occurrences": [
                    { "file": file_a.path().to_str().unwrap(), "start_line": 2, "end_line": 3 },
                    { "file": file_b.path().to_str().unwrap(), "start_line": 2, "end_line": 3 },
                    { "file": file_c.path().to_str().unwrap(), "start_line": 2, "end_line": 3 }
                ]
            }
        })),
    ))
    .await
    .expect("Expected response");

    assert!(resp.error.is_none());
}

#[tokio::test]
async fn test_mcp_ast_refactor_tool() {
    use std::io::Write;
    let mut file_a = tempfile::NamedTempFile::with_suffix(".rs").unwrap();
    let mut file_b = tempfile::NamedTempFile::with_suffix(".rs").unwrap();
    writeln!(file_a, "fn a() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
    writeln!(file_b, "fn b() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();

    let resp = handle_mcp_request(make_test_req(
        50,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::AST_REFACTOR,
            "arguments": {
                "custom_function_name": "mcp_ast_helper",
                "occurrences": [
                    { "path": file_a.path().to_str().unwrap(), "start_line": 2, "end_line": 3 },
                    { "path": file_b.path().to_str().unwrap(), "start_line": 2, "end_line": 3 }
                ]
            }
        })),
    ))
    .await
    .expect("Expected response");

    assert!(resp.error.is_none());
}

#[tokio::test]
async fn test_mcp_verify_refactor_tool() {
    let resp = handle_mcp_request(make_test_req(
        51,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::VERIFY_REFACTOR,
            "arguments": { "directory": ".", "test_command": "cargo --version" }
        })),
    ))
    .await
    .expect("Expected response");

    assert!(resp.error.is_none());
}

#[tokio::test]
async fn test_mcp_semantic_graph_tools() {
    let resp = handle_mcp_request(make_test_req(
        70,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::GET_SEMANTIC_GRAPH,
            "arguments": {
                "code": "fn test() { let a = 1; if a > 0 { let b = 2; } }",
                "language": "Rust"
            }
        })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.error.is_none());

    let resp_comp = handle_mcp_request(make_test_req(
        71,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::COMPARE_SEMANTIC_GRAPHS,
            "arguments": {
                "code_a": "pub fn calc(a: i32) -> i32 { let mut x = a; if x > 0 { x += 1; } return x; }",
                "code_b": "export function calc(b: number): number { let x = b; if (x > 0) { x += 1; } return x; }",
                "language_a": "Rust",
                "language_b": "TypeScript"
            }
        })),
    ))
    .await
    .expect("Expected response");
    assert!(resp_comp.error.is_none());

    let resp_scan = handle_mcp_request(make_test_req(
        72,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::SCAN_CROSS_LANGUAGE,
            "arguments": {
                "directory": ".",
                "threshold": 0.70
            }
        })),
    ))
    .await
    .expect("Expected response");
    assert!(resp_scan.error.is_none());
}

#[tokio::test]
async fn test_mcp_extract_shared_module_tool() {
    let resp = handle_mcp_request(make_test_req(
        73,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::EXTRACT_SHARED_MODULE,
            "arguments": {
                "directory": ".",
                "target": "crates/test_shared",
                "fn_name": "helper_test",
                "dry_run": true
            }
        })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.error.is_none());
}

#[tokio::test]
async fn test_mcp_detect_overlap_tool() {
    let resp = handle_mcp_request(make_test_req(
        74,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::DETECT_OVERLAP,
            "arguments": {
                "directory": ".",
                "threshold": 0.1
            }
        })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.error.is_none());
}
