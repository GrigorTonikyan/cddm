#![forbid(unsafe_code)]

use axum::{extract::Json, http::StatusCode};
use cddm_core::{
    EcosystemAlgorithm, OverlapScanResult, get_canonical_algorithms, scan_workspace_overlap,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Request payload for overlap scan endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverlapScanApiRequest {
    #[serde(default = "default_dir")]
    pub directory: PathBuf,
    #[serde(default = "default_threshold")]
    pub threshold: f64,
}

fn default_dir() -> PathBuf {
    PathBuf::from(".")
}

fn default_threshold() -> f64 {
    0.3
}

/// Handler for `GET /api/overlap/catalog`
pub async fn overlap_catalog_handler() -> Json<Vec<EcosystemAlgorithm>> {
    Json(get_canonical_algorithms())
}

/// Handler for `POST /api/overlap/scan`
pub async fn overlap_scan_handler(
    Json(payload): Json<OverlapScanApiRequest>,
) -> Result<Json<OverlapScanResult>, (StatusCode, String)> {
    scan_workspace_overlap(&payload.directory, payload.threshold)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
