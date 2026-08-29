#![forbid(unsafe_code)]

use axum::extract::State;

use crate::serve::build_app;
use crate::serve::coverage_handlers::{coverage_correlate_handler, coverage_ingest_handler};
use crate::serve::types::{CoverageCorrelateRequest, CoverageIngestRequest};

#[tokio::test]
async fn test_coverage_ingest_handler() {
    let lcov_content = r#"
SF:src/auth.ts
DA:10,5
DA:11,5
end_of_record
"#;

    let req = CoverageIngestRequest {
        report_content: Some(lcov_content.to_string()),
        report_path: None,
        format: Some("lcov".to_string()),
    };

    let res = coverage_ingest_handler(axum::Json(req)).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_coverage_correlate_handler() {
    let (state, _) = build_app();

    let scan_result = super::create_dummy_scan_result("src/auth.ts", "src/helpers.ts");

    {
        let mut latest = state.latest_result.write().await;
        *latest = Some(scan_result);
    }

    let lcov_content = r#"
SF:src/auth.ts
DA:10,5
DA:11,5
end_of_record
SF:src/helpers.ts
DA:1,100
DA:2,150
end_of_record
"#;

    let req = CoverageCorrelateRequest {
        report_content: Some(lcov_content.to_string()),
        report_path: None,
        format: Some("lcov".to_string()),
        directory: None,
        min_tokens: None,
        dead_code_only: Some(false),
        min_hits: Some(1),
        risk_threshold: None,
    };

    let res = coverage_correlate_handler(State(state), axum::Json(req)).await;
    assert!(res.is_ok());
}
