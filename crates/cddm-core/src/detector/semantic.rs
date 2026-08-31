#![forbid(unsafe_code)]

use super::types::{ParsedFile, count_tokens_in_line_span};
use crate::semantic_graph::scan_semantic_workspace_with_progress;
use crate::suppression::SuppressionEngine;
use crate::types::{ClonePair, CloneType, ScanConfig};
use std::path::Path;

/// Minimum semantic similarity threshold for Type-4 clone inclusion.
pub const DEFAULT_SEMANTIC_MATCH_THRESHOLD: f64 = 0.75;

/// Evaluates semantic (CFG and Weisfeiler-Lehman) clone pairs and merges non-overlapping pairs into the result.
pub fn evaluate_and_merge_semantic_clones<F>(
    pairs: &mut Vec<ClonePair>,
    parsed_files: &[ParsedFile],
    config: &ScanConfig,
    suppression_engine: &SuppressionEngine,
    progress_callback: Option<F>,
) where
    F: Fn(usize, usize, &str) + Send + Sync,
{
    if !config.detect_type4 && !config.cross_language {
        return;
    }

    let allow_same = config.detect_type4;
    let allow_cross = config.cross_language;

    let scan_res = scan_semantic_workspace_with_progress(
        config,
        DEFAULT_SEMANTIC_MATCH_THRESHOLD,
        allow_same,
        allow_cross,
        progress_callback,
    );

    let cross_pairs = match scan_res {
        Ok(p) => p,
        Err(_) => return,
    };

    for cp in cross_pairs {
        let path_a = Path::new(&cp.file_a);
        let path_b = Path::new(&cp.file_b);

        if suppression_engine.is_path_ignored(path_a, None)
            || suppression_engine.is_path_ignored(path_b, None)
        {
            continue;
        }

        if suppression_engine.is_clone_type_ignored(path_a, &CloneType::Semantic)
            || suppression_engine.is_clone_type_ignored(path_b, &CloneType::Semantic)
        {
            continue;
        }

        let eff_a = suppression_engine.get_effective_min_tokens(path_a, config.min_tokens);
        let eff_b = suppression_engine.get_effective_min_tokens(path_b, config.min_tokens);
        let req_min = eff_a.max(eff_b);

        let norm_cp_a = normalize_path_str(&cp.file_a);
        let norm_cp_b = normalize_path_str(&cp.file_b);

        // Check if an existing clone pair already covers this exact or overlapping snippet
        let already_covered = pairs.iter().any(|p| {
            let norm_p_a = normalize_path_str(&p.file_a);
            let norm_p_b = normalize_path_str(&p.file_b);

            (norm_p_a == norm_cp_a
                && norm_p_b == norm_cp_b
                && p.start_line_a <= cp.lines_a.1
                && p.end_line_a >= cp.lines_a.0
                && p.start_line_b <= cp.lines_b.1
                && p.end_line_b >= cp.lines_b.0)
                || (norm_p_a == norm_cp_b
                    && norm_p_b == norm_cp_a
                    && p.start_line_a <= cp.lines_b.1
                    && p.end_line_a >= cp.lines_b.0
                    && p.start_line_b <= cp.lines_a.1
                    && p.end_line_b >= cp.lines_a.0)
        });

        if already_covered {
            continue;
        }

        let tok_a = count_tokens_for_path(parsed_files, &cp.file_a, cp.lines_a.0, cp.lines_a.1);
        let tok_b = count_tokens_for_path(parsed_files, &cp.file_b, cp.lines_b.0, cp.lines_b.1);
        let token_count = tok_a.max(tok_b);

        if token_count < req_min {
            continue;
        }

        let fragment_hash = format!(
            "semantic-{}:{}-{}:{}-{}",
            cp.file_a,
            cp.lines_a.0,
            cp.file_b,
            cp.lines_b.0,
            (cp.hybrid_score * 10000.0) as u64
        );

        pairs.push(ClonePair {
            file_a: cp.file_a,
            start_line_a: cp.lines_a.0,
            end_line_a: cp.lines_a.1,
            file_b: cp.file_b,
            start_line_b: cp.lines_b.0,
            end_line_b: cp.lines_b.1,
            token_count,
            similarity: cp.hybrid_score,
            fragment_hash,
            clone_type: CloneType::Semantic,
            author_a: Some(format!("Language: {}", cp.language_a)),
            author_b: Some(format!("Language: {}", cp.language_b)),
        });
    }
}

fn normalize_path_str(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches("./").to_string()
}

fn count_tokens_for_path(
    parsed_files: &[ParsedFile],
    path: &str,
    start_line: usize,
    end_line: usize,
) -> usize {
    let norm = path.replace('\\', "/");
    parsed_files
        .iter()
        .find(|f| f.path == path || f.path.replace('\\', "/") == norm)
        .map(|f| count_tokens_in_line_span(&f.token_spans, start_line, end_line))
        .unwrap_or(0)
}
