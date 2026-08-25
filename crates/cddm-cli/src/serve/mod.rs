#![forbid(unsafe_code)]

pub mod assets;
pub mod policy_handlers;
pub mod refactor_handlers;
pub mod scan_handlers;
pub mod semantic_handlers;
pub mod timeline_handlers;
pub mod types;

pub use types::*;

use assets::static_asset_handler;
use axum::{
    Router,
    routing::{get, post},
};
use cddm_core::{CddmWatcher, ScanConfig};
use policy_handlers::*;
use refactor_handlers::*;
use scan_handlers::*;
use semantic_handlers::*;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use timeline_handlers::*;
use tokio::sync::{RwLock, broadcast};
use tower_http::cors::CorsLayer;

/// Builds the Axum application router and default shared state.
pub fn build_app() -> (AppState, Router) {
    let (broadcast_tx, _) = broadcast::channel(200);
    let state = AppState {
        broadcast_tx,
        current_config: Arc::new(RwLock::new(ScanConfig::default())),
        latest_result: Arc::new(RwLock::new(None)),
    };
    let router = build_app_with_state(state.clone());
    (state, router)
}

/// Builds the Axum application router with explicitly provided shared state.
pub fn build_app_with_state(state: AppState) -> Router {
    Router::new()
        .route(ROUTE_API_HEALTH, get(health_handler))
        .route(ROUTE_API_SCAN, post(scan_handler))
        .route(ROUTE_API_SNIPPET, get(snippet_handler))
        .route(ROUTE_API_REFACTOR, post(refactor_handler))
        .route(ROUTE_API_REFACTOR_CLUSTER, post(refactor_cluster_handler))
        .route(ROUTE_API_APPLY_PATCH, post(apply_patch_handler))
        .route(ROUTE_API_EVENTS, get(events_handler))
        .route(ROUTE_API_TIMELINE, get(timeline_handler))
        .route(ROUTE_API_HOOKS, get(hooks_status_handler))
        .route(ROUTE_API_HOOKS_INSTALL, post(install_hook_handler))
        .route(
            ROUTE_API_SUPPRESSION_RULES,
            get(suppression_rules_get_handler).post(suppression_rules_post_handler),
        )
        .route(
            ROUTE_API_POLICY_RULES,
            get(policy_rules_get_handler).post(policy_rules_post_handler),
        )
        .route(ROUTE_API_POLICY_EVALUATE, post(policy_evaluate_handler))
        .route(ROUTE_API_REFACTOR_SANDBOX, post(refactor_sandbox_handler))
        .route(
            ROUTE_API_REFACTOR_APPLY_BRANCH,
            post(refactor_apply_branch_handler),
        )
        .route(
            ROUTE_API_REFACTOR_AI_PROMPT,
            post(refactor_ai_prompt_handler),
        )
        .route(ROUTE_API_REFACTOR_AST, post(refactor_ast_handler))
        .route(ROUTE_API_REFACTOR_VERIFY, post(refactor_verify_handler))
        .route(ROUTE_API_REFACTOR_HEAL, post(refactor_heal_handler))
        .route(ROUTE_API_CACHE_EXPORT, post(cache_export_handler))
        .route(ROUTE_API_CACHE_IMPORT, post(cache_import_handler))
        .route(ROUTE_API_MONOREPO, post(monorepo_handler))
        .route(ROUTE_API_SEMANTIC_GRAPH, post(semantic_graph_handler))
        .fallback(static_asset_handler)
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Starts the Axum web server embedding the React WebUI and API endpoints with background watching.
pub async fn start_server(port: u16, open_browser: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (state, app) = build_app();

    let addr = SocketAddr::from((DEFAULT_HOST_IP, port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let server_url = format!("http://localhost:{}", port);

    println!("\nCDDM Studio WebUI server listening at {}", server_url);

    // Initialize background directory watcher for real-time live push updates
    let watcher_state = state.clone();
    tokio::spawn(async move {
        let watch_dir = PathBuf::from(".");
        if let Ok(watcher) = CddmWatcher::watch_directory(&watch_dir) {
            let mut interval =
                tokio::time::interval(std::time::Duration::from_millis(DEFAULT_WATCH_INTERVAL_MS));
            loop {
                interval.tick().await;
                let ignores = {
                    let cfg = watcher_state.current_config.read().await;
                    cfg.ignore_patterns.clone()
                };
                let changed_files = watcher.collect_changed_paths(&ignores);
                if !changed_files.is_empty() {
                    scan_handlers::execute_background_refresh(&watcher_state).await;
                }
            }
        }
    });

    if open_browser {
        let _ = opener::open(&server_url);
    }

    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests;
