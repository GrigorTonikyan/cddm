#![forbid(unsafe_code)]

use axum::body::to_bytes;
use axum::extract::State;
use axum::http::StatusCode;

use crate::serve::build_app;
use crate::serve::metrics_handlers::{
    METRIC_CLONES_ACTIVE, METRIC_DRY_SCORE, METRIC_DUPLICATION_PERCENTAGE,
    METRIC_SCAN_DURATION_SECONDS, METRIC_SCANS_TOTAL, METRIC_SSE_CLIENTS, PROMETHEUS_CONTENT_TYPE,
    get_or_init_prometheus, metrics_handler, record_scan_metrics, update_live_metrics,
};

#[tokio::test]
async fn test_metrics_handler_endpoint_empty_state() {
    let (state, _) = build_app();

    let response = metrics_handler(State(state)).await;
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok());
    assert_eq!(content_type, Some(PROMETHEUS_CONTENT_TYPE));

    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let text = String::from_utf8_lossy(&bytes);

    assert!(text.contains(METRIC_SSE_CLIENTS));
    assert!(text.contains("HELP"));
}

#[tokio::test]
async fn test_metrics_with_populated_scan_result() {
    let (state, _) = build_app();

    let mut scan_result = super::create_dummy_scan_result("src/a.rs", "src/b.rs");
    scan_result.total_clones = 7;
    scan_result.dry_health_score = 92.5;
    scan_result.duplication_percentage = 4.2;
    scan_result.duration_ms = 850;

    {
        let mut latest = state.latest_result.write().await;
        *latest = Some(scan_result);
    }

    update_live_metrics(&state).await;

    let response = metrics_handler(State(state)).await;
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let text = String::from_utf8_lossy(&bytes);

    assert!(text.contains(METRIC_CLONES_ACTIVE));
    assert!(text.contains(METRIC_DRY_SCORE));
    assert!(text.contains(METRIC_DUPLICATION_PERCENTAGE));
    assert!(text.contains(METRIC_SCAN_DURATION_SECONDS));
}

#[tokio::test]
async fn test_record_scan_metrics_helper() {
    record_scan_metrics(12, 88.0, 1500);

    let handle = get_or_init_prometheus();
    let text = handle.render();

    assert!(text.contains(METRIC_CLONES_ACTIVE));
    assert!(text.contains(METRIC_DRY_SCORE));
    assert!(text.contains(METRIC_SCANS_TOTAL));
}
