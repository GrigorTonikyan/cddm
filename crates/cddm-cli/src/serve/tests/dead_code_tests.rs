#![forbid(unsafe_code)]

use std::fs;
use tempfile::tempdir;

use super::super::dead_code_handlers::{
    dead_code_get_handler, dead_code_prune_handler, dead_code_reachability_handler,
    dead_code_scan_handler,
};
use super::super::types::{DeadCodePruneRequest, DeadCodeScanRequest};
use axum::extract::State;

use axum::response::IntoResponse;

#[tokio::test]
async fn test_dead_code_scan_and_get_handlers() {
    let (state, _) = super::super::build_app();

    let get_res = dead_code_get_handler(State(state.clone())).await;
    assert!(get_res.is_ok());

    let reach_res = dead_code_reachability_handler(State(state.clone())).await;
    assert!(reach_res.is_ok());

    let scan_req = DeadCodeScanRequest {
        directory: Some(".".to_string()),
        min_tokens: Some(50),
        static_only: Some(true),
        ..Default::default()
    };

    let post_res = dead_code_scan_handler(State(state), axum::Json(scan_req)).await;
    assert!(post_res.is_ok());
}

#[tokio::test]
async fn test_dead_code_prune_handler_dry_run() {
    let temp = tempdir().unwrap();
    let file_path = temp.path().join("dead.rs");
    let code = r#"fn main() {
    active_fn();
}

fn active_fn() {
    println!("active");
}

fn dead_unused_fn() {
    println!("dead");
}
"#;
    fs::write(&file_path, code).unwrap();

    let (state, _) = super::super::build_app();
    let prune_req = DeadCodePruneRequest {
        directory: Some(temp.path().to_string_lossy().to_string()),
        min_tokens: Some(2),
        dry_run: Some(true),
        safe_only: Some(true),
        threshold: Some(0.7),
        ..Default::default()
    };

    let res = dead_code_prune_handler(State(state), axum::Json(prune_req)).await;
    assert!(res.is_ok());
    let response = res.unwrap().into_response();
    assert_eq!(response.status(), axum::http::StatusCode::OK);
}
