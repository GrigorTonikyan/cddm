#![forbid(unsafe_code)]

pub mod clone_tools;
pub mod helpers;
pub mod policy_tools;
pub mod refactor_tools;
pub mod scan_tools;
pub mod schemas;
pub mod semantic_tools;

use crate::protocol::{
    JSONRPC_VERSION, JsonRpcResponse, make_error_response, mcp_tools, rpc_errors,
};
use serde_json::json;

/// Generates the MCP tools list response containing all registered tool schemas.
pub fn tools_list_response(id: Option<serde_json::Value>) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
        result: Some(json!({
            "tools": schemas::get_tool_definitions()
        })),
        error: None,
    }
}

/// Dispatches an incoming MCP tool invocation by name to its corresponding handler.
pub async fn dispatch_tool_call(
    id: Option<serde_json::Value>,
    params: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let tool_name = params
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("");
    let args = params.and_then(|p| p.get("arguments"));

    match tool_name {
        mcp_tools::SCAN_CODEBASE => scan_tools::handle_scan_codebase(id, args).await,
        mcp_tools::DIFF_SCAN => scan_tools::handle_diff_scan(id, args).await,
        mcp_tools::GET_CLONE_PAIR => clone_tools::handle_get_clone_pair(id, args),
        mcp_tools::SUGGEST_REFACTOR => refactor_tools::handle_suggest_refactor(id, args),
        mcp_tools::GET_CLONE_CLUSTER => clone_tools::handle_get_clone_cluster(id, args).await,
        mcp_tools::SUGGEST_CLUSTER_REFACTOR => {
            refactor_tools::handle_suggest_cluster_refactor(id, args).await
        }
        mcp_tools::EXPORT_SARIF => scan_tools::handle_export_sarif(id, args).await,
        mcp_tools::GET_TIMELINE => scan_tools::handle_get_timeline(id, args),
        mcp_tools::CHECK_SUPPRESSION => policy_tools::handle_check_suppression(id, args),
        mcp_tools::APPLY_CLUSTER_REFACTOR => {
            refactor_tools::handle_apply_cluster_refactor(id, args)
        }
        mcp_tools::GENERATE_AI_PROMPT => refactor_tools::handle_generate_ai_prompt(id, args),
        mcp_tools::AST_REFACTOR => refactor_tools::handle_ast_refactor(id, args),
        mcp_tools::VERIFY_REFACTOR => refactor_tools::handle_verify_refactor(id, args),
        mcp_tools::CHECK_POLICIES => policy_tools::handle_check_policies(id, args).await,
        mcp_tools::HEAL_REFACTOR => refactor_tools::handle_heal_refactor(id, args).await,
        mcp_tools::EXPORT_CACHE_PACK => scan_tools::handle_export_cache_pack(id, args),
        mcp_tools::IMPORT_CACHE_PACK => scan_tools::handle_import_cache_pack(id, args),
        mcp_tools::SCAN_MONOREPO => scan_tools::handle_scan_monorepo(id, args).await,
        mcp_tools::GET_SEMANTIC_GRAPH => semantic_tools::handle_get_semantic_graph(id, args),
        mcp_tools::COMPARE_SEMANTIC_GRAPHS => {
            semantic_tools::handle_compare_semantic_graphs(id, args)
        }
        _ => make_error_response(
            id,
            rpc_errors::METHOD_NOT_FOUND,
            format!("Tool '{}' not found", tool_name),
        ),
    }
}
