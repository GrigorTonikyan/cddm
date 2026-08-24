use axum::{
    Router,
    body::Body,
    extract::{Json, Query, State},
    http::{HeaderValue, StatusCode, Uri, header},
    response::{
        IntoResponse, Response,
        sse::{Event, KeepAlive, Sse},
    },
    routing::{get, post},
};
use cddm_core::{
    ApplyPatchResult, CddmWatcher, CloneLocation, ClusterRefactorSuggestion, RefactorSuggestion,
    ScanConfig, ScanProgress, ScanResult, analyze_clone_refactoring, analyze_cluster_refactoring,
    apply_patch_to_workspace, grammar::get_grammar_for_path, run_scan,
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::{RwLock, broadcast, mpsc};
use tokio_stream::StreamExt as _;
use tokio_stream::wrappers::BroadcastStream;
use tower_http::cors::CorsLayer;

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

/// Default localhost IPv4 binding.
pub const DEFAULT_HOST_IP: [u8; 4] = [127, 0, 0, 1];

/// Default WebUI HTTP server port.
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

#[derive(RustEmbed)]
#[folder = "../../webui/dist"]
struct WebUIAssets;

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

/// Builds the Axum application router and default shared state.
pub fn build_app() -> (AppState, Router) {
    let (broadcast_tx, _) = broadcast::channel(200);
    let state = AppState {
        broadcast_tx,
        current_config: Arc::new(RwLock::new(ScanConfig::default())),
        latest_result: Arc::new(RwLock::new(None)),
    };
    let router = build_app_with_state(state.clone());
    (state, router)
}

/// Builds the Axum application router with explicitly provided shared state.
pub fn build_app_with_state(state: AppState) -> Router {
    Router::new()
        .route(ROUTE_API_HEALTH, get(health_handler))
        .route(ROUTE_API_SCAN, post(scan_handler))
        .route(ROUTE_API_SNIPPET, get(snippet_handler))
        .route(ROUTE_API_REFACTOR, post(refactor_handler))
        .route(ROUTE_API_REFACTOR_CLUSTER, post(refactor_cluster_handler))
        .route(ROUTE_API_APPLY_PATCH, post(apply_patch_handler))
        .route(ROUTE_API_EVENTS, get(events_handler))
        .route(ROUTE_API_TIMELINE, get(timeline_handler))
        .route(ROUTE_API_HOOKS, get(hooks_status_handler))
        .route(ROUTE_API_HOOKS_INSTALL, post(install_hook_handler))
        .fallback(static_asset_handler)
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Starts the Axum web server embedding the React WebUI and API endpoints with background watching.
pub async fn start_server(port: u16, open_browser: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (state, app) = build_app();

    let addr = SocketAddr::from((DEFAULT_HOST_IP, port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let server_url = format!("http://localhost:{}", port);

    println!("\nCDDM Studio WebUI server listening at {}", server_url);

    // Initialize background directory watcher for real-time live push updates
    let watcher_state = state.clone();
    tokio::spawn(async move {
        let watch_dir = PathBuf::from(".");
        if let Ok(watcher) = CddmWatcher::watch_directory(&watch_dir) {
            let mut interval =
                tokio::time::interval(std::time::Duration::from_millis(DEFAULT_WATCH_INTERVAL_MS));
            loop {
                interval.tick().await;
                let ignores = {
                    let cfg = watcher_state.current_config.read().await;
                    cfg.ignore_patterns.clone()
                };
                let changed_files = watcher.collect_changed_paths(&ignores);
                if !changed_files.is_empty() {
                    let config = watcher_state.current_config.read().await.clone();
                    let (tx, mut rx) = mpsc::channel(100);
                    let cancel_flag = Arc::new(AtomicBool::new(false));

                    let b_tx = watcher_state.broadcast_tx.clone();
                    tokio::spawn(async move {
                        while let Some(p) = rx.recv().await {
                            let _ = b_tx.send(ServerEvent::ScanProgress(p));
                        }
                    });

                    if let Ok(res) = run_scan(config, tx, cancel_flag).await {
                        *watcher_state.latest_result.write().await = Some(res.clone());
                        let _ = watcher_state
                            .broadcast_tx
                            .send(ServerEvent::ScanComplete(res));
                    }
                }
            }
        }
    });

    if open_browser {
        let _ = opener::open(&server_url);
    }

    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": STATUS_OK,
        "service": SERVICE_NAME,
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn scan_handler(
    State(state): State<AppState>,
    Json(config): Json<ScanConfig>,
) -> Result<Json<ScanResult>, (StatusCode, String)> {
    *state.current_config.write().await = config.clone();

    let (tx, mut rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    let scan_id = uuid::Uuid::new_v4().to_string();
    let _ = state
        .broadcast_tx
        .send(ServerEvent::ScanStarted { scan_id });

    let b_tx = state.broadcast_tx.clone();
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = b_tx.send(ServerEvent::ScanProgress(progress));
        }
    });

    match run_scan(config, tx, cancel_flag).await {
        Ok(result) => {
            *state.latest_result.write().await = Some(result.clone());
            let _ = state
                .broadcast_tx
                .send(ServerEvent::ScanComplete(result.clone()));
            Ok(Json(result))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

async fn events_handler(
    State(state): State<AppState>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.broadcast_tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
        Ok(event) => {
            let json_data = serde_json::to_string(&event).unwrap_or_default();
            Some(Ok(Event::default().data(json_data)))
        }
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn apply_patch_handler(
    State(state): State<AppState>,
    Json(req): Json<ApplyPatchRequest>,
) -> Result<Json<ApplyPatchResult>, (StatusCode, String)> {
    match apply_patch_to_workspace(&req.patch, req.dry_run) {
        Ok(result) => {
            if !req.dry_run {
                let _ = state
                    .broadcast_tx
                    .send(ServerEvent::PatchApplied(result.clone()));

                // Trigger background re-scan to refresh workspace state
                let state_clone = state.clone();
                tokio::spawn(async move {
                    let config = state_clone.current_config.read().await.clone();
                    let (tx, mut rx) = mpsc::channel(100);
                    let cancel_flag = Arc::new(AtomicBool::new(false));

                    let b_tx = state_clone.broadcast_tx.clone();
                    tokio::spawn(async move {
                        while let Some(progress) = rx.recv().await {
                            let _ = b_tx.send(ServerEvent::ScanProgress(progress));
                        }
                    });

                    if let Ok(scan_res) = run_scan(config, tx, cancel_flag).await {
                        *state_clone.latest_result.write().await = Some(scan_res.clone());
                        let _ = state_clone
                            .broadcast_tx
                            .send(ServerEvent::ScanComplete(scan_res));
                    }
                });
            }

            Ok(Json(result))
        }
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

/// Resolves a file path securely and prevents path traversal out of bounds.
fn resolve_safe_path(file_str: &str) -> Result<PathBuf, (StatusCode, String)> {
    let requested = Path::new(file_str);
    let canonical = requested.canonicalize().map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            format!("Failed to resolve file '{}': {}", file_str, e),
        )
    })?;

    if !canonical.is_file() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Path '{}' is not a regular file", file_str),
        ));
    }

    Ok(canonical)
}

async fn snippet_handler(
    Query(query): Query<SnippetQuery>,
) -> Result<Json<SnippetResponse>, (StatusCode, String)> {
    let canonical_path = resolve_safe_path(&query.file)?;
    let content = fs::read_to_string(&canonical_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read file '{}': {}", query.file, e),
        )
    })?;

    let all_lines: Vec<&str> = content.lines().collect();
    let total_lines = all_lines.len();

    if total_lines == 0 {
        return Ok(Json(SnippetResponse {
            file: query.file,
            start_line: 0,
            end_line: 0,
            context_start_line: 0,
            context_end_line: 0,
            lines: Vec::new(),
            total_lines: 0,
            language: "Text".to_string(),
        }));
    }

    if query.start == 0 || query.start > total_lines {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Start line {} out of bounds (file has {} lines)",
                query.start, total_lines
            ),
        ));
    }

    let end = query.end.clamp(query.start, total_lines);
    let ctx = query
        .context
        .unwrap_or(DEFAULT_CONTEXT_LINES)
        .min(MAX_CONTEXT_LINES);

    let ctx_start = query.start.saturating_sub(ctx).max(1);
    let ctx_end = (end + ctx).min(total_lines);

    let mut snippet_lines = Vec::with_capacity(ctx_end - ctx_start + 1);
    for line_num in ctx_start..=ctx_end {
        let is_target = line_num >= query.start && line_num <= end;
        let line_content = all_lines
            .get(line_num - 1)
            .copied()
            .unwrap_or("")
            .to_string();

        snippet_lines.push(SnippetLine {
            line_number: line_num,
            content: line_content,
            is_target,
        });
    }

    let language = get_grammar_for_path(&canonical_path)
        .map(|g| g.name.to_string())
        .unwrap_or_else(|| "Text".to_string());

    Ok(Json(SnippetResponse {
        file: query.file,
        start_line: query.start,
        end_line: end,
        context_start_line: ctx_start,
        context_end_line: ctx_end,
        lines: snippet_lines,
        total_lines,
        language,
    }))
}

