#![forbid(unsafe_code)]

use super::*;
use protocol::{
    JSONRPC_VERSION, MCP_PROTOCOL_VERSION, SERVER_NAME, mcp_methods, mcp_prompts, mcp_resources,
    mcp_tools,
};
use serde_json::json;

fn make_test_req(id: u64, method: &str, params: Option<serde_json::Value>) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id: Some(json!(id)),
        method: method.to_string(),
        params,
    }
}

#[tokio::test]
async fn test_mcp_initialize() {
    let resp = handle_mcp_request(make_test_req(1, mcp_methods::INITIALIZE, None))
        .await
        .expect("Expected response");
    assert_eq!(resp.jsonrpc, JSONRPC_VERSION);
    assert_eq!(resp.id, Some(json!(1)));
    assert!(resp.error.is_none());

    let res = resp.result.unwrap();
    assert_eq!(res["protocolVersion"], MCP_PROTOCOL_VERSION);
    assert_eq!(res["serverInfo"]["name"], SERVER_NAME);
    assert!(res["capabilities"]["tools"].is_object());
    assert!(res["capabilities"]["resources"].is_object());
    assert!(res["capabilities"]["prompts"].is_object());
}

#[tokio::test]
async fn test_mcp_ping() {
    let resp = handle_mcp_request(make_test_req(2, mcp_methods::PING, None))
        .await
        .expect("Expected response");
    assert_eq!(resp.id, Some(json!(2)));
    assert!(resp.result.is_some());
}

async fn list_mcp_items(method: &'static str, key: &'static str) -> Vec<serde_json::Value> {
    let resp = handle_mcp_request(make_test_req(100, method, None))
        .await
        .expect("Expected response");
    resp.result.unwrap()[key].as_array().unwrap().to_vec()
}

#[tokio::test]
async fn test_mcp_tools_list() {
    let tools = list_mcp_items(mcp_methods::TOOLS_LIST, "tools").await;
    assert_eq!(tools.len(), 14);
    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&mcp_tools::SCAN_CODEBASE));
    assert!(tool_names.contains(&mcp_tools::GET_CLONE_PAIR));
    assert!(tool_names.contains(&mcp_tools::SUGGEST_REFACTOR));
    assert!(tool_names.contains(&mcp_tools::GET_CLONE_CLUSTER));
    assert!(tool_names.contains(&mcp_tools::SUGGEST_CLUSTER_REFACTOR));
    assert!(tool_names.contains(&mcp_tools::EXPORT_SARIF));
    assert!(tool_names.contains(&mcp_tools::DIFF_SCAN));
    assert!(tool_names.contains(&mcp_tools::GET_TIMELINE));
    assert!(tool_names.contains(&mcp_tools::CHECK_SUPPRESSION));
    assert!(tool_names.contains(&mcp_tools::APPLY_CLUSTER_REFACTOR));
    assert!(tool_names.contains(&mcp_tools::GENERATE_AI_PROMPT));
    assert!(tool_names.contains(&mcp_tools::AST_REFACTOR));
    assert!(tool_names.contains(&mcp_tools::VERIFY_REFACTOR));
    assert!(tool_names.contains(&mcp_tools::CHECK_POLICIES));
}

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
                    {
                        "path": "src/a.rs",
                        "start_line": 1,
                        "end_line": 5,
                        "snippet": "let x = a + b;"
                    }
                ],
                "invariant_body": "let x = a + b;",
                "parameters": ["a", "b"]
            }
        })),
    ))
    .await
    .expect("Expected response");

    assert!(resp.error.is_none());
    let res = resp.result.unwrap();
    let content = res["content"][0]["text"].as_str().unwrap();
    assert!(content.contains("compute_total"));
    assert!(content.contains("src/calc.rs"));
    assert!(content.contains("src/a.rs:1-5"));
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
    assert_eq!(
        resp.error.unwrap()["code"].as_i64().unwrap(),
        rpc_errors::INVALID_PARAMS
    );
}

#[tokio::test]
async fn test_mcp_check_suppression_tool() {
    let resp = handle_mcp_request(make_test_req(
        35,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::CHECK_SUPPRESSION,
            "arguments": {
                "path": "test_file.rs"
            }
        })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.error.is_none());
    let text = resp.result.unwrap()["content"][0]["text"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(text.contains("path_ignored"));
}

