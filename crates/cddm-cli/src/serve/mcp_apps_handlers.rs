#![forbid(unsafe_code)]

use axum::{Json, extract::State, http::StatusCode};
use cddm_mcp::apps::{get_supported_app_widgets, handle_apps_render};
use serde_json::json;

use super::types::AppState;

/// GET /api/mcp/apps/widgets - Returns all generative UI widgets available in the MCP Apps system.
pub async fn mcp_apps_widgets_handler(State(_state): State<AppState>) -> Json<serde_json::Value> {
    let widgets = get_supported_app_widgets();
    Json(json!({
        "status": "ok",
        "widgets": widgets,
    }))
}

/// POST /api/mcp/apps/render - Renders an interactive MCP Apps widget on demand.
pub async fn mcp_apps_render_handler(
    State(_state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let resp = handle_apps_render(None, Some(&payload)).await;
    if let Some(err) = resp.error {
        let msg = err
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("Failed to render widget");
        return Err((StatusCode::BAD_REQUEST, msg.to_string()));
    }

    if let Some(res) = resp.result {
        Ok(Json(res))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "No result returned by widget renderer".to_string(),
        ))
    }
}
