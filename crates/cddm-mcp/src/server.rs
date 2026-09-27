#![forbid(unsafe_code)]

use std::collections::HashMap;

use crate::prompts::{handle_prompt_get, prompts_list_response};
use crate::protocol::{
    JSONRPC_VERSION, JsonRpcRequest, JsonRpcResponse, MCP_PROTOCOL_VERSION, SERVER_NAME,
    make_error_response, mcp_methods, rpc_errors,
};
use crate::resources::{
    handle_resource_read, resources_list_response, resources_templates_list_response,
};
use crate::tasks::{
    handle_tasks_call, handle_tasks_cancel, handle_tasks_list, handle_tasks_status,
};
use crate::tools::{dispatch_tool_call, tools_list_response};
use serde_json::json;

/// Extracts an optional case-insensitive header from request top-level headers or `_meta.headers`.
fn extract_header<'a>(req: &'a JsonRpcRequest, key: &str) -> Option<&'a str> {
    if let Some(ref headers) = req.headers {
        for (k, v) in headers {
            if k.eq_ignore_ascii_case(key) {
                return Some(v.as_str());
            }
        }
    }
    if let Some(params) = req.params.as_ref()
        && let Some(meta) = params.get("_meta").and_then(|m| m.get("headers"))
        && let Some(obj) = meta.as_object()
    {
        for (k, v) in obj {
            if k.eq_ignore_ascii_case(key) {
                return v.as_str();
            }
        }
    }
    None
}

/// Dispatches an incoming MCP JSON-RPC request and returns the response if not a notification.
pub async fn handle_mcp_request(req: JsonRpcRequest) -> Option<JsonRpcResponse> {
    // 1. Resolve header-based routing: Mcp-Method overrides generic or dispatch methods
    let effective_method = extract_header(&req, "Mcp-Method")
        .map(|s| s.to_string())
        .unwrap_or_else(|| req.method.clone());

    // 2. Resolve header-based tool/task routing: Mcp-Name populates name if missing
    let mut params = req.params.clone();
    if let Some(header_name) = extract_header(&req, "Mcp-Name") {
        if let Some(ref mut p) = params {
            if p.get("name").is_none() && p.get("task").is_none() && p.get("tool").is_none() {
                p["name"] = json!(header_name);
            }
        } else {
            params = Some(json!({ "name": header_name }));
        }
    }

    let req_id = req.id;

    let mut response = match effective_method.as_str() {
        mcp_methods::INITIALIZE => Some(JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: req_id,
            result: Some(json!({
                "protocolVersion": MCP_PROTOCOL_VERSION,
                "capabilities": {
                    "tools": { "listChanged": false },
                    "resources": { "subscribe": true, "listChanged": false },
                    "prompts": { "listChanged": false },
                    "tasks": { "listChanged": false, "cancel": true },
                    "logging": {}
                },
                "serverInfo": {
                    "name": SERVER_NAME,
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
            error: None,
            headers: None,
        }),

        mcp_methods::INITIALIZED | mcp_methods::INITIALIZED_ALT | mcp_methods::CANCELLED => None,

        mcp_methods::PING => Some(JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: req_id,
            result: Some(json!({})),
            error: None,
            headers: None,
        }),

        mcp_methods::TOOLS_LIST => Some(tools_list_response(req_id)),

        mcp_methods::TOOLS_CALL => Some(dispatch_tool_call(req_id, params.as_ref()).await),

        mcp_methods::TASKS_LIST => Some(handle_tasks_list(req_id).await),

        mcp_methods::TASKS_CALL => Some(handle_tasks_call(req_id, params.as_ref()).await),

        mcp_methods::TASKS_STATUS => Some(handle_tasks_status(req_id, params.as_ref()).await),

        mcp_methods::TASKS_CANCEL => Some(handle_tasks_cancel(req_id, params.as_ref()).await),

        mcp_methods::RESOURCES_LIST => Some(resources_list_response(req_id)),

        mcp_methods::RESOURCES_TEMPLATES_LIST => Some(resources_templates_list_response(req_id)),

        mcp_methods::RESOURCES_SUBSCRIBE => Some(JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: req_id,
            result: Some(json!({ "subscribed": true })),
            error: None,
            headers: None,
        }),

        mcp_methods::RESOURCES_UNSUBSCRIBE => Some(JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: req_id,
            result: Some(json!({ "unsubscribed": true })),
            error: None,
            headers: None,
        }),

        mcp_methods::RESOURCES_READ => Some(handle_resource_read(req_id, params.as_ref()).await),

        mcp_methods::PROMPTS_LIST => Some(prompts_list_response(req_id)),

        mcp_methods::PROMPTS_GET => Some(handle_prompt_get(req_id, params.as_ref())),

        _ => {
            // In JSON-RPC 2.0, notifications (id == None) must NOT be responded to
            if req_id.is_none() {
                None
            } else {
                Some(make_error_response(
                    req_id,
                    rpc_errors::METHOD_NOT_FOUND,
                    format!("Method '{}' not found", effective_method),
                ))
            }
        }
    };

    // Attach MCP 2026-07-28 response headers (routing & protocol version)
    if let Some(ref mut resp) = response {
        let mut headers = HashMap::new();
        headers.insert(
            "Mcp-Protocol-Version".to_string(),
            MCP_PROTOCOL_VERSION.to_string(),
        );
        headers.insert("Mcp-Method".to_string(), effective_method);
        resp.headers = Some(headers);
    }

    response
}
