use axum::{
    Router,
    body::Body,
    extract::{Json, Query},
    http::{HeaderValue, StatusCode, Uri, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use cddm_core::{
    CloneLocation, ClusterRefactorSuggestion, RefactorSuggestion, ScanConfig, ScanResult,
    analyze_clone_refactoring, analyze_cluster_refactoring, grammar::get_grammar_for_path,
    run_scan,
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;
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

#[derive(RustEmbed)]
#[folder = "../../webui/dist"]
struct WebUIAssets;

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

/// Builds the Axum application router with all routes and middleware configured.
pub fn build_app() -> Router {
    Router::new()
        .route(ROUTE_API_HEALTH, get(health_handler))
        .route(ROUTE_API_SCAN, post(scan_handler))
        .route(ROUTE_API_SNIPPET, get(snippet_handler))
        .route(ROUTE_API_REFACTOR, post(refactor_handler))
        .route(ROUTE_API_REFACTOR_CLUSTER, post(refactor_cluster_handler))
        .fallback(static_asset_handler)
        .layer(CorsLayer::permissive())
}

/// Starts the Axum web server embedding the React WebUI and API endpoints.
pub async fn start_server(port: u16, open_browser: bool) -> Result<(), Box<dyn std::error::Error>> {
    let app = build_app();
    let addr = SocketAddr::from((DEFAULT_HOST_IP, port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let server_url = format!("http://localhost:{}", port);

    println!("\nCDDM Studio WebUI server listening at {}", server_url);

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
    Json(config): Json<ScanConfig>,
) -> Result<Json<ScanResult>, (StatusCode, String)> {
    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    match run_scan(config, tx, cancel_flag).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
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
}
