#![forbid(unsafe_code)]

use super::helpers::clone_pair_input_schema;
use crate::protocol::mcp_tools;
use cddm_core::DEFAULT_MIN_TOKENS;
use serde_json::json;

fn tool_def(name: &str, description: &str, input_schema: serde_json::Value) -> serde_json::Value {
    let category = if name.contains("scan")
        || name.contains("diff")
        || name.contains("timeline")
        || name.contains("overlap")
        || name.contains("coverage")
        || name.contains("dead_clones")
        || name.contains("graph")
    {
        "detection"
    } else if name.contains("refactor") || name.contains("extract") || name.contains("heal") {
        "refactoring"
    } else if name.contains("polic") || name.contains("suppression") || name.contains("sarif") {
        "governance"
    } else {
        "synchronization"
    };

    json!({
        "name": name,
        "description": description,
        "inputSchema": input_schema,
        "x-cddm-category": category
    })
}

fn obj_schema(props: serde_json::Value, required: &[&str]) -> serde_json::Value {
    if required.is_empty() {
        json!({ "type": "object", "properties": props })
    } else {
        json!({ "type": "object", "properties": props, "required": required })
    }
}

fn dir_and_tokens_schema() -> serde_json::Value {
    obj_schema(
        json!({
            mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path (default: current directory)" },
            mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
        }),
        &[],
    )
}

