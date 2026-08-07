use axum::{
    body::Body,
    extract::Json,
    http::{header, HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use cddm_core::{run_scan, ScanConfig, ScanResult};
use rust_embed::RustEmbed;
use std::net::SocketAddr;
use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::mpsc;
use tower_http::cors::CorsLayer;

#[derive(RustEmbed)]
#[folder = "../../webui/dist"]
struct WebUIAssets;

/// Starts the Axum web server embedding the React WebUI and API endpoints.
pub async fn start_server(port: u16, open_browser: bool) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/scan", post(scan_handler))
        .fallback(static_asset_handler)
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let server_url = format!("http://localhost:{}", port);

    println!("\n🚀 CDDM Studio WebUI server listening at {}", server_url);

    if open_browser {
        let _ = opener::open(&server_url);
    }

    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "CDDM Studio",
        "version": "0.1.0"
    }))
}

async fn scan_handler(Json(config): Json<ScanConfig>) -> Result<Json<ScanResult>, (StatusCode, String)> {
    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    match run_scan(config, tx, cancel_flag).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

async fn static_asset_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let asset_path = if path.is_empty() { "index.html" } else { path };

    match WebUIAssets::get(asset_path) {
        Some(content) => {
            let mime_type = mime_guess::from_path(asset_path).first_or_octet_stream();
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, HeaderValue::from_str(mime_type.as_ref()).unwrap())
                .body(Body::from(content.data))
                .unwrap()
        }
        None => {
            // SPA fallback to index.html
            match WebUIAssets::get("index.html") {
                Some(index_content) => Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, HeaderValue::from_static("text/html"))
                    .body(Body::from(index_content.data))
                    .unwrap(),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("404 Not Found"))
                    .unwrap(),
            }
        }
    }
}
