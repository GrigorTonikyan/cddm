#![forbid(unsafe_code)]

use std::sync::OnceLock;
use std::sync::atomic::Ordering;

use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

use crate::serve::types::AppState;

/// Metric name for active duplicate clone pairs in the workspace.
pub const METRIC_CLONES_ACTIVE: &str = "cddm_clones_active";

/// Metric name for the current DRY (Don't Repeat Yourself) health score percentage.
pub const METRIC_DRY_SCORE: &str = "cddm_dry_health_score";

/// Metric name for duplication percentage across analyzed tokens.
pub const METRIC_DUPLICATION_PERCENTAGE: &str = "cddm_duplication_percentage";

/// Metric name for scan execution durations in seconds.
pub const METRIC_SCAN_DURATION_SECONDS: &str = "cddm_scan_duration_seconds";

/// Metric name for currently active Server-Sent Events (SSE) client connections.
pub const METRIC_SSE_CLIENTS: &str = "cddm_sse_clients_active";

/// Metric name for total cumulative scans executed by the engine.
pub const METRIC_SCANS_TOTAL: &str = "cddm_scans_total";

/// Metric MIME Content-Type header for Prometheus text format.
pub const PROMETHEUS_CONTENT_TYPE: &str = "text/plain; version=0.0.4; charset=utf-8";

static PROMETHEUS_HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

/// Returns the singleton PrometheusHandle, initializing the global Prometheus metrics recorder if needed.
pub fn get_or_init_prometheus() -> PrometheusHandle {
    PROMETHEUS_HANDLE
        .get_or_init(|| {
            let handle = PrometheusBuilder::new()
                .install_recorder()
                .expect("failed to install global Prometheus recorder");

            register_metric_descriptions();
            handle
        })
        .clone()
}

/// Registers standard HELP metadata for all exported CDDM metrics.
fn register_metric_descriptions() {
    metrics::describe_gauge!(
        METRIC_CLONES_ACTIVE,
        "Total number of duplicate clone instances detected in the workspace"
    );
    metrics::describe_gauge!(
        METRIC_DRY_SCORE,
        "Current DRY (Don't Repeat Yourself) health score percentage (0 to 100)"
    );
    metrics::describe_gauge!(
        METRIC_DUPLICATION_PERCENTAGE,
        "Percentage of analyzed source tokens that are duplicated (0 to 100)"
    );
    metrics::describe_histogram!(
        METRIC_SCAN_DURATION_SECONDS,
        "Duration of CDDM workspace deduplication scans in seconds"
    );
    metrics::describe_gauge!(
        METRIC_SSE_CLIENTS,
        "Number of currently connected Server-Sent Events client connections"
    );
    metrics::describe_counter!(
        METRIC_SCANS_TOTAL,
        "Cumulative count of completed workspace deduplication scans"
    );
}

/// Synchronizes gauges from current AppState into the global metrics recorder.
pub async fn update_live_metrics(state: &AppState) {
    let _ = get_or_init_prometheus();

    // 1. Active SSE client connections
    let sse_clients = state.broadcast_tx.receiver_count() as f64;
    metrics::gauge!(METRIC_SSE_CLIENTS).set(sse_clients);

    // 2. Scan count
    let total_scans = state.sync_count.load(Ordering::Relaxed) as u64;
    metrics::counter!(METRIC_SCANS_TOTAL).absolute(total_scans);

    // 3. Clone count, DRY score, duplication percentage, and duration from latest scan result
    let result_opt = state.latest_result.read().await;
    if let Some(ref result) = *result_opt {
        metrics::gauge!(METRIC_CLONES_ACTIVE).set(result.total_clones as f64);
        metrics::gauge!(METRIC_DRY_SCORE).set(result.dry_health_score);
        metrics::gauge!(METRIC_DUPLICATION_PERCENTAGE).set(result.duplication_percentage);
        let duration_secs = result.duration_ms as f64 / 1000.0;
        metrics::histogram!(METRIC_SCAN_DURATION_SECONDS).record(duration_secs);
    }
}

/// Record a completed scan event into the metrics subsystem.
pub fn record_scan_metrics(clones_count: usize, dry_score: f64, duration_ms: u64) {
    let _ = get_or_init_prometheus();
    metrics::counter!(METRIC_SCANS_TOTAL).increment(1);
    metrics::gauge!(METRIC_CLONES_ACTIVE).set(clones_count as f64);
    metrics::gauge!(METRIC_DRY_SCORE).set(dry_score);
    let duration_secs = duration_ms as f64 / 1000.0;
    metrics::histogram!(METRIC_SCAN_DURATION_SECONDS).record(duration_secs);
}

/// Axum GET /metrics endpoint returning OpenMetrics / Prometheus plaintext representation.
pub async fn metrics_handler(State(state): State<AppState>) -> Response {
    let handle = get_or_init_prometheus();
    update_live_metrics(&state).await;

    let body = handle.render();

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, PROMETHEUS_CONTENT_TYPE)],
        body,
    )
        .into_response()
}
