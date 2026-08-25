#![forbid(unsafe_code)]

use cddm_core::{ApplyPatchResult, CloneLocation, ScanConfig, ScanProgress, ScanResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

/// API endpoint path for health checks.
pub const ROUTE_API_HEALTH: &str = "/api/health";

/// API endpoint path for asynchronous code duplication scans.
pub const ROUTE_API_SCAN: &str = "/api/scan";

/// API endpoint path for snippet source line retrieval.
pub const ROUTE_API_SNIPPET: &str = "/api/snippet";

/// API endpoint path for on-demand refactoring patch synthesis.
pub const ROUTE_API_REFACTOR: &str = "/api/refactor";

/// API endpoint path for on-demand multi-site cluster refactoring patch synthesis.
pub const ROUTE_API_REFACTOR_CLUSTER: &str = "/api/refactor-cluster";

/// API endpoint path for applying synthesized refactoring patches directly to workspace files.
pub const ROUTE_API_APPLY_PATCH: &str = "/api/apply-patch";

/// API endpoint path for Server-Sent Events (SSE) live updates.
pub const ROUTE_API_EVENTS: &str = "/api/events";

/// API endpoint path for Git timeline historical trends.
pub const ROUTE_API_TIMELINE: &str = "/api/timeline";

/// API endpoint path for Git hook status inspection.
pub const ROUTE_API_HOOKS: &str = "/api/workflow/hooks";

/// API endpoint path for Git hook installation.
pub const ROUTE_API_HOOKS_INSTALL: &str = "/api/workflow/hooks/install";

/// API endpoint path for suppression rules retrieval and inspection.
pub const ROUTE_API_SUPPRESSION_RULES: &str = "/api/suppression/rules";

/// API endpoint path for interactive refactoring preview sandbox.
pub const ROUTE_API_REFACTOR_SANDBOX: &str = "/api/refactor/sandbox";

/// API endpoint path for applying refactoring patch directly to a dedicated Git branch.
pub const ROUTE_API_REFACTOR_APPLY_BRANCH: &str = "/api/refactor/apply-branch";

/// API endpoint path for synthesizing an LLM AI refactoring prompt specification.
pub const ROUTE_API_REFACTOR_AI_PROMPT: &str = "/api/refactor/ai-prompt";

/// API endpoint path for AST-native tree-sitter refactoring preview.
pub const ROUTE_API_REFACTOR_AST: &str = "/api/refactor/ast";

/// API endpoint path for closed-loop test suite verification.
pub const ROUTE_API_REFACTOR_VERIFY: &str = "/api/refactor/verify";

/// API endpoint path for architectural policy rules retrieval and configuration.
pub const ROUTE_API_POLICY_RULES: &str = "/api/policy/rules";

/// API endpoint path for on-demand policy evaluation against scan results.
pub const ROUTE_API_POLICY_EVALUATE: &str = "/api/policy/evaluate";

/// API endpoint path for AI Code Surgeon autonomous healing.
pub const ROUTE_API_REFACTOR_HEAL: &str = "/api/refactor/heal";

/// API endpoint path for cache pack export.
pub const ROUTE_API_CACHE_EXPORT: &str = "/api/cache/export";

/// API endpoint path for cache pack import.
pub const ROUTE_API_CACHE_IMPORT: &str = "/api/cache/import";

/// API endpoint path for monorepo workspace discovery and scan.
pub const ROUTE_API_MONOREPO: &str = "/api/monorepo";

/// API endpoint path for semantic graph extraction and comparison.
pub const ROUTE_API_SEMANTIC_GRAPH: &str = "/api/semantic-graph";

/// Default localhost IPv4 binding.
pub const DEFAULT_HOST_IP: [u8; 4] = [127, 0, 0, 1];

/// Default WebUI HTTP server port.
#[allow(dead_code)]
pub const DEFAULT_PORT: u16 = 3000;

/// Default service name returned in health check metadata.
pub const SERVICE_NAME: &str = "CDDM Studio";

/// Status response string for operational services.
pub const STATUS_OK: &str = "ok";

/// SPA default fallback index HTML asset.
pub const INDEX_HTML: &str = "index.html";

/// Default MIME type for HTML document responses.
pub const MIME_TEXT_HTML: &str = "text/html";

/// Not found error message.
pub const NOT_FOUND_MSG: &str = "404 Not Found";

/// Default surrounding context line count.
pub const DEFAULT_CONTEXT_LINES: usize = 3;

/// Maximum allowed surrounding context line count.
pub const MAX_CONTEXT_LINES: usize = 20;

/// Default watch polling interval in milliseconds.
pub const DEFAULT_WATCH_INTERVAL_MS: u64 = 300;

/// Server-Sent Event payload broadcasted to connected clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerEvent {
    #[serde(rename = "scan_started")]
    ScanStarted { scan_id: String },
    #[serde(rename = "scan_progress")]
    ScanProgress(ScanProgress),
    #[serde(rename = "scan_complete")]
    ScanComplete(ScanResult),
    #[serde(rename = "patch_applied")]
    PatchApplied(ApplyPatchResult),
}

