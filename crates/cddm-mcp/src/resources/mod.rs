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
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_HUB,
                    "name": "Organization Federation Hub",
                    "description": "Multi-repository organization duplication metrics, member repositories, and cross-repo clusters.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_COVERAGE,
                    "name": "Workspace Runtime Coverage Correlation",
                    "description": "Runtime execution hit counts, dead code duplicates, and hot path risk analysis correlated with code clones.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uri": mcp_resources::URI_WORKSPACE_NEURAL_EMBEDDINGS,
                    "name": "Workspace Neural Code Embeddings & Algorithmic Equivalence",
                    "description": "Dense subword embedding vectors and cross-language algorithmic equivalence clone pairs.",
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
            "resourceTemplates": [
                {
                    "uriTemplate": "cddm://file/{path}/clones",
                    "name": "File Code Clones",
                    "description": "Code clone pairs involving a specific repository file path.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uriTemplate": "cddm://cluster/{cluster_id}/details",
                    "name": "Clone Cluster Details",
                    "description": "Occurrence details and consensus refactoring suggestions for a specific cluster ID.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                },
                {
                    "uriTemplate": "cddm://file/{path}/tokens",
                    "name": "File Token Spans",
                    "description": "Normalized token breakdown and line spans for a specific file path.",
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON
                }
            ]
        })),
        error: None,
    }
}

fn make_resource_json_response<T: serde::Serialize>(
    id: Option<serde_json::Value>,
    uri: &str,
    payload: &T,
) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
        result: Some(json!({
            "contents": [
                {
                    "uri": uri,
                    "mimeType": mcp_resources::MIME_APPLICATION_JSON,
                    "text": serde_json::to_string_pretty(payload).unwrap_or_default()
                }
            ]
        })),
        error: None,
    }
}

