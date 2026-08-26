#![forbid(unsafe_code)]

use axum::{extract::Json, http::StatusCode};
use cddm_core::{
    ExtractRequest, ExtractResult, apply_shared_extraction, generate_shared_extraction,
};
use std::path::Path;

/// Handler for `POST /api/extract/preview`
pub async fn extract_preview_handler(
    Json(payload): Json<ExtractRequest>,
) -> Result<Json<ExtractResult>, (StatusCode, String)> {
    generate_shared_extraction(Path::new("."), &payload)
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

/// Handler for `POST /api/extract/apply`
pub async fn extract_apply_handler(
    Json(payload): Json<ExtractRequest>,
) -> Result<Json<ExtractResult>, (StatusCode, String)> {
    apply_shared_extraction(Path::new("."), &payload)
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}
