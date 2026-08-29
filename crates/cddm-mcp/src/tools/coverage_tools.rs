#![forbid(unsafe_code)]

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use cddm_core::{
    CoverageFormat, ScanConfig, correlate_coverage, load_coverage_report, parse_coverage_data,
    run_scan,
};
use serde_json::{Value, json};
use tokio::sync::mpsc;

use crate::protocol::{JsonRpcResponse, make_error_response, make_text_response, rpc_errors};

/// Handle tool `cddm_correlate_coverage`: correlates test/runtime execution coverage with duplicate clones.
pub async fn handle_correlate_coverage(id: Option<Value>, args: Option<&Value>) -> JsonRpcResponse {
    let report_content = args
        .and_then(|a| a.get("report_content"))
        .and_then(|c| c.as_str());
    let report_path = args
        .and_then(|a| a.get("report_path"))
        .and_then(|p| p.as_str());
    let format_str = args.and_then(|a| a.get("format")).and_then(|f| f.as_str());
    let directory = args
        .and_then(|a| a.get("directory"))
        .and_then(|d| d.as_str())
        .unwrap_or(".");
    let min_tokens = args
        .and_then(|a| a.get("min_tokens"))
        .and_then(|m| m.as_u64())
        .unwrap_or(50) as usize;
    let min_hits = args
        .and_then(|a| a.get("min_hits"))
        .and_then(|m| m.as_u64());
    let dead_code_only = args
        .and_then(|a| a.get("dead_code_only"))
        .and_then(|d| d.as_bool())
        .unwrap_or(false);

    let format = match format_str {
        Some("lcov") => CoverageFormat::Lcov,
        Some("cobertura") => CoverageFormat::Cobertura,
        Some("istanbul") => CoverageFormat::Istanbul,
        _ => CoverageFormat::Auto,
    };

    let coverage_report = if let Some(content) = report_content {
        match parse_coverage_data(content, format) {
            Ok(rep) => rep,
            Err(err) => return make_error_response(id, rpc_errors::INVALID_PARAMS, err),
        }
    } else if let Some(path_str) = report_path {
        let p = Path::new(path_str);
        match load_coverage_report(p, format) {
            Ok(rep) => rep,
            Err(err) => return make_error_response(id, rpc_errors::INVALID_PARAMS, err),
        }
    } else if Path::new("lcov.info").exists() {
        match load_coverage_report(Path::new("lcov.info"), CoverageFormat::Auto) {
            Ok(rep) => rep,
            Err(err) => return make_error_response(id, rpc_errors::INTERNAL_ERROR, err),
        }
    } else {
        return make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Either report_content or report_path must be provided (or lcov.info in workspace \
             root)",
        );
    };

    let scan_config = ScanConfig {
        directory: directory.to_string(),
        min_tokens,
        ..Default::default()
    };
    let (tx, _rx) = mpsc::channel(100);
    let cancel = Arc::new(AtomicBool::new(false));

    let scan_result = match run_scan(scan_config, tx, cancel).await {
        Ok(res) => res,
        Err(err) => return make_error_response(id, rpc_errors::INTERNAL_ERROR, err),
    };

    let mut summary = correlate_coverage(&scan_result, &coverage_report);

    if dead_code_only {
        summary.metrics.retain(|m| m.is_dead_code);
    }
    if let Some(hits) = min_hits {
        summary.metrics.retain(|m| m.total_combined_hits >= hits);
    }

    make_text_response(
        id,
        serde_json::to_string_pretty(&summary).unwrap_or_default(),
    )
}

/// Handle tool `cddm_detect_dead_clones`: finds duplicate code fragments with zero runtime executions.
pub async fn handle_detect_dead_clones(id: Option<Value>, args: Option<&Value>) -> JsonRpcResponse {
    let mut modified_args = args.cloned().unwrap_or_else(|| json!({}));
    if let Some(obj) = modified_args.as_object_mut() {
        obj.insert("dead_code_only".to_string(), json!(true));
    }
    handle_correlate_coverage(id, Some(&modified_args)).await
}
