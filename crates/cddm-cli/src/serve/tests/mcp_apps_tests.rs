#![forbid(unsafe_code)]

use crate::serve::{build_app, mcp_apps_handlers::*};
use axum::{Json, extract::State};
use serde_json::json;

#[tokio::test]
async fn test_mcp_apps_widgets_handler() {
    let (state, _) = build_app();
    let res = mcp_apps_widgets_handler(State(state)).await;
    assert_eq!(res["status"], "ok");
    let widgets = res["widgets"].as_array().expect("widgets array");
    assert_eq!(widgets.len(), 2);
    assert!(widgets.iter().any(|w| w["id"] == "diff-split-view"));
    assert!(widgets.iter().any(|w| w["id"] == "cluster-treemap"));
}

#[tokio::test]
async fn test_mcp_apps_render_handler_diff() {
    let (state, _) = build_app();
    let payload = json!({
        "widget_type": "diff-split-view",
        "title": "Interactive Diff Test",
        "file_a": "a.rs",
        "file_b": "b.rs",
        "original_code": "fn test() { 1 }",
        "refactored_code": "fn test() { 2 }",
    });

    let res = mcp_apps_render_handler(State(state), Json(payload))
        .await
        .expect("render diff result");
    let widget = &res["widget"];
    assert_eq!(widget["type"], "app_widget");
    assert_eq!(widget["widgetType"], "diff-split-view");
    assert!(
        widget["html"]
            .as_str()
            .unwrap()
            .contains("Interactive Diff Test")
    );
}

#[tokio::test]
async fn test_mcp_apps_render_handler_invalid_type() {
    let (state, _) = build_app();
    let payload = json!({
        "widget_type": "invalid_type",
    });

    let res = mcp_apps_render_handler(State(state), Json(payload)).await;
    assert!(res.is_err());
    let (status, msg) = res.unwrap_err();
    assert_eq!(status, axum::http::StatusCode::BAD_REQUEST);
    assert!(msg.contains("Unsupported widget_type"));
}
