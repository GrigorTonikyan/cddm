#![forbid(unsafe_code)]

use super::helpers::run_scan_from_mcp_args;
use crate::protocol::{
    JsonRpcResponse, make_error_response, make_text_response, mcp_tools, rpc_errors,
};
use cddm_core::{
    DEFAULT_DIRECTORY, DEFAULT_MIN_TOKENS, ScanConfig, generate_sarif_json, run_diff_scan,
};
use std::path::Path;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

pub async fn handle_scan_codebase(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let git_blame = args
        .and_then(|a| a.get(mcp_tools::PARAM_ENABLE_GIT_BLAME))
        .and_then(|b| b.as_bool())
        .unwrap_or(false);

    match run_scan_from_mcp_args(args, git_blame).await {
        Ok(scan_res) => make_text_response(
            id,
            serde_json::to_string_pretty(&scan_res).unwrap_or_default(),
        ),
        Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
    }
}

pub async fn handle_diff_scan(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let base_ref = args
        .and_then(|a| a.get(mcp_tools::PARAM_BASE_REF))
        .and_then(|b| b.as_str());

    if let Some(base) = base_ref {
        let dir = args
            .and_then(|a| a.get(mcp_tools::PARAM_DIRECTORY))
            .and_then(|d| d.as_str())
            .unwrap_or(DEFAULT_DIRECTORY);
        let target = args
            .and_then(|a| a.get(mcp_tools::PARAM_TARGET_REF))
            .and_then(|t| t.as_str());
        let min_tokens = args
            .and_then(|a| a.get(mcp_tools::PARAM_MIN_TOKENS))
            .and_then(|t| t.as_u64())
            .unwrap_or(DEFAULT_MIN_TOKENS as u64) as usize;

        let config = ScanConfig {
            directory: dir.to_string(),
            min_tokens,
            languages: vec![],
            ignore_patterns: ScanConfig::default().ignore_patterns,
            detect_type2: true,
            scan_self: true,
            enable_git_blame: false,
            cache_dir: None,
            enable_cache: true,
            cddmignore_path: None,
            ignore_tests: false,
            ignore_mocks: false,
            ignore_generated: true,
            rules_path: None,
            enforce_policies: false,
        };

        let (tx, _rx) = mpsc::channel(100);
        let cancel_flag = Arc::new(AtomicBool::new(false));

        match run_diff_scan(base, target, config, tx, cancel_flag).await {
            Ok(diff_res) => make_text_response(
                id,
                serde_json::to_string_pretty(&diff_res).unwrap_or_default(),
            ),
            Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        }
    } else {
        make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Missing required 'base_ref' argument",
        )
    }
}

pub async fn handle_export_sarif(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    match run_scan_from_mcp_args(args, false).await {
        Ok(scan_res) => {
            let sarif = generate_sarif_json(&scan_res);
            make_text_response(id, serde_json::to_string_pretty(&sarif).unwrap_or_default())
        }
        Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
    }
}

pub fn handle_get_timeline(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let dir = args
        .and_then(|a| a.get(mcp_tools::PARAM_DIRECTORY))
        .and_then(|d| d.as_str())
        .unwrap_or(DEFAULT_DIRECTORY);
    let max_samples = args
        .and_then(|a| a.get(mcp_tools::PARAM_MAX_SAMPLES))
        .and_then(|s| s.as_u64())
        .unwrap_or(10) as usize;
    let min_tokens = args
        .and_then(|a| a.get(mcp_tools::PARAM_MIN_TOKENS))
        .and_then(|t| t.as_u64())
        .unwrap_or(DEFAULT_MIN_TOKENS as u64) as usize;

    let cancel_flag = Arc::new(AtomicBool::new(false));
    match cddm_core::collect_git_timeline(Path::new(dir), max_samples, min_tokens, cancel_flag) {
        Ok(trend) => {
            make_text_response(id, serde_json::to_string_pretty(&trend).unwrap_or_default())
        }
        Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
    }
}
