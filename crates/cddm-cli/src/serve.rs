use axum::{
    Router,
    body::Body,
    extract::Json,
    http::{HeaderValue, StatusCode, Uri, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use cddm_core::{ScanConfig, ScanResult, run_scan};
use rust_embed::RustEmbed;
use std::net::SocketAddr;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;
use tower_http::cors::CorsLayer;

/// API endpoint path for health checks.
pub const ROUTE_API_HEALTH: &str = "/api/health";

/// API endpoint path for asynchronous code duplication scans.
pub const ROUTE_API_SCAN: &str = "/api/scan";

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

#[derive(RustEmbed)]
#[folder = "../../webui/dist"]
struct WebUIAssets;

/// Starts the Axum web server embedding the React WebUI and API endpoints.
pub async fn start_server(port: u16, open_browser: bool) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route(ROUTE_API_HEALTH, get(health_handler))
        .route(ROUTE_API_SCAN, post(scan_handler))
        .fallback(static_asset_handler)
        .layer(CorsLayer::permissive());

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
