#![forbid(unsafe_code)]

use super::helpers::{parse_clone_pair_args, run_scan_from_mcp_args};
use crate::protocol::{
    JsonRpcResponse, make_app_widget_response, make_error_response, make_text_response, mcp_tools,
    rpc_errors,
};
use cddm_core::refactor::read_file_lines_range;
use serde_json::json;
use std::path::Path;

pub fn handle_get_clone_pair(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    if let Some((fa, sa, ea, fb, sb, eb)) = parse_clone_pair_args(args) {
        let lines_a = read_file_lines_range(Path::new(fa), sa, ea);
        let lines_b = read_file_lines_range(Path::new(fb), sb, eb);

        match (lines_a, lines_b) {
            (Ok(la), Ok(lb)) => {
                let payload = json!({
                    "fragment_a": {
                        "file": fa,
                        "start_line": sa,
                        "end_line": ea,
                        "line_count": la.len(),
                        "lines": la
                    },
                    "fragment_b": {
                        "file": fb,
                        "start_line": sb,
                        "end_line": eb,
                        "line_count": lb.len(),
                        "lines": lb
                    }
                });
                make_text_response(
                    id,
                    serde_json::to_string_pretty(&payload).unwrap_or_default(),
                )
            }
            (Err(e), _) | (_, Err(e)) => make_error_response(id, rpc_errors::INVALID_PARAMS, e),
        }
    } else {
        make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Missing required line range parameters",
        )
    }
}

pub async fn handle_get_clone_cluster(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let cluster_id = args
        .and_then(|a| a.get(mcp_tools::PARAM_CLUSTER_ID))
        .and_then(|cid| cid.as_u64())
        .map(|cid| cid as usize);

    if let Some(target_id) = cluster_id {
        match run_scan_from_mcp_args(args, true).await {
            Ok(scan_res) => {
                let found = scan_res.clone_clusters.iter().find(|c| c.id == target_id);

                if let Some(cluster) = found {
                    let mut occurrences_with_code = Vec::new();
                    for occ in &cluster.occurrences {
                        let code_lines = read_file_lines_range(
                            Path::new(&occ.file),
                            occ.start_line,
                            occ.end_line,
                        )
                        .unwrap_or_default();
                        occurrences_with_code.push(json!({
                            "file": occ.file,
                            "start_line": occ.start_line,
                            "end_line": occ.end_line,
                            "author": occ.author,
                            "code": code_lines.join("\n")
                        }));
                    }

                    let payload = json!({
                        "cluster_id": cluster.id,
                        "clone_type": format!("{:?}", cluster.clone_type),
                        "token_count": cluster.token_count,
                        "similarity": cluster.similarity,
                        "fragment_hash": cluster.fragment_hash,
                        "total_occurrences": cluster.occurrences.len(),
                        "occurrences": occurrences_with_code
                    });

                    let json_text = serde_json::to_string_pretty(&payload).unwrap_or_default();
                    let include_widget = args
                        .and_then(|a| a.get(mcp_tools::PARAM_INCLUDE_WIDGET))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    if include_widget {
                        let item = crate::apps::TreemapClusterItem {
                            id: cluster.id,
                            name: format!("Cluster #{}", cluster.id),
                            clone_type: format!("{:?}", cluster.clone_type),
                            occurrence_count: cluster.occurrences.len(),
                            token_count: cluster.token_count,
                            files: cluster.occurrences.iter().map(|o| o.file.clone()).collect(),
                        };
                        let widget = crate::apps::generate_cluster_treemap_widget(
                            &format!("Cluster #{} Treemap", cluster.id),
                            &[item],
                            scan_res.dry_health_score,
                        );
                        let widget_val = serde_json::to_value(&widget).unwrap_or_default();
                        make_app_widget_response(id, json_text, &widget_val)
                    } else {
                        make_text_response(id, json_text)
                    }
                } else {
                    make_error_response(
                        id,
                        rpc_errors::INVALID_PARAMS,
                        format!("Cluster #{} not found in scan results", target_id),
                    )
                }
            }
            Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        }
    } else {
        make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Missing required 'cluster_id' parameter",
        )
    }
}
