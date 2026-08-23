use cddm_core::{
    DEFAULT_DIRECTORY, DEFAULT_MIN_TOKENS, ScanConfig, analyze_clone_refactoring,
    generate_sarif_json, refactor::read_file_lines_range, run_scan,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, BufRead, Write};
use std::path::Path;
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
    pub const INVALID_REQUEST: i64 = -32600;
    pub const METHOD_NOT_FOUND: i64 = -32601;
    pub const INVALID_PARAMS: i64 = -32602;
    pub const INTERNAL_ERROR: i64 = -32603;
}

/// Supported MCP protocol method names.
pub mod mcp_methods {
    pub const INITIALIZE: &str = "initialize";
    pub const INITIALIZED: &str = "notifications/initialized";
    pub const PING: &str = "ping";
    pub const TOOLS_LIST: &str = "tools/list";
    pub const TOOLS_CALL: &str = "tools/call";
    pub const RESOURCES_LIST: &str = "resources/list";
    pub const RESOURCES_READ: &str = "resources/read";
    pub const RESOURCES_TEMPLATES_LIST: &str = "resources/templates/list";
    pub const PROMPTS_LIST: &str = "prompts/list";
    pub const PROMPTS_GET: &str = "prompts/get";
}

/// Exposed tool identifiers and parameters.
pub mod mcp_tools {
    pub const SCAN_CODEBASE: &str = "scan_codebase";
    pub const GET_CLONE_PAIR: &str = "cddm_get_clone_pair";
    pub const SUGGEST_REFACTOR: &str = "cddm_suggest_refactor";
    pub const EXPORT_SARIF: &str = "cddm_export_sarif";

    pub const PARAM_DIRECTORY: &str = "directory";
    pub const PARAM_MIN_TOKENS: &str = "min_tokens";
    pub const PARAM_ENABLE_GIT_BLAME: &str = "enable_git_blame";
    pub const PARAM_FILE_A: &str = "file_a";
    pub const PARAM_START_LINE_A: &str = "start_line_a";
    pub const PARAM_END_LINE_A: &str = "end_line_a";
    pub const PARAM_FILE_B: &str = "file_b";
    pub const PARAM_START_LINE_B: &str = "start_line_b";
    pub const PARAM_END_LINE_B: &str = "end_line_b";
}

/// Exposed resource identifiers and MIME types.
pub mod mcp_resources {
    pub const URI_WORKSPACE_HEALTH: &str = "cddm://workspace/health";
    pub const URI_WORKSPACE_CLONES: &str = "cddm://workspace/clones";
    pub const MIME_APPLICATION_JSON: &str = "application/json";
}

/// Exposed prompt template identifiers.
pub mod mcp_prompts {
    pub const AUDIT_DRY_HEALTH: &str = "audit_dry_health";
    pub const REFACTOR_CLONE_PAIR: &str = "refactor_clone_pair";
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

fn make_error_response(
    id: Option<serde_json::Value>,
    code: i64,
    message: impl Into<String>,
) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
        result: None,
        error: Some(json!({
            "code": code,
            "message": message.into(),
        })),
    }
}

fn make_text_response(id: Option<serde_json::Value>, text: impl Into<String>) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
        result: Some(json!({
            "content": [
                {
                    "type": "text",
                    "text": text.into(),
                }
            ]
        })),
        error: None,
    }
}

fn make_prompt_response(
    id: Option<serde_json::Value>,
    description: &str,
    user_prompt: impl Into<String>,
) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
        result: Some(json!({
            "description": description,
            "messages": [
                {
                    "role": "user",
                    "content": {
                        "type": "text",
                        "text": user_prompt.into(),
                    }
                }
            ]
        })),
        error: None,
    }
}

fn clone_pair_input_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            mcp_tools::PARAM_FILE_A: {
                "type": "string",
                "description": "File path of fragment A"
            },
            mcp_tools::PARAM_START_LINE_A: {
                "type": "number",
                "description": "1-based start line of fragment A"
            },
            mcp_tools::PARAM_END_LINE_A: {
                "type": "number",
                "description": "1-based end line of fragment A"
            },
            mcp_tools::PARAM_FILE_B: {
                "type": "string",
                "description": "File path of fragment B"
            },
            mcp_tools::PARAM_START_LINE_B: {
                "type": "number",
                "description": "1-based start line of fragment B"
            },
            mcp_tools::PARAM_END_LINE_B: {
                "type": "number",
                "description": "1-based end line of fragment B"
            }
        },
        "required": [
            mcp_tools::PARAM_FILE_A,
            mcp_tools::PARAM_START_LINE_A,
            mcp_tools::PARAM_END_LINE_A,
            mcp_tools::PARAM_FILE_B,
            mcp_tools::PARAM_START_LINE_B,
            mcp_tools::PARAM_END_LINE_B
        ]
    })
}

