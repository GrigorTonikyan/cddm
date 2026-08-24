#![forbid(unsafe_code)]

use cddm_core::{
    CloneLocation, DEFAULT_DIRECTORY, DEFAULT_MIN_TOKENS, ScanConfig, ScanResult,
    analyze_clone_refactoring, analyze_cluster_refactoring, generate_sarif_json,
    refactor::read_file_lines_range, run_scan,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
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
    pub const INITIALIZED_ALT: &str = "initialized";
    pub const CANCELLED: &str = "notifications/cancelled";
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
    pub const GET_CLONE_CLUSTER: &str = "cddm_get_clone_cluster";
    pub const SUGGEST_CLUSTER_REFACTOR: &str = "cddm_suggest_cluster_refactor";
    pub const EXPORT_SARIF: &str = "cddm_export_sarif";
    pub const DIFF_SCAN: &str = "cddm_diff_scan";
    pub const GET_TIMELINE: &str = "cddm_get_timeline";
    pub const CHECK_SUPPRESSION: &str = "cddm_check_suppression";
    pub const APPLY_CLUSTER_REFACTOR: &str = "cddm_apply_cluster_refactor";
    pub const GENERATE_AI_PROMPT: &str = "cddm_generate_ai_prompt";

    pub const PARAM_DIRECTORY: &str = "directory";
    pub const PARAM_MIN_TOKENS: &str = "min_tokens";
    pub const PARAM_ENABLE_GIT_BLAME: &str = "enable_git_blame";
    pub const PARAM_BASE_REF: &str = "base_ref";
    pub const PARAM_TARGET_REF: &str = "target_ref";
    pub const PARAM_FILE_A: &str = "file_a";
    pub const PARAM_START_LINE_A: &str = "start_line_a";
    pub const PARAM_END_LINE_A: &str = "end_line_a";
    pub const PARAM_FILE_B: &str = "file_b";
    pub const PARAM_START_LINE_B: &str = "start_line_b";
    pub const PARAM_END_LINE_B: &str = "end_line_b";
    pub const PARAM_CLUSTER_ID: &str = "cluster_id";
    pub const PARAM_OCCURRENCES: &str = "occurrences";
    pub const PARAM_MAX_SAMPLES: &str = "max_samples";
    pub const PARAM_PATH: &str = "path";
    pub const PARAM_LINE: &str = "line";
    pub const PARAM_CDDMIGNORE: &str = "cddmignore";
    pub const PARAM_IGNORE_TESTS: &str = "ignore_tests";
    pub const PARAM_IGNORE_MOCKS: &str = "ignore_mocks";
    pub const PARAM_IGNORE_GENERATED: &str = "ignore_generated";
    pub const PARAM_PATCH: &str = "patch";
    pub const PARAM_BRANCH_NAME: &str = "branch_name";
    pub const PARAM_CREATE_BRANCH: &str = "create_branch";
}