fn occurrences_item_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "file": { "type": "string" },
            "start_line": { "type": "integer" },
            "end_line": { "type": "integer" }
        },
        "required": ["file", "start_line", "end_line"]
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
        tool_def(
            mcp_tools::SCAN_CODEBASE,
            "Run CDDM polyglot code duplication and DRY health score analysis on a target \
             directory.",
            obj_schema(
                json!({
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path to analyze (default: current directory)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) },
                    mcp_tools::PARAM_ENABLE_GIT_BLAME: { "type": "boolean", "description": "Annotate duplicate lines with in-process git blame author metadata" },
                    "detect_type3": { "type": "boolean", "description": "Enable Type-3 (near-miss modified statements) clone detection (default: true)" }
                }),
                &[],
            ),
        ),
        tool_def(
            mcp_tools::GET_CLONE_PAIR,
            "Fetch localized source lines, token counts, and git blame context for a duplicate \
             clone pair.",
            clone_pair_input_schema(),
        ),
        tool_def(
            mcp_tools::SUGGEST_REFACTOR,
            "Run invariant extraction on duplicate clone fragments and generate a structural \
             refactoring suggestion with unified .patch format.",
            clone_pair_input_schema(),
        ),
        tool_def(
            mcp_tools::GET_CLONE_CLUSTER,
            "Fetch localized source lines, token counts, and occurrences context for an N-way \
             clone cluster.",
            obj_schema(
                json!({
                    mcp_tools::PARAM_CLUSTER_ID: { "type": "number", "description": "1-based cluster index" },
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path (default: current directory)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
                }),
                &[mcp_tools::PARAM_CLUSTER_ID],
            ),
        ),
        tool_def(
            mcp_tools::SUGGEST_CLUSTER_REFACTOR,
            "Generate an automated multi-site refactoring patch synthesizing a single shared \
             abstraction and updating all N occurrence call-sites.",
            obj_schema(
                json!({
                    mcp_tools::PARAM_CLUSTER_ID: { "type": "number", "description": "1-based cluster index" },
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path (default: current directory)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) },
                    mcp_tools::PARAM_OCCURRENCES: {
                        "type": "array",
                        "description": "Explicit list of cluster occurrence locations",
                        "items": occurrences_item_schema()
                    }
                }),
                &[],
            ),
        ),
        tool_def(
            mcp_tools::EXPORT_SARIF,
            "Generate a standard SARIF (Static Analysis Results Interchange Format) JSON report \
             for CI/CD and security dashboards.",
            dir_and_tokens_schema(),
        ),
        tool_def(
            mcp_tools::DIFF_SCAN,
            "Perform git-aware differential duplication analysis between two git references.",
            obj_schema(
                json!({
                    mcp_tools::PARAM_BASE_REF: { "type": "string", "description": "Base git reference or commit SHA" },
                    mcp_tools::PARAM_TARGET_REF: { "type": "string", "description": "Target git reference or commit SHA (optional, defaults to working tree)" },
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target repository directory path (default: current directory)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
                }),
                &[mcp_tools::PARAM_BASE_REF],
            ),
        ),
        tool_def(
            mcp_tools::GET_TIMELINE,
            "Analyze historical duplication trend, file churn metrics, and author hotspots over \
             git commits.",
            obj_schema(
                json!({
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target repository directory path (default: current directory)" },
                    mcp_tools::PARAM_MAX_SAMPLES: { "type": "number", "description": "Maximum historical commit samples to inspect (default: 10)" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
                }),
                &[],
            ),
        ),
        tool_def(
            mcp_tools::CHECK_SUPPRESSION,
            "Test whether a file path or line span is suppressed by `.cddmignore` glob rules, \
             inline directives, or AST type filters.",
            obj_schema(
                json!({
                    mcp_tools::PARAM_PATH: { "type": "string", "description": "File path to test against suppression rules" },
                    mcp_tools::PARAM_LINE: { "type": "number", "description": "1-based line number (optional)" },
                    mcp_tools::PARAM_CDDMIGNORE: { "type": "string", "description": "Custom path to .cddmignore file" },
                    mcp_tools::PARAM_IGNORE_TESTS: { "type": "boolean", "description": "Exclude tests (default: false)" },
                    mcp_tools::PARAM_IGNORE_MOCKS: { "type": "boolean", "description": "Exclude mocks (default: false)" },
                    mcp_tools::PARAM_IGNORE_GENERATED: { "type": "boolean", "description": "Exclude generated files (default: true)" }
                }),
                &[mcp_tools::PARAM_PATH],
            ),
        ),
        tool_def(
            mcp_tools::APPLY_CLUSTER_REFACTOR,
            "Apply synthesized cluster deduplication patch to the working tree or create an \
             isolated branch.",
            obj_schema(
                json!({
                    "patch": { "type": "string", "description": "Unified diff patch content" },
                    mcp_tools::PARAM_CREATE_BRANCH: { "type": "string", "description": "Target branch name to create and commit onto (e.g. 'cddm/refactor-cluster-1')" },
                    "dry_run": { "type": "boolean", "description": "Validate patch applicability without modifying disk (default: false)" }
                }),
                &["patch"],
            ),
        ),
        tool_def(
            mcp_tools::GENERATE_AI_PROMPT,
            "Synthesize a zero-shot AI refactoring prompt containing extracted invariants, \
             signature AST, and caller locations.",
            obj_schema(
                json!({
                    mcp_tools::PARAM_CLUSTER_ID: { "type": "number", "description": "1-based cluster index" },
                    mcp_tools::PARAM_OCCURRENCES: { "type": "array", "items": occurrences_item_schema() },
                    "custom_function_name": { "type": "string", "description": "Suggested function name (optional)" },
                    "target_module": { "type": "string", "description": "Suggested target module path (optional)" },
                    "model_target": { "type": "string", "description": "Target AI model (default: gemini-2.5-pro, supports claude-3-7-sonnet, gpt-4.5-preview)" }
                }),
                &[],
            ),
        ),
        tool_def(
            mcp_tools::AST_REFACTOR,
            "AST-native semantic refactoring preview synthesizing helper function AST and rewrite \
             replacements.",
            obj_schema(
                json!({
                    "occurrences": { "type": "array", "items": occurrences_item_schema() },
                    "custom_function_name": { "type": "string" },
                    "target_module_path": { "type": "string" },
                    "custom_parameter_names": { "type": "array", "items": { "type": "string" } }
                }),
                &["occurrences"],
            ),
        ),
        tool_def(
            mcp_tools::VERIFY_REFACTOR,
            "Run closed-loop test verification to validate that a refactoring patch does not \
             break existing test suites.",
            obj_schema(
                json!({
                    "patch": { "type": "string", "description": "Unified diff patch content" },
                    "test_command": { "type": "string", "description": "Custom test command to execute (e.g. 'cargo test', 'npm test')" },
                    "timeout_seconds": { "type": "number", "description": "Execution timeout in seconds (default: 30)" }
                }),
                &["patch"],
            ),
        ),
        tool_def(
            mcp_tools::CHECK_POLICIES,
            "Evaluate workspace code duplication and architectural boundaries against \
             .cddmrules.toml policies.",
            obj_schema(
                json!({
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target workspace directory path (default: .)" },
                    mcp_tools::PARAM_RULES: { "type": "string", "description": "Custom path to .cddmrules.toml policy rules file" },
                    mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
                }),
                &[],
            ),
        ),
        tool_def(
            mcp_tools::HEAL_REFACTOR,
            "Run autonomous AI Code Surgeon loop: synthesize refactoring, run compiler/tests, \
             diagnose errors, and heal patch automatically.",
            obj_schema(
                json!({
                    "cluster_id": { "type": "number", "description": "1-based cluster index" },
                    "max_iterations": { "type": "number", "description": "Maximum healing retry iterations (default: 3)" },
                    "provider": { "type": "string", "description": "AI provider backend (mock, openai, anthropic, gemini)" },
                    "api_key": { "type": "string", "description": "Optional API key for AI provider" },
                    "model": { "type": "string", "description": "Model identifier (e.g. gemini-2.5-pro, claude-3-7-sonnet, gpt-4.5-preview)" },
                    "test_command": { "type": "string", "description": "Test command to verify refactoring (e.g. 'cargo test')" }
                }),
                &["cluster_id"],
            ),
        ),
        tool_def(
            mcp_tools::EXPORT_CACHE_PACK,
            "Export CDDM incremental fingerprint cache into a compressed portable .cddmpack \
             archive for CI/CD sharing.",
            obj_schema(
                json!({
                    "output_file": { "type": "string", "description": "Destination path for .cddmpack archive (default: cddm-cache.cddmpack)" },
                    "cache_dir": { "type": "string", "description": "Path to persistent cache directory (default: .cddm/cache.db)" }
                }),
                &[],
            ),
        ),
        tool_def(
            mcp_tools::IMPORT_CACHE_PACK,
            "Import a shared .cddmpack cache archive to warm up local or CI incremental scan \
             cache.",
            obj_schema(
                json!({
                    "pack_file": { "type": "string", "description": "Path to .cddmpack archive file to import" },
                    "target_dir": { "type": "string", "description": "Destination cache directory (default: .cddm/cache.db)" }
                }),
                &["pack_file"],
            ),
        ),
        tool_def(
            mcp_tools::SCAN_MONOREPO,
            "Discover all workspaces and packages across Cargo, npm/pnpm/yarn, and Python Poetry \
             monorepos and analyze cross-package duplicates.",
            dir_and_tokens_schema(),
        ),
        tool_def(
            mcp_tools::GET_SEMANTIC_GRAPH,
            "Extract Control Flow Graph (CFG) and Program Dependence Graph (PDG) from source code \
             for AST semantic analysis.",
            obj_schema(
                json!({
                    "file_path": { "type": "string", "description": "File path of source code" },
                    "code_content": { "type": "string", "description": "Source code snippet (optional if file_path is provided)" },
                    "language": { "type": "string", "description": "Programming language (e.g. rust, typescript, python)" }
                }),
                &[],
            ),
        ),
        tool_def(
            mcp_tools::COMPARE_SEMANTIC_GRAPHS,
            "Compare two code snippets or files using semantic CFG/PDG graph isomorphism and \
             Weisfeiler-Lehman graph kernels.",
            obj_schema(
                json!({
                    "code_a": { "type": "string", "description": "First code snippet" },
                    "language_a": { "type": "string", "description": "Language of first code snippet" },
                    "code_b": { "type": "string", "description": "Second code snippet" },
                    "language_b": { "type": "string", "description": "Language of second code snippet" }
                }),
                &["code_a", "code_b"],
            ),
        ),
        tool_def(
            mcp_tools::SCAN_CROSS_LANGUAGE,
            "Scan workspace for cross-language semantic clones across Rust, TypeScript, Python, \
             and Go using CFG WL graph kernels.",
            obj_schema(
                json!({
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target workspace directory path (default: .)" },
                    "threshold": { "type": "number", "description": "Minimum similarity threshold from 0.0 to 1.0 (default: 0.70)" },
                    "min_tokens": { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) },
                    "languages": { "type": "array", "items": { "type": "string" }, "description": "Subset of languages to scan" },
                    "threads": { "type": "number", "description": "Maximum parallel worker threads" }
                }),
                &[],
            ),
        ),
        tool_def(
            mcp_tools::EXTRACT_SHARED_MODULE,
            "Automated shared crate or module extraction generator with caller AST rewrites and \
             manifest updates.",
            obj_schema(
                json!({
                    "cluster_id": { "type": "number", "description": "1-based cluster index to extract" },
                    "target_kind": { "type": "string", "description": "Target extraction kind: new_crate or existing_module (default: new_crate)" },
                    "target_path": { "type": "string", "description": "Destination crate or module path (e.g. 'crates/shared_utils')" },
                    "function_name": { "type": "string", "description": "Custom name for extracted function" },
                    "generate_tests": { "type": "boolean", "description": "Generate companion unit test file" },
                    "generate_benchmarks": { "type": "boolean", "description": "Generate performance micro-benchmark file" },
                    "dry_run": { "type": "boolean", "description": "Preview without writing to disk (default: true)" }
                }),
                &["cluster_id"],
            ),
        ),
        tool_def(
            mcp_tools::DETECT_OVERLAP,
            "Scan workspace for custom algorithms that reimplement standard library or popular \
             open-source packages.",
            obj_schema(
                json!({
                    mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target workspace directory path (default: .)" },
                    "threshold": { "type": "number", "description": "Confidence threshold from 0.0 to 1.0 (default: 0.3)" }
                }),
                &[],
            ),
        ),
        tool_def(
            mcp_tools::SCAN_HUB,
            "Scan multiple repositories in an Organization Federation Hub for cross-repository \
             code duplication.",
            obj_schema(
                json!({
                    "config_path": { "type": "string", "description": "Path to .cddmhub.toml configuration file" },
                    "repositories": { "type": "array", "items": { "type": "string" }, "description": "List of repository directory paths to scan" },
                    "min_tokens": { "type": "number", "description": "Minimum token threshold (default: 50)" }
                }),
                &[],
            ),
        ),
        tool_def(
            mcp_tools::EXTRACT_HUB_PACKAGE,
            "Extract a cross-repository clone cluster into a standalone shared package with \
             caller repository PR updates.",
            obj_schema(
                json!({
                    "cluster_id": { "type": "number", "description": "1-based cross-repository cluster index" },
                    "package_name": { "type": "string", "description": "Name for the shared package" },
                    "package_type": { "type": "string", "description": "Package ecosystem: npm, cargo, pypi, go (default: npm)" },
                    "target_dir": { "type": "string", "description": "Destination path for standalone package directory" },
                    "config_path": { "type": "string", "description": "Optional path to .cddmhub.toml" },
                    "dry_run": { "type": "boolean", "description": "Preview without writing to disk (default: true)" }
                }),
                &["cluster_id"],
            ),
        ),
        tool_def(
            mcp_tools::CORRELATE_COVERAGE,
            "Correlate test and runtime execution coverage tracefiles (LCOV, Cobertura, Istanbul) \
             with duplicate clone pairs.",
            coverage_schema(true),
        ),
        tool_def(
            mcp_tools::DETECT_DEAD_CLONES,
            "Find duplicate code fragments across the codebase that have zero runtime/test \
             executions (dead code elimination candidates).",
            coverage_schema(false),
        ),
        tool_def(
            mcp_tools::SEMANTIC_NEURAL_SCAN,
            "In-process local neural code embedding & algorithmic equivalence scan using subword \
             projections and cosine similarity.",
            obj_schema(
                json!({
                    "directory": { "type": "string", "description": "Target workspace directory path (default: .)" },
                    "threshold": { "type": "number", "description": "Minimum cosine similarity threshold from 0.0 to 1.0 (default: 0.85)" },
                    "dimension": { "type": "number", "description": "Dense embedding vector dimensionality (default: 256)" },
                    "code_a": { "type": "string", "description": "Optional snippet A for direct pairwise comparison" },
                    "language_a": { "type": "string", "description": "Programming language for snippet A" },
                    "code_b": { "type": "string", "description": "Optional snippet B for direct pairwise comparison" },
                    "language_b": { "type": "string", "description": "Programming language for snippet B" }
                }),
                &[],
            ),
        ),
        tool_def(
            mcp_tools::DIFF_MATRIX,
            "Evaluate multi-branch and Git worktree clone drift divergence matrix across multiple \
             branches/revisions.",
            obj_schema(
                json!({
                    "branches": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "List of 2 or more Git branch names or commit hashes to compare"
                    },
                    "directory": { "type": "string", "description": "Target Git repository directory path (default: .)" },
                    "min_tokens": { "type": "number", "description": "Minimum token threshold for clone identification (default: 50)" }
                }),
                &["branches"],
            ),
        ),
    ]
}
