#![forbid(unsafe_code)]

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use cddm_core::{
    CoverageFormat, CoverageReport, ScanConfig, correlate_coverage, load_coverage_report,
    parse_coverage_data, run_scan,
};
use serde_json::json;
use tokio::sync::mpsc;

use super::types::{AppState, CoverageCorrelateRequest, CoverageIngestRequest};

/// Parse coverage format string into typed enum.
fn parse_format_str(fmt: Option<&str>) -> CoverageFormat {
    match fmt.map(|s| s.to_lowercase()).as_deref() {
        Some("lcov") => CoverageFormat::Lcov,
        Some("cobertura") => CoverageFormat::Cobertura,
        Some("istanbul") => CoverageFormat::Istanbul,
        _ => CoverageFormat::Auto,
    }
}

fn load_or_parse_coverage(
    content: Option<String>,
    path_str: Option<String>,
    format_str: Option<&str>,
    fallback_default_lcov: bool,
) -> Result<CoverageReport, (StatusCode, Json<serde_json::Value>)> {
    let format = parse_format_str(format_str);
    if let Some(c) = content {
        parse_coverage_data(&c, format).map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Failed to parse coverage content: {err}") })),
            )
        })
    } else if let Some(p) = path_str {
        load_coverage_report(Path::new(&p), format).map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Failed to read coverage report file: {err}") })),
            )
        })
    } else if fallback_default_lcov {
        if Path::new("lcov.info").exists() {
            load_coverage_report(Path::new("lcov.info"), CoverageFormat::Auto).map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Failed to load default lcov.info: {err}") })),
                )
            })
        } else {
            Ok(CoverageReport::default())
        }
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Either report_content or report_path must be provided" })),
        ))
    }
}

/// Handler for POST /api/coverage/ingest: parses coverage tracefile and returns stats.
pub async fn coverage_ingest_handler(
    Json(payload): Json<CoverageIngestRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let report = load_or_parse_coverage(
        payload.report_content,
        payload.report_path,
        payload.format.as_deref(),
        false,
    )?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "files_count": report.file_line_hits.len(),
            "total_hits": report.total_hits,
            "total_instrumented_lines": report.total_instrumented_lines,
        })),
    ))
}

/// Handler for POST /api/coverage/correlate: correlates coverage report with duplicate clones.
pub async fn coverage_correlate_handler(
    State(state): State<AppState>,
    Json(payload): Json<CoverageCorrelateRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let coverage_report = load_or_parse_coverage(
        payload.report_content,
        payload.report_path,
        payload.format.as_deref(),
        true,
    )?;

    // Get latest scan result or execute fresh scan
    let scan_result = {
        let guard = state.latest_result.read().await;
        guard.clone()
    };

    let scan_result = match scan_result {
        Some(res) => res,
        None => {
            let dir = payload.directory.unwrap_or_else(|| ".".to_string());
            let min_tok = payload.min_tokens.unwrap_or(cddm_core::DEFAULT_MIN_TOKENS);
            let scan_config = ScanConfig {
                directory: dir,
                min_tokens: min_tok,
                ..Default::default()
            };
            let (tx, _rx) = mpsc::channel(100);
            let cancel = Arc::new(AtomicBool::new(false));

            run_scan(scan_config, tx, cancel).await.map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Scan failed: {err}") })),
                )
            })?
        }
    };

    let mut summary = correlate_coverage(&scan_result, &coverage_report);

    if payload.dead_code_only.unwrap_or(false) {
        summary.metrics.retain(|m| m.is_dead_code);
    }
    if let Some(min_hits) = payload.min_hits {
        summary
            .metrics
            .retain(|m| m.total_combined_hits >= min_hits);
    }
    if let Some(threshold) = payload.risk_threshold {
        summary.metrics.retain(|m| m.risk_score >= threshold);
    }

    Ok((StatusCode::OK, Json(summary)))
}
