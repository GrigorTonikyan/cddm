#![forbid(unsafe_code)]

use super::types::*;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use cddm_core::{ScanResult, WatchDeltaReport, run_scan};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc;

/// Returns the current status of the workspace watch daemon and recent events.
pub async fn watch_status_handler(State(state): State<AppState>) -> Json<WatchStatusResponse> {
    let is_active = state.watch_active.load(Ordering::SeqCst);
    let config = state.current_config.read().await;
    let recent_events = state.watch_events_log.read().await.clone();
    let sync_count = state.sync_count.load(Ordering::SeqCst);
    let last_sync_ts = state.last_sync_timestamp.load(Ordering::SeqCst);
    let last_sync_timestamp = if last_sync_ts > 0 {
        Some(last_sync_ts)
    } else {
        None
    };

    let last_duration_ms = recent_events.last().map(|e| e.duration_ms);

    Json(WatchStatusResponse {
        is_active,
        watch_directory: config.directory.clone(),
        debounce_ms: DEFAULT_WATCH_INTERVAL_MS,
        last_sync_timestamp,
        sync_count,
        last_duration_ms,
        recent_events,
    })
}

/// Toggles or sets the active status of the workspace file watcher.
pub async fn watch_toggle_handler(
    State(state): State<AppState>,
    Json(req): Json<WatchToggleRequest>,
) -> Json<serde_json::Value> {
    let new_state = if let Some(target) = req.active {
        state.watch_active.store(target, Ordering::SeqCst);
        target
    } else {
        let current = state.watch_active.load(Ordering::SeqCst);
        let toggled = !current;
        state.watch_active.store(toggled, Ordering::SeqCst);
        toggled
    };

    let _ = state.broadcast_tx.send(ServerEvent::WatchStatusChanged {
        is_active: new_state,
    });

    Json(serde_json::json!({
        "status": STATUS_OK,
        "is_active": new_state
    }))
}

/// Manually triggers an immediate scan refresh and broadcasts live events.
pub async fn watch_rescan_handler(
    State(state): State<AppState>,
) -> Result<Json<ScanResult>, (StatusCode, String)> {
    let config = state.current_config.read().await.clone();
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

            state.sync_count.fetch_add(1, Ordering::SeqCst);
            let now_millis = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            state
                .last_sync_timestamp
                .store(now_millis, Ordering::SeqCst);

            Ok(Json(result))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

/// Performs an incremental watch scan after file modifications, recording deltas and broadcasting SSE.
pub async fn execute_watch_incremental_scan(state: &AppState, changed_paths: &[PathBuf]) {
    if !state.watch_active.load(Ordering::SeqCst) {
        return;
    }

    let changed_file_strings: Vec<String> = changed_paths
        .iter()
        .map(|p| p.to_string_lossy().to_string().replace('\\', "/"))
        .collect();

    let now_millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let _ = state.broadcast_tx.send(ServerEvent::WatchFileChanged {
        files: changed_file_strings.clone(),
        timestamp: now_millis,
    });

    let config = state.current_config.read().await.clone();
    let previous_result = state.latest_result.read().await.clone();

    let (tx, mut rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let start_time = std::time::Instant::now();

    let b_tx = state.broadcast_tx.clone();
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = b_tx.send(ServerEvent::ScanProgress(progress));
        }
    });

    if let Ok(new_result) = run_scan(config, tx, cancel_flag).await {
        let duration_ms = start_time.elapsed().as_millis();
        let delta = WatchDeltaReport::compute(
            previous_result.as_ref(),
            &new_result,
            changed_paths,
            duration_ms,
        );

        *state.latest_result.write().await = Some(new_result.clone());
        {
            let mut log = state.watch_events_log.write().await;
            log.push(delta.clone());
            if log.len() > 50 {
                log.remove(0);
            }
        }

        state.sync_count.fetch_add(1, Ordering::SeqCst);
        state
            .last_sync_timestamp
            .store(now_millis, Ordering::SeqCst);

        let _ = state.broadcast_tx.send(ServerEvent::WatchScanDelta(delta));
        let _ = state
            .broadcast_tx
            .send(ServerEvent::ScanComplete(new_result));
    }
}
