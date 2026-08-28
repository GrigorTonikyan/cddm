#![forbid(unsafe_code)]

use super::helpers::run_scan_from_mcp_args;
use crate::protocol::{
    JsonRpcResponse, make_error_response, make_text_response, mcp_tools, rpc_errors,
};
use cddm_core::{
    CloneLocation, ExtractRequest, ExtractTargetKind, apply_shared_extraction,
    generate_shared_extraction,
};
use std::path::Path;

/// Handler for the `cddm_extract_shared_module` MCP tool.
pub async fn handle_extract_shared_module(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let target_path = args
        .and_then(|a| {
            a.get("target_path")
                .or_else(|| a.get("target"))
                .or_else(|| a.get("destination"))
        })
        .and_then(|t| t.as_str())
        .unwrap_or("crates/shared_utils");

    let fn_name = args
        .and_then(|a| {
            a.get("custom_function_name")
                .or_else(|| a.get("fn_name"))
                .or_else(|| a.get("function_name"))
        })
        .and_then(|f| f.as_str())
        .map(|s| s.to_string());

    let crate_type_str = args
        .and_then(|a| a.get("crate_type").or_else(|| a.get("target_kind")))
        .and_then(|c| c.as_str());

    let target_kind = match crate_type_str {
        Some("crate") | Some("new_crate") => ExtractTargetKind::NewCrate,
        Some("module") | Some("new_module") => ExtractTargetKind::NewModule,
        Some("existing") | Some("existing_module") => ExtractTargetKind::ExistingModule,
        _ => ExtractTargetKind::Auto,
    };

    let dry_run = args
        .and_then(|a| a.get("dry_run"))
        .and_then(|d| d.as_bool())
        .unwrap_or(false);

    let generate_tests = args
        .and_then(|a| a.get("generate_tests").or_else(|| a.get("tests")))
        .and_then(|g| g.as_bool())
        .unwrap_or(false);

    let occurrences_arr = args
        .and_then(|a| a.get(mcp_tools::PARAM_OCCURRENCES))
        .and_then(|o| o.as_array());

    let cluster_id_opt = args
        .and_then(|a| a.get(mcp_tools::PARAM_CLUSTER_ID))
        .and_then(|cid| cid.as_u64())
        .map(|cid| cid as usize);

    let occurrences = if let Some(occs_arr) = occurrences_arr {
        let mut parsed = Vec::new();
        for item in occs_arr {
            if let (Some(file), Some(start), Some(end)) = (
                item.get("file").and_then(|f| f.as_str()),
                item.get("start_line").and_then(|s| s.as_u64()),
                item.get("end_line").and_then(|e| e.as_u64()),
            ) {
                parsed.push(CloneLocation {
                    file: file.to_string(),
                    start_line: start as usize,
                    end_line: end as usize,
                    author: None,
                });
            }
        }
        parsed
    } else if let Some(cluster_id) = cluster_id_opt {
        match run_scan_from_mcp_args(args, false).await {
            Ok(scan_res) => {
                if let Some(cluster) = scan_res.clone_clusters.iter().find(|c| c.id == cluster_id) {
                    cluster.occurrences.clone()
                } else {
                    return make_error_response(
                        id,
                        rpc_errors::INVALID_PARAMS,
                        format!("Cluster #{} not found in workspace", cluster_id),
                    );
                }
            }
            Err(e) => return make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        }
    } else {
        match run_scan_from_mcp_args(args, false).await {
            Ok(scan_res) => {
                if let Some(first_cluster) = scan_res.clone_clusters.first() {
                    first_cluster.occurrences.clone()
                } else if let Some(first_pair) = scan_res.clone_pairs.first() {
                    vec![
                        CloneLocation {
                            file: first_pair.file_a.clone(),
                            start_line: first_pair.start_line_a,
                            end_line: first_pair.end_line_a,
                            author: None,
                        },
                        CloneLocation {
                            file: first_pair.file_b.clone(),
                            start_line: first_pair.start_line_b,
                            end_line: first_pair.end_line_b,
                            author: None,
                        },
                    ]
                } else {
                    return make_error_response(
                        id,
                        rpc_errors::INVALID_PARAMS,
                        "No duplicate code clones found in workspace to extract",
                    );
                }
            }
            Err(e) => return make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        }
    };

    if occurrences.is_empty() {
        return make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "No valid occurrences found for extraction",
        );
    }

    let request = ExtractRequest {
        occurrences,
        target_path: target_path.to_string(),
        custom_function_name: fn_name,
        target_kind,
        custom_parameter_names: None,
        generate_tests,
        dry_run,
    };

    let result = if dry_run {
        generate_shared_extraction(Path::new("."), &request)
    } else {
        apply_shared_extraction(Path::new("."), &request)
    };

    match result {
        Ok(res) => make_text_response(id, serde_json::to_string_pretty(&res).unwrap_or_default()),
        Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
    }
}
