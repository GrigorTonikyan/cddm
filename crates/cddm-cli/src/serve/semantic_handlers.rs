#![forbid(unsafe_code)]

use super::scan_handlers::resolve_safe_path;
use super::types::*;
use axum::{extract::Json, http::StatusCode};
use cddm_core::grammar::get_grammar_for_path;
use cddm_core::semantic_graph::{
    CrossLanguageClonePair, build_pdg_from_cfg, compute_hybrid_similarity,
    extract_cfgs_from_source, scan_cross_language_workspace,
};
use cddm_core::{DEFAULT_DIRECTORY, DEFAULT_MIN_TOKENS, ScanConfig};
use std::fs;
use std::path::Path;

/// Resolves code content and detected language from request fields or filesystem.
fn resolve_code_and_language(
    file_opt: Option<&str>,
    code_opt: Option<&str>,
    lang_opt: Option<&str>,
) -> Result<(String, String, String), (StatusCode, String)> {
    if let Some(code) = code_opt {
        let path = file_opt.unwrap_or("snippet.rs").to_string();
        let lang = if let Some(l) = lang_opt {
            l.to_string()
        } else if let Some(file_str) = file_opt {
            get_grammar_for_path(Path::new(file_str))
                .map(|g| g.name.to_string())
                .unwrap_or_else(|| "Rust".to_string())
        } else {
            "Rust".to_string()
        };
        return Ok((path, code.to_string(), lang));
    }

    if let Some(file_str) = file_opt {
        let canonical = resolve_safe_path(file_str)?;
        let content = fs::read_to_string(&canonical).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read file '{}': {}", file_str, e),
            )
        })?;
        let lang = if let Some(l) = lang_opt {
            l.to_string()
        } else {
            get_grammar_for_path(&canonical)
                .map(|g| g.name.to_string())
                .unwrap_or_else(|| "Rust".to_string())
        };
        return Ok((file_str.to_string(), content, lang));
    }

    Err((
        StatusCode::BAD_REQUEST,
        "Either 'code' or 'file' must be provided".to_string(),
    ))
}

/// Handler for `POST /api/semantic-graph` extracting CFGs, PDGs, and computing graph similarities.
pub async fn semantic_graph_handler(
    Json(req): Json<SemanticGraphRequest>,
) -> Result<Json<SemanticGraphResponse>, (StatusCode, String)> {
    let (path_a, code_a, lang_a) = resolve_code_and_language(
        req.file.as_deref(),
        req.code.as_deref(),
        req.language.as_deref(),
    )?;

    let mut cfgs = extract_cfgs_from_source(&path_a, &code_a, &lang_a);
    let mut pdgs = Vec::with_capacity(cfgs.len());
    for cfg in &cfgs {
        pdgs.push(build_pdg_from_cfg(cfg.clone()));
    }

    let mut comparison = None;

    // If fragment B is provided, extract CFGs/PDGs for B and compute hybrid similarity
    if req.code_b.is_some() || req.file_b.is_some() {
        let (path_b, code_b, lang_b) = resolve_code_and_language(
            req.file_b.as_deref(),
            req.code_b.as_deref(),
            req.language_b.as_deref(),
        )?;

        let cfgs_b = extract_cfgs_from_source(&path_b, &code_b, &lang_b);
        for cfg_b in &cfgs_b {
            pdgs.push(build_pdg_from_cfg(cfg_b.clone()));
        }

        let target_a = req
            .function_a
            .as_ref()
            .and_then(|name| cfgs.iter().find(|c| c.function_name == *name))
            .or_else(|| {
                req.lines_a
                    .as_ref()
                    .and_then(|(s, _)| cfgs.iter().find(|c| c.line_start == *s))
            })
            .or_else(|| cfgs.first());

        let target_b = req
            .function_b
            .as_ref()
            .and_then(|name| cfgs_b.iter().find(|c| c.function_name == *name))
            .or_else(|| {
                req.lines_b
                    .as_ref()
                    .and_then(|(s, _)| cfgs_b.iter().find(|c| c.line_start == *s))
            })
            .or_else(|| cfgs_b.first());

        if let (Some(selected_a), Some(selected_b)) = (target_a, target_b) {
            let is_cross_lang = lang_a != lang_b;

            let lines_a: Vec<&str> = code_a.lines().collect();
            let start_a = selected_a.line_start.saturating_sub(1);
            let end_a = selected_a.line_end.min(lines_a.len());
            let snippet_a = if start_a < end_a && start_a < lines_a.len() {
                lines_a[start_a..end_a].join("\n")
            } else {
                code_a.clone()
            };

            let lines_b: Vec<&str> = code_b.lines().collect();
            let start_b = selected_b.line_start.saturating_sub(1);
            let end_b = selected_b.line_end.min(lines_b.len());
            let snippet_b = if start_b < end_b && start_b < lines_b.len() {
                lines_b[start_b..end_b].join("\n")
            } else {
                code_b.clone()
            };

            let hybrid = compute_hybrid_similarity(
                selected_a,
                &snippet_a,
                selected_b,
                &snippet_b,
                is_cross_lang,
            );
            let is_semantic_clone = hybrid.hybrid_score >= 0.70;
            comparison = Some(SemanticComparisonResponse {
                similarity: hybrid.hybrid_score,
                graph_similarity: hybrid.graph_similarity,
                token_similarity: hybrid.token_similarity,
                hybrid_score: hybrid.hybrid_score,
                is_semantic_clone,
                is_cross_language: is_cross_lang,
                wl_hash_a: selected_a.wl_hash,
                wl_hash_b: selected_b.wl_hash,
            });
        }

        cfgs.extend(cfgs_b);
    }

    Ok(Json(SemanticGraphResponse {
        cfgs,
        pdgs,
        comparison,
    }))
}

/// Handler for `POST /api/semantic/scan` executing on-demand cross-language clone discovery.
pub async fn semantic_scan_handler(
    Json(req): Json<SemanticScanRequest>,
) -> Result<Json<Vec<CrossLanguageClonePair>>, (StatusCode, String)> {
    let dir = req
        .directory
        .unwrap_or_else(|| DEFAULT_DIRECTORY.to_string());
    let threshold = req.threshold.unwrap_or(0.70);
    let min_tokens = req.min_tokens.unwrap_or(DEFAULT_MIN_TOKENS);
    let languages = req.languages.unwrap_or_default();
    let ignore_patterns = req
        .ignore
        .unwrap_or_else(|| ScanConfig::default().ignore_patterns.into_iter().collect());

    let config = ScanConfig {
        directory: dir,
        min_tokens,
        languages,
        ignore_patterns,
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
        cross_language: true,
        threads: req.threads,
    };

    scan_cross_language_workspace(&config, threshold)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

/// Handler for `POST /api/semantic/neural` executing in-process neural code embedding equivalence scan.
pub async fn semantic_neural_handler(
    Json(req): Json<SemanticNeuralRequest>,
) -> Result<Json<cddm_core::NeuralScanResult>, (StatusCode, String)> {
    let dir = req
        .directory
        .unwrap_or_else(|| DEFAULT_DIRECTORY.to_string());
    let path = Path::new(&dir);
    if !path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Directory '{}' does not exist", dir),
        ));
    }

    let config = cddm_core::NeuralEmbeddingConfig {
        dimension: req.dimension.unwrap_or(256),
        similarity_threshold: req.threshold.unwrap_or(0.85),
        max_subwords: req.max_subwords.unwrap_or(512),
    };

    cddm_core::scan_neural_clones(path, &config)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
