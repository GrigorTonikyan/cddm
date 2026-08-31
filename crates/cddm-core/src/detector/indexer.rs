#![forbid(unsafe_code)]

use super::types::{Location, ParsedFile, count_tokens_in_line_span};
use crate::fingerprint::MIN_K_GRAM;
use crate::suppression::SuppressionEngine;
use crate::types::{ClonePair, CloneType, ScanConfig};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;

pub fn index_and_match_clone_pairs(
    parsed_files: &[ParsedFile],
    config: &ScanConfig,
    suppression_engine: &SuppressionEngine,
) -> (Vec<ClonePair>, usize) {
    let mut index: HashMap<(u64, u64), Vec<Location>> = HashMap::new();
    let mut total_tokens = 0;

    for (file_idx, pf) in parsed_files.iter().enumerate() {
        total_tokens += pf.token_count;
        for fp in &pf.fingerprints {
            index.entry(fp.hash).or_default().push(Location {
                file_idx,
                span: fp.span.clone(),
            });
        }
    }

    let mut raw_pairs = Vec::new();
    let repo_root = Path::new(&config.directory);
    let default_author = if config.enable_git_blame {
        crate::blame::get_line_author(repo_root, "", 0)
    } else {
        None
    };

    let k = std::cmp::max(MIN_K_GRAM, config.min_tokens / 2);

    for (hash, locations) in index {
        if locations.len() > 1 {
            for i in 0..locations.len() {
                for j in (i + 1)..locations.len() {
                    let loc_a = &locations[i];
                    let loc_b = &locations[j];

                    if loc_a.file_idx == loc_b.file_idx {
                        if !config.scan_self {
                            continue;
                        }
                        // Skip overlapping spans in the same file to prevent self-cloning
                        let spans_overlap = loc_a.span.line_start <= loc_b.span.line_end
                            && loc_b.span.line_start <= loc_a.span.line_end;
                        if spans_overlap {
                            continue;
                        }
                    }

                    let (author_a, author_b) = if config.enable_git_blame {
                        (
                            default_author.clone().map(|(n, d)| {
                                format!("{} (line {}, {})", n, loc_a.span.line_start, d)
                            }),
                            default_author.clone().map(|(n, d)| {
                                format!("{} (line {}, {})", n, loc_b.span.line_start, d)
                            }),
                        )
                    } else {
                        (None, None)
                    };

                    raw_pairs.push(ClonePair {
                        file_a: parsed_files[loc_a.file_idx].path.clone(),
                        start_line_a: loc_a.span.line_start,
                        end_line_a: loc_a.span.line_end,
                        file_b: parsed_files[loc_b.file_idx].path.clone(),
                        start_line_b: loc_b.span.line_start,
                        end_line_b: loc_b.span.line_end,
                        token_count: k,
                        similarity: 1.0,
                        fragment_hash: format!("{:x}-{:x}", hash.0, hash.1),
                        clone_type: CloneType::Exact,
                        author_a,
                        author_b,
                    });
                }
            }
        }
    }

    let mut merged_pairs = merge_overlapping_clone_pairs(raw_pairs, config.min_tokens, k, |path| {
        parsed_files
            .iter()
            .find(|f| f.path == path)
            .map(|f| f.token_spans.as_slice())
    });

    merged_pairs.par_iter_mut().for_each(|pair| {
        let ext_a = Path::new(&pair.file_a)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let ext_b = Path::new(&pair.file_b)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let snippet_a = crate::refactor::read_file_lines_range(
            Path::new(&pair.file_a),
            pair.start_line_a,
            pair.end_line_a,
        )
        .ok()
        .map(|lines| lines.join("\n"));
        let snippet_b = crate::refactor::read_file_lines_range(
            Path::new(&pair.file_b),
            pair.start_line_b,
            pair.end_line_b,
        )
        .ok()
        .map(|lines| lines.join("\n"));

        if let (Some(code_a), Some(code_b)) = (snippet_a, snippet_b) {
            let (classified_type, sim) =
                crate::ast::classify_ast_clone(&code_a, ext_a, &code_b, ext_b);
            pair.clone_type = classified_type;
            pair.similarity = sim;
        }
    });

    merged_pairs.retain(|pair| {
        pair.similarity >= 0.70
            || pair.clone_type == CloneType::Semantic
            || pair.clone_type == CloneType::Exact
    });

    // Filter out clone pairs if detect_type3 is disabled
    if !config.detect_type3 {
        merged_pairs.retain(|pair| pair.clone_type != crate::types::CloneType::NearMiss);
    }

    // Filter out clone pairs if detect_type4 is disabled
    if !config.detect_type4 {
        merged_pairs.retain(|pair| pair.clone_type != crate::types::CloneType::Semantic);
    }

    // Filter out clone pairs matching suppression type exclusions or custom thresholds
    merged_pairs.retain(|pair| {
        !suppression_engine.is_clone_type_ignored(Path::new(&pair.file_a), &pair.clone_type)
            && !suppression_engine.is_clone_type_ignored(Path::new(&pair.file_b), &pair.clone_type)
    });

    merged_pairs.retain(|pair| {
        let eff_a =
            suppression_engine.get_effective_min_tokens(Path::new(&pair.file_a), config.min_tokens);
        let eff_b =
            suppression_engine.get_effective_min_tokens(Path::new(&pair.file_b), config.min_tokens);
        let req_min = eff_a.max(eff_b);
        pair.token_count >= req_min
    });

    (merged_pairs, total_tokens)
}