#[tokio::test]
async fn test_mcp_resources_list() {
    let resources = list_mcp_items(mcp_methods::RESOURCES_LIST, "resources").await;
    assert_eq!(resources.len(), 6);
    assert_eq!(resources[0]["uri"], mcp_resources::URI_WORKSPACE_HEALTH);
    assert_eq!(resources[1]["uri"], mcp_resources::URI_WORKSPACE_CLONES);
    assert_eq!(resources[2]["uri"], mcp_resources::URI_WORKSPACE_CLUSTERS);
    assert_eq!(resources[3]["uri"], mcp_resources::URI_WORKSPACE_TIMELINE);
    assert_eq!(
        resources[4]["uri"],
        mcp_resources::URI_WORKSPACE_SUPPRESSIONS
    );
    assert_eq!(resources[5]["uri"], mcp_resources::URI_WORKSPACE_POLICIES);
}

#[tokio::test]
async fn test_mcp_resources_read_suppressions() {
    let resp = handle_mcp_request(make_test_req(
        36,
        mcp_methods::RESOURCES_READ,
        Some(json!({ "uri": mcp_resources::URI_WORKSPACE_SUPPRESSIONS })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.result.is_some());
    let contents = resp.result.unwrap()["contents"].as_array().unwrap().clone();
    assert_eq!(contents.len(), 1);
    assert_eq!(
        contents[0]["uri"],
        mcp_resources::URI_WORKSPACE_SUPPRESSIONS
    );
}

#[tokio::test]
async fn test_mcp_resources_read_timeline() {
    let resp = handle_mcp_request(make_test_req(
        23,
        mcp_methods::RESOURCES_READ,
        Some(json!({ "uri": mcp_resources::URI_WORKSPACE_TIMELINE })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.result.is_some());
    let contents = resp.result.unwrap()["contents"].as_array().unwrap().clone();
    assert_eq!(contents.len(), 1);
    assert_eq!(contents[0]["uri"], mcp_resources::URI_WORKSPACE_TIMELINE);
}

#[tokio::test]
async fn test_mcp_get_timeline_tool() {
    let resp = handle_mcp_request(make_test_req(
        24,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::GET_TIMELINE,
            "arguments": {
                "max_samples": 3,
                "min_tokens": 50
            }
        })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.error.is_none());
    let text = resp.result.unwrap()["content"][0]["text"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(text.contains("snapshots"));
}

#[tokio::test]
async fn test_mcp_resources_read_clusters() {
    let resp = handle_mcp_request(make_test_req(
        22,
        mcp_methods::RESOURCES_READ,
        Some(json!({ "uri": mcp_resources::URI_WORKSPACE_CLUSTERS })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.result.is_some());
    let contents = resp.result.unwrap()["contents"].as_array().unwrap().clone();
    assert_eq!(contents.len(), 1);
    assert_eq!(contents[0]["uri"], mcp_resources::URI_WORKSPACE_CLUSTERS);
}

#[tokio::test]
async fn test_mcp_cluster_refactor_explicit_occurrences() {
    let mut file_a = tempfile::NamedTempFile::new().unwrap();
    let mut file_b = tempfile::NamedTempFile::new().unwrap();
    let mut file_c = tempfile::NamedTempFile::new().unwrap();

    use std::io::Write;
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
                    {
                        "file": file_a.path().to_str().unwrap(),
                        "start_line": 2,
                        "end_line": 3
                    },
                    {
                        "file": file_b.path().to_str().unwrap(),
                        "start_line": 2,
                        "end_line": 3
                    },
                    {
                        "file": file_c.path().to_str().unwrap(),
                        "start_line": 2,
                        "end_line": 3
                    }
                ]
            }
        })),
    ))
    .await
    .expect("Expected response");

    assert!(resp.error.is_none());
    let text = resp.result.unwrap()["content"][0]["text"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(text.contains("extracted_shared_helper"));
    assert!(text.contains("--- a/"));
}

