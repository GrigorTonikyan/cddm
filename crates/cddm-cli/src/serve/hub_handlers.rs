#![forbid(unsafe_code)]

use axum::{Json, http::StatusCode, response::IntoResponse};
use cddm_core::{
    HubConfig, HubExtractRequest, generate_default_hub_config, generate_hub_extraction,
    load_hub_config, run_hub_scan,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct HubScanApiRequest {
    pub config_path: Option<String>,
    pub hub_config: Option<HubConfig>,
}

/// Handler for `GET /api/hub/config`.
pub async fn hub_config_get_handler() -> impl IntoResponse {
    let default_path = cddm_core::DEFAULT_HUB_CONFIG_FILE;
    if Path::new(default_path).exists() {
        match load_hub_config(default_path) {
            Ok(cfg) => (StatusCode::OK, Json(cfg)).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": err })),
            )
                .into_response(),
        }
    } else {
        let sample = generate_default_hub_config(Some("my-organization"));
        let parsed: HubConfig = toml::from_str(&sample).unwrap_or_else(|_| HubConfig {
            name: "my-organization".to_string(),
            repositories: vec![],
            min_tokens: 50,
            fail_threshold: 15.0,
            ignore_patterns: vec![],
        });
        (StatusCode::OK, Json(parsed)).into_response()
    }
}

/// Handler for `POST /api/hub/config`.
pub async fn hub_config_post_handler(Json(cfg): Json<HubConfig>) -> impl IntoResponse {
    match toml::to_string_pretty(&cfg) {
        Ok(toml_str) => {
            if let Err(e) = fs::write(cddm_core::DEFAULT_HUB_CONFIG_FILE, toml_str) {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": format!("Failed to write config: {e}") })),
                )
                    .into_response()
            } else {
                (
                    StatusCode::OK,
                    Json(serde_json::json!({ "status": "saved" })),
                )
                    .into_response()
            }
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("Serialization failed: {e}") })),
        )
            .into_response(),
    }
}

/// Handler for `POST /api/hub/scan`.
pub async fn hub_scan_handler(Json(payload): Json<HubScanApiRequest>) -> impl IntoResponse {
    let config = if let Some(cfg) = payload.hub_config {
        cfg
    } else if let Some(path) = payload.config_path {
        match load_hub_config(&path) {
            Ok(c) => c,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": e })),
                )
                    .into_response();
            }
        }
    } else if Path::new(cddm_core::DEFAULT_HUB_CONFIG_FILE).exists() {
        match load_hub_config(cddm_core::DEFAULT_HUB_CONFIG_FILE) {
            Ok(c) => c,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": e })),
                )
                    .into_response();
            }
        }
    } else {
        HubConfig {
            name: "ad-hoc-hub".to_string(),
            repositories: vec![],
            min_tokens: 50,
            fail_threshold: 15.0,
            ignore_patterns: vec![],
        }
    };

    match run_hub_scan(&config).await {
        Ok(summary) => (StatusCode::OK, Json(summary)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": err })),
        )
            .into_response(),
    }
}

/// Handler for `POST /api/hub/extract`.
pub async fn hub_extract_handler(Json(req): Json<HubExtractRequest>) -> impl IntoResponse {
    let hub_cfg = req.hub_config.clone().unwrap_or_else(|| {
        if Path::new(cddm_core::DEFAULT_HUB_CONFIG_FILE).exists() {
            load_hub_config(cddm_core::DEFAULT_HUB_CONFIG_FILE).unwrap_or_else(|_| HubConfig {
                name: "default-hub".to_string(),
                repositories: vec![],
                min_tokens: 50,
                fail_threshold: 15.0,
                ignore_patterns: vec![],
            })
        } else {
            HubConfig {
                name: "default-hub".to_string(),
                repositories: vec![],
                min_tokens: 50,
                fail_threshold: 15.0,
                ignore_patterns: vec![],
            }
        }
    });

    let summary = match run_hub_scan(&hub_cfg).await {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e })),
            )
                .into_response();
        }
    };

    match generate_hub_extraction(&summary, &req) {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": err })),
        )
            .into_response(),
    }
}
