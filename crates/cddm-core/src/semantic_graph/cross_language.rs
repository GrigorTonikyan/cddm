#![forbid(unsafe_code)]

use super::cfg::extract_cfgs_from_source;
use super::embedding::compute_hybrid_similarity;
use super::types::{ControlFlowGraph, CrossLanguageClonePair};
use crate::detector::discovery::discover_candidate_files;
use crate::detector::discovery::init_suppression_engine;
use crate::grammar::get_grammar_for_path;
use crate::types::{CloneType, ScanConfig};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

/// Extracts all function CFGs with source code snippets across a set of workspace files.
pub fn extract_workspace_cfgs(
    file_paths: &[std::path::PathBuf],
) -> Vec<(ControlFlowGraph, String, String)> {
    let mut results = Vec::new();

    for path in file_paths {
        let grammar = match get_grammar_for_path(path) {
            Some(g) => g,
            None => continue,
        };

        let content = match crate::io::read_file_source(path) {
            Ok(c) => c.to_string(),
            Err(_) => continue,
        };

        let path_str = path.to_string_lossy().to_string();
        let cfgs = extract_cfgs_from_source(&path_str, &content, grammar.name);
        let lines: Vec<&str> = content.lines().collect();

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
                results.push((cfg, snippet, grammar.name.to_string()));
            }
        }
    }

    results
}

/// Scans workspace files for cross-language semantic clone pairs across different programming languages.
pub fn scan_cross_language_workspace(
    config: &ScanConfig,
    similarity_threshold: f64,
) -> Result<Vec<CrossLanguageClonePair>, String> {
    let suppression_engine = init_suppression_engine(config);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    let file_paths = discover_candidate_files(config, &suppression_engine, &cancel_flag)?;
    let extracted = extract_workspace_cfgs(&file_paths);

    let mut clone_pairs = Vec::new();

    for i in 0..extracted.len() {
        for j in (i + 1)..extracted.len() {
            let (cfg_a, snippet_a, lang_a) = &extracted[i];
            let (cfg_b, snippet_b, lang_b) = &extracted[j];

            // Only compare across different programming languages
            if lang_a == lang_b {
                continue;
            }

            // Quick node size ratio check (skip if wildly different)
            let len_a = cfg_a.nodes.len();
            let len_b = cfg_b.nodes.len();
            let max_len = len_a.max(len_b);
            let min_len = len_a.min(len_b);
            if (min_len as f64) / (max_len as f64) < 0.25 {
                continue;
            }

            let hybrid = compute_hybrid_similarity(cfg_a, snippet_a, cfg_b, snippet_b, true);

            if hybrid.hybrid_score >= similarity_threshold {
                clone_pairs.push(CrossLanguageClonePair {
                    file_a: cfg_a.file_path.clone(),
                    language_a: lang_a.clone(),
                    function_a: cfg_a.function_name.clone(),
                    lines_a: (cfg_a.line_start, cfg_a.line_end),
                    file_b: cfg_b.file_path.clone(),
                    language_b: lang_b.clone(),
                    function_b: cfg_b.function_name.clone(),
                    lines_b: (cfg_b.line_start, cfg_b.line_end),
                    graph_similarity: hybrid.graph_similarity,
                    token_similarity: hybrid.token_similarity,
                    hybrid_score: hybrid.hybrid_score,
                    clone_type: CloneType::Semantic,
                });
            }
        }
    }

    // Sort by hybrid score descending
    clone_pairs.sort_by(|a, b| {
        b.hybrid_score
            .partial_cmp(&a.hybrid_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(clone_pairs)
}