#[tokio::test]
async fn test_mcp_prompts_list_and_get() {
    let prompts = list_mcp_items(mcp_methods::PROMPTS_LIST, "prompts").await;
    assert_eq!(prompts.len(), 2);

    let resp_get = handle_mcp_request(make_test_req(
        6,
        mcp_methods::PROMPTS_GET,
        Some(json!({ "name": mcp_prompts::AUDIT_DRY_HEALTH })),
    ))
    .await
    .expect("Expected response");
    assert!(resp_get.result.unwrap()["messages"].is_array());
}

#[tokio::test]
async fn test_mcp_unknown_method() {
    let resp = handle_mcp_request(make_test_req(99, "nonexistent_method", None))
        .await
        .expect("Expected response");
    assert!(resp.error.is_some());
    assert_eq!(
        resp.error.unwrap()["code"].as_i64().unwrap(),
        rpc_errors::METHOD_NOT_FOUND
    );
}

#[tokio::test]
async fn test_mcp_notification_returns_none() {
    let req = JsonRpcRequest {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id: None,
        method: mcp_methods::INITIALIZED.to_string(),
        params: None,
    };
    let resp = handle_mcp_request(req).await;
    assert!(resp.is_none());

    let unknown_notif = JsonRpcRequest {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id: None,
        method: "some/unknown_notification".to_string(),
        params: None,
    };
    let resp_unknown = handle_mcp_request(unknown_notif).await;
    assert!(resp_unknown.is_none());
}

#[tokio::test]
async fn test_mcp_resource_templates_list() {
    let resp = handle_mcp_request(make_test_req(
        16,
        mcp_methods::RESOURCES_TEMPLATES_LIST,
        None,
    ))
    .await
    .expect("Expected response");
    assert!(resp.result.is_some());
    assert!(resp.result.unwrap()["resourceTemplates"].is_array());
}

#[tokio::test]
async fn test_mcp_ast_refactor_tool() {
    use std::io::Write;
    use tempfile::NamedTempFile;

    let mut file_a = NamedTempFile::with_suffix(".rs").unwrap();
    let mut file_b = NamedTempFile::with_suffix(".rs").unwrap();
    writeln!(file_a, "fn a() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
    writeln!(file_b, "fn b() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
    file_a.flush().unwrap();
    file_b.flush().unwrap();

    let resp = handle_mcp_request(make_test_req(
        50,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::AST_REFACTOR,
            "arguments": {
                "custom_function_name": "mcp_ast_helper",
                "occurrences": [
                    {
                        "path": file_a.path().to_str().unwrap(),
                        "start_line": 2,
                        "end_line": 3
                    },
                    {
                        "path": file_b.path().to_str().unwrap(),
                        "start_line": 2,
                        "end_line": 3
                    }
                ]
            }
        })),
    ))
    .await
    .expect("Expected response");

    assert!(resp.error.is_none());
    let text = resp.result.unwrap()["content"][0]["text"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(text.contains("mcp_ast_helper"));
}

#[tokio::test]
async fn test_mcp_verify_refactor_tool() {
    let resp = handle_mcp_request(make_test_req(
        51,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::VERIFY_REFACTOR,
            "arguments": {
                "directory": ".",
                "test_command": "cargo --version"
            }
        })),
    ))
    .await
    .expect("Expected response");

    assert!(resp.error.is_none());
    let text = resp.result.unwrap()["content"][0]["text"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(text.contains("cargo"));
}

#[tokio::test]
async fn test_mcp_check_policies_tool_and_resource() {
    let tools = list_mcp_items(mcp_methods::TOOLS_LIST, "tools").await;
    assert!(tools.iter().any(|t| t["name"] == mcp_tools::CHECK_POLICIES));

    let resources = list_mcp_items(mcp_methods::RESOURCES_LIST, "resources").await;
    assert!(
        resources
            .iter()
            .any(|r| r["uri"] == mcp_resources::URI_WORKSPACE_POLICIES)
    );

    let resp_call = handle_mcp_request(make_test_req(
        60,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::CHECK_POLICIES,
            "arguments": {
                "directory": "."
            }
        })),
    ))
    .await
    .expect("Expected response");

    assert!(resp_call.error.is_none());

    let resp_read = handle_mcp_request(make_test_req(
        61,
        mcp_methods::RESOURCES_READ,
        Some(json!({
            "uri": mcp_resources::URI_WORKSPACE_POLICIES
        })),
    ))
    .await
    .expect("Expected response");

    assert!(resp_read.error.is_none());
}
