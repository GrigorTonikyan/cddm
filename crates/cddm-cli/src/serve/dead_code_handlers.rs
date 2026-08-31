#![forbid(unsafe_code)]

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use cddm_core::dead_code::{DeadCodeConfig, run_dead_code_detection};
use serde_json::json;

use super::types::{AppState, DeadCodeScanRequest};

/// Handler for POST /api/dead-code/scan: performs on-demand polyglot dead code analysis.
pub async fn dead_code_scan_handler(
    State(_state): State<AppState>,
    Json(payload): Json<DeadCodeScanRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        directory = ?payload.directory,
        min_tokens = ?payload.min_tokens,
        "Received REST request for dead code scan"
    );

    let config = DeadCodeConfig {
        directory: payload.directory.unwrap_or_else(|| ".".to_string()),
        min_tokens: payload.min_tokens.unwrap_or(30),
        static_only: payload.static_only.unwrap_or(false),
        report_path: payload.report_path,
        report_content: payload.report_content,
        languages: payload.languages,
        ignore: payload.ignore,
    };

    let summary = run_dead_code_detection(config).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Dead code detection failed: {err}") })),
        )
    })?;

    Ok((StatusCode::OK, Json(summary)))
}

/// Handler for GET /api/dead-code: runs dead code scan on current workspace.
pub async fn dead_code_get_handler(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let config = DeadCodeConfig {
        directory: ".".to_string(),
        min_tokens: 30,
        static_only: false,
        ..Default::default()
    };

    let summary = run_dead_code_detection(config).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Dead code analysis failed: {err}") })),
        )
    })?;

    Ok((StatusCode::OK, Json(summary)))
}