async fn run_default_scan() -> Result<cddm_core::ScanResult, String> {
    let config = ScanConfig::default();
    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));
    run_scan(config, tx, cancel_flag).await
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

    match uri {
        mcp_resources::URI_WORKSPACE_HEALTH => match run_default_scan().await {
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
                make_resource_json_response(id, uri, &payload)
            }
            Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        },

        mcp_resources::URI_WORKSPACE_CLONES => match run_default_scan().await {
            Ok(res) => make_resource_json_response(id, uri, &res.clone_pairs),
            Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        },

        mcp_resources::URI_WORKSPACE_CLUSTERS => match run_default_scan().await {
            Ok(res) => {
                let payload = json!({
                    "total_clusters": res.total_clusters,
                    "clone_clusters": res.clone_clusters
                });
                make_resource_json_response(id, uri, &payload)
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
                Ok(trend) => make_resource_json_response(id, uri, &trend),
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
            make_resource_json_response(id, uri, engine.config())
        }

        mcp_resources::URI_WORKSPACE_POLICIES => {
            let root_path = Path::new(DEFAULT_RULES_FILE);
            let engine = if root_path.exists() {
                PolicyEngine::from_file(root_path).unwrap_or_else(|_| PolicyEngine::empty())
            } else {
                PolicyEngine::empty()
            };
            make_resource_json_response(id, uri, engine.config())
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
            make_resource_json_response(id, uri, &payload)
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
                    make_resource_json_response(id, uri, &payload)
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
            make_resource_json_response(id, uri, &payload)
        }

        mcp_resources::URI_WORKSPACE_OVERLAP => {
            match cddm_core::scan_workspace_overlap(Path::new("."), 0.3) {
                Ok(result) => make_resource_json_response(id, uri, &result),
                Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
            }
        }

        mcp_resources::URI_WORKSPACE_HUB => {
            let config = if Path::new(cddm_core::DEFAULT_HUB_CONFIG_FILE).exists() {
                cddm_core::load_hub_config(cddm_core::DEFAULT_HUB_CONFIG_FILE).unwrap_or_else(
                    |_| cddm_core::build_adhoc_hub_config("hub", &[Path::new(".")], 50),
                )
            } else {
                cddm_core::build_adhoc_hub_config("hub", &[Path::new(".")], 50)
            };

            match cddm_core::run_hub_scan(&config).await {
                Ok(summary) => make_resource_json_response(id, uri, &summary),
                Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
            }
        }

        mcp_resources::URI_WORKSPACE_COVERAGE => {
            let report = if Path::new("lcov.info").exists() {
                cddm_core::load_coverage_report(
                    Path::new("lcov.info"),
                    cddm_core::CoverageFormat::Auto,
                )
                .unwrap_or_default()
            } else {
                cddm_core::CoverageReport::default()
            };

            match run_default_scan().await {
                Ok(scan_result) => {
                    let summary = cddm_core::correlate_coverage(&scan_result, &report);
                    make_resource_json_response(id, uri, &summary)
                }
                Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
            }
        }

        mcp_resources::URI_WORKSPACE_NEURAL_EMBEDDINGS => {
            let config = cddm_core::NeuralEmbeddingConfig::default();
            match cddm_core::scan_neural_clones(Path::new("."), &config) {
                Ok(result) => make_resource_json_response(id, uri, &result),
                Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
            }
        }

        uri if uri.starts_with("cddm://file/") && uri.ends_with("/clones") => {
            let raw_path = &uri[12..uri.len() - 7];
            let target_path = urlencoding_decode(raw_path);
            match run_default_scan().await {
                Ok(res) => {
                    let matching: Vec<_> = res
                        .clone_pairs
                        .into_iter()
                        .filter(|p| {
                            p.file_a.contains(&target_path) || p.file_b.contains(&target_path)
                        })
                        .collect();
                    let payload = json!({
                        "file": target_path,
                        "total_clones": matching.len(),
                        "clone_pairs": matching
                    });
                    make_resource_json_response(id, uri, &payload)
                }
                Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
            }
        }

        uri if uri.starts_with("cddm://cluster/") && uri.ends_with("/details") => {
            let cluster_id_str = &uri[15..uri.len() - 8];
            let cluster_id: usize = cluster_id_str.parse().unwrap_or(0);
            match run_default_scan().await {
                Ok(res) => {
                    let clusters = if res.clone_clusters.is_empty() {
                        cddm_core::cluster_clone_pairs(&res.clone_pairs)
                    } else {
                        res.clone_clusters
                    };
                    if let Some(cluster) = clusters.into_iter().find(|c| c.id == cluster_id) {
                        make_resource_json_response(id, uri, &cluster)
                    } else {
                        make_error_response(
                            id,
                            rpc_errors::INVALID_PARAMS,
                            format!("Cluster ID '{}' not found", cluster_id_str),
                        )
                    }
                }
                Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
            }
        }

        uri if uri.starts_with("cddm://file/") && uri.ends_with("/tokens") => {
            let raw_path = &uri[12..uri.len() - 7];
            let target_path = urlencoding_decode(raw_path);
            let path = Path::new(&target_path);
            match cddm_core::read_file_source(path) {
                Ok(content) => {
                    if let Some(grammar) = cddm_core::grammar::get_grammar_for_path(path) {
                        let tokens = cddm_core::tokenizer::tokenize(&content, grammar, true);
                        let token_spans: Vec<_> =
                            tokens.iter().map(|(_, span)| span.clone()).collect();
                        let payload = json!({
                            "file": target_path,
                            "language": grammar.name,
                            "token_count": tokens.len(),
                            "token_spans": token_spans
                        });
                        make_resource_json_response(id, uri, &payload)
                    } else {
                        make_error_response(
                            id,
                            rpc_errors::INVALID_PARAMS,
                            format!("Unsupported file language for '{}'", target_path),
                        )
                    }
                }
                Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e.to_string()),
            }
        }

        _ => make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            format!("Resource URI '{}' not found", uri),
        ),
    }
}

fn urlencoding_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            let h1 = chars.next();
            let h2 = chars.next();
            if let (Some(c1), Some(c2)) = (h1, h2) {
                let hex_str = format!("{}{}", c1, c2);
                if let Ok(byte) = u8::from_str_radix(&hex_str, 16) {
                    result.push(byte as char);
                    continue;
                } else {
                    result.push('%');
                    result.push(c1);
                    result.push(c2);
                    continue;
                }
            } else {
                result.push('%');
                if let Some(c1) = h1 {
                    result.push(c1);
                }
                continue;
            }
        } else {
            result.push(ch);
        }
    }
    result
}
