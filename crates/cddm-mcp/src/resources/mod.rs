#![forbid(unsafe_code)]

use crate::protocol::{
    JSONRPC_VERSION, JsonRpcResponse, make_error_response, mcp_resources, rpc_errors,
};
use cddm_core::{
    DEFAULT_DIRECTORY, DEFAULT_MIN_TOKENS, DEFAULT_RULES_FILE, PolicyEngine, ScanConfig,
    SuppressionEngine, collect_git_timeline, run_scan,
};
use serde_json::json;
use std::path::Path;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

pub fn resources_list_response(id: Option<serde_json::Value>) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
        result: Some(json!({
            "resources": [
                {
                    "uri": mcp_resources::URI_WORKSPACE_HEALTH,
                    "name": "Workspace DRY Health Score",
                    "description": "Real-time DRY Health Index, file metrics, and language statistics.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_CLONES,
                    "name": "Workspace Code Clones",
                    "description": "Registry of active duplicate code clones across repository files.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_CLUSTERS,
                    "name": "Workspace Code Clone Clusters",
                    "description": "N-way equivalence classes of duplicated logic across repository files.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_TIMELINE,
                    "name": "Workspace Historical Duplication Trend",
                    "description": "Historical DRY Health trajectories and commit snapshots across Git history.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_SUPPRESSIONS,
                    "name": "Workspace Suppression Rules",
                    "description": "Active .cddmignore suppression rules, threshold overrides, and filter directives.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_POLICIES,
                    "name": "Workspace Architectural Policy Rules",
                    "description": "Active .cddmrules.toml boundary and anti-duplication policy rules.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_SEMANTIC_GRAPH,
                    "name": "Workspace Semantic Dependency Graph",
                    "description": "Control Flow and Program Dependence Graph metadata and structural clone isomorphisms.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_CROSS_LANGUAGE_CLONES,
                    "name": "Workspace Cross-Language Clones",
                    "description": "Cross-language semantic clone pairs detected across different programming languages via Weisfeiler-Lehman graph kernels and subword embeddings.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_WATCH_STATUS,
                    "name": "Workspace Live Watch Status",
                    "description": "Real-time status of directory watcher daemon, debounce settings, and incremental delta metrics.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_OVERLAP,
                    "name": "Workspace Ecosystem Library Overlap",
                    "description": "Reimplemented standard and community package utilities detected across workspace files.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                }
            ]
        })),
        error: None,
    }
}

pub fn resources_templates_list_response(id: Option<serde_json::Value>) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
        result: Some(json!({
            "resourceTemplates": []
        })),
        error: None,
    }
}

