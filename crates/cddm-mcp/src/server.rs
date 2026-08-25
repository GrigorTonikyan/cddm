#![forbid(unsafe_code)]

use crate::prompts::{handle_prompt_get, prompts_list_response};
use crate::protocol::{
    JSONRPC_VERSION, JsonRpcRequest, JsonRpcResponse, MCP_PROTOCOL_VERSION, SERVER_NAME,
    make_error_response, mcp_methods, rpc_errors,
};
use crate::resources::{
    handle_resource_read, resources_list_response, resources_templates_list_response,
};
use crate::tools::{dispatch_tool_call, tools_list_response};
use serde_json::json;

/// Dispatches an incoming MCP JSON-RPC request and returns the response if not a notification.
pub async fn handle_mcp_request(req: JsonRpcRequest) -> Option<JsonRpcResponse> {
    match req.method.as_str() {
        mcp_methods::INITIALIZE => Some(JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: req.id,
            result: Some(json!({
                "protocolVersion": MCP_PROTOCOL_VERSION,
                "capabilities": {
                    "tools": { "listChanged": false },
                    "resources": { "subscribe": false, "listChanged": false },
                    "prompts": { "listChanged": false },
                    "logging": {}
                },
                "serverInfo": {
                    "name": SERVER_NAME,
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
            error: None,
        }),

        mcp_methods::INITIALIZED | mcp_methods::INITIALIZED_ALT | mcp_methods::CANCELLED => None,

        mcp_methods::PING => Some(JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: req.id,
            result: Some(json!({})),
            error: None,
        }),

        mcp_methods::TOOLS_LIST => Some(tools_list_response(req.id)),

        mcp_methods::TOOLS_CALL => Some(dispatch_tool_call(req.id, req.params.as_ref()).await),

        mcp_methods::RESOURCES_LIST => Some(resources_list_response(req.id)),

        mcp_methods::RESOURCES_TEMPLATES_LIST => Some(resources_templates_list_response(req.id)),

        mcp_methods::RESOURCES_READ => {
            Some(handle_resource_read(req.id, req.params.as_ref()).await)
        }

        mcp_methods::PROMPTS_LIST => Some(prompts_list_response(req.id)),

        mcp_methods::PROMPTS_GET => Some(handle_prompt_get(req.id, req.params.as_ref())),

        _ => {
            // In JSON-RPC 2.0, notifications (id == None) must NOT be responded to
            if req.id.is_none() {
                None
            } else {
                Some(make_error_response(
                    req.id,
                    rpc_errors::METHOD_NOT_FOUND,
                    format!("Method '{}' not found", req.method),
                ))
            }
        }
    }
}
