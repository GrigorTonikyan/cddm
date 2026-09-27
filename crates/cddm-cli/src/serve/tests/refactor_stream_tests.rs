#![forbid(unsafe_code)]

use super::super::*;
use axum::response::IntoResponse;
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_refactor_stream_handler_mock() {
    let mut file_a = NamedTempFile::new().unwrap();
    let mut file_b = NamedTempFile::new().unwrap();

    writeln!(file_a, "fn test() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();
    writeln!(file_b, "fn other() {{\n    let x = 1;\n    let y = 2;\n}}").unwrap();

    let req = RefactorStreamRequest {
        file_a: file_a.path().to_str().unwrap().to_string(),
        start_line_a: 2,
        end_line_a: 3,
        file_b: file_b.path().to_str().unwrap().to_string(),
        start_line_b: 2,
        end_line_b: 3,
        provider: Some("mock".to_string()),
        model: Some("test patch\n".to_string()),
        endpoint: None,
        api_key: None,
        temperature: None,
    };

    let sse_resp = refactor_stream_handler(axum::Json(req)).await;
    let response = sse_resp.into_response();
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/event-stream"
    );
}
