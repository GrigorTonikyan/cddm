#![forbid(unsafe_code)]

use cddm_core::{ApplyPatchResult, CloneLocation, ScanConfig, ScanProgress, ScanResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

/// Core REST API endpoint routes.
pub const ROUTE_API_HEALTH: &str = "/api/health";
pub const ROUTE_API_SCAN: &str = "/api/scan";
pub const ROUTE_API_DIFF: &str = "/api/diff";
pub const ROUTE_API_SNIPPET: &str = "/api/snippet";
pub const ROUTE_API_REFACTOR: &str = "/api/refactor";
pub const ROUTE_API_REFACTOR_CLUSTER: &str = "/api/refactor-cluster";
pub const ROUTE_API_APPLY_PATCH: &str = "/api/apply-patch";
pub const ROUTE_API_EVENTS: &str = "/api/events";
pub const ROUTE_API_TIMELINE: &str = "/api/timeline";
pub const ROUTE_API_HOOKS: &str = "/api/workflow/hooks";
pub const ROUTE_API_HOOKS_INSTALL: &str = "/api/workflow/hooks/install";
pub const ROUTE_API_SUPPRESSION_RULES: &str = "/api/suppression/rules";
pub const ROUTE_API_REFACTOR_SANDBOX: &str = "/api/refactor/sandbox";
pub const ROUTE_API_REFACTOR_APPLY_BRANCH: &str = "/api/refactor/apply-branch";
pub const ROUTE_API_REFACTOR_AI_PROMPT: &str = "/api/refactor/ai-prompt";
pub const ROUTE_API_REFACTOR_AST: &str = "/api/refactor/ast";
pub const ROUTE_API_REFACTOR_VERIFY: &str = "/api/refactor/verify";
pub const ROUTE_API_POLICY_RULES: &str = "/api/policy/rules";
pub const ROUTE_API_POLICY_EVALUATE: &str = "/api/policy/evaluate";
pub const ROUTE_API_REFACTOR_HEAL: &str = "/api/refactor/heal";
pub const ROUTE_API_CACHE_EXPORT: &str = "/api/cache/export";
pub const ROUTE_API_CACHE_IMPORT: &str = "/api/cache/import";
pub const ROUTE_API_MONOREPO: &str = "/api/monorepo";
pub const ROUTE_API_SEMANTIC_GRAPH: &str = "/api/semantic-graph";
pub const ROUTE_API_SEMANTIC_SCAN: &str = "/api/semantic/scan";
pub const ROUTE_API_SEMANTIC_NEURAL: &str = "/api/semantic/neural";
pub const ROUTE_API_WATCH_STATUS: &str = "/api/watch/status";
pub const ROUTE_API_WATCH_TOGGLE: &str = "/api/watch/toggle";
pub const ROUTE_API_WATCH_RESCAN: &str = "/api/watch/rescan";
pub const ROUTE_API_EXTRACT_PREVIEW: &str = "/api/extract/preview";
pub const ROUTE_API_EXTRACT_APPLY: &str = "/api/extract/apply";
pub const ROUTE_API_OVERLAP_CATALOG: &str = "/api/overlap/catalog";
pub const ROUTE_API_OVERLAP_SCAN: &str = "/api/overlap/scan";
pub const ROUTE_API_HUB_CONFIG: &str = "/api/hub/config";
pub const ROUTE_API_HUB_SCAN: &str = "/api/hub/scan";
pub const ROUTE_API_HUB_EXTRACT: &str = "/api/hub/extract";
pub const ROUTE_API_COVERAGE_INGEST: &str = "/api/coverage/ingest";
pub const ROUTE_API_COVERAGE_CORRELATE: &str = "/api/coverage/correlate";

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
    #[serde(rename = "watch_file_changed")]
    WatchFileChanged { files: Vec<String>, timestamp: u64 },
    #[serde(rename = "watch_scan_delta")]
    WatchScanDelta(cddm_core::WatchDeltaReport),
    #[serde(rename = "watch_status_changed")]
    WatchStatusChanged { is_active: bool },
}

/// Shared application state for Axum router.
#[derive(Clone)]
pub struct AppState {
    pub broadcast_tx: broadcast::Sender<ServerEvent>,
    pub current_config: Arc<RwLock<ScanConfig>>,
    pub latest_result: Arc<RwLock<Option<ScanResult>>>,
    pub watch_active: Arc<std::sync::atomic::AtomicBool>,
    pub watch_events_log: Arc<RwLock<Vec<cddm_core::WatchDeltaReport>>>,
    pub last_sync_timestamp: Arc<std::sync::atomic::AtomicU64>,
    pub sync_count: Arc<std::sync::atomic::AtomicUsize>,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish_non_exhaustive()
    }
}

/// Status response for real-time workspace watch daemon.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WatchStatusResponse {
    pub is_active: bool,
    pub watch_directory: String,
    pub debounce_ms: u64,
    pub last_sync_timestamp: Option<u64>,
    pub sync_count: usize,
    pub last_duration_ms: Option<u128>,
    pub recent_events: Vec<cddm_core::WatchDeltaReport>,
}

/// Request payload for toggling watch state.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct WatchToggleRequest {
    pub active: Option<bool>,
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

/// Request payload for differential git clone scan.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiffScanRequest {
    pub base_ref: String,
    pub target_ref: Option<String>,
    #[serde(default)]
    pub config: ScanConfig,
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
    pub function_a: Option<String>,
    pub lines_a: Option<(usize, usize)>,
    pub file_b: Option<String>,
    pub code_b: Option<String>,
    pub language_b: Option<String>,
    pub function_b: Option<String>,
    pub lines_b: Option<(usize, usize)>,
}

/// Request payload for on-demand workspace cross-language clone scans.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct SemanticScanRequest {
    pub directory: Option<String>,
    pub threshold: Option<f64>,
    pub min_tokens: Option<usize>,
    pub languages: Option<Vec<String>>,
    pub ignore: Option<Vec<String>>,
}

/// Comparison metrics between two semantic graphs.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SemanticComparisonResponse {
    pub similarity: f64,
    pub graph_similarity: f64,
    pub token_similarity: f64,
    pub hybrid_score: f64,
    pub is_semantic_clone: bool,
    pub is_cross_language: bool,
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

/// Request payload for ingesting coverage report content.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CoverageIngestRequest {
    pub report_content: Option<String>,
    pub report_path: Option<String>,
    pub format: Option<String>,
}

/// Request payload for correlating coverage with scan results.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CoverageCorrelateRequest {
    pub report_path: Option<String>,
    pub report_content: Option<String>,
    pub format: Option<String>,
    pub directory: Option<String>,
    pub min_tokens: Option<usize>,
    pub dead_code_only: Option<bool>,
    pub min_hits: Option<u64>,
    pub risk_threshold: Option<f64>,
}

/// Request payload for in-process neural code embedding equivalence scan.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct SemanticNeuralRequest {
    pub directory: Option<String>,
    pub threshold: Option<f32>,
    pub dimension: Option<usize>,
    pub max_subwords: Option<usize>,
}
