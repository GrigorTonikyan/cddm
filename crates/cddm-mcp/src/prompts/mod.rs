#![forbid(unsafe_code)]

use crate::protocol::{
    JSONRPC_VERSION, JsonRpcResponse, make_error_response, make_prompt_response, mcp_prompts,
    rpc_errors,
};
use serde_json::json;

pub fn prompts_list_response(id: Option<serde_json::Value>) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
        result: Some(json!({
            "prompts": [
                {
                    "name": mcp_prompts::AUDIT_DRY_HEALTH,
                    "description": "Analyze codebase DRY health score and prioritize duplication hotspots for refactoring.",
                    "arguments": [
                        {
                            "name": "directory",
                            "description": "Target workspace directory",
                            "required": false
                        }
                    ]
                },
                {
                    "name": mcp_prompts::REFACTOR_CLONE_PAIR,
                    "description": "Extract duplicate clone fragments into a shared helper function with a unified patch.",
                    "arguments": [
                        {
                            "name": "file_a",
                            "description": "Path to primary fragment file",
                            "required": true
                        },
                        {
                            "name": "start_line_a",
                            "description": "Start line of fragment A",
                            "required": true
                        },
                        {
                            "name": "end_line_a",
                            "description": "End line of fragment A",
                            "required": true
                        },
                        {
                            "name": "file_b",
                            "description": "Path to counterpart fragment file",
                            "required": true
                        },
                        {
                            "name": "start_line_b",
                            "description": "Start line of fragment B",
                            "required": true
                        },
                        {
                            "name": "end_line_b",
                            "description": "End line of fragment B",
                            "required": true
                        }
                    ]
                },
                {
                    "name": mcp_prompts::AUDIT_CROSS_LANGUAGE,
                    "description": "Audit cross-language semantic duplication and find isomorphic functions across polyglot boundaries.",
                    "arguments": [
                        {
                            "name": "directory",
                            "description": "Target workspace directory",
                            "required": false
                        },
                        {
                            "name": "threshold",
                            "description": "Minimum hybrid similarity threshold (0.0 to 1.0, default: 0.70)",
                            "required": false
                        }
                    ]
                }
            ]
        })),
        error: None,
    }
}

pub fn handle_prompt_get(
    id: Option<serde_json::Value>,
    params: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let prompt_name = params
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("");

    match prompt_name {
        mcp_prompts::AUDIT_DRY_HEALTH => make_prompt_response(
            id,
            "Audit codebase DRY Health Score and identify duplication hotspots.",
            "Please run CDDM duplication analysis on this workspace, audit the DRY health score, \
             and list the top duplicate clone pairs with actionable refactoring recommendations.",
        ),

        mcp_prompts::REFACTOR_CLONE_PAIR => {
            let args = params.and_then(|p| p.get("arguments"));
            let file_a = args
                .and_then(|a| a.get("file_a"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let file_b = args
                .and_then(|a| a.get("file_b"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            make_prompt_response(
                id,
                "Refactor a specific clone pair into a common helper.",
                format!(
                    "Please refactor the duplicate code clone between '{}' and '{}' by extracting \
                     common invariants and generating a clean, unified patch.",
                    file_a, file_b
                ),
            )
        }

        mcp_prompts::AUDIT_CROSS_LANGUAGE => make_prompt_response(
            id,
            "Audit cross-language semantic duplicates across polyglot codebases.",
            "Please run CDDM cross-language semantic analysis on this workspace using \
             Weisfeiler-Lehman graph kernels and subword vector embeddings to detect isomorphic \
             algorithms and business logic.",
        ),

        _ => make_error_response(
            id,
            rpc_errors::METHOD_NOT_FOUND,
            format!("Prompt '{}' not found", prompt_name),
        ),
    }
}
