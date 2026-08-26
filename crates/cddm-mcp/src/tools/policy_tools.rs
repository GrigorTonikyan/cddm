#![forbid(unsafe_code)]

use crate::protocol::{
    JsonRpcResponse, make_error_response, make_text_response, mcp_tools, rpc_errors,
};
use cddm_core::{
    DEFAULT_DIRECTORY, DEFAULT_MIN_TOKENS, DEFAULT_RULES_FILE, PolicyEngine, ScanConfig,
    SuppressionEngine, run_scan,
};
use serde_json::json;
use std::path::Path;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

pub fn handle_check_suppression(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let path_str = args
        .and_then(|a| a.get(mcp_tools::PARAM_PATH))
        .and_then(|p| p.as_str());

    if let Some(path) = path_str {
        let line_opt = args
            .and_then(|a| a.get(mcp_tools::PARAM_LINE))
            .and_then(|l| l.as_u64())
            .map(|l| l as usize);
        let custom_ignore = args
            .and_then(|a| a.get(mcp_tools::PARAM_CDDMIGNORE))
            .and_then(|i| i.as_str());
        let ignore_tests = args
            .and_then(|a| a.get(mcp_tools::PARAM_IGNORE_TESTS))
            .and_then(|b| b.as_bool())
            .unwrap_or(false);
        let ignore_mocks = args
            .and_then(|a| a.get(mcp_tools::PARAM_IGNORE_MOCKS))
            .and_then(|b| b.as_bool())
            .unwrap_or(false);
        let ignore_generated = args
            .and_then(|a| a.get(mcp_tools::PARAM_IGNORE_GENERATED))
            .and_then(|b| b.as_bool())
            .unwrap_or(true);

        let engine = if let Some(custom_p) = custom_ignore {
            match SuppressionEngine::from_file(
                Path::new(custom_p),
                ignore_tests,
                ignore_mocks,
                ignore_generated,
            ) {
                Ok(eng) => eng,
                Err(err) => {
                    return make_error_response(id, rpc_errors::INTERNAL_ERROR, err);
                }
            }
        } else if Path::new(".cddmignore").exists() {
            match SuppressionEngine::from_file(
                Path::new(".cddmignore"),
                ignore_tests,
                ignore_mocks,
                ignore_generated,
            ) {
                Ok(eng) => eng,
                Err(err) => {
                    return make_error_response(id, rpc_errors::INTERNAL_ERROR, err);
                }
            }
        } else {
            SuppressionEngine::with_options(ignore_tests, ignore_mocks, ignore_generated)
        };

        let file_content = std::fs::read_to_string(path).ok();
        let path_ignored = engine.is_path_ignored(Path::new(path), file_content.as_deref());
        let mut line_ignored = false;

        if let Some(target_line) = line_opt
            && let Some(ref text) = file_content
        {
            let mut eng = engine.clone();
            eng.register_file_directives(path, text);
            line_ignored = eng.is_span_suppressed(path, target_line, target_line);
        }

        let res = json!({
            "path": path,
            "path_ignored": path_ignored,
            "line": line_opt,
            "line_ignored": line_ignored,
            "is_ignored": path_ignored || line_ignored,
        });

        make_text_response(id, serde_json::to_string_pretty(&res).unwrap_or_default())
    } else {
        make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Missing required 'path' parameter",
        )
    }
}

pub async fn handle_check_policies(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let dir = args
        .and_then(|a| a.get(mcp_tools::PARAM_DIRECTORY))
        .and_then(|d| d.as_str())
        .unwrap_or(DEFAULT_DIRECTORY);
    let rules_path = args
        .and_then(|a| a.get(mcp_tools::PARAM_RULES))
        .and_then(|r| r.as_str())
        .map(|s| s.to_string());
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
        rules_path: rules_path.clone(),
        enforce_policies: true,
        cross_language: false,
    };

    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    match run_scan(config, tx, cancel_flag).await {
        Ok(res) => {
            let engine = if let Some(ref p) = rules_path {
                PolicyEngine::from_file(Path::new(p)).unwrap_or_else(|_| PolicyEngine::empty())
            } else {
                let root_path = Path::new(DEFAULT_RULES_FILE);
                if root_path.exists() {
                    PolicyEngine::from_file(root_path).unwrap_or_else(|_| PolicyEngine::empty())
                } else {
                    PolicyEngine::empty()
                }
            };
            let eval = engine.evaluate(&res);
            make_text_response(id, serde_json::to_string_pretty(&eval).unwrap_or_default())
        }
        Err(err) => make_error_response(
            id,
            rpc_errors::INTERNAL_ERROR,
            format!("Scan failed during policy check: {err}"),
        ),
    }
}
