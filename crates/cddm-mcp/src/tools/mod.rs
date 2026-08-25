#![forbid(unsafe_code)]

pub mod clone_tools;
pub mod helpers;
pub mod policy_tools;
pub mod refactor_tools;
pub mod scan_tools;

use crate::protocol::{
    JSONRPC_VERSION, JsonRpcResponse, make_error_response, mcp_tools, rpc_errors,
};
use cddm_core::DEFAULT_MIN_TOKENS;
use helpers::clone_pair_input_schema;
use serde_json::json;

pub fn tools_list_response(id: Option<serde_json::Value>) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
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
                },
                {
                    "name": mcp_tools::AST_REFACTOR,
                    "description": "Synthesize a Tree-sitter AST-native refactoring transformation with inferred types, import synthesis, and concrete syntax tree node substitutions.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "occurrences": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "path": { "type": "string" },
                                        "start_line": { "type": "integer" },
                                        "end_line": { "type": "integer" }
                                    },
                                    "required": ["path", "start_line", "end_line"]
                                },
                                "description": "List of clone occurrence locations"
                            },
                            "custom_function_name": {
                                "type": "string",
                                "description": "Optional extracted function name"
                            },
                            "target_module_path": {
                                "type": "string",
                                "description": "Optional target destination module or file path"
                            },
                            "custom_parameter_names": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Optional customized parameter names"
                            }
                        },
                        "required": ["occurrences"]
                    }
                },
                {
                    "name": mcp_tools::VERIFY_REFACTOR,
                    "description": "Run closed-loop test suite verification on the workspace or refactored branch to ensure zero behavioral regressions.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "directory": {
                                "type": "string",
                                "description": "Workspace root directory (default: current directory)"
                            },
                            "test_command": {
                                "type": "string",
                                "description": "Optional custom test command (e.g. 'cargo test', 'bun test', 'pytest')"
                            },
                            "branch_name": {
                                "type": "string",
                                "description": "Optional Git branch name to test"
                            },
                            "timeout_seconds": {
                                "type": "integer",
                                "description": "Timeout in seconds (default: 60)"
                            }
                        }
                    }
                },
                {
                    "name": mcp_tools::CHECK_POLICIES,
                    "description": "Evaluate architectural boundary and zero-duplication policies against the workspace (.cddmrules.toml).",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            mcp_tools::PARAM_DIRECTORY: {
                                "type": "string",
                                "description": "Target codebase directory path (default: current directory)"
                            },
                            mcp_tools::PARAM_RULES: {
                                "type": "string",
                                "description": "Custom path to .cddmrules.toml file"
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
    }
}

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
        _ => make_error_response(
            id,
            rpc_errors::METHOD_NOT_FOUND,
            format!("Tool '{}' not found", tool_name),
        ),
    }
}
