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
            cross_language: false,
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

pub fn handle_export_cache_pack(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let db_path_str = args
        .and_then(|a| a.get("cache_dir"))
        .and_then(|v| v.as_str())
        .unwrap_or(".cddm/cache.db");
    let out_path_str = args
        .and_then(|a| a.get("output_pack_path"))
        .and_then(|v| v.as_str())
        .unwrap_or("cddm-cache.cddmpack");

    match cddm_core::export_cache_pack(Path::new(db_path_str), Path::new(out_path_str)) {
        Ok(summary) => {
            let json_str = serde_json::to_string_pretty(&summary).unwrap_or_default();
            make_text_response(id, json_str)
        }
        Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
    }
}

pub fn handle_import_cache_pack(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    if let Some(pack_str) = args
        .and_then(|a| a.get("pack_file"))
        .and_then(|v| v.as_str())
    {
        let target_dir_str = args
            .and_then(|a| a.get("target_cache_dir"))
            .and_then(|v| v.as_str())
            .unwrap_or(".cddm");

        match cddm_core::import_cache_pack(Path::new(pack_str), Path::new(target_dir_str)) {
            Ok(summary) => {
                let json_str = serde_json::to_string_pretty(&summary).unwrap_or_default();
                make_text_response(id, json_str)
            }
            Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        }
    } else {
        make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Missing required 'pack_file' parameter",
        )
    }
}

pub async fn handle_scan_monorepo(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let dir_str = args
        .and_then(|a| a.get(mcp_tools::PARAM_DIRECTORY))
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_DIRECTORY);
    let min_tokens = args
        .and_then(|a| a.get(mcp_tools::PARAM_MIN_TOKENS))
        .and_then(|v| v.as_u64())
        .unwrap_or(DEFAULT_MIN_TOKENS as u64) as usize;

    let config = ScanConfig {
        directory: dir_str.to_string(),
        min_tokens,
        languages: vec![],
        ignore_patterns: vec![],
        detect_type2: true,
        scan_self: false,
        enable_git_blame: false,
        cache_dir: None,
        enable_cache: false,
        cddmignore_path: None,
        ignore_tests: false,
        ignore_mocks: false,
        ignore_generated: true,
        rules_path: None,
        enforce_policies: false,
        cross_language: false,
    };

    match cddm_core::run_monorepo_scan(Path::new(dir_str), &config).await {
        Ok(summary) => {
            let json_str = serde_json::to_string_pretty(&summary).unwrap_or_default();
            make_text_response(id, json_str)
        }
        Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
    }
}
