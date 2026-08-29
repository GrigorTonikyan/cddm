#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::json;

/// JSON-RPC 2.0 protocol version.
pub const JSONRPC_VERSION: &str = "2.0";

/// MCP protocol specification version supported by this server.
pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

/// MCP Server human-readable display name.
pub const SERVER_NAME: &str = "CDDM Code De-Duplication Meister MCP Server";

/// JSON-RPC 2.0 standard error codes.
#[allow(dead_code)]
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
    pub const AST_REFACTOR: &str = "cddm_ast_refactor";
    pub const VERIFY_REFACTOR: &str = "cddm_verify_refactor";
    pub const CHECK_POLICIES: &str = "cddm_check_policies";
    pub const HEAL_REFACTOR: &str = "cddm_heal_refactor";
    pub const EXPORT_CACHE_PACK: &str = "cddm_export_cache_pack";
    pub const IMPORT_CACHE_PACK: &str = "cddm_import_cache_pack";
    pub const SCAN_MONOREPO: &str = "cddm_scan_monorepo";
    pub const GET_SEMANTIC_GRAPH: &str = "cddm_get_semantic_graph";
    pub const COMPARE_SEMANTIC_GRAPHS: &str = "cddm_compare_semantic_graphs";
    pub const SCAN_CROSS_LANGUAGE: &str = "cddm_scan_cross_language";
    pub const EXTRACT_SHARED_MODULE: &str = "cddm_extract_shared_module";
    pub const DETECT_OVERLAP: &str = "cddm_detect_overlap";
    pub const SCAN_HUB: &str = "cddm_scan_hub";
    pub const EXTRACT_HUB_PACKAGE: &str = "cddm_extract_hub_package";
    pub const CORRELATE_COVERAGE: &str = "cddm_correlate_coverage";
    pub const DETECT_DEAD_CLONES: &str = "cddm_detect_dead_clones";
    pub const SEMANTIC_NEURAL_SCAN: &str = "cddm_semantic_neural_scan";

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
    pub const PARAM_RULES: &str = "rules";
    pub const PARAM_IGNORE_TESTS: &str = "ignore_tests";
    pub const PARAM_IGNORE_MOCKS: &str = "ignore_mocks";
    pub const PARAM_IGNORE_GENERATED: &str = "ignore_generated";
    pub const PARAM_PATCH: &str = "patch";
    pub const PARAM_BRANCH_NAME: &str = "branch_name";
    pub const PARAM_CREATE_BRANCH: &str = "create_branch";
}

macro_rules! define_mcp_constants {
    ($($name:ident => $val:expr),* $(,)?) => {
        $( pub const $name: &str = $val; )*
    };
}

/// Exposed resource identifiers and MIME types.
pub mod mcp_resources {
    define_mcp_constants! {
        URI_WORKSPACE_HEALTH => "cddm://workspace/health",
        URI_WORKSPACE_CLONES => "cddm://workspace/clones",
        URI_WORKSPACE_CLUSTERS => "cddm://workspace/clusters",
        URI_WORKSPACE_TIMELINE => "cddm://workspace/timeline",
        URI_WORKSPACE_SUPPRESSIONS => "cddm://workspace/suppressions",
        URI_WORKSPACE_POLICIES => "cddm://workspace/policies",
        URI_WORKSPACE_SEMANTIC_GRAPH => "cddm://workspace/semantic_graph",
        URI_WORKSPACE_CROSS_LANGUAGE_CLONES => "cddm://workspace/cross_language_clones",
        URI_WORKSPACE_WATCH_STATUS => "cddm://workspace/watch_status",
        URI_WORKSPACE_OVERLAP => "cddm://workspace/overlap",
        URI_WORKSPACE_HUB => "cddm://workspace/hub",
        URI_WORKSPACE_COVERAGE => "cddm://workspace/coverage",
        URI_WORKSPACE_NEURAL_EMBEDDINGS => "cddm://workspace/neural_embeddings",
        MIME_APPLICATION_JSON => "application/json",
    }
}

/// Exposed prompt template identifiers.
pub mod mcp_prompts {
    define_mcp_constants! {
        AUDIT_DRY_HEALTH => "audit_dry_health",
        REFACTOR_CLONE_PAIR => "refactor_clone_pair",
        AUDIT_CROSS_LANGUAGE => "cross_language_audit",
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

pub fn make_error_response(
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

pub fn make_text_response(
    id: Option<serde_json::Value>,
    text: impl Into<String>,
) -> JsonRpcResponse {
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

pub fn make_prompt_response(
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