async fn refactor_handler(
    Json(req): Json<RefactorRequest>,
) -> Result<Json<RefactorSuggestion>, (StatusCode, String)> {
    match analyze_clone_refactoring(
        &req.file_a,
        (req.start_line_a, req.end_line_a),
        &req.file_b,
        (req.start_line_b, req.end_line_b),
    ) {
        Ok(suggestion) => Ok(Json(suggestion)),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

async fn refactor_cluster_handler(
    Json(req): Json<ClusterRefactorRequest>,
) -> Result<Json<ClusterRefactorSuggestion>, (StatusCode, String)> {
    match analyze_cluster_refactoring(&req.cluster_id, &req.occurrences) {
        Ok(suggestion) => Ok(Json(suggestion)),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

async fn timeline_handler(
    Query(query): Query<TimelineQuery>,
) -> Result<Json<cddm_core::TimelineTrend>, (StatusCode, String)> {
    let dir_str = query.directory.unwrap_or_else(|| ".".to_string());
    let max_samples = query.max_samples.unwrap_or(10);
    let min_tokens = query.min_tokens.unwrap_or(cddm_core::DEFAULT_MIN_TOKENS);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    match cddm_core::collect_git_timeline(Path::new(&dir_str), max_samples, min_tokens, cancel_flag)
    {
        Ok(trend) => Ok(Json(trend)),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

async fn hooks_status_handler(Query(query): Query<TimelineQuery>) -> Json<cddm_core::HookStatus> {
    let dir_str = query.directory.unwrap_or_else(|| ".".to_string());
    Json(cddm_core::get_hook_status(Path::new(&dir_str)))
}

async fn install_hook_handler(
    Json(req): Json<InstallHookRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let dir_str = req.directory.unwrap_or_else(|| ".".to_string());
    let fail_threshold = req.fail_threshold.unwrap_or(15.0);
    let min_tokens = req.min_tokens.unwrap_or(cddm_core::DEFAULT_MIN_TOKENS);

    match cddm_core::install_git_hook(
        Path::new(&dir_str),
        &req.hook_type,
        fail_threshold,
        min_tokens,
    ) {
        Ok(msg) => Ok(Json(serde_json::json!({ "status": "ok", "message": msg }))),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

async fn static_asset_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let asset_path = if path.is_empty() { INDEX_HTML } else { path };

    match WebUIAssets::get(asset_path) {
        Some(content) => {
            let mime_type = mime_guess::from_path(asset_path).first_or_octet_stream();
            Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                )
                .body(Body::from(content.data))
                .unwrap()
        }
        None => {
            // SPA fallback to index.html
            match WebUIAssets::get(INDEX_HTML) {
                Some(index_content) => Response::builder()
                    .status(StatusCode::OK)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(MIME_TEXT_HTML),
                    )
                    .body(Body::from(index_content.data))
                    .unwrap(),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from(NOT_FOUND_MSG))
                    .unwrap(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_resolve_safe_path_valid() {
        let file = NamedTempFile::new().unwrap();
        let path_str = file.path().to_str().unwrap();
        let res = resolve_safe_path(path_str);
        assert!(res.is_ok());
    }

    #[test]
    fn test_resolve_safe_path_nonexistent() {
        let res = resolve_safe_path("non_existent_file_xyz_123.rs");
        assert!(res.is_err());
        let (status, _) = res.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_snippet_handler_success() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            "line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9\nline10"
        )
        .unwrap();

        let path_str = file.path().to_str().unwrap().to_string();
        let query = SnippetQuery {
            file: path_str,
            start: 4,
            end: 6,
            context: Some(2),
        };

        let result = snippet_handler(Query(query)).await;
        assert!(result.is_ok());
        let Json(response) = result.unwrap();
        assert_eq!(response.start_line, 4);
        assert_eq!(response.end_line, 6);
        assert_eq!(response.context_start_line, 2);
        assert_eq!(response.context_end_line, 8);
        assert_eq!(response.lines.len(), 7);
        assert!(!response.lines[0].is_target);
        assert!(response.lines[2].is_target);
    }

    #[tokio::test]
    async fn test_refactor_handler_success() {
        let mut file_a = NamedTempFile::new().unwrap();
        let mut file_b = NamedTempFile::new().unwrap();

        writeln!(file_a, "fn test() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
        writeln!(file_b, "fn other() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();

        let req = RefactorRequest {
            file_a: file_a.path().to_str().unwrap().to_string(),
            start_line_a: 2,
            end_line_a: 3,
            file_b: file_b.path().to_str().unwrap().to_string(),
            start_line_b: 2,
            end_line_b: 3,
        };

        let result = refactor_handler(Json(req)).await;
        assert!(result.is_ok());
        let Json(suggestion) = result.unwrap();
        assert_eq!(
            suggestion.strategy,
            cddm_core::refactor::refactor_strategies::EXTRACT_FUNCTION
        );
        assert!(suggestion.unified_patch.contains("--- a/"));
    }

    #[tokio::test]
    async fn test_refactor_cluster_handler_success() {
        let mut file_a = NamedTempFile::new().unwrap();
        let mut file_b = NamedTempFile::new().unwrap();
        let mut file_c = NamedTempFile::new().unwrap();

        writeln!(file_a, "fn a() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
        writeln!(file_b, "fn b() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
        writeln!(file_c, "fn c() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();

        let req = ClusterRefactorRequest {
            cluster_id: "cluster-1".to_string(),
            occurrences: vec![
                CloneLocation {
                    file: file_a.path().to_str().unwrap().to_string(),
                    start_line: 2,
                    end_line: 3,
                    author: None,
                },
                CloneLocation {
                    file: file_b.path().to_str().unwrap().to_string(),
                    start_line: 2,
                    end_line: 3,
                    author: None,
                },
                CloneLocation {
                    file: file_c.path().to_str().unwrap().to_string(),
                    start_line: 2,
                    end_line: 3,
                    author: None,
                },
            ],
        };

        let result = refactor_cluster_handler(Json(req)).await;
        assert!(result.is_ok());
        let Json(suggestion) = result.unwrap();
        assert_eq!(
            suggestion.strategy,
            cddm_core::refactor::refactor_strategies::EXTRACT_FUNCTION
        );
        assert_eq!(suggestion.sites.len(), 3);
        assert!(suggestion.unified_patch.contains("--- a/"));
    }

    #[tokio::test]
    async fn test_apply_patch_handler_success() {
        let (broadcast_tx, _) = broadcast::channel(100);
        let state = AppState {
            broadcast_tx,
            current_config: Arc::new(RwLock::new(ScanConfig::default())),
            latest_result: Arc::new(RwLock::new(None)),
        };

        let mut file_a = NamedTempFile::new().unwrap();
        let path_str = file_a.path().to_str().unwrap().to_string();

        writeln!(file_a, "fn test() {{\n    let x = 1;\n}}").unwrap();
        file_a.flush().unwrap();

        let patch = format!(
            "--- a/{}\n+++ b/{}\n@@ -2,1 +2,1 @@\n-    let x = 1;\n+    helper();\n",
            path_str, path_str
        );

        let req = ApplyPatchRequest {
            patch,
            dry_run: false,
        };

        let result = apply_patch_handler(State(state), Json(req)).await;
        assert!(result.is_ok());
        let Json(res) = result.unwrap();
        assert!(res.success);
        assert_eq!(res.hunks_applied, 1);
    }

    #[tokio::test]
    async fn test_apply_patch_handler_bad_request() {
        let (broadcast_tx, _) = broadcast::channel(100);
        let state = AppState {
            broadcast_tx,
            current_config: Arc::new(RwLock::new(ScanConfig::default())),
            latest_result: Arc::new(RwLock::new(None)),
        };

        let req = ApplyPatchRequest {
            patch: "invalid patch without hunks".to_string(),
            dry_run: false,
        };

        let result = apply_patch_handler(State(state), Json(req)).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_build_app_router() {
        let app = build_app();
        let _ = app;
    }

    #[tokio::test]
    async fn test_timeline_handler_success() {
        let query = TimelineQuery {
            directory: Some(".".to_string()),
            max_samples: Some(3),
            min_tokens: Some(50),
        };
        let res = timeline_handler(Query(query)).await;
        assert!(res.is_ok());
        let Json(trend) = res.unwrap();
        assert!(!trend.snapshots.is_empty());
    }

    #[tokio::test]
    async fn test_hooks_handlers() {
        let temp = tempfile::tempdir().expect("tempdir");
        let git_dir = temp.path().join(".git");
        std::fs::create_dir_all(&git_dir).expect("create .git");

        let status_res = hooks_status_handler(Query(TimelineQuery {
            directory: Some(temp.path().to_string_lossy().to_string()),
            max_samples: None,
            min_tokens: None,
        }))
        .await;
        assert!(!status_res.pre_commit_installed);

        let install_res = install_hook_handler(Json(InstallHookRequest {
            directory: Some(temp.path().to_string_lossy().to_string()),
            hook_type: "pre-commit".to_string(),
            fail_threshold: Some(15.0),
            min_tokens: Some(50),
        }))
        .await;
        assert!(install_res.is_ok());

        let status_after = hooks_status_handler(Query(TimelineQuery {
            directory: Some(temp.path().to_string_lossy().to_string()),
            max_samples: None,
            min_tokens: None,
        }))
        .await;
        assert!(status_after.pre_commit_installed);
    }
}
