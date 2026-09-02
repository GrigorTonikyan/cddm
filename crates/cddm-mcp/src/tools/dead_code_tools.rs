#![forbid(unsafe_code)]

use cddm_core::dead_code::{DeadCodeConfig, run_dead_code_detection};
use serde_json::Value;

use crate::protocol::{JsonRpcResponse, make_error_response, make_text_response, rpc_errors};

/// Handle tool `cddm_detect_dead_code`: finds unreferenced functions, unreachable blocks, and dead clones.
pub async fn handle_detect_dead_code(id: Option<Value>, args: Option<&Value>) -> JsonRpcResponse {
    let directory = args
        .and_then(|a| a.get("directory"))
        .and_then(|d| d.as_str())
        .unwrap_or(".");
    let min_tokens = args
        .and_then(|a| a.get("min_tokens"))
        .and_then(|m| m.as_u64())
        .unwrap_or(30) as usize;
    let static_only = args
        .and_then(|a| a.get("static_only"))
        .and_then(|s| s.as_bool())
        .unwrap_or(false);
    let report_path = args
        .and_then(|a| a.get("report_path"))
        .and_then(|p| p.as_str())
        .map(String::from);
    let report_content = args
        .and_then(|a| a.get("report_content"))
        .and_then(|c| c.as_str())
        .map(String::from);

    let config = DeadCodeConfig {
        directory: directory.to_string(),
        min_tokens,
        static_only,
        report_path,
        report_content,
        languages: None,
        ignore: None,
    };

    match run_dead_code_detection(config).await {
        Ok(summary) => {
            let json_str = serde_json::to_string_pretty(&summary).unwrap_or_default();
            make_text_response(id, json_str)
        }
        Err(err) => make_error_response(id, rpc_errors::INTERNAL_ERROR, err.to_string()),
    }
}

/// Handle tool `cddm_prune_dead_clones`: safely prunes dead clone clusters with closed-loop reachability verification.
pub async fn handle_prune_dead_clones(id: Option<Value>, args: Option<&Value>) -> JsonRpcResponse {
    let directory = args
        .and_then(|a| a.get("directory"))
        .and_then(|d| d.as_str())
        .unwrap_or(".");
    let min_tokens = args
        .and_then(|a| a.get("min_tokens"))
        .and_then(|m| m.as_u64())
        .unwrap_or(30) as usize;
    let dry_run = args
        .and_then(|a| a.get("dry_run"))
        .and_then(|s| s.as_bool())
        .unwrap_or(false);
    let safe_only = args
        .and_then(|a| a.get("safe_only"))
        .and_then(|s| s.as_bool())
        .unwrap_or(true);
    let threshold = args
        .and_then(|a| a.get("threshold"))
        .and_then(|t| t.as_f64())
        .unwrap_or(0.90);
    let item_ids = args
        .and_then(|a| a.get("item_ids"))
        .and_then(|ids| ids.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64().map(|n| n as usize))
                .collect()
        });

    let config = cddm_core::dead_code::DeadClonePruneConfig {
        directory: directory.to_string(),
        min_tokens,
        dry_run,
        safe_only,
        confidence_threshold: threshold,
        item_ids,
        languages: None,
        ignore: None,
    };

    match cddm_core::dead_code::prune_dead_clone_clusters(config).await {
        Ok(result) => {
            let json_str = serde_json::to_string_pretty(&result).unwrap_or_default();
            make_text_response(id, json_str)
        }
        Err(err) => make_error_response(id, rpc_errors::INTERNAL_ERROR, err.to_string()),
    }
}

/// Handle tool `cddm_trace_reachability`: computes cross-package call-graph reachability for polyglot monorepos.
pub async fn handle_trace_reachability(id: Option<Value>, args: Option<&Value>) -> JsonRpcResponse {
    let directory = args
        .and_then(|a| a.get("directory"))
        .and_then(|d| d.as_str())
        .unwrap_or(".");
    let min_tokens = args
        .and_then(|a| a.get("min_tokens"))
        .and_then(|m| m.as_u64())
        .unwrap_or(30) as usize;

    let config = DeadCodeConfig {
        directory: directory.to_string(),
        min_tokens,
        static_only: false,
        ..Default::default()
    };

    match run_dead_code_detection(config).await {
        Ok(summary) => {
            let reachability = summary.reachability_summary.unwrap_or_default();
            let json_str = serde_json::to_string_pretty(&reachability).unwrap_or_default();
            make_text_response(id, json_str)
        }
        Err(err) => make_error_response(id, rpc_errors::INTERNAL_ERROR, err.to_string()),
    }
}