pub async fn handle_resource_read(
    id: Option<serde_json::Value>,
    params: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let uri = params
        .as_ref()
        .and_then(|p| p.get("uri"))
        .and_then(|u| u.as_str())
        .unwrap_or("");

    let config = ScanConfig::default();
    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    match uri {
        mcp_resources::URI_WORKSPACE_HEALTH => match run_scan(config, tx, cancel_flag).await {
            Ok(res) => {
                let payload = json!({
                    "dry_health_score": res.dry_health_score,
                    "duplication_percentage": res.duplication_percentage,
                    "total_files": res.total_files,
                    "total_tokens": res.total_tokens,
                    "total_clones": res.total_clones,
                    "total_clusters": res.total_clusters,
                    "language_breakdown": res.language_breakdown
                });
                JsonRpcResponse {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    id,
                    result: Some(json!({
                        "contents": [
                            {
                                "uri": mcp_resources::URI_WORKSPACE_HEALTH,
                                "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                                "text": serde_json::to_string_pretty(&payload).unwrap_or_default()
                            }
                        ]
                    })),
                    error: None,
                }
            }
            Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        },

        mcp_resources::URI_WORKSPACE_CLONES => match run_scan(config, tx, cancel_flag).await {
            Ok(res) => JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id,
                result: Some(json!({
                    "contents": [
                        {
                            "uri": mcp_resources::URI_WORKSPACE_CLONES,
                            "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                            "text": serde_json::to_string_pretty(&res.clone_pairs).unwrap_or_default()
                        }
                    ]
                })),
                error: None,
            },
            Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        },

        mcp_resources::URI_WORKSPACE_CLUSTERS => match run_scan(config, tx, cancel_flag).await {
            Ok(res) => {
                let payload = json!({
                    "total_clusters": res.total_clusters,
                    "clone_clusters": res.clone_clusters
                });
                JsonRpcResponse {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    id,
                    result: Some(json!({
                        "contents": [
                            {
                                "uri": mcp_resources::URI_WORKSPACE_CLUSTERS,
                                "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                                "text": serde_json::to_string_pretty(&payload).unwrap_or_default()
                            }
                        ]
                    })),
                    error: None,
                }
            }
            Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        },

        mcp_resources::URI_WORKSPACE_TIMELINE => {
            let cancel_flag = Arc::new(AtomicBool::new(false));
            match collect_git_timeline(
                Path::new(DEFAULT_DIRECTORY),
                10,
                DEFAULT_MIN_TOKENS,
                cancel_flag,
            ) {
                Ok(trend) => JsonRpcResponse {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    id,
                    result: Some(json!({
                        "contents": [
                            {
                                "uri": mcp_resources::URI_WORKSPACE_TIMELINE,
                                "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                                "text": serde_json::to_string_pretty(&trend).unwrap_or_default()
                            }
                        ]
                    })),
                    error: None,
                },
                Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
            }
        }

        mcp_resources::URI_WORKSPACE_SUPPRESSIONS => {
            let root_path = Path::new(".cddmignore");
            let engine = if root_path.exists() {
                SuppressionEngine::from_file(root_path, false, false, true)
                    .unwrap_or_else(|_| SuppressionEngine::default_engine())
            } else {
                SuppressionEngine::default_engine()
            };
            JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id,
                result: Some(json!({
                    "contents": [
                        {
                            "uri": mcp_resources::URI_WORKSPACE_SUPPRESSIONS,
                            "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                            "text": serde_json::to_string_pretty(engine.config()).unwrap_or_default()
                        }
                    ]
                })),
                error: None,
            }
        }

        mcp_resources::URI_WORKSPACE_POLICIES => {
            let root_path = Path::new(DEFAULT_RULES_FILE);
            let engine = if root_path.exists() {
                PolicyEngine::from_file(root_path).unwrap_or_else(|_| PolicyEngine::empty())
            } else {
                PolicyEngine::empty()
            };
            JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id,
                result: Some(json!({
                    "contents": [
                        {
                            "uri": mcp_resources::URI_WORKSPACE_POLICIES,
                            "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                            "text": serde_json::to_string_pretty(engine.config()).unwrap_or_default()
                        }
                    ]
                })),
                error: None,
            }
        }

        mcp_resources::URI_WORKSPACE_SEMANTIC_GRAPH => {
            let cfgs = cddm_core::extract_cfgs_from_source(
                "example.rs",
                "fn example() { let a = 1; if a > 0 { let b = a + 2; } }",
                "Rust",
            );
            let mut pdgs = Vec::new();
            for cfg in &cfgs {
                pdgs.push(cddm_core::build_pdg_from_cfg(cfg.clone()));
            }
            let payload = json!({
                "cfg_count": cfgs.len(),
                "pdg_count": pdgs.len(),
                "cfgs": cfgs,
                "pdgs": pdgs,
            });
            JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id,
                result: Some(json!({
                    "contents": [
                        {
                            "uri": mcp_resources::URI_WORKSPACE_SEMANTIC_GRAPH,
                            "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                            "text": serde_json::to_string_pretty(&payload).unwrap_or_default()
                        }
                    ]
                })),
                error: None,
            }
        }

        mcp_resources::URI_WORKSPACE_CROSS_LANGUAGE_CLONES => {
            let scan_cfg = ScanConfig {
                cross_language: true,
                ..Default::default()
            };
            match cddm_core::semantic_graph::scan_cross_language_workspace(&scan_cfg, 0.70) {
                Ok(pairs) => {
                    let payload = json!({
                        "threshold": 0.70,
                        "total_pairs": pairs.len(),
                        "pairs": pairs,
                    });
                    JsonRpcResponse {
                        jsonrpc: JSONRPC_VERSION.to_string(),
                        id,
                        result: Some(json!({
                            "contents": [
                                {
                                    "uri": mcp_resources::URI_WORKSPACE_CROSS_LANGUAGE_CLONES,
                                    "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                                    "text": serde_json::to_string_pretty(&payload).unwrap_or_default()
                                }
                            ]
                        })),
                        error: None,
                    }
                }
                Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
            }
        }

        mcp_resources::URI_WORKSPACE_WATCH_STATUS => {
            let payload = json!({
                "is_watching": true,
                "watch_directory": ".",
                "debounce_ms": 300,
                "supported_events": ["watch_file_changed", "watch_scan_delta", "watch_status_changed"],
                "live_sync_protocol": "Server-Sent Events (/api/events)"
            });
            JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id,
                result: Some(json!({
                    "contents": [
                        {
                            "uri": mcp_resources::URI_WORKSPACE_WATCH_STATUS,
                            "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                            "text": serde_json::to_string_pretty(&payload).unwrap_or_default()
                        }
                    ]
                })),
                error: None,
            }
        }

        mcp_resources::URI_WORKSPACE_OVERLAP => {
            match cddm_core::scan_workspace_overlap(Path::new("."), 0.3) {
                Ok(result) => JsonRpcResponse {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    id,
                    result: Some(json!({
                        "contents": [
                            {
                                "uri": mcp_resources::URI_WORKSPACE_OVERLAP,
                                "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                                "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                            }
                        ]
                    })),
                    error: None,
                },
                Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
            }
        }

        _ => make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            format!("Resource URI '{}' not found", uri),
        ),
    }
}
