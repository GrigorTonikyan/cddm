#![forbid(unsafe_code)]

use super::helpers::{get_bool_arg, get_str_arg, get_usize_arg};
use crate::protocol::{
    JsonRpcResponse, make_error_response, make_text_response, mcp_tools, rpc_errors,
};
use cddm_core::{
    PolicyEngine, PolicyEvaluationOptions, ScanConfig, SuppressionEngine,
    evaluate_dead_code_policy, run_scan,
};
use serde_json::json;
use std::path::Path;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

pub fn handle_check_suppression(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    if let Some(path) = get_str_arg(args, mcp_tools::PARAM_PATH) {
        let line_opt = get_usize_arg(args, mcp_tools::PARAM_LINE);
        let custom_ignore = get_str_arg(args, mcp_tools::PARAM_CDDMIGNORE);
        let ignore_tests = get_bool_arg(args, mcp_tools::PARAM_IGNORE_TESTS, false);
        let ignore_mocks = get_bool_arg(args, mcp_tools::PARAM_IGNORE_MOCKS, false);
        let ignore_generated = get_bool_arg(args, mcp_tools::PARAM_IGNORE_GENERATED, true);

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
    let (dir, min_tokens) = crate::tools::helpers::parse_dir_and_tokens(args);
    let rules_path = args
        .and_then(|a| a.get(mcp_tools::PARAM_RULES))
        .and_then(|r| r.as_str())
        .map(|s| s.to_string());

    let dir_path = Path::new(dir);
    let engine = PolicyEngine::discover(dir_path, rules_path.as_deref().map(Path::new));

    let explicit_ignore_tests = args
        .and_then(|a| a.get(mcp_tools::PARAM_IGNORE_TESTS))
        .and_then(|v| v.as_bool());
    let explicit_ignore_mocks = args
        .and_then(|a| a.get(mcp_tools::PARAM_IGNORE_MOCKS))
        .and_then(|v| v.as_bool());
    let explicit_include_ignored = args
        .and_then(|a| a.get(mcp_tools::PARAM_INCLUDE_IGNORED))
        .and_then(|v| v.as_bool());
    let ignore_generated = get_bool_arg(args, mcp_tools::PARAM_IGNORE_GENERATED, true);

    let effective_ignore_tests = explicit_ignore_tests
        .or(engine.config().effective_ignore_tests())
        .unwrap_or(true);
    let effective_ignore_mocks = explicit_ignore_mocks
        .or(engine.config().effective_ignore_mocks())
        .unwrap_or(true);
    let effective_include_ignored = explicit_include_ignored
        .or(engine.config().include_ignored)
        .unwrap_or(false);

    let mut config = ScanConfig {
        directory: dir.to_string(),
        min_tokens,
        rules_path: rules_path.clone(),
        enforce_policies: true,
        ignore_tests: effective_ignore_tests,
        ignore_mocks: effective_ignore_mocks,
        ignore_generated,
        include_ignored: effective_include_ignored,
        ..Default::default()
    };
    if let Some(ref scan_policy) = engine.config().scan {
        scan_policy.apply_to(&mut config);
    }

    let eval_opts = PolicyEvaluationOptions {
        ignore_tests: effective_ignore_tests,
        ignore_mocks: effective_ignore_mocks,
        include_ignored: effective_include_ignored,
        force_ignore_tests: explicit_ignore_tests,
        force_ignore_mocks: explicit_ignore_mocks,
        force_include_ignored: explicit_include_ignored,
    };

    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    match run_scan(config, tx, cancel_flag).await {
        Ok(res) => {
            let mut eval = engine.evaluate_with_options(&res, &eval_opts);
            match evaluate_dead_code_policy(dir, engine.config(), effective_include_ignored).await {
                Ok(dead_violations) => eval.push_violations(dead_violations),
                Err(err) => {
                    return make_error_response(
                        id,
                        rpc_errors::INTERNAL_ERROR,
                        format!("Dead code policy check failed during policy evaluation: {err}"),
                    );
                }
            }
            if crate::tools::helpers::is_summary_requested(args) {
                let compact = cddm_core::CompactPolicyResult::from_evaluation(&eval, 5);
                crate::tools::helpers::make_json_payload_response(id, &compact)
            } else {
                make_text_response(id, serde_json::to_string_pretty(&eval).unwrap_or_default())
            }
        }
        Err(err) => make_error_response(
            id,
            rpc_errors::INTERNAL_ERROR,
            format!("Scan failed during policy check: {err}"),
        ),
    }
}
