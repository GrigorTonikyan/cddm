#![forbid(unsafe_code)]

use super::types::*;
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use cddm_core::{
    DEFAULT_MIN_TOKENS, HookStatus, TimelineTrend, collect_git_timeline, get_hook_status,
    install_git_hook,
};
use std::path::Path;
use std::sync::{Arc, atomic::AtomicBool};

pub async fn timeline_handler(
    Query(query): Query<TimelineQuery>,
) -> Result<Json<TimelineTrend>, (StatusCode, String)> {
    let dir_str = query.directory.unwrap_or_else(|| ".".to_string());
    let max_samples = query.max_samples.unwrap_or(10);
    let min_tokens = query.min_tokens.unwrap_or(DEFAULT_MIN_TOKENS);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    match collect_git_timeline(Path::new(&dir_str), max_samples, min_tokens, cancel_flag) {
        Ok(trend) => Ok(Json(trend)),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

pub async fn hooks_status_handler(Query(query): Query<TimelineQuery>) -> Json<HookStatus> {
    let dir_str = query.directory.unwrap_or_else(|| ".".to_string());
    Json(get_hook_status(Path::new(&dir_str)))
}

pub async fn install_hook_handler(
    Json(req): Json<InstallHookRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let dir_str = req.directory.unwrap_or_else(|| ".".to_string());
    let fail_threshold = req.fail_threshold.unwrap_or(15.0);
    let min_tokens = req.min_tokens.unwrap_or(DEFAULT_MIN_TOKENS);

    match install_git_hook(
        Path::new(&dir_str),
        &req.hook_type,
        fail_threshold,
        min_tokens,
    ) {
        Ok(msg) => Ok(Json(serde_json::json!({ "status": "ok", "message": msg }))),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}
