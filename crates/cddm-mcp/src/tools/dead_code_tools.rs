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
