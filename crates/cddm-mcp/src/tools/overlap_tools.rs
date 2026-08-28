#![forbid(unsafe_code)]

use crate::protocol::{
    JsonRpcResponse, make_error_response, make_text_response, mcp_tools, rpc_errors,
};
use cddm_core::scan_workspace_overlap;
use std::path::Path;

/// Handler for the `cddm_detect_overlap` MCP tool.
pub async fn handle_detect_overlap(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let dir_str = args
        .and_then(|a| a.get(mcp_tools::PARAM_DIRECTORY))
        .and_then(|d| d.as_str())
        .unwrap_or(".");

    let threshold = args
        .and_then(|a| a.get("threshold"))
        .and_then(|t| t.as_f64())
        .unwrap_or(0.3);

    let workspace_path = Path::new(dir_str);
    if !workspace_path.exists() {
        return make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            format!("Target directory '{}' does not exist", dir_str),
        );
    }

    match scan_workspace_overlap(workspace_path, threshold) {
        Ok(result) => {
            let json_str = serde_json::to_string_pretty(&result).unwrap_or_default();
            make_text_response(id, json_str)
        }
        Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
    }
}
