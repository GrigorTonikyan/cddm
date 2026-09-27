#![forbid(unsafe_code)]

use super::diff_widget::generate_diff_split_view_widget;
use super::treemap_widget::{TreemapClusterItem, generate_cluster_treemap_widget};
use super::types::{AppWidgetDescriptor, widget_types};
use crate::protocol::{JSONRPC_VERSION, JsonRpcResponse, make_error_response, rpc_errors};
use serde_json::json;

/// Returns the descriptor list of generative UI widgets supported by this MCP server.
pub fn get_supported_app_widgets() -> Vec<AppWidgetDescriptor> {
    vec![
        AppWidgetDescriptor {
            id: widget_types::DIFF_SPLIT_VIEW.to_string(),
            widget_type: widget_types::DIFF_SPLIT_VIEW.to_string(),
            title: "Interactive Diff Split-View".to_string(),
            name: "Interactive Diff Split-View".to_string(),
            description: "High-fidelity side-by-side or unified diff comparison widget with one-click patch copy and accept action hooks."
                .to_string(),
            supported_modes: vec![
                "inline".to_string(),
                "modal".to_string(),
                "side-by-side".to_string(),
            ],
            supported_params: vec![
                "title".to_string(),
                "file_a".to_string(),
                "file_b".to_string(),
                "original_code".to_string(),
                "refactored_code".to_string(),
                "diff_patch".to_string(),
            ],
        },
        AppWidgetDescriptor {
            id: widget_types::CLUSTER_TREEMAP.to_string(),
            widget_type: widget_types::CLUSTER_TREEMAP.to_string(),
            title: "Clone Cluster Treemap".to_string(),
            name: "Clone Cluster Treemap".to_string(),
            description: "Interactive visual duplication treemap with color-coded severity tiles, filter search, and cluster inspection."
                .to_string(),
            supported_modes: vec!["inline".to_string(), "modal".to_string()],
            supported_params: vec![
                "title".to_string(),
                "directory".to_string(),
                "clusters".to_string(),
                "dry_health_score".to_string(),
            ],
        },
    ]
}

/// Handles `apps/widgets/list` JSON-RPC method.
pub fn handle_apps_widgets_list(id: Option<serde_json::Value>) -> JsonRpcResponse {
    let widgets = get_supported_app_widgets();
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id,
        result: Some(json!({
            "widgets": widgets,
        })),
        error: None,
        headers: None,
    }
}

