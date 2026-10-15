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
use serde::Deserialize;
use std::fs;
use std::path::Path;

fn parse_cddmignore_options(content: &str) -> (Option<bool>, Option<bool>, Option<bool>) {
    let mut ignore_tests = None;
    let mut ignore_mocks = None;
    let mut ignore_generated = None;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# @options:") || trimmed.starts_with("# options:") {
            let opts = trimmed
                .trim_start_matches("# @options:")
                .trim_start_matches("# options:");
            for opt in opts.split([',', ' ']) {
                let kv: Vec<&str> = opt.split('=').collect();
                if kv.len() == 2 {
                    match kv[0].trim() {
                        "ignore_tests" => ignore_tests = kv[1].trim().parse().ok(),
                        "ignore_mocks" => ignore_mocks = kv[1].trim().parse().ok(),
                        "ignore_generated" => ignore_generated = kv[1].trim().parse().ok(),
                        _ => {}
                    }
                }
            }
        }
    }
    (ignore_tests, ignore_mocks, ignore_generated)
}

fn inject_cddmignore_options(
    raw: &str,
    ignore_tests: bool,
    ignore_mocks: bool,
    ignore_generated: bool,
) -> String {
    let header = format!(
        "# @options: ignore_tests={ignore_tests}, ignore_mocks={ignore_mocks}, ignore_generated={ignore_generated}"
    );
    let mut filtered_lines: Vec<&str> = raw
        .lines()
        .filter(|l| {
            let trimmed = l.trim();
            !trimmed.starts_with("# @options:") && !trimmed.starts_with("# options:")
        })
        .collect();

    if !filtered_lines.is_empty() && filtered_lines[0].trim().starts_with("# CDDM") {
        let first = filtered_lines.remove(0);
        format!("{first}\n{header}\n{}", filtered_lines.join("\n"))
    } else {
        format!("{header}\n{}", filtered_lines.join("\n"))
    }
}

pub async fn suppression_rules_get_handler(
    State(state): State<AppState>,
) -> Json<SuppressionConfig> {
    let root_path = Path::new(".cddmignore");
    let scan_cfg = state.current_config.read().await;
    let (mut ign_tests, mut ign_mocks, mut ign_gen) = (
        scan_cfg.ignore_tests,
        scan_cfg.ignore_mocks,
        scan_cfg.ignore_generated,
    );

    let engine = if root_path.exists() {
        if let Ok(raw_content) = fs::read_to_string(root_path) {
            let (file_tests, file_mocks, file_gen) = parse_cddmignore_options(&raw_content);
            if let Some(val) = file_tests {
                ign_tests = val;
            }
            if let Some(val) = file_mocks {
                ign_mocks = val;
            }
            if let Some(val) = file_gen {
                ign_gen = val;
            }
        }
        SuppressionEngine::from_file(root_path, ign_tests, ign_mocks, ign_gen)
            .unwrap_or_else(|_| SuppressionEngine::with_options(ign_tests, ign_mocks, ign_gen))
    } else {
        SuppressionEngine::with_options(ign_tests, ign_mocks, ign_gen)
    };
    Json(engine.config().clone())
}

pub async fn suppression_rules_post_handler(
    State(state): State<AppState>,
    Json(config): Json<SuppressionConfig>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    {
        let mut scan_cfg = state.current_config.write().await;
        scan_cfg.ignore_tests = config.ignore_tests;
        scan_cfg.ignore_mocks = config.ignore_mocks;
        scan_cfg.ignore_generated = config.ignore_generated;
    }

    let raw = config
        .raw_cddmignore
        .clone()
        .unwrap_or_else(SuppressionEngine::generate_default_cddmignore);
    let with_options = inject_cddmignore_options(
        &raw,
        config.ignore_tests,
        config.ignore_mocks,
        config.ignore_generated,
    );

    fs::write(".cddmignore", &with_options).map_err(|e| {
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

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(untagged)]
pub enum PolicyEvaluatePayload {
    Detailed {
        directory: Option<String>,
        config: Option<PolicyConfig>,
    },
    Direct(PolicyConfig),
    #[default]
    None,
}

pub async fn policy_evaluate_handler(
    State(state): State<AppState>,
    Json(payload): Json<PolicyEvaluatePayload>,
) -> Result<Json<PolicyEvaluationResult>, (StatusCode, String)> {
    let (directory, explicit_cfg) = match payload {
        PolicyEvaluatePayload::Detailed { directory, config } => (directory, config),
        PolicyEvaluatePayload::Direct(cfg) => (None, Some(cfg)),
        PolicyEvaluatePayload::None => (None, None),
    };

    let engine = if let Some(cfg) = explicit_cfg {
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
    let scan_result = if let Some(latest) = latest_lock.as_ref() {
        latest.clone()
    } else {
        drop(latest_lock);
        let mut scan_config = state.current_config.read().await.clone();
        if let Some(dir) = directory {
            scan_config.directory = dir;
        }
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let cancel_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        if let Ok(res) = cddm_core::run_scan(scan_config, tx, cancel_flag).await {
            *state.latest_result.write().await = Some(res.clone());
            res
        } else {
            cddm_core::ScanResult::default()
        }
    };

    Ok(Json(engine.evaluate(&scan_result)))
}
