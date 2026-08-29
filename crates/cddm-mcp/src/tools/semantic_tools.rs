#![forbid(unsafe_code)]

use crate::protocol::{JsonRpcResponse, make_error_response, make_text_response, rpc_errors};
use cddm_core::grammar::get_grammar_for_path;
use cddm_core::semantic_graph::{build_pdg_from_cfg, extract_cfgs_from_source};
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
    let default_lang = args_obj
        .get("language")
        .and_then(|v| v.as_str())
        .unwrap_or("Rust");
    let lang_a = args_obj
        .get("language_a")
        .and_then(|v| v.as_str())
        .unwrap_or(default_lang);
    let lang_b = args_obj
        .get("language_b")
        .and_then(|v| v.as_str())
        .unwrap_or(default_lang);

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

    let cfgs_a = extract_cfgs_from_source("a.rs", src_a, lang_a);
    let cfgs_b = extract_cfgs_from_source("b.rs", src_b, lang_b);

    if cfgs_a.is_empty() || cfgs_b.is_empty() {
        return make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            "Could not extract Control Flow Graphs from one or both source inputs",
        );
    }

    let is_cross_lang = lang_a != lang_b;
    let hybrid = cddm_core::semantic_graph::compute_hybrid_similarity(
        &cfgs_a[0],
        src_a,
        &cfgs_b[0],
        src_b,
        is_cross_lang,
    );
    let is_semantic_clone = hybrid.hybrid_score >= 0.70;

    let payload = json!({
        "similarity": hybrid.hybrid_score,
        "graph_similarity": hybrid.graph_similarity,
        "token_similarity": hybrid.token_similarity,
        "hybrid_score": hybrid.hybrid_score,
        "is_semantic_clone": is_semantic_clone,
        "is_cross_language": is_cross_lang,
        "language_a": lang_a,
        "language_b": lang_b,
        "function_a": cfgs_a[0].function_name,
        "function_b": cfgs_b[0].function_name,
        "wl_hash_a": cfgs_a[0].wl_hash,
        "wl_hash_b": cfgs_b[0].wl_hash,
        "nodes_a_count": cfgs_a[0].nodes.len(),
        "nodes_b_count": cfgs_b[0].nodes.len(),
    });

    make_text_response(
        id,
        serde_json::to_string_pretty(&payload).unwrap_or_default(),
    )
}

/// Handler for `cddm_scan_cross_language` MCP tool.
pub fn handle_scan_cross_language(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let args_obj = match args {
        Some(serde_json::Value::Object(map)) => map,
        _ => {
            return make_error_response(
                id,
                rpc_errors::INVALID_PARAMS,
                "Missing arguments object for 'cddm_scan_cross_language'",
            );
        }
    };

    let dir = args_obj
        .get("directory")
        .and_then(|v| v.as_str())
        .unwrap_or(cddm_core::DEFAULT_DIRECTORY);
    let threshold = args_obj
        .get("threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.70);
    let min_tokens = args_obj
        .get("min_tokens")
        .and_then(|v| v.as_u64())
        .unwrap_or(cddm_core::DEFAULT_MIN_TOKENS as u64) as usize;

    let languages = args_obj
        .get("languages")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let ignore_patterns = args_obj
        .get("ignore")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_else(|| {
            cddm_core::ScanConfig::default()
                .ignore_patterns
                .into_iter()
                .collect()
        });

    let threads = args_obj
        .get("threads")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize);

    let config = cddm_core::ScanConfig {
        directory: dir.to_string(),
        min_tokens,
        languages,
        ignore_patterns,
        detect_type2: true,
        detect_type3: true,
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
        cross_language: true,
        threads,
    };

    match cddm_core::semantic_graph::scan_cross_language_workspace(&config, threshold) {
        Ok(pairs) => {
            let payload = json!({
                "directory": dir,
                "threshold": threshold,
                "total_pairs": pairs.len(),
                "pairs": pairs,
            });
            make_text_response(
                id,
                serde_json::to_string_pretty(&payload).unwrap_or_default(),
            )
        }
        Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
    }
}

/// Handler for `cddm_semantic_neural_scan` MCP tool.
pub fn handle_semantic_neural_scan(
    id: Option<serde_json::Value>,
    args: Option<&serde_json::Value>,
) -> JsonRpcResponse {
    let empty_map = serde_json::Map::new();
    let args_obj = args.and_then(|v| v.as_object()).unwrap_or(&empty_map);

    let threshold = args_obj
        .get("threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.85) as f32;
    let dimension = args_obj
        .get("dimension")
        .and_then(|v| v.as_u64())
        .unwrap_or(256) as usize;
    let neural_config = cddm_core::NeuralEmbeddingConfig {
        dimension,
        similarity_threshold: threshold,
        max_subwords: 512,
    };

    // Case 1: Direct pairwise code comparison
    if let (Some(code_a), Some(code_b)) = (
        args_obj.get("code_a").and_then(|v| v.as_str()),
        args_obj.get("code_b").and_then(|v| v.as_str()),
    ) {
        let lang_a = args_obj
            .get("language_a")
            .and_then(|v| v.as_str())
            .unwrap_or("rs");
        let lang_b = args_obj
            .get("language_b")
            .and_then(|v| v.as_str())
            .unwrap_or("rs");

        let sim =
            cddm_core::compare_code_embeddings(code_a, lang_a, code_b, lang_b, &neural_config);

        let payload = json!({
            "cosine_similarity": sim,
            "is_equivalent": sim >= threshold,
            "threshold": threshold,
            "language_a": lang_a,
            "language_b": lang_b,
        });

        return make_text_response(
            id,
            serde_json::to_string_pretty(&payload).unwrap_or_default(),
        );
    }

    // Case 2: Workspace directory scan
    let dir = args_obj
        .get("directory")
        .and_then(|v| v.as_str())
        .unwrap_or(cddm_core::DEFAULT_DIRECTORY);

    let path = Path::new(dir);
    if !path.exists() {
        return make_error_response(
            id,
            rpc_errors::INVALID_PARAMS,
            format!("Target directory '{}' does not exist", dir),
        );
    }

    match cddm_core::scan_neural_clones(path, &neural_config) {
        Ok(result) => {
            let payload = json!({
                "directory": dir,
                "threshold": threshold,
                "dimension": dimension,
                "total_blocks_embedded": result.total_blocks_embedded,
                "total_neural_pairs": result.total_neural_pairs,
                "high_confidence_count": result.high_confidence_count,
                "pairs": result.pairs,
            });
            make_text_response(
                id,
                serde_json::to_string_pretty(&payload).unwrap_or_default(),
            )
        }
        Err(e) => make_error_response(id, rpc_errors::INTERNAL_ERROR, e),
    }
}