fn parse_clone_pair_args(
    args: Option<&serde_json::Value>,
) -> Option<(&str, usize, usize, &str, usize, usize)> {
    let a = args?;
    Some((
        a.get(mcp_tools::PARAM_FILE_A)?.as_str()?,
        a.get(mcp_tools::PARAM_START_LINE_A)?.as_u64()? as usize,
        a.get(mcp_tools::PARAM_END_LINE_A)?.as_u64()? as usize,
        a.get(mcp_tools::PARAM_FILE_B)?.as_str()?,
        a.get(mcp_tools::PARAM_START_LINE_B)?.as_u64()? as usize,
        a.get(mcp_tools::PARAM_END_LINE_B)?.as_u64()? as usize,
    ))
}

/// Dispatches an incoming MCP JSON-RPC request and returns the response if not a notification.
async fn handle_mcp_request(req: JsonRpcRequest) -> Option<JsonRpcResponse> {
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

        mcp_methods::INITIALIZED => None,

        mcp_methods::PING => Some(JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: req.id,
            result: Some(json!({})),
            error: None,
        }),

        mcp_methods::TOOLS_LIST => Some(JsonRpcResponse {
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
                                    "description": "Target directory path to analyze (default: current directory)"
                                },
                                mcp_tools::PARAM_MIN_TOKENS: {
                                    "type": "number",
                                    "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS)
                                },
                                mcp_tools::PARAM_ENABLE_GIT_BLAME: {
                                    "type": "boolean",
                                    "description": "Annotate duplicate lines with in-process git blame author metadata"
                                }
                            }
                        }
                    },
                    {
                        "name": mcp_tools::GET_CLONE_PAIR,
                        "description": "Fetch localized source lines, token counts, and git blame context for a duplicate clone pair.",
                        "inputSchema": clone_pair_input_schema()
                    },
                    {
                        "name": mcp_tools::SUGGEST_REFACTOR,
                        "description": "Run invariant extraction on duplicate clone fragments and generate a structural refactoring suggestion with unified .patch format.",
                        "inputSchema": clone_pair_input_schema()
                    },
                    {
                        "name": mcp_tools::EXPORT_SARIF,
                        "description": "Run codebase duplication analysis and emit an OASIS SARIF v2.1.0 report for GitHub Code Scanning / IDE diagnostics.",
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
                                }
                            }
                        }
                    }
                ]
            })),
            error: None,
        }),

        mcp_methods::TOOLS_CALL => {
            let tool_name = req
                .params
                .as_ref()
                .and_then(|p| p.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("");

            match tool_name {
                mcp_tools::SCAN_CODEBASE => {
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
                        Ok(scan_res) => Some(make_text_response(
                            req.id,
                            serde_json::to_string_pretty(&scan_res).unwrap_or_default(),
                        )),
                        Err(e) => Some(make_error_response(req.id, rpc_errors::INTERNAL_ERROR, e)),
                    }
                }

                mcp_tools::GET_CLONE_PAIR => {
                    if let Some((fa, sa, ea, fb, sb, eb)) =
                        parse_clone_pair_args(req.params.as_ref().and_then(|p| p.get("arguments")))
                    {
                        let lines_a = read_file_lines_range(Path::new(fa), sa, ea);
                        let lines_b = read_file_lines_range(Path::new(fb), sb, eb);

                        match (lines_a, lines_b) {
                            (Ok(la), Ok(lb)) => {
                                let payload = json!({
                                    "fragment_a": {
                                        "file": fa,
                                        "start_line": sa,
                                        "end_line": ea,
                                        "line_count": la.len(),
                                        "lines": la
                                    },
                                    "fragment_b": {
                                        "file": fb,
                                        "start_line": sb,
                                        "end_line": eb,
                                        "line_count": lb.len(),
                                        "lines": lb
                                    }
                                });
                                Some(make_text_response(
                                    req.id,
                                    serde_json::to_string_pretty(&payload).unwrap_or_default(),
                                ))
                            }
                            (Err(e), _) | (_, Err(e)) => {
                                Some(make_error_response(req.id, rpc_errors::INVALID_PARAMS, e))
                            }
                        }
                    } else {
                        Some(make_error_response(
                            req.id,
                            rpc_errors::INVALID_PARAMS,
                            "Missing required line range parameters",
                        ))
                    }
                }

                mcp_tools::SUGGEST_REFACTOR => {
                    if let Some((fa, sa, ea, fb, sb, eb)) =
                        parse_clone_pair_args(req.params.as_ref().and_then(|p| p.get("arguments")))
                    {
                        match analyze_clone_refactoring(fa, (sa, ea), fb, (sb, eb)) {
                            Ok(suggestion) => Some(make_text_response(
                                req.id,
                                serde_json::to_string_pretty(&suggestion).unwrap_or_default(),
                            )),
                            Err(e) => {
                                Some(make_error_response(req.id, rpc_errors::INVALID_PARAMS, e))
                            }
                        }
                    } else {
                        Some(make_error_response(
                            req.id,
                            rpc_errors::INVALID_PARAMS,
                            "Missing required line range parameters",
                        ))
                    }
                }

                mcp_tools::EXPORT_SARIF => {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));
                    let dir = args
                        .and_then(|a| a.get(mcp_tools::PARAM_DIRECTORY))
                        .and_then(|d| d.as_str())
                        .unwrap_or(DEFAULT_DIRECTORY);
                    let min_tokens =
                        args.and_then(|a| a.get(mcp_tools::PARAM_MIN_TOKENS))
                            .and_then(|t| t.as_u64())
                            .unwrap_or(DEFAULT_MIN_TOKENS as u64) as usize;

                    let config = ScanConfig {
                        directory: dir.to_string(),
                        min_tokens,
                        languages: vec![],
                        ignore_patterns: ScanConfig::default().ignore_patterns,
                        detect_type2: true,
                        scan_self: true,
                        enable_git_blame: false,
                    };

                    let (tx, _rx) = mpsc::channel(100);
                    let cancel_flag = Arc::new(AtomicBool::new(false));

                    match run_scan(config, tx, cancel_flag).await {
                        Ok(scan_res) => {
                            let sarif = generate_sarif_json(&scan_res);
                            Some(make_text_response(
                                req.id,
                                serde_json::to_string_pretty(&sarif).unwrap_or_default(),
                            ))
                        }
                        Err(e) => Some(make_error_response(req.id, rpc_errors::INTERNAL_ERROR, e)),
                    }
                }

                _ => Some(make_error_response(
                    req.id,
                    rpc_errors::METHOD_NOT_FOUND,
                    format!("Tool '{}' not found", tool_name),
                )),
            }
        }

        mcp_methods::RESOURCES_LIST | mcp_methods::RESOURCES_TEMPLATES_LIST => {
            Some(JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id: req.id,
                result: Some(json!({
                    "resources": [
                        {
                            "uri": mcp_resources::URI_WORKSPACE_HEALTH,
                            "name": "Workspace DRY Health Score",
                            "description": "Real-time DRY Health Index, file metrics, and language statistics.",
                            "mimeType": mcp_resources::MIME_APPLICATION_JSON
                        },
                        {
                            "uri": mcp_resources::URI_WORKSPACE_CLONES,
                            "name": "Workspace Code Clones",
                            "description": "Registry of active duplicate code clones across repository files.",
                            "mimeType": mcp_resources::MIME_APPLICATION_JSON
                        }
                    ]
                })),
                error: None,
            })
        }

        mcp_methods::RESOURCES_READ => {
            let uri = req
                .params
                .as_ref()
                .and_then(|p| p.get("uri"))
                .and_then(|u| u.as_str())
                .unwrap_or("");

            let config = ScanConfig::default();
            let (tx, _rx) = mpsc::channel(100);
            let cancel_flag = Arc::new(AtomicBool::new(false));

            match uri {
                mcp_resources::URI_WORKSPACE_HEALTH => {
                    match run_scan(config, tx, cancel_flag).await {
                        Ok(res) => {
                            let payload = json!({
                                "dry_health_score": res.dry_health_score,
                                "duplication_percentage": res.duplication_percentage,
                                "total_files": res.total_files,
                                "total_tokens": res.total_tokens,
                                "total_clones": res.total_clones,
                                "language_breakdown": res.language_breakdown
                            });
                            Some(JsonRpcResponse {
                                jsonrpc: JSONRPC_VERSION.to_string(),
                                id: req.id,
                                result: Some(json!({
                                    "contents": [
                                        {
                                            "uri": mcp_resources::URI_WORKSPACE_HEALTH,
                                            "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                                            "text": serde_json::to_string_pretty(&payload).unwrap_or_default()
                                        }
                                    ]
                                })),
                                error: None,
                            })
                        }
                        Err(e) => Some(make_error_response(req.id, rpc_errors::INTERNAL_ERROR, e)),
                    }
                }

                mcp_resources::URI_WORKSPACE_CLONES => {
                    match run_scan(config, tx, cancel_flag).await {
                        Ok(res) => Some(JsonRpcResponse {
                            jsonrpc: JSONRPC_VERSION.to_string(),
                            id: req.id,
                            result: Some(json!({
                                "contents": [
                                    {
                                        "uri": mcp_resources::URI_WORKSPACE_CLONES,
                                        "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                                        "text": serde_json::to_string_pretty(&res.clone_pairs).unwrap_or_default()
                                    }
                                ]
                            })),
                            error: None,
                        }),
                        Err(e) => Some(make_error_response(req.id, rpc_errors::INTERNAL_ERROR, e)),
                    }
                }

                _ => Some(make_error_response(
                    req.id,
                    rpc_errors::INVALID_PARAMS,
                    format!("Resource URI '{}' not found", uri),
                )),
            }
        }

        mcp_methods::PROMPTS_LIST => Some(JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: req.id,
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
                    }
                ]
            })),
            error: None,
        }),

        mcp_methods::PROMPTS_GET => {
            let prompt_name = req
                .params
                .as_ref()
                .and_then(|p| p.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("");

            match prompt_name {
                mcp_prompts::AUDIT_DRY_HEALTH => Some(make_prompt_response(
                    req.id,
                    "Audit codebase DRY Health Score and identify duplication hotspots.",
                    "Please run CDDM duplication analysis on this workspace, audit the DRY health \
                     score, and list the top duplicate clone pairs with actionable refactoring \
                     recommendations.",
                )),

                mcp_prompts::REFACTOR_CLONE_PAIR => {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));
                    let file_a = args
                        .and_then(|a| a.get("file_a"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let file_b = args
                        .and_then(|a| a.get("file_b"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    Some(make_prompt_response(
                        req.id,
                        "Refactor a specific clone pair into a common helper.",
                        format!(
                            "Please refactor the duplicate code clone between '{}' and '{}' by \
                             extracting common invariants and generating a clean, unified patch.",
                            file_a, file_b
                        ),
                    ))
                }

                _ => Some(make_error_response(
                    req.id,
                    rpc_errors::METHOD_NOT_FOUND,
                    format!("Prompt '{}' not found", prompt_name),
                )),
            }
        }

        _ => Some(make_error_response(
            req.id,
            rpc_errors::METHOD_NOT_FOUND,
            format!("Method '{}' not found", req.method),
        )),
    }
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
                let err_resp = make_error_response(
                    None,
                    rpc_errors::PARSE_ERROR,
                    format!("Parse error: {}", e),
                );
                let _ = writeln!(handle, "{}", serde_json::to_string(&err_resp)?);
                let _ = handle.flush();
                continue;
            }
        };

        if let Some(response) = handle_mcp_request(req).await {
            let _ = writeln!(handle, "{}", serde_json::to_string(&response)?);
            let _ = handle.flush();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(tools.len(), 4);
        let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert!(tool_names.contains(&mcp_tools::SCAN_CODEBASE));
        assert!(tool_names.contains(&mcp_tools::GET_CLONE_PAIR));
        assert!(tool_names.contains(&mcp_tools::SUGGEST_REFACTOR));
        assert!(tool_names.contains(&mcp_tools::EXPORT_SARIF));
    }

    #[tokio::test]
    async fn test_mcp_resources_list() {
        let resources = list_mcp_items(mcp_methods::RESOURCES_LIST, "resources").await;
        assert_eq!(resources.len(), 2);
        assert_eq!(resources[0]["uri"], mcp_resources::URI_WORKSPACE_HEALTH);
        assert_eq!(resources[1]["uri"], mcp_resources::URI_WORKSPACE_CLONES);
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
}
