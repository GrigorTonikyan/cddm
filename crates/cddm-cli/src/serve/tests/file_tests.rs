#![forbid(unsafe_code)]

use crate::serve::*;
use axum::extract::{Json, Query, State};
use axum::http::StatusCode;
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_file_read_handler_success() {
    let mut file = tempfile::Builder::new()
        .suffix(".rs")
        .tempfile_in(".")
        .unwrap();
    writeln!(file, "fn sample_function() -> i32 {{\n    42\n}}").unwrap();

    let path_str = file.path().to_str().unwrap().to_string();
    let query = FileReadQuery { path: path_str };

    let res = file_read_handler(Query(query)).await;
    assert!(res.is_ok());
    let Json(data) = res.unwrap();
    assert!(data.content.contains("fn sample_function"));
    assert_eq!(data.total_lines, 3);
    assert_eq!(data.language.to_lowercase(), "rust");
    assert!(data.size_bytes > 0);
}

#[tokio::test]
async fn test_file_read_handler_traversal_forbidden() {
    let query = FileReadQuery {
        path: "../../Cargo.lock".to_string(),
    };
    let res = file_read_handler(Query(query)).await;
    assert!(res.is_err());
    let (status, _) = res.unwrap_err();
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_file_read_handler_not_found() {
    let query = FileReadQuery {
        path: "non_existent_file_definitely_not_here.rs".to_string(),
    };
    let res = file_read_handler(Query(query)).await;
    assert!(res.is_err());
    let (status, _) = res.unwrap_err();
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_file_save_handler_success() {
    let (state, _) = build_app();
    let file = NamedTempFile::new_in(".").unwrap();
    let path_str = file.path().to_str().unwrap().to_string();

    let req = FileWriteRequest {
        path: path_str.clone(),
        content: "const GREETING: &str = \"Hello CDDM\";\n".to_string(),
    };

    let res = file_save_handler(State(state), Json(req)).await;
    assert!(res.is_ok());
    let Json(resp) = res.unwrap();
    assert!(resp.success);
    assert_eq!(resp.bytes_written, 37);

    // Read back content to verify persistence
    let read_query = FileReadQuery { path: path_str };
    let read_res = file_read_handler(Query(read_query)).await.unwrap();
    assert_eq!(
        read_res.0.content,
        "const GREETING: &str = \"Hello CDDM\";\n"
    );
}

#[tokio::test]
async fn test_file_save_handler_traversal_forbidden() {
    let (state, _) = build_app();
    let req = FileWriteRequest {
        path: "../../some_malicious_file.txt".to_string(),
        content: "malicious content".to_string(),
    };

    let res = file_save_handler(State(state), Json(req)).await;
    assert!(res.is_err());
    let (status, _) = res.unwrap_err();
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_file_tree_handler_success() {
    let res = file_tree_handler().await;
    assert!(res.is_ok());
    let Json(resp) = res.unwrap();
    assert!(!resp.root.is_empty());
    assert!(!resp.files.is_empty());
    let has_cargo = resp.files.iter().any(|f| f.path.contains("Cargo.toml"));
    assert!(has_cargo);
}
