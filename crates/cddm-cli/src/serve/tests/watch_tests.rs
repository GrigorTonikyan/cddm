#![forbid(unsafe_code)]

use crate::serve::*;
use axum::extract::State;
use cddm_core::ScanResult;
use std::path::PathBuf;

#[tokio::test]
async fn test_watch_status_and_toggle_handler() {
    let (state, _) = build_app();

    // Check initial status
    let status_res = watch_status_handler(State(state.clone())).await;
    let axum::Json(status) = status_res;
    assert!(status.is_active);
    assert_eq!(status.sync_count, 0);

    // Toggle watch to inactive
    let toggle_req = WatchToggleRequest {
        active: Some(false),
    };
    let toggle_res = watch_toggle_handler(State(state.clone()), axum::Json(toggle_req)).await;
    let axum::Json(body) = toggle_res;
    assert_eq!(body["is_active"], false);

    // Verify status reflects inactive
    let status_res2 = watch_status_handler(State(state.clone())).await;
    let axum::Json(status2) = status_res2;
    assert!(!status2.is_active);

    // Toggle back to active
    let toggle_res2 = watch_toggle_handler(
        State(state.clone()),
        axum::Json(WatchToggleRequest::default()),
    )
    .await;
    let axum::Json(body2) = toggle_res2;
    assert_eq!(body2["is_active"], true);
}

#[tokio::test]
async fn test_execute_watch_incremental_scan() {
    let (state, _) = build_app();
    let scan_result = ScanResult {
        dry_health_score: 88.0,
        total_clones: 4,
        total_clusters: 2,
        ..Default::default()
    };

    {
        let mut latest = state.latest_result.write().await;
        *latest = Some(scan_result);
    }

    let changed = vec![PathBuf::from("src/test.rs")];
    execute_watch_incremental_scan(&state, &changed).await;

    let log = state.watch_events_log.read().await;
    assert!(!log.is_empty());
    assert_eq!(log[0].changed_files, vec!["src/test.rs".to_string()]);
    assert_eq!(
        state.sync_count.load(std::sync::atomic::Ordering::SeqCst),
        1
    );
}
