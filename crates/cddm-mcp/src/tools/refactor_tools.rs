#![forbid(unsafe_code)]

use super::helpers::{parse_clone_pair_args, run_scan_from_mcp_args};
use crate::protocol::{
    JsonRpcResponse, make_error_response, make_text_response, mcp_tools, rpc_errors,
};
use cddm_core::{
    AiOccurrenceContext, AiRefactorPromptRequest, CloneLocation, CloneType, LineSpan,
    analyze_clone_refactoring, analyze_cluster_refactoring, apply_cluster_refactor_branch,
    generate_ai_refactor_prompt, generate_ast_cluster_refactor, verify_refactor_test_suite,
};
use std::path::Path;

pub fn handle_suggest_refactor(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    if let Some((fa, sa, ea, fb, sb, eb)) = parse_clone_pair_args(args) {
        match analyze_clone_refactoring(fa, (sa, ea), fb, (sb, eb)) {
            Ok(suggestion) => make_text_response(
                id,
                serde_json::to_string_pretty(&suggestion).unwrap_or_default(),
            ),
            Err(e) => make_error_response(id, rpc_errors::INVALID_PARAMS, e),
        }
    } else {
        make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Missing required line range parameters",
        )
    }
}

pub async fn handle_suggest_cluster_refactor(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let cluster_id_opt = args
        .and_then(|a| a.get(mcp_tools::PARAM_CLUSTER_ID))
        .and_then(|cid| cid.as_u64())
        .map(|cid| cid as usize);

    let explicit_occs = args
        .and_then(|a| a.get(mcp_tools::PARAM_OCCURRENCES))
        .and_then(|o| o.as_array());

    if let Some(occs_arr) = explicit_occs {
        let mut parsed_occs = Vec::new();
        for item in occs_arr {
            if let (Some(file), Some(start), Some(end)) = (
                item.get("file").and_then(|f| f.as_str()),
                item.get("start_line").and_then(|s| s.as_u64()),
                item.get("end_line").and_then(|e| e.as_u64()),
            ) {
                parsed_occs.push(CloneLocation {
                    file: file.to_string(),
                    start_line: start as usize,
                    end_line: end as usize,
                    author: None,
                });
            }
        }

        if parsed_occs.len() < 2 {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "At least 2 occurrence locations required for cluster refactoring",
            );
        }

        match analyze_cluster_refactoring("cluster-custom", &parsed_occs) {
            Ok(suggestion) => make_text_response(
                id,
                serde_json::to_string_pretty(&suggestion).unwrap_or_default(),
            ),
            Err(e) => make_error_response(id, rpc_errors::INVALID_PARAMS, e),
        }
    } else if let Some(target_id) = cluster_id_opt {
        match run_scan_from_mcp_args(args, false).await {
            Ok(scan_res) => {
                let found = scan_res.clone_clusters.iter().find(|c| c.id == target_id);

                if let Some(cluster) = found {
                    match analyze_cluster_refactoring(&cluster.id.to_string(), &cluster.occurrences)
                    {
                        Ok(suggestion) => make_text_response(
                            id,
                            serde_json::to_string_pretty(&suggestion).unwrap_or_default(),
                        ),
                        Err(e) => make_error_response(id, rpc_errors::INVALID_PARAMS, e),
                    }
                } else {
                    make_error_response(
                        id,
                        rpc_errors::INVALID_PARAMS,
                        format!("Cluster #{} not found in scan results", target_id),
                    )
                }
            }
            Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        }
    } else {
        make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Must specify either 'cluster_id' or 'occurrences' parameter",
        )
    }
}

pub fn handle_apply_cluster_refactor(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let patch_str = args
        .and_then(|a| a.get(mcp_tools::PARAM_PATCH))
        .and_then(|p| p.as_str());

    if let Some(patch) = patch_str {
        let branch_name = args
            .and_then(|a| a.get(mcp_tools::PARAM_BRANCH_NAME))
            .and_then(|b| b.as_str());
        let create_branch = args
            .and_then(|a| a.get(mcp_tools::PARAM_CREATE_BRANCH))
            .and_then(|b| b.as_bool())
            .unwrap_or(false);

        match apply_cluster_refactor_branch(Path::new("."), patch, branch_name, create_branch) {
            Ok(res) => {
                make_text_response(id, serde_json::to_string_pretty(&res).unwrap_or_default())
            }
            Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        }
    } else {
        make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Missing required 'patch' parameter",
        )
    }
}

