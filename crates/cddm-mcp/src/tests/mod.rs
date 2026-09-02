#![forbid(unsafe_code)]

mod tool_tests;

use super::*;
use protocol::{
    JSONRPC_VERSION, MCP_PROTOCOL_VERSION, SERVER_NAME, mcp_methods, mcp_prompts, mcp_resources,
    mcp_tools,
};
use serde_json::json;

pub fn make_test_req(id: u64, method: &str, params: Option<serde_json::Value>) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id: Some(json!(id)),
        method: method.to_string(),
        params,
    }
}

pub async fn list_mcp_items(method: &'static str, key: &'static str) -> Vec<serde_json::Value> {
    let resp = handle_mcp_request(make_test_req(100, method, None))
        .await
        .expect("Expected response");
    resp.result.unwrap()[key].as_array().unwrap().to_vec()
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
    assert!(res["capabilities"]["sampling"].is_object());
}

#[tokio::test]
async fn test_mcp_sampling_create_message() {
    let resp = handle_mcp_request(make_test_req(
        15,
        mcp_methods::SAMPLING_CREATE_MESSAGE,
        Some(json!({
            "messages": [{"role": "user", "content": {"type": "text", "text": "test"}}]
        })),
    ))
    .await
    .expect("Expected response");
    assert_eq!(resp.id, Some(json!(15)));
    let res = resp.result.unwrap();
    assert_eq!(res["role"], "assistant");
    assert!(
        res["content"]["text"]
            .as_str()
            .unwrap()
            .contains("sampling")
    );
}

#[tokio::test]
async fn test_mcp_ping() {
    let resp = handle_mcp_request(make_test_req(2, mcp_methods::PING, None))
        .await
        .expect("Expected response");
    assert_eq!(resp.id, Some(json!(2)));
    assert!(resp.result.is_some());
}

#[tokio::test]
async fn test_mcp_tools_list() {
    let tools = list_mcp_items(mcp_methods::TOOLS_LIST, "tools").await;
    assert_eq!(tools.len(), 31);
    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    for expected in [
        mcp_tools::SCAN_CODEBASE,
        mcp_tools::GET_CLONE_PAIR,
        mcp_tools::SUGGEST_REFACTOR,
        mcp_tools::GET_CLONE_CLUSTER,
        mcp_tools::SUGGEST_CLUSTER_REFACTOR,
        mcp_tools::EXPORT_SARIF,
        mcp_tools::DIFF_SCAN,
        mcp_tools::GET_TIMELINE,
        mcp_tools::CHECK_SUPPRESSION,
        mcp_tools::APPLY_CLUSTER_REFACTOR,
        mcp_tools::GENERATE_AI_PROMPT,
        mcp_tools::AST_REFACTOR,
        mcp_tools::VERIFY_REFACTOR,
        mcp_tools::CHECK_POLICIES,
        mcp_tools::HEAL_REFACTOR,
        mcp_tools::EXPORT_CACHE_PACK,
        mcp_tools::IMPORT_CACHE_PACK,
        mcp_tools::SCAN_MONOREPO,
        mcp_tools::GET_SEMANTIC_GRAPH,
        mcp_tools::COMPARE_SEMANTIC_GRAPHS,
        mcp_tools::SCAN_CROSS_LANGUAGE,
        mcp_tools::EXTRACT_SHARED_MODULE,
        mcp_tools::DETECT_OVERLAP,
        mcp_tools::SCAN_HUB,
        mcp_tools::EXTRACT_HUB_PACKAGE,
        mcp_tools::CORRELATE_COVERAGE,
        mcp_tools::DETECT_DEAD_CLONES,
        mcp_tools::DETECT_DEAD_CODE,
        mcp_tools::PRUNE_DEAD_CLONES,
        mcp_tools::SEMANTIC_NEURAL_SCAN,
        mcp_tools::DIFF_MATRIX,
    ] {
        assert!(tool_names.contains(&expected));
    }
}

#[tokio::test]
async fn test_mcp_resources_list() {
    let resources = list_mcp_items(mcp_methods::RESOURCES_LIST, "resources").await;
    assert_eq!(resources.len(), 14);
}

async fn assert_resource_readable(uri: &str) {
    let resp = handle_mcp_request(make_test_req(
        1,
        mcp_methods::RESOURCES_READ,
        Some(json!({ "uri": uri })),
    ))
    .await
    .expect("Expected response");
    assert!(resp.result.is_some());
}

#[tokio::test]
async fn test_mcp_resources_read_endpoints() {
    for uri in [
        mcp_resources::URI_WORKSPACE_WATCH_STATUS,
        mcp_resources::URI_WORKSPACE_SUPPRESSIONS,
        mcp_resources::URI_WORKSPACE_TIMELINE,
        mcp_resources::URI_WORKSPACE_CLUSTERS,
        mcp_resources::URI_WORKSPACE_DEAD_CODE,
    ] {
        assert_resource_readable(uri).await;
    }
}

#[tokio::test]
async fn test_mcp_prompts_list_and_get() {
    let prompts = list_mcp_items(mcp_methods::PROMPTS_LIST, "prompts").await;
    assert_eq!(prompts.len(), 3);

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
}

#[tokio::test]
async fn test_mcp_check_policies_tool_and_resource() {
    let tools = list_mcp_items(mcp_methods::TOOLS_LIST, "tools").await;
    assert!(tools.iter().any(|t| t["name"] == mcp_tools::CHECK_POLICIES));

    let resp_call = handle_mcp_request(make_test_req(
        60,
        mcp_methods::TOOLS_CALL,
        Some(json!({
            "name": mcp_tools::CHECK_POLICIES,
            "arguments": { "directory": "." }
        })),
    ))
    .await
    .expect("Expected response");

    assert!(resp_call.error.is_none());
}
