use cddm_core::{ScanConfig, run_scan};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, BufRead, Write};
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

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
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(
                        json!({ "code": -32700, "message": format!("Parse error: {}", e) }),
                    ),
                };
                let _ = writeln!(handle, "{}", serde_json::to_string(&err_resp)?);
                let _ = handle.flush();
                continue;
            }
        };

        let response = match req.method.as_str() {
            "initialize" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": { "listChanged": false },
                        "resources": { "subscribe": false, "listChanged": false }
                    },
                    "serverInfo": {
                        "name": "CDDM Code De-Duplication Meister MCP Server",
                        "version": "0.1.0"
                    }
                })),
                error: None,
            },

            "tools/list" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(json!({
                    "tools": [
                        {
                            "name": "scan_codebase",
                            "description": "Run CDDM polyglot code duplication and DRY health score analysis on a target directory.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "directory": { "type": "string", "description": "Target directory path to analyze" },
                                    "min_tokens": { "type": "number", "description": "Minimum token threshold (default: 50)" },
                                    "enable_git_blame": { "type": "boolean", "description": "Annotate duplicate lines with git author" }
                                },
                                "required": ["directory"]
                            }
                        }
                    ]
                })),
                error: None,
            },

            "tools/call" => {
                let tool_name = req
                    .params
                    .as_ref()
                    .and_then(|p| p.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("");

                if tool_name == "scan_codebase" {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));
                    let dir = args
                        .and_then(|a| a.get("directory"))
                        .and_then(|d| d.as_str())
                        .unwrap_or(".");
                    let min_tokens = args
                        .and_then(|a| a.get("min_tokens"))
                        .and_then(|t| t.as_u64())
                        .unwrap_or(50) as usize;
                    let git_blame = args
                        .and_then(|a| a.get("enable_git_blame"))
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
                            jsonrpc: "2.0".to_string(),
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
                            jsonrpc: "2.0".to_string(),
                            id: req.id,
                            result: None,
                            error: Some(json!({ "code": -32603, "message": e })),
                        },
                    }
                } else {
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: None,
                        error: Some(
                            json!({ "code": -32601, "message": "Method or tool not found" }),
                        ),
                    }
                }
            }

            _ => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: None,
                error: Some(json!({ "code": -32601, "message": "Method not found" })),
            },
        };

        let _ = writeln!(handle, "{}", serde_json::to_string(&response)?);
        let _ = handle.flush();
    }

    Ok(())
}