pub fn handle_generate_ai_prompt(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    if let Some(args_val) = args {
        let fn_name = args_val
            .get("function_name")
            .and_then(|v| v.as_str())
            .unwrap_or("extracted_helper");
        let target_mod = args_val
            .get("target_module")
            .and_then(|v| v.as_str())
            .unwrap_or("src/utils.rs");
        let inv_body = args_val
            .get("invariant_body")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let custom_inst = args_val
            .get("custom_instructions")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let params: Vec<String> = args_val
            .get("parameters")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let occurrences: Vec<AiOccurrenceContext> = args_val
            .get("occurrences")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|item| {
                        let path = item
                            .get("path")
                            .and_then(|p| p.as_str())
                            .unwrap_or("")
                            .to_string();
                        let s_line =
                            item.get("start_line").and_then(|l| l.as_u64()).unwrap_or(1) as usize;
                        let e_line = item
                            .get("end_line")
                            .and_then(|l| l.as_u64())
                            .unwrap_or(s_line as u64) as usize;
                        let snippet = item
                            .get("snippet")
                            .and_then(|s| s.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| {
                                let file_content =
                                    std::fs::read_to_string(&path).unwrap_or_default();
                                let lines: Vec<&str> = file_content.lines().collect();
                                if s_line > 0 && s_line <= lines.len() {
                                    let end = e_line.min(lines.len());
                                    lines[s_line - 1..end].join("\n")
                                } else {
                                    String::new()
                                }
                            });
                        AiOccurrenceContext {
                            path,
                            span: LineSpan {
                                line_start: s_line,
                                line_end: e_line,
                                byte_offset: 0,
                            },
                            snippet,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let prompt_req = AiRefactorPromptRequest {
            clone_type: CloneType::Renamed,
            similarity: 0.90,
            token_count: 100,
            lines_saved_est: occurrences.len() * 10,
            function_name: fn_name.to_string(),
            target_module: target_mod.to_string(),
            occurrences,
            invariant_body: inv_body.to_string(),
            parameters: params,
            custom_instructions: custom_inst,
        };

        let prompt_text = generate_ai_refactor_prompt(&prompt_req);
        make_text_response(id, prompt_text)
    } else {
        make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Missing arguments for cddm_generate_ai_prompt",
        )
    }
}

pub fn handle_ast_refactor(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    if let Some(args_val) = args {
        let fn_name = args_val
            .get("custom_function_name")
            .and_then(|v| v.as_str());
        let target_mod = args_val.get("target_module_path").and_then(|v| v.as_str());
        let custom_params: Option<Vec<String>> = args_val
            .get("custom_parameter_names")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                    .collect()
            });

        let occurrences: Vec<CloneLocation> = args_val
            .get("occurrences")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|item| {
                        let path = item
                            .get("path")
                            .and_then(|p| p.as_str())
                            .unwrap_or("")
                            .to_string();
                        let s_line =
                            item.get("start_line").and_then(|l| l.as_u64()).unwrap_or(1) as usize;
                        let e_line = item
                            .get("end_line")
                            .and_then(|l| l.as_u64())
                            .unwrap_or(s_line as u64) as usize;
                        CloneLocation {
                            file: path,
                            start_line: s_line,
                            end_line: e_line,
                            author: None,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        if occurrences.is_empty() {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "No occurrences provided for AST refactoring",
            );
        }

        match generate_ast_cluster_refactor(
            &occurrences,
            fn_name,
            target_mod,
            custom_params.as_deref(),
        ) {
            Ok(ast_res) => {
                let json_str = serde_json::to_string_pretty(&ast_res).unwrap_or_default();
                make_text_response(id, json_str)
            }
            Err(err) => make_error_response(id, rpc_errors::INTERNAL_ERROR, err),
        }
    } else {
        make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Missing arguments for cddm_ast_refactor",
        )
    }
}

pub fn handle_verify_refactor(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let dir_str = args
        .and_then(|a| a.get("directory"))
        .and_then(|v| v.as_str())
        .unwrap_or(".");
    let test_cmd = args
        .and_then(|a| a.get("test_command"))
        .and_then(|v| v.as_str());
    let branch = args
        .and_then(|a| a.get("branch_name"))
        .and_then(|v| v.as_str());
    let timeout = args
        .and_then(|a| a.get("timeout_seconds"))
        .and_then(|v| v.as_u64());

    match verify_refactor_test_suite(Path::new(dir_str), test_cmd, branch, timeout) {
        Ok(v_res) => {
            let json_str = serde_json::to_string_pretty(&v_res).unwrap_or_default();
            make_text_response(id, json_str)
        }
        Err(err) => make_error_response(id, rpc_errors::INTERNAL_ERROR, err),
    }
}

pub async fn handle_heal_refactor(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    if let Some(args_val) = args {
        let dir_str = args_val
            .get("directory")
            .and_then(|v| v.as_str())
            .unwrap_or(".");
        let cluster_id = args_val
            .get("cluster_id")
            .and_then(|v| v.as_u64())
            .map(|c| c as usize);
        let pair_id = args_val
            .get("pair_id")
            .and_then(|v| v.as_u64())
            .map(|p| p as usize);
        let provider_str = args_val
            .get("provider")
            .and_then(|v| v.as_str())
            .unwrap_or("mock");
        let model = args_val
            .get("model")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let api_key = args_val
            .get("api_key")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let endpoint = args_val
            .get("endpoint")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let max_iters = args_val
            .get("max_iterations")
            .and_then(|v| v.as_u64())
            .unwrap_or(3) as usize;
        let verify = args_val
            .get("verify")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let test_cmd = args_val
            .get("test_command")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let branch = args_val
            .get("branch_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let fn_name = args_val
            .get("function_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let target_mod = args_val
            .get("target_module")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let provider_kind = match provider_str.to_lowercase().as_str() {
            "gemini" => cddm_core::AiProviderKind::Gemini,
            "claude" => cddm_core::AiProviderKind::Claude,
            "openai" => cddm_core::AiProviderKind::OpenAi,
            "ollama" => cddm_core::AiProviderKind::Ollama,
            _ => cddm_core::AiProviderKind::Mock,
        };

        let req = cddm_core::HealRefactorRequest {
            cluster_id,
            pair_id,
            occurrences: Vec::new(),
            function_name: fn_name,
            target_module: target_mod,
            custom_instructions: None,
            provider_config: cddm_core::AiProviderConfig {
                provider: provider_kind,
                model,
                api_key,
                endpoint,
                temperature: Some(0.2),
                timeout_secs: Some(60),
            },
            max_iterations: max_iters,
            apply_branch: branch,
            verify,
            test_cmd,
            workspace_root: Some(Path::new(dir_str).to_path_buf()),
        };

        match cddm_core::heal_cluster_refactor(Path::new(dir_str), &req).await {
            Ok(heal_res) => {
                let json_str = serde_json::to_string_pretty(&heal_res).unwrap_or_default();
                make_text_response(id, json_str)
            }
            Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
        }
    } else {
        make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Missing arguments for cddm_heal_refactor",
        )
    }
}
