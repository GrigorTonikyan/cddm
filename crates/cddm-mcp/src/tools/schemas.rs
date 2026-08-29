#![forbid(unsafe_code)]

use super::helpers::clone_pair_input_schema;
use crate::protocol::mcp_tools;
use cddm_core::DEFAULT_MIN_TOKENS;
use serde_json::json;

fn dir_and_tokens_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path (default: current directory)" },
            mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
        }
    })
}

fn coverage_schema(include_extra: bool) -> serde_json::Value {
    let mut props = serde_json::Map::new();
    props.insert(
        "report_path".to_string(),
        json!({ "type": "string", "description": "Path to coverage tracefile (e.g. lcov.info, coverage.xml)" }),
    );
    props.insert(
        "report_content".to_string(),
        json!({ "type": "string", "description": "Raw coverage report file content" }),
    );
    props.insert(
        "directory".to_string(),
        json!({ "type": "string", "description": "Target workspace directory path (default: .)" }),
    );
    props.insert(
        "min_tokens".to_string(),
        json!({ "type": "number", "description": "Minimum token threshold (default: 50)" }),
    );
    if include_extra {
        props.insert(
            "format".to_string(),
            json!({ "type": "string", "description": "Coverage format: lcov, cobertura, istanbul, auto (default: auto)" }),
        );
        props.insert(
            "dead_code_only".to_string(),
            json!({ "type": "boolean", "description": "Filter for dead code duplicates with 0 runtime executions" }),
        );
        props.insert(
            "min_hits".to_string(),
            json!({ "type": "number", "description": "Minimum combined runtime execution hits" }),
        );
    }
    json!({
        "type": "object",
        "properties": props
    })
}

