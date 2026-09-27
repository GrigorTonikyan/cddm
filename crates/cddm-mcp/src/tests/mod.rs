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
        headers: None,
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
    assert!(res["capabilities"]["tasks"].is_object());
    assert!(res["capabilities"]["tasks"]["cancel"].as_bool().unwrap());
    assert!(res["capabilities"]["apps"].is_object());
    assert!(
        res["capabilities"]["apps"]["generativeUi"]
            .as_bool()
            .unwrap()
    );
    assert!(res["capabilities"]["sampling"].is_null());
    assert!(res["capabilities"]["roots"].is_null());

    let headers = resp.headers.unwrap();
    assert_eq!(headers.get("Mcp-Protocol-Version").unwrap(), "2026-07-28");
}

#[tokio::test]
async fn test_mcp_deprecated_roots_and_sampling_return_method_not_found() {
    for deprecated in ["sampling/createMessage", "roots/list"] {
        let resp = handle_mcp_request(make_test_req(15, deprecated, None))
            .await
            .expect("Expected response");
        assert_eq!(resp.id, Some(json!(15)));
        assert!(resp.error.is_some());
        let err = resp.error.unwrap();
        assert_eq!(err["code"], rpc_errors::METHOD_NOT_FOUND);
    }
}

#[tokio::test]
async fn test_mcp_tasks_framework() {
    // 1. Call task
    let call_resp = handle_mcp_request(make_test_req(
        20,
        mcp_methods::TASKS_CALL,
        Some(json!({
            "name": mcp_tools::SCAN_CODEBASE,
            "arguments": { "directory": "." }
        })),
    ))
    .await
    .expect("Expected response");
    assert!(call_resp.error.is_none());
    let call_res = call_resp.result.unwrap();
    let task_id = call_res["taskId"].as_str().unwrap();
    assert!(task_id.starts_with("task-"));
    assert_eq!(call_res["status"], "running");

    // 2. List tasks
    let list_resp = handle_mcp_request(make_test_req(21, mcp_methods::TASKS_LIST, None))
        .await
        .expect("Expected response");
    assert!(list_resp.error.is_none());
    let tasks = list_resp.result.unwrap()["tasks"]
        .as_array()
        .unwrap()
        .clone();
    assert!(tasks.iter().any(|t| t["taskId"] == task_id));

    // 3. Status check
    let status_resp = handle_mcp_request(make_test_req(
        22,
        mcp_methods::TASKS_STATUS,
        Some(json!({ "taskId": task_id })),
    ))
    .await
    .expect("Expected response");
    assert!(status_resp.error.is_none());
    let status_res = status_resp.result.unwrap();
    assert_eq!(status_res["taskId"], task_id);

    // 4. Cancel task
    let cancel_resp = handle_mcp_request(make_test_req(
        23,
        mcp_methods::TASKS_CANCEL,
        Some(json!({ "taskId": task_id })),
    ))
    .await
    .expect("Expected response");
    assert!(cancel_resp.error.is_none());
    let cancel_res = cancel_resp.result.unwrap();
    assert_eq!(cancel_res["taskId"], task_id);
    assert!(cancel_res["cancelled"].as_bool().unwrap());
}

#[tokio::test]
async fn test_mcp_header_based_routing() {
    let mut req = make_test_req(
        30,
        "mcp/dispatch",
        Some(json!({ "arguments": { "directory": "." } })),
    );
    let mut headers = std::collections::HashMap::new();
    headers.insert("Mcp-Method".to_string(), "tools/call".to_string());
    headers.insert("Mcp-Name".to_string(), mcp_tools::SCAN_CODEBASE.to_string());
    req.headers = Some(headers);

    let resp = handle_mcp_request(req).await.expect("Expected response");
    assert!(resp.error.is_none());
    let res = resp.result.unwrap();
    assert!(res["content"].is_array());

    let resp_headers = resp.headers.unwrap();
    assert_eq!(resp_headers.get("Mcp-Method").unwrap(), "tools/call");
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
    assert_eq!(tools.len(), 33);
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
        mcp_tools::SYNC_HUB,
        mcp_tools::CORRELATE_COVERAGE,
        mcp_tools::DETECT_DEAD_CLONES,
        mcp_tools::DETECT_DEAD_CODE,
        mcp_tools::PRUNE_DEAD_CLONES,
        mcp_tools::TRACE_REACHABILITY,
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
        headers: None,
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

#[tokio::test]
async fn test_mcp_apps_widgets_list() {
    let resp = handle_mcp_request(make_test_req(70, mcp_methods::APPS_WIDGETS_LIST, None))
        .await
        .expect("Expected response");
    assert_eq!(resp.id, Some(json!(70)));
    assert!(resp.error.is_none());

    let res = resp.result.unwrap();
    let widgets = res["widgets"].as_array().expect("widgets array");
    assert_eq!(widgets.len(), 2);
    assert!(widgets.iter().any(|w| w["id"] == "diff-split-view"));
    assert!(widgets.iter().any(|w| w["id"] == "cluster-treemap"));
}

#[tokio::test]
async fn test_mcp_apps_render_diff_split_view() {
    let params = json!({
        "widget_type": "diff-split-view",
        "title": "Refactoring Review",
        "file_a": "src/main.rs",
        "file_b": "src/extracted.rs",
        "original_code": "fn duplicate() { println!(\"old\"); }",
        "refactored_code": "fn extracted() { println!(\"new\"); }",
        "diff_patch": "--- a/src/main.rs\n+++ b/src/extracted.rs\n@@ -1 +1 @@\n-old\n+new"
    });

    let resp = handle_mcp_request(make_test_req(71, mcp_methods::APPS_RENDER, Some(params)))
        .await
        .expect("Expected response");
    assert_eq!(resp.id, Some(json!(71)));
    assert!(resp.error.is_none());

    let res = resp.result.unwrap();
    let widget = &res["widget"];
    assert_eq!(widget["type"], "app_widget");
    assert_eq!(widget["widgetType"], "diff-split-view");
    assert!(
        widget["html"]
            .as_str()
            .unwrap()
            .contains("Refactoring Review")
    );
    assert!(widget["html"].as_str().unwrap().contains("diff-table"));
}

#[tokio::test]
async fn test_mcp_apps_render_cluster_treemap() {
    let params = json!({
        "widget_type": "cluster-treemap",
        "title": "Codebase Duplication Treemap",
        "clusters": [
            {
                "id": 1,
                "name": "Cluster 1",
                "clone_type": "exact",
                "occurrence_count": 3,
                "token_count": 120,
                "files": ["crates/a.rs", "crates/b.rs"]
            }
        ],
        "dry_health_score": 92.5
    });

    let resp = handle_mcp_request(make_test_req(72, mcp_methods::APPS_RENDER, Some(params)))
        .await
        .expect("Expected response");
    assert_eq!(resp.id, Some(json!(72)));
    assert!(resp.error.is_none());

    let res = resp.result.unwrap();
    let widget = &res["widget"];
    assert_eq!(widget["type"], "app_widget");
    assert_eq!(widget["widgetType"], "cluster-treemap");
    assert!(widget["html"].as_str().unwrap().contains("Cluster 1"));
    assert!(widget["html"].as_str().unwrap().contains("treemap-grid"));
}