/// Merges overlapping and adjacent clone pairs based on token distance.
pub fn merge_overlapping_clone_pairs<'a, F>(
    mut raw_pairs: Vec<ClonePair>,
    min_tokens: usize,
    k: usize,
    mut get_spans: F,
) -> Vec<ClonePair>
where
    F: FnMut(&str) -> Option<&'a [crate::types::LineSpan]>,
{
    if raw_pairs.is_empty() {
        return Vec::new();
    }

    raw_pairs.sort_by(|a, b| {
        a.file_a
            .cmp(&b.file_a)
            .then(a.file_b.cmp(&b.file_b))
            .then(a.start_line_a.cmp(&b.start_line_a))
            .then(a.start_line_b.cmp(&b.start_line_b))
    });

    let mut merged_pairs = Vec::new();
    let mut push_pair_if_valid = |mut pair: ClonePair| {
        let count_a = get_spans(&pair.file_a)
            .map(|spans| count_tokens_in_line_span(spans, pair.start_line_a, pair.end_line_a))
            .unwrap_or(k);
        let count_b = get_spans(&pair.file_b)
            .map(|spans| count_tokens_in_line_span(spans, pair.start_line_b, pair.end_line_b))
            .unwrap_or(k);
        pair.token_count = std::cmp::max(k, std::cmp::min(count_a, count_b));
        if pair.token_count >= min_tokens {
            merged_pairs.push(pair);
        }
    };

    let mut current = raw_pairs[0].clone();

    for next in raw_pairs.into_iter().skip(1) {
        let is_same_file = current.file_a == current.file_b;
        let candidate_end_a = std::cmp::max(current.end_line_a, next.end_line_a);
        let candidate_end_b = std::cmp::max(current.end_line_b, next.end_line_b);
        let (first_end, second_start) = if current.start_line_a <= current.start_line_b {
            (candidate_end_a, current.start_line_b)
        } else {
            (candidate_end_b, current.start_line_a)
        };
        let would_overlap = is_same_file && (first_end >= second_start);

        if current.file_a == next.file_a
            && current.file_b == next.file_b
            && next.start_line_a <= current.end_line_a + 3
            && next.start_line_b <= current.end_line_b + 3
            && !would_overlap
        {
            current.end_line_a = candidate_end_a;
            current.end_line_b = candidate_end_b;
        } else {
            push_pair_if_valid(current);
            current = next;
        }
    }

    push_pair_if_valid(current);
    crate::types::deduplicate_clone_pairs(&mut merged_pairs);
    merged_pairs.sort_by_key(|b| std::cmp::Reverse(b.token_count));
    merged_pairs
}
