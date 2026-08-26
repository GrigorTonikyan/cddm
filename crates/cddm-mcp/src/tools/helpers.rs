#![forbid(unsafe_code)]

use crate::protocol::mcp_tools;
use cddm_core::{DEFAULT_DIRECTORY, DEFAULT_MIN_TOKENS, ScanConfig, ScanResult, run_scan};
use serde_json::json;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

pub fn clone_pair_input_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            mcp_tools::PARAM_FILE_A: {
                "type": "string",
                "description": "File path of fragment A"
            },
            mcp_tools::PARAM_START_LINE_A: {
                "type": "number",
                "description": "1-based start line of fragment A"
            },
            mcp_tools::PARAM_END_LINE_A: {
                "type": "number",
                "description": "1-based end line of fragment A"
            },
            mcp_tools::PARAM_FILE_B: {
                "type": "string",
                "description": "File path of fragment B"
            },
            mcp_tools::PARAM_START_LINE_B: {
                "type": "number",
                "description": "1-based start line of fragment B"
            },
            mcp_tools::PARAM_END_LINE_B: {
                "type": "number",
                "description": "1-based end line of fragment B"
            }
        },
        "required": [
            mcp_tools::PARAM_FILE_A,
            mcp_tools::PARAM_START_LINE_A,
            mcp_tools::PARAM_END_LINE_A,
            mcp_tools::PARAM_FILE_B,
            mcp_tools::PARAM_START_LINE_B,
            mcp_tools::PARAM_END_LINE_B
        ]
    })
}

pub fn parse_clone_pair_args(
    args: Option<&serde_json::Value>,
) -> Option<(&str, usize, usize, &str, usize, usize)> {
    let a = args?;
    Some((
        a.get(mcp_tools::PARAM_FILE_A)?.as_str()?,
        a.get(mcp_tools::PARAM_START_LINE_A)?.as_u64()? as usize,
        a.get(mcp_tools::PARAM_END_LINE_A)?.as_u64()? as usize,
        a.get(mcp_tools::PARAM_FILE_B)?.as_str()?,
        a.get(mcp_tools::PARAM_START_LINE_B)?.as_u64()? as usize,
        a.get(mcp_tools::PARAM_END_LINE_B)?.as_u64()? as usize,
    ))
}

pub fn parse_dir_and_tokens(args: Option<&serde_json::Value>) -> (&str, usize) {
    let dir = args
        .and_then(|a| a.get(mcp_tools::PARAM_DIRECTORY))
        .and_then(|d| d.as_str())
        .unwrap_or(DEFAULT_DIRECTORY);
    let min_tokens = args
        .and_then(|a| a.get(mcp_tools::PARAM_MIN_TOKENS))
        .and_then(|t| t.as_u64())
        .unwrap_or(DEFAULT_MIN_TOKENS as u64) as usize;
    (dir, min_tokens)
}

pub async fn run_scan_from_mcp_args(
    args: Option<&serde_json::Value>,
    enable_git_blame: bool,
) -> Result<ScanResult, String> {
    let (dir, min_tokens) = parse_dir_and_tokens(args);
    let cross_language = args
        .and_then(|a| a.get("cross_language"))
        .and_then(|b| b.as_bool())
        .unwrap_or(false);

    let config = ScanConfig {
        directory: dir.to_string(),
        min_tokens,
        enable_git_blame,
        cross_language,
        ..Default::default()
    };

    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    run_scan(config, tx, cancel_flag).await
}