/// Shared application state for Axum router.
#[derive(Clone)]
pub struct AppState {
    pub broadcast_tx: broadcast::Sender<ServerEvent>,
    pub current_config: Arc<RwLock<ScanConfig>>,
    pub latest_result: Arc<RwLock<Option<ScanResult>>>,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish_non_exhaustive()
    }
}

/// Query parameters for snippet extraction.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SnippetQuery {
    pub file: String,
    pub start: usize,
    pub end: usize,
    pub context: Option<usize>,
}

/// A single source line in a snippet response.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnippetLine {
    pub line_number: usize,
    pub content: String,
    pub is_target: bool,
}

/// Structured response containing source snippet lines with context.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnippetResponse {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub context_start_line: usize,
    pub context_end_line: usize,
    pub lines: Vec<SnippetLine>,
    pub total_lines: usize,
    pub language: String,
}

/// Request payload for synthesizing refactoring suggestions.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RefactorRequest {
    pub file_a: String,
    pub start_line_a: usize,
    pub end_line_a: usize,
    pub file_b: String,
    pub start_line_b: usize,
    pub end_line_b: usize,
}

/// Request payload for synthesizing multi-site cluster refactoring suggestions.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClusterRefactorRequest {
    pub cluster_id: String,
    pub occurrences: Vec<CloneLocation>,
}

/// Request query parameters for fetching Git timeline historical metrics.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TimelineQuery {
    pub directory: Option<String>,
    pub max_samples: Option<usize>,
    pub min_tokens: Option<usize>,
}

/// Request payload for installing a Git hook.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InstallHookRequest {
    pub directory: Option<String>,
    pub hook_type: String,
    pub fail_threshold: Option<f64>,
    pub min_tokens: Option<usize>,
}

/// Request payload for applying a refactoring patch to the workspace.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApplyPatchRequest {
    pub patch: String,
    #[serde(default)]
    pub dry_run: bool,
}

/// Response payload containing the synthesized AI refactoring prompt specification.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AiPromptResponse {
    /// Formatted AI assistant prompt specification markdown
    pub prompt: String,
}

/// Request payload for exporting persistent cache pack.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CacheExportRequest {
    pub cache_dir: Option<std::path::PathBuf>,
    pub output_pack_path: Option<std::path::PathBuf>,
}

/// Request payload for importing persistent cache pack.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheImportRequest {
    pub pack_file: std::path::PathBuf,
    pub target_cache_dir: Option<std::path::PathBuf>,
}

/// Request payload for monorepo workspace scan.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MonorepoScanRequest {
    pub directory: Option<std::path::PathBuf>,
    pub min_tokens: Option<usize>,
}

/// Request payload for semantic graph extraction and comparison.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub struct SemanticGraphRequest {
    pub file: Option<String>,
    pub code: Option<String>,
    pub language: Option<String>,
    pub file_b: Option<String>,
    pub code_b: Option<String>,
    pub language_b: Option<String>,
}

/// Comparison metrics between two semantic graphs.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SemanticComparisonResponse {
    pub similarity: f64,
    pub is_semantic_clone: bool,
    pub wl_hash_a: u64,
    pub wl_hash_b: u64,
}

/// Response payload containing extracted CFGs, PDGs, and optional comparison metrics.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SemanticGraphResponse {
    pub cfgs: Vec<cddm_core::semantic_graph::ControlFlowGraph>,
    pub pdgs: Vec<cddm_core::semantic_graph::ProgramDependenceGraph>,
    pub comparison: Option<SemanticComparisonResponse>,
}
