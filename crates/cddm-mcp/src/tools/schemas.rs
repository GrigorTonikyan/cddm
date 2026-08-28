#![forbid(unsafe_code)]

use super::helpers::clone_pair_input_schema;
use crate::protocol::mcp_tools;
use cddm_core::DEFAULT_MIN_TOKENS;
use serde_json::json;

/// Returns the complete list of available MCP tool definitions and input schemas.
pub fn get_tool_definitions() -> Vec<serde_json::Value> {
    vec![
        json!({
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
        }),
        json!({
            "name": mcp_tools::GET_CLONE_PAIR,
            "description": "Fetch localized source lines, token counts, and git blame context for a duplicate clone pair.",
            "inputSchema": clone_pair_input_schema()
        }),
        json!({
            "name": mcp_tools::SUGGEST_REFACTOR,
            "description": "Run invariant extraction on duplicate clone fragments and generate a structural refactoring suggestion with unified .patch format.",
            "inputSchema": clone_pair_input_schema()
        }),
        json!({
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
        }),
        json!({
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
        }),
        json!({
            "name": mcp_tools::EXPORT_SARIF,
            "description": "Generate a standard SARIF (Static Analysis Results Interchange Format) JSON report for CI/CD and security dashboards.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_DIRECTORY: {
                        "type": "string",
                        "description": "Target directory path (default: current directory)"
                    },
                    mcp_tools::PARAM_MIN_TOKENS: {
                        "type": "number",
                        "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS)
                    }
                }
            }
        }),
        json!({
            "name": mcp_tools::DIFF_SCAN,
            "description": "Perform git-aware differential duplication analysis between two git references.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_BASE_REF: {
                        "type": "string",
                        "description": "Base Git reference branch/tag/commit (e.g. 'main')"
                    },
                    mcp_tools::PARAM_TARGET_REF: {
                        "type": "string",
                        "description": "Target Git reference branch/tag/commit (default: 'HEAD')"
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
                "required": [mcp_tools::PARAM_BASE_REF]
            }
        }),
        json!({
            "name": mcp_tools::GET_TIMELINE,
            "description": "Retrieve historical code duplication trends and DRY health score evolution from git commit log snapshots.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_DIRECTORY: {
                        "type": "string",
                        "description": "Target directory path (default: current directory)"
                    },
                    mcp_tools::PARAM_MAX_SAMPLES: {
                        "type": "number",
                        "description": "Maximum historical commit samples to inspect (default: 10)"
                    },
                    mcp_tools::PARAM_MIN_TOKENS: {
                        "type": "number",
                        "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS)
                    }
                }
            }
        }),
        json!({
            "name": mcp_tools::CHECK_SUPPRESSION,
            "description": "Check if a file path or line number is suppressed by .cddmignore rules or inline suppression directives.",
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
                        "description": "Custom path to .cddmignore file"
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
        }),
        json!({
            "name": mcp_tools::APPLY_CLUSTER_REFACTOR,
            "description": "Applies a synthesized refactoring patch directly to workspace files on disk with optional automatic Git branch creation.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_PATCH: {
                        "type": "string",
                        "description": "Unified diff patch content synthesized by CDDM"
                    },
                    mcp_tools::PARAM_BRANCH_NAME: {
                        "type": "string",
                        "description": "Optional target Git branch name (default: cddm/refactor-auto)"
                    },
                    mcp_tools::PARAM_CREATE_BRANCH: {
                        "type": "boolean",
                        "description": "Whether to create a new Git branch for the refactoring changes (default: false)"
                    }
                },
                "required": [mcp_tools::PARAM_PATCH]
            }
        }),
        json!({
            "name": mcp_tools::GENERATE_AI_PROMPT,
            "description": "Generate an LLM AI refactoring prompt specification from clone occurrences and proposed extracted helper function.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "occurrences": {
                        "type": "array",
                        "description": "List of clone occurrences",
                        "items": {
                            "type": "object",
                            "properties": {
                                "file": { "type": "string" },
                                "start_line": { "type": "integer" },
                                "end_line": { "type": "integer" }
                            },
                            "required": ["file", "start_line", "end_line"]
                        }
                    },
                    "target_language": { "type": "string" },
                    "suggested_function_name": { "type": "string" },
                    "target_module_path": { "type": "string" },
                    "custom_instructions": { "type": "string" }
                },
                "required": ["occurrences", "target_language"]
            }
        }),
        json!({
            "name": mcp_tools::AST_REFACTOR,
            "description": "AST-native semantic refactoring preview synthesizing helper function AST and rewrite replacements.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "occurrences": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "file": { "type": "string" },
                                "start_line": { "type": "integer" },
                                "end_line": { "type": "integer" }
                            },
                            "required": ["file", "start_line", "end_line"]
                        }
                    },
                    "custom_function_name": { "type": "string" },
                    "target_module_path": { "type": "string" },
                    "custom_parameter_names": {
                        "type": "array",
                        "items": { "type": "string" }
                    }
                },
                "required": ["occurrences"]
            }
        }),
        json!({
            "name": mcp_tools::VERIFY_REFACTOR,
            "description": "Run closed-loop test suite verification on the workspace or refactored branch to ensure zero behavioral regressions.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "directory": { "type": "string" },
                    "test_command": { "type": "string" },
                    "branch_name": { "type": "string" },
                    "timeout_seconds": { "type": "integer" }
                }
            }
        }),
        json!({
            "name": mcp_tools::CHECK_POLICIES,
            "description": "Evaluate architectural boundary and zero-duplication policies against the workspace (.cddmrules.toml).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_DIRECTORY: { "type": "string" },
                    mcp_tools::PARAM_RULES: { "type": "string" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number" }
                }
            }
        }),
        json!({
            "name": mcp_tools::HEAL_REFACTOR,
            "description": "Autonomous AI Code Surgeon healing loop with test error-feedback repair and automated branch creation.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "directory": { "type": "string" },
                    "cluster_id": { "type": "number" },
                    "pair_id": { "type": "number" },
                    "provider": { "type": "string", "description": "AI provider (gemini, claude, openai, ollama, mock)" },
                    "model": { "type": "string" },
                    "api_key": { "type": "string" },
                    "endpoint": { "type": "string" },
                    "max_iterations": { "type": "number", "default": 3 },
                    "verify": { "type": "boolean", "default": true },
                    "test_command": { "type": "string" },
                    "branch_name": { "type": "string" },
                    "function_name": { "type": "string" },
                    "target_module": { "type": "string" }
                }
            }
        }),
        json!({
            "name": mcp_tools::EXPORT_CACHE_PACK,
            "description": "Export persistent fingerprint cache database into a portable .cddmpack archive.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cache_dir": { "type": "string", "description": "Path to cache database (default: .cddm/cache.db)" },
                    "output_pack_path": { "type": "string", "description": "Target .cddmpack output path (default: cddm-cache.cddmpack)" }
                }
            }
        }),
        json!({
            "name": mcp_tools::IMPORT_CACHE_PACK,
            "description": "Import a portable .cddmpack archive into persistent fingerprint cache database.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "pack_file": { "type": "string", "description": "Path to .cddmpack file" },
                    "target_cache_dir": { "type": "string", "description": "Target cache directory (default: .cddm)" }
                },
                "required": ["pack_file"]
            }
        }),
        json!({
            "name": mcp_tools::SCAN_MONOREPO,
            "description": "Discover monorepo workspaces and execute comprehensive cross-package duplication scan.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Monorepo root directory" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": "Minimum token threshold" }
                }
            }
        }),
        json!({
            "name": mcp_tools::GET_SEMANTIC_GRAPH,
            "description": "Extract Control Flow Graph (CFG) and Program Dependence Graph (PDG) structures and Weisfeiler-Lehman hash from source code or file.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "code": { "type": "string", "description": "Raw source code snippet" },
                    "file": { "type": "string", "description": "File path to read source from" },
                    "language": { "type": "string", "description": "Target programming language (default: Rust)" }
                }
            }
        }),
        json!({
            "name": mcp_tools::COMPARE_SEMANTIC_GRAPHS,
            "description": "Compare two code snippets for Type-4 semantic clone similarity via Weisfeiler-Lehman graph kernels and subword embeddings.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "code_a": { "type": "string", "description": "First code snippet to compare" },
                    "code_b": { "type": "string", "description": "Second code snippet to compare" },
                    "language": { "type": "string", "description": "Default programming language (default: Rust)" },
                    "language_a": { "type": "string", "description": "Language of first snippet" },
                    "language_b": { "type": "string", "description": "Language of second snippet" }
                },
                "required": ["code_a", "code_b"]
            }
        }),
        json!({
            "name": mcp_tools::SCAN_CROSS_LANGUAGE,
            "description": "Discover cross-language semantic clones across different programming languages via Weisfeiler-Lehman graph kernels and subword vector embeddings.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target workspace directory to scan (default: current directory)" },
                    "threshold": { "type": "number", "description": "Hybrid similarity threshold (0.0 to 1.0, default: 0.70)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": "Minimum token threshold (default: 50)" },
                    "languages": { "type": "array", "items": { "type": "string" }, "description": "Optional list of languages to restrict analysis to" },
                    "ignore": { "type": "array", "items": { "type": "string" }, "description": "Optional glob patterns to ignore" }
                }
            }
        }),
        json!({
            "name": mcp_tools::EXTRACT_SHARED_MODULE,
            "description": "Automate extracting duplicate code into a standalone shared crate or module with manifest updates and caller rewrites.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "target": { "type": "string", "description": "Target crate path or module path (e.g. crates/shared_utils)" },
                    "fn_name": { "type": "string", "description": "Custom function name for extracted helper" },
                    "crate_type": { "type": "string", "description": "Packaging strategy: auto, crate, module, existing" },
                    "dry_run": { "type": "boolean", "description": "Preview extraction without writing to disk (default: false)" },
                    "generate_tests": { "type": "boolean", "description": "Automatically synthesize unit tests for the extracted helper" },
                    "generate_benchmarks": { "type": "boolean", "description": "Automatically synthesize performance micro-benchmarks for the extracted helper" },
                    mcp_tools::PARAM_CLUSTER_ID: { "type": "number", "description": "1-based cluster index" },
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": "Minimum token threshold" },
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
        }),
        json!({
            "name": mcp_tools::DETECT_OVERLAP,
            "description": "Detect reimplemented ecosystem library algorithms and suggest standard packages.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target workspace directory path (default: .)" },
                    "threshold": { "type": "number", "description": "Confidence threshold from 0.0 to 1.0 (default: 0.3)" }
                }
            }
        }),
    ]
}