/// Handles `apps/render` JSON-RPC method to render an interactive widget on demand.
pub async fn handle_apps_render(
    id: Option<serde_json::Value>,
    params: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let args = match params {
        Some(p) => p,
        None => {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "Missing parameters for apps/render",
            );
        }
    };

    let widget_type = match args
        .get("widget_type")
        .or_else(|| args.get("widgetType"))
        .and_then(|v| v.as_str())
    {
        Some(wt) => wt,
        None => {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "Missing required 'widget_type' parameter",
            );
        }
    };

    let props = args.get("props");
    let get_val = |k1: &str, k2: &str| -> Option<&serde_json::Value> {
        args.get(k1)
            .or_else(|| args.get(k2))
            .or_else(|| props.and_then(|p| p.get(k1).or_else(|| p.get(k2))))
    };

    let title = get_val("title", "name")
        .and_then(|v| v.as_str())
        .unwrap_or("CDDM Interactive Widget");

    match widget_type {
        widget_types::DIFF_SPLIT_VIEW => {
            let file_a = get_val("file_a", "fileA")
                .and_then(|v| v.as_str())
                .unwrap_or("a.rs");
            let file_b = get_val("file_b", "fileB")
                .and_then(|v| v.as_str())
                .unwrap_or("b.rs");
            let start_a = get_val("start_line_a", "startLineA")
                .and_then(|v| v.as_u64())
                .unwrap_or(1) as usize;
            let end_a = get_val("end_line_a", "endLineA")
                .and_then(|v| v.as_u64())
                .unwrap_or(1) as usize;
            let start_b = get_val("start_line_b", "startLineB")
                .and_then(|v| v.as_u64())
                .unwrap_or(1) as usize;
            let end_b = get_val("end_line_b", "endLineB")
                .and_then(|v| v.as_u64())
                .unwrap_or(1) as usize;
            let orig = get_val("original_code", "originalCode")
                .and_then(|v| v.as_str())
                .unwrap_or("// Original Code");
            let refac = get_val("refactored_code", "refactoredCode")
                .and_then(|v| v.as_str())
                .unwrap_or("// Refactored Code");
            let patch = get_val("diff_patch", "patch").and_then(|v| v.as_str());

            let widget = generate_diff_split_view_widget(
                title,
                file_a,
                (start_a, end_a),
                file_b,
                (start_b, end_b),
                orig,
                refac,
                patch,
            );

            JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id,
                result: Some(json!({
                    "widget": widget,
                    "widgetType": widget.widget_type,
                    "title": widget.title,
                    "html": widget.html,
                    "data": widget.data,
                })),
                error: None,
                headers: None,
            }
        }

        widget_types::CLUSTER_TREEMAP => {
            // If explicit clusters are passed in params, use them; otherwise run a quick scan
            let clusters_val = get_val("clusters", "cluster_list");
            let (clusters, dry_score): (Vec<TreemapClusterItem>, f64) =
                if let Some(items) = clusters_val.and_then(|v| v.as_array()) {
                    let parsed = items
                        .iter()
                        .map(|it| {
                            let cid = it.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                            let name = it
                                .get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Cluster")
                                .to_string();
                            let ctype = it
                                .get("clone_type")
                                .or_else(|| it.get("cloneType"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("renamed")
                                .to_string();
                            let occs = it
                                .get("occurrence_count")
                                .or_else(|| it.get("occurrenceCount"))
                                .or_else(|| it.get("instances"))
                                .and_then(|v| v.as_u64())
                                .unwrap_or(1) as usize;
                            let tokens = it
                                .get("token_count")
                                .or_else(|| it.get("tokenCount"))
                                .or_else(|| it.get("totalLines"))
                                .or_else(|| it.get("total_lines"))
                                .and_then(|v| v.as_u64())
                                .unwrap_or(50) as usize;
                            let files = it
                                .get("files")
                                .and_then(|v| v.as_array())
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                                        .collect()
                                })
                                .unwrap_or_default();
                            TreemapClusterItem {
                                id: cid,
                                name,
                                clone_type: ctype,
                                occurrence_count: occs,
                                token_count: tokens,
                                files,
                            }
                        })
                        .collect();
                    let score = get_val("dry_health_score", "dryHealthScore")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(95.0);
                    (parsed, score)
                } else {
                    match crate::tools::helpers::run_scan_from_mcp_args(params, false).await {
                        Ok(scan_res) => {
                            let parsed = scan_res
                                .clone_clusters
                                .iter()
                                .map(|c| {
                                    let files: Vec<String> =
                                        c.occurrences.iter().map(|o| o.file.clone()).collect();
                                    TreemapClusterItem {
                                        id: c.id,
                                        name: format!("Cluster #{}", c.id),
                                        clone_type: format!("{:?}", c.clone_type),
                                        occurrence_count: c.occurrences.len(),
                                        token_count: c.token_count,
                                        files,
                                    }
                                })
                                .collect();
                            (parsed, scan_res.dry_health_score)
                        }
                        Err(e) => {
                            return make_error_response(id, rpc_errors::INTERNAL_ERROR, e);
                        }
                    }
                };

            let widget = generate_cluster_treemap_widget(title, &clusters, dry_score);

            JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id,
                result: Some(json!({
                    "widget": widget,
                    "widgetType": widget.widget_type,
                    "title": widget.title,
                    "html": widget.html,
                    "data": widget.data,
                })),
                error: None,
                headers: None,
            }
        }

        _ => make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            format!("Unsupported widget_type '{}'", widget_type),
        ),
    }
}