/// Exposed resource identifiers and MIME types.
pub mod mcp_resources {
    pub const URI_WORKSPACE_HEALTH: &str = "cddm://workspace/health";
    pub const URI_WORKSPACE_CLONES: &str = "cddm://workspace/clones";
    pub const URI_WORKSPACE_CLUSTERS: &str = "cddm://workspace/clusters";
    pub const URI_WORKSPACE_TIMELINE: &str = "cddm://workspace/timeline";
    pub const URI_WORKSPACE_SUPPRESSIONS: &str = "cddm://workspace/suppressions";
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

async fn run_scan_from_mcp_args(
    args: Option<&serde_json::Value>,
    enable_git_blame: bool,
) -> Result<ScanResult, String> {
    let dir = args
        .and_then(|a| a.get(mcp_tools::PARAM_DIRECTORY))
        .and_then(|d| d.as_str())
        .unwrap_or(DEFAULT_DIRECTORY);
    let min_tokens = args
        .and_then(|a| a.get(mcp_tools::PARAM_MIN_TOKENS))
        .and_then(|t| t.as_u64())
        .unwrap_or(DEFAULT_MIN_TOKENS as u64) as usize;

    let config = ScanConfig {
        directory: dir.to_string(),
        min_tokens,
        languages: vec![],
        ignore_patterns: ScanConfig::default().ignore_patterns,
        detect_type2: true,
        scan_self: true,
        enable_git_blame,
        cache_dir: None,
        enable_cache: true,
        cddmignore_path: None,
        ignore_tests: false,
        ignore_mocks: false,
        ignore_generated: true,
    };

    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    run_scan(config, tx, cancel_flag).await
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

        mcp_methods::INITIALIZED | mcp_methods::INITIALIZED_ALT | mcp_methods::CANCELLED => None,

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
                        "name": mcp_tools::GET_CLONE_CLUSTER,
                        "description": "Fetch localized source lines, token counts, and occurrences context for an N-way clone cluster.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                mcp_tools::PARAM_CLUSTER_ID: {
                                    "type": "number",
                                    "description": "1-based cluster index"
                                },
                                mcp_tools::PARAM_DIRECTORY: {
                                    "type": "string",
                                    "description": "Target directory path (default: current directory)"
                                },
                                mcp_tools::PARAM_MIN_TOKENS: {
                                    "type": "number",
                                    "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS)
                                }
                            },
                            "required": [mcp_tools::PARAM_CLUSTER_ID]
                        }
                    },
                    {
                        "name": mcp_tools::SUGGEST_CLUSTER_REFACTOR,
                        "description": "Generate an automated multi-site refactoring patch synthesizing a single shared abstraction and updating all N occurrence call-sites.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                mcp_tools::PARAM_CLUSTER_ID: {
                                    "type": "number",
                                    "description": "1-based cluster index"
                                },
                                mcp_tools::PARAM_DIRECTORY: {
                                    "type": "string",
                                    "description": "Target directory path (default: current directory)"
                                },
                                mcp_tools::PARAM_MIN_TOKENS: {
                                    "type": "number",
                                    "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS)
                                },
                                mcp_tools::PARAM_OCCURRENCES: {
                                    "type": "array",
                                    "description": "Explicit list of cluster occurrence locations",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "file": { "type": "string" },
                                            "start_line": { "type": "number" },
                                            "end_line": { "type": "number" }
                                        },
                                        "required": ["file", "start_line", "end_line"]
                                    }
                                }
                            }
                        }
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
                    },
                    {
                        "name": mcp_tools::DIFF_SCAN,
                        "description": "Run differential code clone detection comparing working changes against a Git base revision (e.g. main, origin/main, HEAD~1).",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                mcp_tools::PARAM_BASE_REF: {
                                    "type": "string",
                                    "description": "Base Git revision to compare against (e.g. main, origin/main, HEAD~1)"
                                },
                                mcp_tools::PARAM_TARGET_REF: {
                                    "type": "string",
                                    "description": "Target Git revision (default: HEAD / working tree)"
                                },
                                mcp_tools::PARAM_DIRECTORY: {
                                    "type": "string",
                                    "description": "Target Git repository directory path"
                                },
                                mcp_tools::PARAM_MIN_TOKENS: {
                                    "type": "number",
                                    "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS)
                                }
                            },
                            "required": [mcp_tools::PARAM_BASE_REF]
                        }
                    },
                    {
                        "name": mcp_tools::GET_TIMELINE,
                        "description": "Collect historical code duplication metrics, score delta, and DRY Health trajectory across Git repository history.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                mcp_tools::PARAM_DIRECTORY: {
                                    "type": "string",
                                    "description": "Target Git repository directory path (default: current directory)"
                                },
                                mcp_tools::PARAM_MAX_SAMPLES: {
                                    "type": "number",
                                    "description": "Maximum number of historical commits to sample (default: 10)"
                                },
                                mcp_tools::PARAM_MIN_TOKENS: {
                                    "type": "number",
                                    "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS)
                                }
                            }
                        }
                    },
                    {
                        "name": mcp_tools::CHECK_SUPPRESSION,
                        "description": "Check whether a specific file path or source line number is ignored by .cddmignore rules or inline suppression directives.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                mcp_tools::PARAM_PATH: {
                                    "type": "string",
                                    "description": "Target file path to check"
                                },
                                mcp_tools::PARAM_LINE: {
                                    "type": "number",
                                    "description": "Optional 1-based line number to check for inline suppression directives"
                                },
                                mcp_tools::PARAM_CDDMIGNORE: {
                                    "type": "string",
                                    "description": "Optional path to custom .cddmignore file"
                                },
                                mcp_tools::PARAM_IGNORE_TESTS: {
                                    "type": "boolean",
                                    "description": "Check with test file suppression enabled"
                                },
                                mcp_tools::PARAM_IGNORE_MOCKS: {
                                    "type": "boolean",
                                    "description": "Check with mock file suppression enabled"
                                },
                                mcp_tools::PARAM_IGNORE_GENERATED: {
                                    "type": "boolean",
                                    "description": "Check with generated file suppression enabled"
                                }
                            },
                            "required": [mcp_tools::PARAM_PATH]
                        }
                    },
                    {
                        "name": mcp_tools::APPLY_CLUSTER_REFACTOR,
                        "description": "Apply a synthesized multi-site refactoring unified patch to workspace files, with optional Git branch creation.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                mcp_tools::PARAM_PATCH: {
                                    "type": "string",
                                    "description": "Unified diff patch content to apply"
                                },
                                mcp_tools::PARAM_BRANCH_NAME: {
                                    "type": "string",
                                    "description": "Name of the Git branch to create (e.g. cddm/refactor-cluster-1)"
                                },
                                mcp_tools::PARAM_CREATE_BRANCH: {
                                    "type": "boolean",
                                    "description": "Whether to create a dedicated Git branch before applying patch"
                                }
                            },
                            "required": [mcp_tools::PARAM_PATCH]
                        }
                    },
                    {
                        "name": mcp_tools::GENERATE_AI_PROMPT,
                        "description": "Generate structured, test-driven AI refactoring prompt specification for LLM coding assistants to eliminate duplicate code clone pairs or clusters.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "function_name": {
                                    "type": "string",
                                    "description": "Proposed extracted function name (e.g. normalize_input)"
                                },
                                "target_module": {
                                    "type": "string",
                                    "description": "Target module path for the extracted helper (e.g. src/utils.rs)"
                                },
                                "invariant_body": {
                                    "type": "string",
                                    "description": "Extracted invariant code body"
                                },
                                "parameters": {
                                    "type": "array",
                                    "items": { "type": "string" },
                                    "description": "List of variable identifiers to parameterize"
                                },
                                "occurrences": {
                                    "type": "array",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "path": { "type": "string" },
                                            "start_line": { "type": "integer" },
                                            "end_line": { "type": "integer" },
                                            "snippet": { "type": "string" }
                                        },
                                        "required": ["path", "start_line", "end_line"]
                                    },
                                    "description": "List of clone occurrence locations"
                                },
                                "custom_instructions": {
                                    "type": "string",
                                    "description": "Optional architectural instructions or constraints"
                                }
                            },
                            "required": ["function_name", "target_module", "occurrences"]
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
                    let git_blame = args
                        .and_then(|a| a.get(mcp_tools::PARAM_ENABLE_GIT_BLAME))
                        .and_then(|b| b.as_bool())
                        .unwrap_or(false);

                    match run_scan_from_mcp_args(args, git_blame).await {
                        Ok(scan_res) => Some(make_text_response(
                            req.id,
                            serde_json::to_string_pretty(&scan_res).unwrap_or_default(),
                        )),
                        Err(e) => Some(make_error_response(req.id, rpc_errors::INTERNAL_ERROR, e)),
                    }
                }

                mcp_tools::DIFF_SCAN => {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));
                    let base_ref = args
                        .and_then(|a| a.get(mcp_tools::PARAM_BASE_REF))
                        .and_then(|b| b.as_str());

                    if let Some(base) = base_ref {
                        let dir = args
                            .and_then(|a| a.get(mcp_tools::PARAM_DIRECTORY))
                            .and_then(|d| d.as_str())
                            .unwrap_or(DEFAULT_DIRECTORY);
                        let target = args
                            .and_then(|a| a.get(mcp_tools::PARAM_TARGET_REF))
                            .and_then(|t| t.as_str());
                        let min_tokens = args
                            .and_then(|a| a.get(mcp_tools::PARAM_MIN_TOKENS))
                            .and_then(|t| t.as_u64())
                            .unwrap_or(DEFAULT_MIN_TOKENS as u64)
                            as usize;

                        let config = ScanConfig {
                            directory: dir.to_string(),
                            min_tokens,
                            languages: vec![],
                            ignore_patterns: ScanConfig::default().ignore_patterns,
                            detect_type2: true,
                            scan_self: true,
                            enable_git_blame: false,
                            cache_dir: None,
                            enable_cache: true,
                            cddmignore_path: None,
                            ignore_tests: false,
                            ignore_mocks: false,
                            ignore_generated: true,
                        };

                        let (tx, _rx) = mpsc::channel(100);
                        let cancel_flag = Arc::new(AtomicBool::new(false));

                        match cddm_core::run_diff_scan(base, target, config, tx, cancel_flag).await
                        {
                            Ok(diff_res) => Some(make_text_response(
                                req.id,
                                serde_json::to_string_pretty(&diff_res).unwrap_or_default(),
                            )),
                            Err(e) => {
                                Some(make_error_response(req.id, rpc_errors::INTERNAL_ERROR, e))
                            }
                        }
                    } else {
                        Some(make_error_response(
                            req.id,
                            rpc_errors::INVALID_PARAMS,
                            "Missing required 'base_ref' argument",
                        ))
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

                mcp_tools::GET_CLONE_CLUSTER => {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));
                    let cluster_id = args
                        .and_then(|a| a.get(mcp_tools::PARAM_CLUSTER_ID))
                        .and_then(|id| id.as_u64())
                        .map(|id| id as usize);

                    if let Some(target_id) = cluster_id {
                        match run_scan_from_mcp_args(args, true).await {
                            Ok(scan_res) => {
                                let found =
                                    scan_res.clone_clusters.iter().find(|c| c.id == target_id);

                                if let Some(cluster) = found {
                                    let mut occurrences_with_code = Vec::new();
                                    for occ in &cluster.occurrences {
                                        let code_lines = read_file_lines_range(
                                            Path::new(&occ.file),
                                            occ.start_line,
                                            occ.end_line,
                                        )
                                        .unwrap_or_default();
                                        occurrences_with_code.push(json!({
                                            "file": occ.file,
                                            "start_line": occ.start_line,
                                            "end_line": occ.end_line,
                                            "author": occ.author,
                                            "code": code_lines.join("\n")
                                        }));
                                    }

                                    let payload = json!({
                                        "cluster_id": cluster.id,
                                        "clone_type": format!("{:?}", cluster.clone_type),
                                        "token_count": cluster.token_count,
                                        "similarity": cluster.similarity,
                                        "fragment_hash": cluster.fragment_hash,
                                        "total_occurrences": cluster.occurrences.len(),
                                        "occurrences": occurrences_with_code
                                    });

                                    Some(make_text_response(
                                        req.id,
                                        serde_json::to_string_pretty(&payload).unwrap_or_default(),
                                    ))
                                } else {
                                    Some(make_error_response(
                                        req.id,
                                        rpc_errors::INVALID_PARAMS,
                                        format!("Cluster #{} not found in scan results", target_id),
                                    ))
                                }
                            }
                            Err(e) => {
                                Some(make_error_response(req.id, rpc_errors::INTERNAL_ERROR, e))
                            }
                        }
                    } else {
                        Some(make_error_response(
                            req.id,
                            rpc_errors::INVALID_PARAMS,
                            "Missing required 'cluster_id' parameter",
                        ))
                    }
                }

                mcp_tools::SUGGEST_CLUSTER_REFACTOR => {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));
                    let cluster_id_opt = args
                        .and_then(|a| a.get(mcp_tools::PARAM_CLUSTER_ID))
                        .and_then(|id| id.as_u64())
                        .map(|id| id as usize);

                    let explicit_occs = args
                        .and_then(|a| a.get(mcp_tools::PARAM_OCCURRENCES))
                        .and_then(|o| o.as_array());

                    if let Some(occs_arr) = explicit_occs {
                        let mut parsed_occs = Vec::new();
                        for item in occs_arr {
                            if let (Some(file), Some(start), Some(end)) = (
                                item.get("file").and_then(|f| f.as_str()),
                                item.get("start_line").and_then(|s| s.as_u64()),
                                item.get("end_line").and_then(|e| e.as_u64()),
                            ) {
                                parsed_occs.push(CloneLocation {
                                    file: file.to_string(),
                                    start_line: start as usize,
                                    end_line: end as usize,
                                    author: None,
                                });
                            }
                        }

                        if parsed_occs.len() < 2 {
                            return Some(make_error_response(
                                req.id,
                                rpc_errors::INVALID_PARAMS,
                                "At least 2 occurrence locations required for cluster refactoring",
                            ));
                        }

                        match analyze_cluster_refactoring("cluster-custom", &parsed_occs) {
                            Ok(suggestion) => Some(make_text_response(
                                req.id,
                                serde_json::to_string_pretty(&suggestion).unwrap_or_default(),
                            )),
                            Err(e) => {
                                Some(make_error_response(req.id, rpc_errors::INVALID_PARAMS, e))
                            }
                        }
                    } else if let Some(target_id) = cluster_id_opt {
                        match run_scan_from_mcp_args(args, false).await {
                            Ok(scan_res) => {
                                let found =
                                    scan_res.clone_clusters.iter().find(|c| c.id == target_id);

                                if let Some(cluster) = found {
                                    match analyze_cluster_refactoring(
                                        &cluster.id.to_string(),
                                        &cluster.occurrences,
                                    ) {
                                        Ok(suggestion) => Some(make_text_response(
                                            req.id,
                                            serde_json::to_string_pretty(&suggestion)
                                                .unwrap_or_default(),
                                        )),
                                        Err(e) => Some(make_error_response(
                                            req.id,
                                            rpc_errors::INVALID_PARAMS,
                                            e,
                                        )),
                                    }
                                } else {
                                    Some(make_error_response(
                                        req.id,
                                        rpc_errors::INVALID_PARAMS,
                                        format!("Cluster #{} not found in scan results", target_id),
                                    ))
                                }
                            }
                            Err(e) => {
                                Some(make_error_response(req.id, rpc_errors::INTERNAL_ERROR, e))
                            }
                        }
                    } else {
                        Some(make_error_response(
                            req.id,
                            rpc_errors::INVALID_PARAMS,
                            "Must specify either 'cluster_id' or 'occurrences' parameter",
                        ))
                    }
                }

                mcp_tools::EXPORT_SARIF => {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));

                    match run_scan_from_mcp_args(args, false).await {
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

                mcp_tools::GET_TIMELINE => {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));
                    let dir = args
                        .and_then(|a| a.get(mcp_tools::PARAM_DIRECTORY))
                        .and_then(|d| d.as_str())
                        .unwrap_or(DEFAULT_DIRECTORY);
                    let max_samples = args
                        .and_then(|a| a.get(mcp_tools::PARAM_MAX_SAMPLES))
                        .and_then(|s| s.as_u64())
                        .unwrap_or(10) as usize;
                    let min_tokens =
                        args.and_then(|a| a.get(mcp_tools::PARAM_MIN_TOKENS))
                            .and_then(|t| t.as_u64())
                            .unwrap_or(DEFAULT_MIN_TOKENS as u64) as usize;

                    let cancel_flag = Arc::new(AtomicBool::new(false));
                    match cddm_core::collect_git_timeline(
                        Path::new(dir),
                        max_samples,
                        min_tokens,
                        cancel_flag,
                    ) {
                        Ok(trend) => Some(make_text_response(
                            req.id,
                            serde_json::to_string_pretty(&trend).unwrap_or_default(),
                        )),
                        Err(e) => Some(make_error_response(req.id, rpc_errors::INTERNAL_ERROR, e)),
                    }
                }

                mcp_tools::CHECK_SUPPRESSION => {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));
                    let path_str = args
                        .and_then(|a| a.get(mcp_tools::PARAM_PATH))
                        .and_then(|p| p.as_str());

                    if let Some(path) = path_str {
                        let line_opt = args
                            .and_then(|a| a.get(mcp_tools::PARAM_LINE))
                            .and_then(|l| l.as_u64())
                            .map(|l| l as usize);
                        let custom_ignore = args
                            .and_then(|a| a.get(mcp_tools::PARAM_CDDMIGNORE))
                            .and_then(|i| i.as_str());
                        let ignore_tests = args
                            .and_then(|a| a.get(mcp_tools::PARAM_IGNORE_TESTS))
                            .and_then(|b| b.as_bool())
                            .unwrap_or(false);
                        let ignore_mocks = args
                            .and_then(|a| a.get(mcp_tools::PARAM_IGNORE_MOCKS))
                            .and_then(|b| b.as_bool())
                            .unwrap_or(false);
                        let ignore_generated = args
                            .and_then(|a| a.get(mcp_tools::PARAM_IGNORE_GENERATED))
                            .and_then(|b| b.as_bool())
                            .unwrap_or(true);

                        let engine = if let Some(custom_p) = custom_ignore {
                            match cddm_core::SuppressionEngine::from_file(
                                Path::new(custom_p),
                                ignore_tests,
                                ignore_mocks,
                                ignore_generated,
                            ) {
                                Ok(eng) => eng,
                                Err(err) => {
                                    return Some(make_error_response(
                                        req.id,
                                        rpc_errors::INTERNAL_ERROR,
                                        err,
                                    ));
                                }
                            }
                        } else if Path::new(".cddmignore").exists() {
                            match cddm_core::SuppressionEngine::from_file(
                                Path::new(".cddmignore"),
                                ignore_tests,
                                ignore_mocks,
                                ignore_generated,
                            ) {
                                Ok(eng) => eng,
                                Err(err) => {
                                    return Some(make_error_response(
                                        req.id,
                                        rpc_errors::INTERNAL_ERROR,
                                        err,
                                    ));
                                }
                            }
                        } else {
                            cddm_core::SuppressionEngine::with_options(
                                ignore_tests,
                                ignore_mocks,
                                ignore_generated,
                            )
                        };

                        let file_content = std::fs::read_to_string(path).ok();
                        let path_ignored =
                            engine.is_path_ignored(Path::new(path), file_content.as_deref());
                        let mut line_ignored = false;

                        if let Some(target_line) = line_opt
                            && let Some(ref text) = file_content
                        {
                            let mut eng = engine.clone();
                            eng.register_file_directives(path, text);
                            line_ignored = eng.is_span_suppressed(path, target_line, target_line);
                        }

                        let res = json!({
                            "path": path,
                            "path_ignored": path_ignored,
                            "line": line_opt,
                            "line_ignored": line_ignored,
                            "is_ignored": path_ignored || line_ignored,
                        });

                        Some(make_text_response(
                            req.id,
                            serde_json::to_string_pretty(&res).unwrap_or_default(),
                        ))
                    } else {
                        Some(make_error_response(
                            req.id,
                            rpc_errors::INVALID_PARAMS,
                            "Missing required 'path' parameter",
                        ))
                    }
                }

                mcp_tools::APPLY_CLUSTER_REFACTOR => {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));
                    let patch_str = args
                        .and_then(|a| a.get(mcp_tools::PARAM_PATCH))
                        .and_then(|p| p.as_str());

                    if let Some(patch) = patch_str {
                        let branch_name = args
                            .and_then(|a| a.get(mcp_tools::PARAM_BRANCH_NAME))
                            .and_then(|b| b.as_str());
                        let create_branch = args
                            .and_then(|a| a.get(mcp_tools::PARAM_CREATE_BRANCH))
                            .and_then(|b| b.as_bool())
                            .unwrap_or(false);

                        match cddm_core::apply_cluster_refactor_branch(
                            Path::new("."),
                            patch,
                            branch_name,
                            create_branch,
                        ) {
                            Ok(res) => Some(make_text_response(
                                req.id,
                                serde_json::to_string_pretty(&res).unwrap_or_default(),
                            )),
                            Err(e) => {
                                Some(make_error_response(req.id, rpc_errors::INTERNAL_ERROR, e))
                            }
                        }
                    } else {
                        Some(make_error_response(
                            req.id,
                            rpc_errors::INVALID_PARAMS,
                            "Missing required 'patch' parameter",
                        ))
                    }
                }

                mcp_tools::GENERATE_AI_PROMPT => {
                    let args = req.params.as_ref().and_then(|p| p.get("arguments"));
                    if let Some(args_val) = args {
                        let fn_name = args_val
                            .get("function_name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("extracted_helper");
                        let target_mod = args_val
                            .get("target_module")
                            .and_then(|v| v.as_str())
                            .unwrap_or("src/utils.rs");
                        let inv_body = args_val
                            .get("invariant_body")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let custom_inst = args_val
                            .get("custom_instructions")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let params: Vec<String> = args_val
                            .get("parameters")
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();

                        let occurrences: Vec<cddm_core::AiOccurrenceContext> = args_val
                            .get("occurrences")
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .map(|item| {
                                        let path = item
                                            .get("path")
                                            .and_then(|p| p.as_str())
                                            .unwrap_or("")
                                            .to_string();
                                        let s_line = item
                                            .get("start_line")
                                            .and_then(|l| l.as_u64())
                                            .unwrap_or(1)
                                            as usize;
                                        let e_line = item
                                            .get("end_line")
                                            .and_then(|l| l.as_u64())
                                            .unwrap_or(s_line as u64)
                                            as usize;
                                        let snippet = item
                                            .get("snippet")
                                            .and_then(|s| s.as_str())
                                            .map(|s| s.to_string())
                                            .unwrap_or_else(|| {
                                                let file_content = std::fs::read_to_string(&path)
                                                    .unwrap_or_default();
                                                let lines: Vec<&str> =
                                                    file_content.lines().collect();
                                                if s_line > 0 && s_line <= lines.len() {
                                                    let end = e_line.min(lines.len());
                                                    lines[s_line - 1..end].join("\n")
                                                } else {
                                                    String::new()
                                                }
                                            });
                                        cddm_core::AiOccurrenceContext {
                                            path,
                                            span: cddm_core::LineSpan {
                                                line_start: s_line,
                                                line_end: e_line,
                                                byte_offset: 0,
                                            },
                                            snippet,
                                        }
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();

                        let prompt_req = cddm_core::AiRefactorPromptRequest {
                            clone_type: cddm_core::CloneType::Renamed,
                            similarity: 0.90,
                            token_count: 100,
                            lines_saved_est: occurrences.len() * 10,
                            function_name: fn_name.to_string(),
                            target_module: target_mod.to_string(),
                            occurrences,
                            invariant_body: inv_body.to_string(),
                            parameters: params,
                            custom_instructions: custom_inst,
                        };

                        let prompt_text = cddm_core::generate_ai_refactor_prompt(&prompt_req);
                        Some(make_text_response(req.id, prompt_text))
                    } else {
                        Some(make_error_response(
                            req.id,
                            rpc_errors::INVALID_PARAMS,
                            "Missing arguments for cddm_generate_ai_prompt",
                        ))
                    }
                }

                _ => Some(make_error_response(
                    req.id,
                    rpc_errors::METHOD_NOT_FOUND,
                    format!("Tool '{}' not found", tool_name),
                )),
            }
        }

        mcp_methods::RESOURCES_LIST => Some(JsonRpcResponse {
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
                    },
                    {
                        "uri": mcp_resources::URI_WORKSPACE_CLUSTERS,
                        "name": "Workspace Code Clone Clusters",
                        "description": "N-way equivalence classes of duplicated logic across repository files.",
                        "mimeType": mcp_resources::MIME_APPLICATION_JSON
                    },
                    {
                        "uri": mcp_resources::URI_WORKSPACE_TIMELINE,
                        "name": "Workspace Historical Duplication Trend",
                        "description": "Historical DRY Health trajectories and commit snapshots across Git history.",
                        "mimeType": mcp_resources::MIME_APPLICATION_JSON
                    },
                    {
                        "uri": mcp_resources::URI_WORKSPACE_SUPPRESSIONS,
                        "name": "Workspace Suppression Rules",
                        "description": "Active .cddmignore suppression rules, threshold overrides, and filter directives.",
                        "mimeType": mcp_resources::MIME_APPLICATION_JSON
                    }
                ]
            })),
            error: None,
        }),

        mcp_methods::RESOURCES_TEMPLATES_LIST => Some(JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: req.id,
            result: Some(json!({
                "resourceTemplates": []
            })),
            error: None,
        }),

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
                                "total_clusters": res.total_clusters,
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

                mcp_resources::URI_WORKSPACE_CLUSTERS => {
                    match run_scan(config, tx, cancel_flag).await {
                        Ok(res) => {
                            let payload = json!({
                                "total_clusters": res.total_clusters,
                                "clone_clusters": res.clone_clusters
                            });
                            Some(JsonRpcResponse {
                                jsonrpc: JSONRPC_VERSION.to_string(),
                                id: req.id,
                                result: Some(json!({
                                    "contents": [
                                        {
                                            "uri": mcp_resources::URI_WORKSPACE_CLUSTERS,
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

                mcp_resources::URI_WORKSPACE_TIMELINE => {
                    let cancel_flag = Arc::new(AtomicBool::new(false));
                    match cddm_core::collect_git_timeline(
                        Path::new(DEFAULT_DIRECTORY),
                        10,
                        DEFAULT_MIN_TOKENS,
                        cancel_flag,
                    ) {
                        Ok(trend) => Some(JsonRpcResponse {
                            jsonrpc: JSONRPC_VERSION.to_string(),
                            id: req.id,
                            result: Some(json!({
                                "contents": [
                                    {
                                        "uri": mcp_resources::URI_WORKSPACE_TIMELINE,
                                        "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                                        "text": serde_json::to_string_pretty(&trend).unwrap_or_default()
                                    }
                                ]
                            })),
                            error: None,
                        }),
                        Err(e) => Some(make_error_response(req.id, rpc_errors::INTERNAL_ERROR, e)),
                    }
                }

                mcp_resources::URI_WORKSPACE_SUPPRESSIONS => {
                    let root_path = Path::new(".cddmignore");
                    let engine = if root_path.exists() {
                        cddm_core::SuppressionEngine::from_file(root_path, false, false, true)
                            .unwrap_or_else(|_| cddm_core::SuppressionEngine::default_engine())
                    } else {
                        cddm_core::SuppressionEngine::default_engine()
                    };
                    Some(JsonRpcResponse {
                        jsonrpc: JSONRPC_VERSION.to_string(),
                        id: req.id,
                        result: Some(json!({
                            "contents": [
                                {
                                    "uri": mcp_resources::URI_WORKSPACE_SUPPRESSIONS,
                                    "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                                    "text": serde_json::to_string_pretty(engine.config()).unwrap_or_default()
                                }
                            ]
                        })),
                        error: None,
                    })
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let mut stdout = tokio::io::stdout();

    while let Ok(Some(line)) = reader.next_line().await {
        let line_str = line.trim();
        if line_str.is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(line_str) {
            Ok(r) => r,
            Err(e) => {
                let err_resp = make_error_response(
                    None,
                    rpc_errors::PARSE_ERROR,
                    format!("Parse error: {}", e),
                );
                if let Ok(json_str) = serde_json::to_string(&err_resp) {
                    let mut payload = json_str.into_bytes();
                    payload.push(b'\n');
                    let _ = stdout.write_all(&payload).await;
                    let _ = stdout.flush().await;
                }
                continue;
            }
        };

        if let Some(response) = handle_mcp_request(req).await
            && let Ok(json_str) = serde_json::to_string(&response)
        {
            let mut payload = json_str.into_bytes();
            payload.push(b'\n');
            let _ = stdout.write_all(&payload).await;
            let _ = stdout.flush().await;
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
        assert_eq!(tools.len(), 11);
        let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert!(tool_names.contains(&mcp_tools::SCAN_CODEBASE));
        assert!(tool_names.contains(&mcp_tools::GET_CLONE_PAIR));
        assert!(tool_names.contains(&mcp_tools::SUGGEST_REFACTOR));
        assert!(tool_names.contains(&mcp_tools::GET_CLONE_CLUSTER));
        assert!(tool_names.contains(&mcp_tools::SUGGEST_CLUSTER_REFACTOR));
        assert!(tool_names.contains(&mcp_tools::EXPORT_SARIF));
        assert!(tool_names.contains(&mcp_tools::DIFF_SCAN));
        assert!(tool_names.contains(&mcp_tools::GET_TIMELINE));
        assert!(tool_names.contains(&mcp_tools::CHECK_SUPPRESSION));
        assert!(tool_names.contains(&mcp_tools::APPLY_CLUSTER_REFACTOR));
        assert!(tool_names.contains(&mcp_tools::GENERATE_AI_PROMPT));
    }

    #[tokio::test]
    async fn test_mcp_generate_ai_prompt_tool() {
        let resp = handle_mcp_request(make_test_req(
            16,
            mcp_methods::TOOLS_CALL,
            Some(json!({
                "name": mcp_tools::GENERATE_AI_PROMPT,
                "arguments": {
                    "function_name": "compute_total",
                    "target_module": "src/calc.rs",
                    "occurrences": [
                        {
                            "path": "src/a.rs",
                            "start_line": 1,
                            "end_line": 5,
                            "snippet": "let x = a + b;"
                        }
                    ],
                    "invariant_body": "let x = a + b;",
                    "parameters": ["a", "b"]
                }
            })),
        ))
        .await
        .expect("Expected response");

        assert!(resp.error.is_none());
        let res = resp.result.unwrap();
        let content = res["content"][0]["text"].as_str().unwrap();
        assert!(content.contains("compute_total"));
        assert!(content.contains("src/calc.rs"));
        assert!(content.contains("src/a.rs:1-5"));
    }

    #[tokio::test]
    async fn test_mcp_diff_scan_missing_params() {
        let resp = handle_mcp_request(make_test_req(
            15,
            mcp_methods::TOOLS_CALL,
            Some(json!({
                "name": mcp_tools::DIFF_SCAN,
                "arguments": {}
            })),
        ))
        .await
        .expect("Expected response");
        assert!(resp.error.is_some());
        assert_eq!(
            resp.error.unwrap()["code"].as_i64().unwrap(),
            rpc_errors::INVALID_PARAMS
        );
    }

    #[tokio::test]
    async fn test_mcp_check_suppression_tool() {
        let resp = handle_mcp_request(make_test_req(
            35,
            mcp_methods::TOOLS_CALL,
            Some(json!({
                "name": mcp_tools::CHECK_SUPPRESSION,
                "arguments": {
                    "path": "test_file.rs"
                }
            })),
        ))
        .await
        .expect("Expected response");
        assert!(resp.error.is_none());
        let text = resp.result.unwrap()["content"][0]["text"]
            .as_str()
            .unwrap()
            .to_string();
        assert!(text.contains("path_ignored"));
    }

    #[tokio::test]
    async fn test_mcp_resources_list() {
        let resources = list_mcp_items(mcp_methods::RESOURCES_LIST, "resources").await;
        assert_eq!(resources.len(), 5);
        assert_eq!(resources[0]["uri"], mcp_resources::URI_WORKSPACE_HEALTH);
        assert_eq!(resources[1]["uri"], mcp_resources::URI_WORKSPACE_CLONES);
        assert_eq!(resources[2]["uri"], mcp_resources::URI_WORKSPACE_CLUSTERS);
        assert_eq!(resources[3]["uri"], mcp_resources::URI_WORKSPACE_TIMELINE);
        assert_eq!(
            resources[4]["uri"],
            mcp_resources::URI_WORKSPACE_SUPPRESSIONS
        );
    }

    #[tokio::test]
    async fn test_mcp_resources_read_suppressions() {
        let resp = handle_mcp_request(make_test_req(
            36,
            mcp_methods::RESOURCES_READ,
            Some(json!({ "uri": mcp_resources::URI_WORKSPACE_SUPPRESSIONS })),
        ))
        .await
        .expect("Expected response");
        assert!(resp.result.is_some());
        let contents = resp.result.unwrap()["contents"].as_array().unwrap().clone();
        assert_eq!(contents.len(), 1);
        assert_eq!(
            contents[0]["uri"],
            mcp_resources::URI_WORKSPACE_SUPPRESSIONS
        );
    }

    #[tokio::test]
    async fn test_mcp_resources_read_timeline() {
        let resp = handle_mcp_request(make_test_req(
            23,
            mcp_methods::RESOURCES_READ,
            Some(json!({ "uri": mcp_resources::URI_WORKSPACE_TIMELINE })),
        ))
        .await
        .expect("Expected response");
        assert!(resp.result.is_some());
        let contents = resp.result.unwrap()["contents"].as_array().unwrap().clone();
        assert_eq!(contents.len(), 1);
        assert_eq!(contents[0]["uri"], mcp_resources::URI_WORKSPACE_TIMELINE);
    }

    #[tokio::test]
    async fn test_mcp_get_timeline_tool() {
        let resp = handle_mcp_request(make_test_req(
            24,
            mcp_methods::TOOLS_CALL,
            Some(json!({
                "name": mcp_tools::GET_TIMELINE,
                "arguments": {
                    "max_samples": 3,
                    "min_tokens": 50
                }
            })),
        ))
        .await
        .expect("Expected response");
        assert!(resp.error.is_none());
        let text = resp.result.unwrap()["content"][0]["text"]
            .as_str()
            .unwrap()
            .to_string();
        assert!(text.contains("snapshots"));
    }

    #[tokio::test]
    async fn test_mcp_resources_read_clusters() {
        let resp = handle_mcp_request(make_test_req(
            22,
            mcp_methods::RESOURCES_READ,
            Some(json!({ "uri": mcp_resources::URI_WORKSPACE_CLUSTERS })),
        ))
        .await
        .expect("Expected response");
        assert!(resp.result.is_some());
        let contents = resp.result.unwrap()["contents"].as_array().unwrap().clone();
        assert_eq!(contents.len(), 1);
        assert_eq!(contents[0]["uri"], mcp_resources::URI_WORKSPACE_CLUSTERS);
    }

    #[tokio::test]
    async fn test_mcp_cluster_refactor_explicit_occurrences() {
        let mut file_a = tempfile::NamedTempFile::new().unwrap();
        let mut file_b = tempfile::NamedTempFile::new().unwrap();
        let mut file_c = tempfile::NamedTempFile::new().unwrap();

        use std::io::Write;
        writeln!(file_a, "fn a() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
        writeln!(file_b, "fn b() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
        writeln!(file_c, "fn c() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();

        let resp = handle_mcp_request(make_test_req(
            30,
            mcp_methods::TOOLS_CALL,
            Some(json!({
                "name": mcp_tools::SUGGEST_CLUSTER_REFACTOR,
                "arguments": {
                    "occurrences": [
                        {
                            "file": file_a.path().to_str().unwrap(),
                            "start_line": 2,
                            "end_line": 3
                        },
                        {
                            "file": file_b.path().to_str().unwrap(),
                            "start_line": 2,
                            "end_line": 3
                        },
                        {
                            "file": file_c.path().to_str().unwrap(),
                            "start_line": 2,
                            "end_line": 3
                        }
                    ]
                }
            })),
        ))
        .await
        .expect("Expected response");

        assert!(resp.error.is_none());
        let text = resp.result.unwrap()["content"][0]["text"]
            .as_str()
            .unwrap()
            .to_string();
        assert!(text.contains("extracted_shared_helper"));
        assert!(text.contains("--- a/"));
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

        let unknown_notif = JsonRpcRequest {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: None,
            method: "some/unknown_notification".to_string(),
            params: None,
        };
        let resp_unknown = handle_mcp_request(unknown_notif).await;
        assert!(resp_unknown.is_none());
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
        assert!(resp.result.unwrap()["resourceTemplates"].is_array());
    }
}
