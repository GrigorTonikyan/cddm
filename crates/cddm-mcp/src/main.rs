use cddm_core::{DEFAULT_DIRECTORY, DEFAULT_MIN_TOKENS, ScanConfig, run_scan};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, BufRead, Write};
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

/// JSON-RPC 2.0 protocol version.
pub const JSONRPC_VERSION: &str = "2.0";

/// MCP protocol specification version supported by this server.
pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

/// MCP Server human-readable display name.
pub const SERVER_NAME: &str = "CDDM Code De-Duplication Meister MCP Server";

/// JSON-RPC 2.0 standard error codes.
pub mod rpc_errors {
    pub const PARSE_ERROR: i64 = -32700;
    pub const METHOD_NOT_FOUND: i64 = -32601;
    pub const INTERNAL_ERROR: i64 = -32603;
}

/// Supported MCP protocol method names.
pub mod mcp_methods {
    pub const INITIALIZE: &str = "initialize";
    pub const TOOLS_LIST: &str = "tools/list";
    pub const TOOLS_CALL: &str = "tools/call";
}

/// Exposed tool identifiers and parameters.
pub mod mcp_tools {
    pub const SCAN_CODEBASE: &str = "scan_codebase";
    pub const PARAM_DIRECTORY: &str = "directory";
    pub const PARAM_MIN_TOKENS: &str = "min_tokens";
    pub const PARAM_ENABLE_GIT_BLAME: &str = "enable_git_blame";
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    method: String,
    params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in stdin.lock().lines() {
        let line_str = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line_str.trim().is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(&line_str) {
            Ok(r) => r,
            Err(e) => {
                let err_resp = JsonRpcResponse {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    id: None,
                    result: None,
                    error: Some(json!({
                        "code": rpc_errors::PARSE_ERROR,
                        "message": format!("Parse error: {}", e)
                    })),
                };
                let _ = writeln!(handle, "{}", serde_json::to_string(&err_resp)?);
                let _ = handle.flush();
                continue;
            }
        };

        let response = match req.method.as_str() {
            mcp_methods::INITIALIZE => JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id: req.id,
                result: Some(json!({
                    "protocolVersion": MCP_PROTOCOL_VERSION,
                    "capabilities": {
                        "tools": { "listChanged": false },
                        "resources": { "subscribe": false, "listChanged": false }
                    },
                    "serverInfo": {
                        "name": SERVER_NAME,
                        "version": env!("CARGO_PKG_VERSION")
                    }
                })),
                error: None,
            },

            mcp_methods::TOOLS_LIST => JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id: req.id,
                result: Some(json!({
                    "tools": [
                        {
                            "name": mcp_tools::SCAN_CODEBASE,
                            "description": "Run CDDM polyglot code duplication and DRY health score analysis on a target directory.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    mcp_tools::PARAM_DIRECTORY: {
                                        "type": "string",
                                        "description": "Target directory path to analyze"
                                    },
                                    mcp_tools::PARAM_MIN_TOKENS: {
                                        "type": "number",
                                        "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS)
                                    },
                                    mcp_tools::PARAM_ENABLE_GIT_BLAME: {
                                        "type": "boolean",
                                        "description": "Annotate duplicate lines with in-process git blame author"
                                    }
                                },
                                "required": [mcp_tools::PARAM_DIRECTORY]
                            }
                        }
                    ]
                })),
                error: None,
            },

            mcp_methods::TOOLS_CALL => {
                let tool_name = req
                    .params
                    .as_ref()
                    .and_then(|p| p.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("");

                if tool_name == mcp_tools::SCAN_CODEBASE {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));
                    let dir = args
                        .and_then(|a| a.get(mcp_tools::PARAM_DIRECTORY))
                        .and_then(|d| d.as_str())
                        .unwrap_or(DEFAULT_DIRECTORY);
                    let min_tokens =
                        args.and_then(|a| a.get(mcp_tools::PARAM_MIN_TOKENS))
                            .and_then(|t| t.as_u64())
                            .unwrap_or(DEFAULT_MIN_TOKENS as u64) as usize;
                    let git_blame = args
                        .and_then(|a| a.get(mcp_tools::PARAM_ENABLE_GIT_BLAME))
                        .and_then(|b| b.as_bool())
                        .unwrap_or(false);

                    let config = ScanConfig {
                        directory: dir.to_string(),
                        min_tokens,
                        languages: vec![],
                        ignore_patterns: ScanConfig::default().ignore_patterns,
                        detect_type2: true,
                        scan_self: true,
                        enable_git_blame: git_blame,
                    };

                    let (tx, _rx) = mpsc::channel(100);
                    let cancel_flag = Arc::new(AtomicBool::new(false));

                    match run_scan(config, tx, cancel_flag).await {
                        Ok(scan_res) => JsonRpcResponse {
                            jsonrpc: JSONRPC_VERSION.to_string(),
                            id: req.id,
                            result: Some(json!({
                                "content": [
                                    {
                                        "type": "text",
                                        "text": serde_json::to_string_pretty(&scan_res).unwrap_or_default()
                                    }
                                ]
                            })),
                            error: None,
                        },
                        Err(e) => JsonRpcResponse {
                            jsonrpc: JSONRPC_VERSION.to_string(),
                            id: req.id,
                            result: None,
                            error: Some(json!({
                                "code": rpc_errors::INTERNAL_ERROR,
                                "message": e
                            })),
                        },
                    }
                } else {
                    JsonRpcResponse {
                        jsonrpc: JSONRPC_VERSION.to_string(),
                        id: req.id,
                        result: None,
                        error: Some(json!({
                            "code": rpc_errors::METHOD_NOT_FOUND,
                            "message": format!("Method or tool '{}' not found", tool_name)
                        })),
                    }
                }
            }

            _ => JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id: req.id,
                result: None,
                error: Some(json!({
                    "code": rpc_errors::METHOD_NOT_FOUND,
                    "message": format!("Method '{}' not found", req.method)
                })),
            },
        };

        let _ = writeln!(handle, "{}", serde_json::to_string(&response)?);
        let _ = handle.flush();
    }

    Ok(())
}