/// Returns the complete list of available MCP tool definitions and input schemas.
pub fn get_tool_definitions() -> Vec<serde_json::Value> {
    vec![
        json!({
            "name": mcp_tools::SCAN_CODEBASE,
            "description": "Run CDDM polyglot code duplication and DRY health score analysis on a target directory.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path to analyze (default: current directory)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) },
                    mcp_tools::PARAM_ENABLE_GIT_BLAME: { "type": "boolean", "description": "Annotate duplicate lines with in-process git blame author metadata" }
                }
            }
        }),
        json!({ "name": mcp_tools::GET_CLONE_PAIR, "description": "Fetch localized source lines, token counts, and git blame context for a duplicate clone pair.", "inputSchema": clone_pair_input_schema() }),
        json!({ "name": mcp_tools::SUGGEST_REFACTOR, "description": "Run invariant extraction on duplicate clone fragments and generate a structural refactoring suggestion with unified .patch format.", "inputSchema": clone_pair_input_schema() }),
        json!({
            "name": mcp_tools::GET_CLONE_CLUSTER,
            "description": "Fetch localized source lines, token counts, and occurrences context for an N-way clone cluster.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_CLUSTER_ID: { "type": "number", "description": "1-based cluster index" },
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path (default: current directory)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
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
                    mcp_tools::PARAM_CLUSTER_ID: { "type": "number", "description": "1-based cluster index" },
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path (default: current directory)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) },
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
            "inputSchema": dir_and_tokens_schema()
        }),
        json!({
            "name": mcp_tools::DIFF_SCAN,
            "description": "Perform git-aware differential duplication analysis between two git references.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_BASE_REF: { "type": "string", "description": "Base Git reference branch/tag/commit (e.g. 'main')" },
                    mcp_tools::PARAM_TARGET_REF: { "type": "string", "description": "Target Git reference branch/tag/commit (default: 'HEAD')" },
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path (default: current directory)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
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
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path (default: current directory)" },
                    mcp_tools::PARAM_MAX_SAMPLES: { "type": "number", "description": "Maximum historical commit samples to inspect (default: 10)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
                }
            }
        }),
        json!({
            "name": mcp_tools::CHECK_SUPPRESSION,
            "description": "Check if a file path or line number is suppressed by .cddmignore rules or inline suppression directives.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_PATH: { "type": "string", "description": "Target file path to check" },
                    mcp_tools::PARAM_LINE: { "type": "number", "description": "Optional 1-based line number to check for inline suppression directives" },
                    mcp_tools::PARAM_CDDMIGNORE: { "type": "string", "description": "Custom path to .cddmignore file" },
                    mcp_tools::PARAM_IGNORE_TESTS: { "type": "boolean", "description": "Check with test file suppression enabled" },
                    mcp_tools::PARAM_IGNORE_MOCKS: { "type": "boolean", "description": "Check with mock file suppression enabled" },
                    mcp_tools::PARAM_IGNORE_GENERATED: { "type": "boolean", "description": "Check with generated file suppression enabled" }
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
                    mcp_tools::PARAM_PATCH: { "type": "string", "description": "Unified diff patch content synthesized by CDDM" },
                    mcp_tools::PARAM_BRANCH_NAME: { "type": "string", "description": "Optional target Git branch name (default: cddm/refactor-auto)" },
                    mcp_tools::PARAM_CREATE_BRANCH: { "type": "boolean", "description": "Whether to create a new Git branch for the refactoring changes (default: false)" }
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
                    "custom_parameter_names": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["occurrences"]
            }
        }),
        json!({
            "name": mcp_tools::VERIFY_REFACTOR,
            "description": "Run closed-loop test verification to validate that a refactoring patch does not break existing test suites.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "patch": { "type": "string", "description": "Unified diff patch content" },
                    "test_command": { "type": "string", "description": "Custom test command to execute (e.g. 'cargo test', 'npm test')" },
                    "timeout_seconds": { "type": "number", "description": "Execution timeout in seconds (default: 30)" }
                },
                "required": ["patch"]
            }
        }),
        json!({
            "name": mcp_tools::CHECK_POLICIES,
            "description": "Evaluate workspace code duplication and architectural boundaries against .cddmrules.toml policies.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target workspace directory path (default: .)" },
                    mcp_tools::PARAM_RULES: { "type": "string", "description": "Custom path to .cddmrules.toml policy rules file" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
                }
            }
        }),
        json!({
            "name": mcp_tools::HEAL_REFACTOR,
            "description": "Run autonomous AI Code Surgeon loop: synthesize refactoring, run compiler/tests, diagnose errors, and heal patch automatically.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cluster_id": { "type": "number", "description": "1-based cluster index" },
                    "max_iterations": { "type": "number", "description": "Maximum healing retry iterations (default: 3)" },
                    "provider": { "type": "string", "description": "AI provider backend (mock, openai, anthropic, gemini)" },
                    "api_key": { "type": "string", "description": "Optional API key for AI provider" },
                    "model": { "type": "string", "description": "Model identifier (e.g. gpt-4o, claude-3-5-sonnet)" },
                    "test_command": { "type": "string", "description": "Test command to verify refactoring (e.g. 'cargo test')" }
                },
                "required": ["cluster_id"]
            }
        }),
        json!({
            "name": mcp_tools::EXPORT_CACHE_PACK,
            "description": "Export CDDM incremental fingerprint cache into a compressed portable .cddmpack archive for CI/CD sharing.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "output_file": { "type": "string", "description": "Destination path for .cddmpack archive (default: cddm-cache.cddmpack)" },
                    "cache_dir": { "type": "string", "description": "Path to persistent cache directory (default: .cddm/cache.db)" }
                }
            }
        }),
        json!({
            "name": mcp_tools::IMPORT_CACHE_PACK,
            "description": "Import a shared .cddmpack cache archive to warm up local or CI incremental scan cache.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "pack_file": { "type": "string", "description": "Path to .cddmpack archive file to import" },
                    "target_dir": { "type": "string", "description": "Destination cache directory (default: .cddm/cache.db)" }
                },
                "required": ["pack_file"]
            }
        }),
        json!({
            "name": mcp_tools::SCAN_MONOREPO,
            "description": "Discover all workspaces and packages across Cargo, npm/pnpm/yarn, and Python Poetry monorepos and analyze cross-package duplicates.",
            "inputSchema": dir_and_tokens_schema()
        }),
        json!({
            "name": mcp_tools::GET_SEMANTIC_GRAPH,
            "description": "Extract Control Flow Graph (CFG) and Program Dependence Graph (PDG) from source code for AST semantic analysis.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_path": { "type": "string", "description": "File path of source code" },
                    "code_content": { "type": "string", "description": "Source code snippet (optional if file_path is provided)" },
                    "language": { "type": "string", "description": "Programming language (e.g. rust, typescript, python)" }
                }
            }
        }),
        json!({
            "name": mcp_tools::COMPARE_SEMANTIC_GRAPHS,
            "description": "Compare two code snippets or files using semantic CFG/PDG graph isomorphism and Weisfeiler-Lehman graph kernels.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "code_a": { "type": "string", "description": "First code snippet" },
                    "language_a": { "type": "string", "description": "Language of first code snippet" },
                    "code_b": { "type": "string", "description": "Second code snippet" },
                    "language_b": { "type": "string", "description": "Language of second code snippet" }
                },
                "required": ["code_a", "code_b"]
            }
        }),
        json!({
            "name": mcp_tools::SCAN_CROSS_LANGUAGE,
            "description": "Scan codebase for cross-language semantic clones and compute language translation parity.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target workspace directory path (default: .)" },
                    "threshold": { "type": "number", "description": "Similarity threshold from 0.0 to 1.0 (default: 0.70)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
                }
            }
        }),
        json!({
            "name": mcp_tools::EXTRACT_SHARED_MODULE,
            "description": "Automated shared crate or module extraction generator with caller AST rewrites and manifest updates.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cluster_id": { "type": "number", "description": "1-based cluster index to extract" },
                    "target_kind": { "type": "string", "description": "Target extraction kind: new_crate or existing_module (default: new_crate)" },
                    "target_path": { "type": "string", "description": "Destination crate or module path (e.g. 'crates/shared_utils')" },
                    "function_name": { "type": "string", "description": "Custom name for extracted function" },
                    "generate_tests": { "type": "boolean", "description": "Generate companion unit test file" },
                    "generate_benchmarks": { "type": "boolean", "description": "Generate performance micro-benchmark file" },
                    "dry_run": { "type": "boolean", "description": "Preview without writing to disk (default: true)" }
                },
                "required": ["cluster_id"]
            }
        }),
        json!({
            "name": mcp_tools::DETECT_OVERLAP,
            "description": "Scan workspace for custom algorithms that reimplement standard library or popular open-source packages.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target workspace directory path (default: .)" },
                    "threshold": { "type": "number", "description": "Confidence threshold from 0.0 to 1.0 (default: 0.3)" }
                }
            }
        }),
        json!({
            "name": mcp_tools::SCAN_HUB,
            "description": "Scan multiple repositories in an Organization Federation Hub for cross-repository code duplication.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "config_path": { "type": "string", "description": "Path to .cddmhub.toml configuration file" },
                    "repositories": { "type": "array", "items": { "type": "string" }, "description": "List of repository directory paths to scan" },
                    "min_tokens": { "type": "number", "description": "Minimum token threshold (default: 50)" }
                }
            }
        }),
        json!({
            "name": mcp_tools::EXTRACT_HUB_PACKAGE,
            "description": "Extract a cross-repository clone cluster into a standalone shared package with caller repository PR updates.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cluster_id": { "type": "number", "description": "1-based cross-repository cluster index" },
                    "package_name": { "type": "string", "description": "Name for the shared package" },
                    "package_type": { "type": "string", "description": "Package ecosystem: npm, cargo, pypi, go (default: npm)" },
                    "target_dir": { "type": "string", "description": "Destination path for standalone package directory" },
                    "config_path": { "type": "string", "description": "Optional path to .cddmhub.toml" },
                    "dry_run": { "type": "boolean", "description": "Preview without writing to disk (default: true)" }
                },
                "required": ["cluster_id"]
            }
        }),
        json!({
            "name": mcp_tools::CORRELATE_COVERAGE,
            "description": "Correlate test and runtime execution coverage tracefiles (LCOV, Cobertura, Istanbul) with duplicate clone pairs.",
            "inputSchema": coverage_schema(true)
        }),
        json!({
            "name": mcp_tools::DETECT_DEAD_CLONES,
            "description": "Find duplicate code fragments across the codebase that have zero runtime/test executions (dead code elimination candidates).",
            "inputSchema": coverage_schema(false)
        }),
        json!({
            "name": mcp_tools::SEMANTIC_NEURAL_SCAN,
            "description": "In-process local neural code embedding & algorithmic equivalence scan using subword projections and cosine similarity.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "directory": { "type": "string", "description": "Target workspace directory path (default: .)" },
                    "threshold": { "type": "number", "description": "Minimum cosine similarity threshold from 0.0 to 1.0 (default: 0.85)" },
                    "dimension": { "type": "number", "description": "Dense embedding vector dimensionality (default: 256)" },
                    "code_a": { "type": "string", "description": "Optional snippet A for direct pairwise comparison" },
                    "language_a": { "type": "string", "description": "Programming language for snippet A" },
                    "code_b": { "type": "string", "description": "Optional snippet B for direct pairwise comparison" },
                    "language_b": { "type": "string", "description": "Programming language for snippet B" }
                }
            }
        }),
    ]
}
