#![forbid(unsafe_code)]

use crate::protocol::{JsonRpcResponse, make_error_response, make_text_response, rpc_errors};
use cddm_core::grammar::get_grammar_for_path;
use cddm_core::semantic_graph::{
    build_pdg_from_cfg, calculate_graph_similarity, extract_cfgs_from_source,
};
use serde_json::json;
use std::fs;
use std::path::Path;

/// Handler for `cddm_get_semantic_graph` MCP tool.
pub fn handle_get_semantic_graph(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let args_obj = match args {
        Some(serde_json::Value::Object(map)) => map,
        _ => {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "Missing arguments object for 'cddm_get_semantic_graph'",
            );
        }
    };

    let code_opt = args_obj.get("code").and_then(|v| v.as_str());
    let file_opt = args_obj.get("file").and_then(|v| v.as_str());
    let lang_opt = args_obj.get("language").and_then(|v| v.as_str());

    let (path_str, code_str, lang_str) = if let Some(code) = code_opt {
        let path = file_opt.unwrap_or("snippet.rs").to_string();
        let lang = lang_opt
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Rust".to_string());
        (path, code.to_string(), lang)
    } else if let Some(file_path) = file_opt {
        let path = Path::new(file_path);
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                return make_error_response(
                    id,
                    rpc_errors::INTERNAL_ERROR,
                    format!("Failed to read file '{}': {}", file_path, e),
                );
            }
        };
        let lang = lang_opt.map(|s| s.to_string()).unwrap_or_else(|| {
            get_grammar_for_path(path)
                .map(|g| g.name.to_string())
                .unwrap_or_else(|| "Rust".to_string())
        });
        (file_path.to_string(), content, lang)
    } else {
        return make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Either 'code' or 'file' must be provided",
        );
    };

    let cfgs = extract_cfgs_from_source(&path_str, &code_str, &lang_str);
    let mut pdgs = Vec::with_capacity(cfgs.len());
    for cfg in &cfgs {
        pdgs.push(build_pdg_from_cfg(cfg.clone()));
    }

    let payload = json!({
        "file": path_str,
        "language": lang_str,
        "cfg_count": cfgs.len(),
        "pdg_count": pdgs.len(),
        "cfgs": cfgs,
        "pdgs": pdgs,
    });

    make_text_response(
        id,
        serde_json::to_string_pretty(&payload).unwrap_or_default(),
    )
}

/// Handler for `cddm_compare_semantic_graphs` MCP tool.
pub fn handle_compare_semantic_graphs(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let args_obj = match args {
        Some(serde_json::Value::Object(map)) => map,
        _ => {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "Missing arguments object for 'cddm_compare_semantic_graphs'",
            );
        }
    };

    let code_a = args_obj.get("code_a").and_then(|v| v.as_str());
    let code_b = args_obj.get("code_b").and_then(|v| v.as_str());
    let lang = args_obj
        .get("language")
        .and_then(|v| v.as_str())
        .unwrap_or("Rust");

    let (src_a, src_b) = match (code_a, code_b) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "Both 'code_a' and 'code_b' must be provided",
            );
        }
    };

    let cfgs_a = extract_cfgs_from_source("a.rs", src_a, lang);
    let cfgs_b = extract_cfgs_from_source("b.rs", src_b, lang);

    if cfgs_a.is_empty() || cfgs_b.is_empty() {
        return make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Could not extract Control Flow Graphs from one or both source inputs",
        );
    }

    let similarity = calculate_graph_similarity(&cfgs_a[0], &cfgs_b[0]);
    let is_semantic_clone = similarity >= 0.75;

    let payload = json!({
        "similarity": similarity,
        "is_semantic_clone": is_semantic_clone,
        "wl_hash_a": cfgs_a[0].wl_hash,
        "wl_hash_b": cfgs_b[0].wl_hash,
        "function_a": cfgs_a[0].function_name,
        "function_b": cfgs_b[0].function_name,
        "nodes_a_count": cfgs_a[0].nodes.len(),
        "nodes_b_count": cfgs_b[0].nodes.len(),
    });

    make_text_response(
        id,
        serde_json::to_string_pretty(&payload).unwrap_or_default(),
    )
}
