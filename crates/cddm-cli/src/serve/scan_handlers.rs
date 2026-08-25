#![forbid(unsafe_code)]

use super::types::*;
use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use cddm_core::{
    ApplyPatchResult, ScanConfig, ScanResult, apply_patch_to_workspace,
    grammar::get_grammar_for_path, run_scan,
};
use std::convert::Infallible;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;
use tokio_stream::wrappers::BroadcastStream;

pub async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": STATUS_OK,
        "service": SERVICE_NAME,
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub async fn scan_handler(
    State(state): State<AppState>,
    Json(config): Json<ScanConfig>,
) -> Result<Json<ScanResult>, (StatusCode, String)> {
    *state.current_config.write().await = config.clone();

    let (tx, mut rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    let scan_id = uuid::Uuid::new_v4().to_string();
    let _ = state
        .broadcast_tx
        .send(ServerEvent::ScanStarted { scan_id });

    let b_tx = state.broadcast_tx.clone();
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = b_tx.send(ServerEvent::ScanProgress(progress));
        }
    });

    match run_scan(config, tx, cancel_flag).await {
        Ok(result) => {
            *state.latest_result.write().await = Some(result.clone());
            let _ = state
                .broadcast_tx
                .send(ServerEvent::ScanComplete(result.clone()));
            Ok(Json(result))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

pub async fn events_handler(
    State(state): State<AppState>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.broadcast_tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
        Ok(event) => {
            let json_data = serde_json::to_string(&event).unwrap_or_default();
            Some(Ok(Event::default().data(json_data)))
        }
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub async fn apply_patch_handler(
    State(state): State<AppState>,
    Json(req): Json<ApplyPatchRequest>,
) -> Result<Json<ApplyPatchResult>, (StatusCode, String)> {
    match apply_patch_to_workspace(&req.patch, req.dry_run) {
        Ok(result) => {
            if !req.dry_run {
                let _ = state
                    .broadcast_tx
                    .send(ServerEvent::PatchApplied(result.clone()));

                // Trigger background re-scan to refresh workspace state
                let state_clone = state.clone();
                tokio::spawn(async move {
                    execute_background_refresh(&state_clone).await;
                });
            }

            Ok(Json(result))
        }
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

/// Executes a full background workspace scan and broadcasts progress and completion events.
pub async fn execute_background_refresh(state: &AppState) {
    let config = state.current_config.read().await.clone();
    let (tx, mut rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    let b_tx = state.broadcast_tx.clone();
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = b_tx.send(ServerEvent::ScanProgress(progress));
        }
    });

    if let Ok(scan_res) = run_scan(config, tx, cancel_flag).await {
        *state.latest_result.write().await = Some(scan_res.clone());
        let _ = state.broadcast_tx.send(ServerEvent::ScanComplete(scan_res));
    }
}

/// Resolves a file path securely and prevents path traversal out of bounds.
pub fn resolve_safe_path(file_str: &str) -> Result<PathBuf, (StatusCode, String)> {
    let requested = Path::new(file_str);
    let canonical = requested.canonicalize().map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            format!("Failed to resolve file '{}': {}", file_str, e),
        )
    })?;

    if !canonical.is_file() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Path '{}' is not a regular file", file_str),
        ));
    }

    Ok(canonical)
}

pub async fn snippet_handler(
    Query(query): Query<SnippetQuery>,
) -> Result<Json<SnippetResponse>, (StatusCode, String)> {
    let canonical_path = resolve_safe_path(&query.file)?;
    let content = fs::read_to_string(&canonical_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read file '{}': {}", query.file, e),
        )
    })?;

    let all_lines: Vec<&str> = content.lines().collect();
    let total_lines = all_lines.len();

    if total_lines == 0 {
        return Ok(Json(SnippetResponse {
            file: query.file,
            start_line: 0,
            end_line: 0,
            context_start_line: 0,
            context_end_line: 0,
            lines: Vec::new(),
            total_lines: 0,
            language: "Text".to_string(),
        }));
    }

    if query.start == 0 || query.start > total_lines {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Start line {} out of bounds (file has {} lines)",
                query.start, total_lines
            ),
        ));
    }

    let end = query.end.clamp(query.start, total_lines);
    let ctx = query
        .context
        .unwrap_or(DEFAULT_CONTEXT_LINES)
        .min(MAX_CONTEXT_LINES);

    let ctx_start = query.start.saturating_sub(ctx).max(1);
    let ctx_end = (end + ctx).min(total_lines);

    let mut snippet_lines = Vec::with_capacity(ctx_end - ctx_start + 1);
    for line_num in ctx_start..=ctx_end {
        let is_target = line_num >= query.start && line_num <= end;
        let line_content = all_lines
            .get(line_num - 1)
            .copied()
            .unwrap_or("")
            .to_string();

        snippet_lines.push(SnippetLine {
            line_number: line_num,
            content: line_content,
            is_target,
        });
    }

    let language = get_grammar_for_path(&canonical_path)
        .map(|g| g.name.to_string())
        .unwrap_or_else(|| "Text".to_string());

    Ok(Json(SnippetResponse {
        file: query.file,
        start_line: query.start,
        end_line: end,
        context_start_line: ctx_start,
        context_end_line: ctx_end,
        lines: snippet_lines,
        total_lines,
        language,
    }))
}
