#![forbid(unsafe_code)]

use super::manager::TaskManager;
use crate::protocol::{JSONRPC_VERSION, JsonRpcResponse, make_error_response, rpc_errors};
use serde_json::json;

/// Handles `tasks/list` request.
pub async fn handle_tasks_list(id: Option<serde_json::Value>) -> JsonRpcResponse {
    let tasks = TaskManager::global().list_tasks().await;
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
        result: Some(json!({
            "tasks": tasks
        })),
        error: None,
        headers: None,
    }
}

/// Handles `tasks/call` request.
pub async fn handle_tasks_call(
    id: Option<serde_json::Value>,
    params: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let name = params
        .and_then(|p| {
            p.get("name")
                .or_else(|| p.get("task"))
                .or_else(|| p.get("tool"))
        })
        .and_then(|v| v.as_str());

    let task_name = match name {
        Some(n) if !n.trim().is_empty() => n.trim().to_string(),
        _ => {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "Missing required parameter 'name' or 'task' for tasks/call",
            );
        }
    };

    let arguments = params.and_then(|p| p.get("arguments")).cloned();
    let record = TaskManager::global().spawn_task(task_name, arguments).await;

    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
        result: Some(json!({
            "taskId": record.task_id,
            "name": record.name,
            "status": record.status,
            "createdAt": record.created_at
        })),
        error: None,
        headers: None,
    }
}

/// Handles `tasks/status` request.
pub async fn handle_tasks_status(
    id: Option<serde_json::Value>,
    params: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let task_id = params
        .and_then(|p| p.get("taskId").or_else(|| p.get("task_id")))
        .and_then(|v| v.as_str());

    let tid = match task_id {
        Some(t) if !t.trim().is_empty() => t.trim(),
        _ => {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "Missing required parameter 'taskId' for tasks/status",
            );
        }
    };

    match TaskManager::global().get_task(tid).await {
        Some(record) => JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            result: Some(json!(record)),
            error: None,
            headers: None,
        },
        None => make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            format!("Task '{}' not found", tid),
        ),
    }
}

/// Handles `tasks/cancel` request.
pub async fn handle_tasks_cancel(
    id: Option<serde_json::Value>,
    params: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let task_id = params
        .and_then(|p| p.get("taskId").or_else(|| p.get("task_id")))
        .and_then(|v| v.as_str());

    let tid = match task_id {
        Some(t) if !t.trim().is_empty() => t.trim(),
        _ => {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "Missing required parameter 'taskId' for tasks/cancel",
            );
        }
    };

    match TaskManager::global().cancel_task(tid).await {
        Ok(record) => JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            result: Some(json!({
                "taskId": record.task_id,
                "status": record.status,
                "cancelled": true
            })),
            error: None,
            headers: None,
        },
        Err(err) => make_error_response(id, rpc_errors::INVALID_PARAMS, err),
    }
}
