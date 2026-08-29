#![forbid(unsafe_code)]

use crate::protocol::{JsonRpcResponse, make_error_response, make_text_response, rpc_errors};
use cddm_core::{
    HubExtractRequest, build_adhoc_hub_config, generate_hub_extraction, load_hub_config,
    run_hub_scan,
};
use std::path::Path;

/// Handler for the `cddm_scan_hub` MCP tool.
pub async fn handle_scan_hub(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let config_path = args
        .and_then(|a| a.get("config_path"))
        .and_then(|p| p.as_str());

    let min_tokens = args
        .and_then(|a| a.get("min_tokens"))
        .and_then(|m| m.as_u64())
        .unwrap_or(50) as usize;

    let config = if let Some(p) = config_path {
        match load_hub_config(p) {
            Ok(cfg) => cfg,
            Err(err) => return make_error_response(id, rpc_errors::INVALID_PARAMS, err),
        }
    } else if let Some(repos) = args
        .and_then(|a| a.get("repositories"))
        .and_then(|r| r.as_array())
    {
        let repo_paths: Vec<&Path> = repos
            .iter()
            .filter_map(|r| r.as_str())
            .map(Path::new)
            .collect();
        build_adhoc_hub_config("mcp-hub", &repo_paths, min_tokens)
    } else if Path::new(cddm_core::DEFAULT_HUB_CONFIG_FILE).exists() {
        match load_hub_config(cddm_core::DEFAULT_HUB_CONFIG_FILE) {
            Ok(cfg) => cfg,
            Err(err) => return make_error_response(id, rpc_errors::INTERNAL_ERROR, err),
        }
    } else {
        build_adhoc_hub_config("mcp-hub", &[Path::new(".")], min_tokens)
    };

    match run_hub_scan(&config).await {
        Ok(summary) => {
            let json_str = serde_json::to_string_pretty(&summary).unwrap_or_default();
            make_text_response(id, json_str)
        }
        Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
    }
}

/// Handler for the `cddm_extract_hub_package` MCP tool.
pub async fn handle_extract_hub_package(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let cluster_id = match args
        .and_then(|a| a.get("cluster_id"))
        .and_then(|c| c.as_u64())
    {
        Some(cid) => cid as usize,
        None => {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "Missing required parameter 'cluster_id'".to_string(),
            );
        }
    };

    let target_package_name = args
        .and_then(|a| a.get("package_name"))
        .and_then(|n| n.as_str())
        .unwrap_or("@org/shared-extracted")
        .to_string();

    let package_type = args
        .and_then(|a| a.get("package_type"))
        .and_then(|t| t.as_str())
        .unwrap_or("npm")
        .to_string();

    let target_dir = args
        .and_then(|a| a.get("target_dir"))
        .and_then(|d| d.as_str())
        .unwrap_or("./packages/shared-extracted")
        .to_string();

    let dry_run = args
        .and_then(|a| a.get("dry_run"))
        .and_then(|d| d.as_bool())
        .unwrap_or(true);

    let config_path = args
        .and_then(|a| a.get("config_path"))
        .and_then(|p| p.as_str());

    let hub_config = if let Some(p) = config_path {
        load_hub_config(p).ok()
    } else if Path::new(cddm_core::DEFAULT_HUB_CONFIG_FILE).exists() {
        load_hub_config(cddm_core::DEFAULT_HUB_CONFIG_FILE).ok()
    } else {
        Some(build_adhoc_hub_config("mcp-hub", &[Path::new(".")], 50))
    };

    let summary = if let Some(ref cfg) = hub_config {
        match run_hub_scan(cfg).await {
            Ok(s) => s,
            Err(e) => return make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        }
    } else {
        return make_error_response(
            id,
            rpc_errors::INTERNAL_ERROR,
            "Failed to resolve Federation Hub configuration".to_string(),
        );
    };

    let request = HubExtractRequest {
        hub_config,
        cluster_id,
        target_package_name,
        package_type,
        target_dir,
        dry_run,
    };

    match generate_hub_extraction(&summary, &request) {
        Ok(result) => {
            let json_str = serde_json::to_string_pretty(&result).unwrap_or_default();
            make_text_response(id, json_str)
        }
        Err(e) => make_error_response(id, rpc_errors::INVALID_PARAMS, e),
    }
}
