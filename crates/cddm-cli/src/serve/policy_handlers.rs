#![forbid(unsafe_code)]

use super::types::*;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use cddm_core::{
    DEFAULT_RULES_FILE, PolicyConfig, PolicyEngine, PolicyEvaluationResult, SuppressionConfig,
    SuppressionEngine,
};
use std::fs;
use std::path::Path;

pub async fn suppression_rules_get_handler() -> Json<SuppressionConfig> {
    let root_path = Path::new(".cddmignore");
    let engine = if root_path.exists() {
        SuppressionEngine::from_file(root_path, false, false, true)
            .unwrap_or_else(|_| SuppressionEngine::default_engine())
    } else {
        SuppressionEngine::default_engine()
    };
    Json(engine.config().clone())
}

pub async fn suppression_rules_post_handler(
    Json(config): Json<SuppressionConfig>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let raw = config
        .raw_cddmignore
        .clone()
        .unwrap_or_else(SuppressionEngine::generate_default_cddmignore);
    fs::write(".cddmignore", &raw).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to save .cddmignore: {e}"),
        )
    })?;
    Ok(Json(serde_json::json!({
        "status": "ok",
        "message": "Successfully saved .cddmignore suppression rules"
    })))
}

pub async fn policy_rules_get_handler() -> Json<PolicyConfig> {
    let root_path = Path::new(DEFAULT_RULES_FILE);
    let engine = if root_path.exists() {
        PolicyEngine::from_file(root_path).unwrap_or_else(|_| PolicyEngine::empty())
    } else {
        PolicyEngine::empty()
    };
    Json(engine.config().clone())
}

pub async fn policy_rules_post_handler(
    Json(config): Json<PolicyConfig>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let engine = PolicyEngine::new(config).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let raw_toml = engine
        .to_toml_string()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    fs::write(DEFAULT_RULES_FILE, &raw_toml).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to save .cddmrules.toml: {e}"),
        )
    })?;
    Ok(Json(serde_json::json!({
        "status": "ok",
        "message": "Successfully saved .cddmrules.toml architectural policy rules"
    })))
}

pub async fn policy_evaluate_handler(
    State(state): State<AppState>,
    Json(config): Json<Option<PolicyConfig>>,
) -> Result<Json<PolicyEvaluationResult>, (StatusCode, String)> {
    let engine = if let Some(cfg) = config {
        PolicyEngine::new(cfg).map_err(|e| (StatusCode::BAD_REQUEST, e))?
    } else {
        let root_path = Path::new(DEFAULT_RULES_FILE);
        if root_path.exists() {
            PolicyEngine::from_file(root_path).map_err(|e| (StatusCode::BAD_REQUEST, e))?
        } else {
            PolicyEngine::empty()
        }
    };

    let latest_lock = state.latest_result.read().await;
    if let Some(ref scan_result) = *latest_lock {
        Ok(Json(engine.evaluate(scan_result)))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            "No active scan results available to evaluate".to_string(),
        ))
    }
}
