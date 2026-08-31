#![forbid(unsafe_code)]

use super::cfg::extract_cfgs_from_source;
use super::embedding::{
    SparseTfVector, compute_hybrid_similarity_with_tf, extract_semantic_tokens,
};
use super::types::{ControlFlowGraph, CrossLanguageClonePair};
use crate::detector::discovery::discover_candidate_files;
use crate::detector::discovery::init_suppression_engine;
use crate::grammar::get_grammar_for_path;
use crate::types::{CloneType, ScanConfig};
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Represents an extracted function CFG with pre-computed term-frequency sparse vector.
#[derive(Clone, Debug)]
pub struct ExtractedCfgItem {
    pub cfg: ControlFlowGraph,
    pub snippet: String,
    pub language: String,
    pub tf_vector: SparseTfVector,
}

/// Extracts all function CFGs with pre-computed sparse TF vectors in parallel across all CPU cores.
pub fn extract_workspace_cfgs_parallel(file_paths: &[std::path::PathBuf]) -> Vec<ExtractedCfgItem> {
    file_paths
        .par_iter()
        .flat_map(|path| {
            let grammar = match get_grammar_for_path(path) {
                Some(g) => g,
                None => return Vec::new(),
            };

            let content = match crate::io::read_file_source(path) {
                Ok(c) => c.to_string(),
                Err(_) => return Vec::new(),
            };

            let path_str = path.to_string_lossy().to_string();
            let cfgs = extract_cfgs_from_source(&path_str, &content, grammar.name);
            let lines: Vec<&str> = content.lines().collect();

            let mut results = Vec::new();
            for cfg in cfgs {
                // Only consider non-trivial functions (>= 3 statements/nodes)
                if cfg.nodes.len() >= 3 {
                    let start_idx = cfg.line_start.saturating_sub(1);
                    let end_idx = cfg.line_end.min(lines.len());
                    let snippet = if start_idx < end_idx && start_idx < lines.len() {
                        lines[start_idx..end_idx].join("\n")
                    } else {
                        content.clone()
                    };
                    let tokens = extract_semantic_tokens(&snippet);
                    let tf_vector = SparseTfVector::from_tokens(&tokens);
                    results.push(ExtractedCfgItem {
                        cfg,
                        snippet,
                        language: grammar.name.to_string(),
                        tf_vector,
                    });
                }
            }
            results
        })
        .collect()
}

/// Extracts all function CFGs with source code snippets across workspace files.
pub fn extract_workspace_cfgs(
    file_paths: &[std::path::PathBuf],
) -> Vec<(ControlFlowGraph, String, String)> {
    extract_workspace_cfgs_parallel(file_paths)
        .into_iter()
        .map(|item| (item.cfg, item.snippet, item.language))
        .collect()
}

/// Scans workspace files for semantic clone pairs with optional same-language and cross-language flags and progress callback.
pub fn scan_semantic_workspace_with_progress<F>(
    config: &ScanConfig,
    similarity_threshold: f64,
    allow_same_language: bool,
    allow_cross_language: bool,
    progress_callback: Option<F>,
) -> Result<Vec<CrossLanguageClonePair>, String>
where
    F: Fn(usize, usize, &str) + Send + Sync,
{
    let suppression_engine = init_suppression_engine(config);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    let file_paths = discover_candidate_files(config, &suppression_engine, &cancel_flag)?;
    let extracted = extract_workspace_cfgs_parallel(&file_paths);

    let mut candidate_pairs = Vec::new();
    for i in 0..extracted.len() {
        for j in (i + 1)..extracted.len() {
            let item_a = &extracted[i];
            let item_b = &extracted[j];

            let is_cross = item_a.language != item_b.language;
            if is_cross && !allow_cross_language {
                continue;
            }
            if !is_cross && !allow_same_language {
                continue;
            }

            // Skip identical function span in the same file to prevent self-cloning
            if item_a.cfg.file_path == item_b.cfg.file_path
                && item_a.cfg.line_start == item_b.cfg.line_start
                && item_a.cfg.line_end == item_b.cfg.line_end
            {
                continue;
            }

            // If same file and scan_self is disabled, skip
            if item_a.cfg.file_path == item_b.cfg.file_path && !config.scan_self {
                continue;
            }

            let len_a = item_a.cfg.nodes.len();
            let len_b = item_b.cfg.nodes.len();
            let max_len = len_a.max(len_b);
            let min_len = len_a.min(len_b);
            if (min_len as f64) / (max_len as f64) < 0.25 {
                continue;
            }

            candidate_pairs.push((i, j, is_cross));
        }
    }

    let total_candidates = candidate_pairs.len();
    if total_candidates == 0 {
        return Ok(Vec::new());
    }

    let evaluated_count = AtomicUsize::new(0);

    let mut clone_pairs: Vec<CrossLanguageClonePair> = candidate_pairs
        .par_iter()
        .filter_map(|&(i, j, is_cross)| {
            let item_a = &extracted[i];
            let item_b = &extracted[j];

            let hybrid = compute_hybrid_similarity_with_tf(
                &item_a.cfg,
                &item_a.tf_vector,
                &item_b.cfg,
                &item_b.tf_vector,
                is_cross,
            );

            let current = evaluated_count.fetch_add(1, Ordering::Relaxed) + 1;
            if let Some(ref cb) = progress_callback
                && (current.is_multiple_of(32) || current == total_candidates)
            {
                cb(
                    current,
                    total_candidates,
                    if is_cross {
                        "Evaluating cross-language semantic pairs..."
                    } else {
                        "Evaluating semantic AST/CFG pairs..."
                    },
                );
            }

            if hybrid.hybrid_score >= similarity_threshold {
                Some(CrossLanguageClonePair {
                    file_a: item_a.cfg.file_path.clone(),
                    language_a: item_a.language.clone(),
                    function_a: item_a.cfg.function_name.clone(),
                    lines_a: (item_a.cfg.line_start, item_a.cfg.line_end),
                    file_b: item_b.cfg.file_path.clone(),
                    language_b: item_b.language.clone(),
                    function_b: item_b.cfg.function_name.clone(),
                    lines_b: (item_b.cfg.line_start, item_b.cfg.line_end),
                    graph_similarity: hybrid.graph_similarity,
                    token_similarity: hybrid.token_similarity,
                    hybrid_score: hybrid.hybrid_score,
                    clone_type: CloneType::Semantic,
                })
            } else {
                None
            }
        })
        .collect();

    // Sort by hybrid score descending
    clone_pairs.sort_by(|a, b| {
        b.hybrid_score
            .partial_cmp(&a.hybrid_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(clone_pairs)
}

/// Scans workspace files for cross-language semantic clone pairs with optional fine-grained progress callback.
pub fn scan_cross_language_workspace_with_progress<F>(
    config: &ScanConfig,
    similarity_threshold: f64,
    progress_callback: Option<F>,
) -> Result<Vec<CrossLanguageClonePair>, String>
where
    F: Fn(usize, usize, &str) + Send + Sync,
{
    scan_semantic_workspace_with_progress(
        config,
        similarity_threshold,
        false,
        true,
        progress_callback,
    )
}

/// Scans workspace files for semantic clone pairs (both same-language and cross-language).
pub fn scan_semantic_workspace(
    config: &ScanConfig,
    similarity_threshold: f64,
    allow_same_language: bool,
    allow_cross_language: bool,
) -> Result<Vec<CrossLanguageClonePair>, String> {
    scan_semantic_workspace_with_progress::<fn(usize, usize, &str)>(
        config,
        similarity_threshold,
        allow_same_language,
        allow_cross_language,
        None,
    )
}

/// Scans workspace files for cross-language semantic clone pairs across different programming languages.
pub fn scan_cross_language_workspace(
    config: &ScanConfig,
    similarity_threshold: f64,
) -> Result<Vec<CrossLanguageClonePair>, String> {
    scan_cross_language_workspace_with_progress::<fn(usize, usize, &str)>(
        config,
        similarity_threshold,
        None,
    )
}
