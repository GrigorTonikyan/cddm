#![forbid(unsafe_code)]

use super::types::ParsedFile;
use crate::types::{CloneCluster, ClonePair, LanguageStats, MAX_HEALTH_SCORE, MIN_HEALTH_SCORE};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ScanScoringMetrics {
    pub language_breakdown: Vec<LanguageStats>,
    pub duplication_percentage: f64,
    pub dry_health_score: f64,
    pub clone_clusters: Vec<CloneCluster>,
}

pub fn compute_scan_scoring(
    parsed_files: &[ParsedFile],
    merged_pairs: &[ClonePair],
    total_tokens: usize,
) -> ScanScoringMetrics {
    let mut lang_stats_map: HashMap<String, LanguageStats> = HashMap::new();
    for pf in parsed_files.iter() {
        let stats = lang_stats_map
            .entry(pf.language.clone())
            .or_insert(LanguageStats {
                language: pf.language.clone(),
                files: 0,
                tokens: 0,
                clones: 0,
            });
        stats.files += 1;
        stats.tokens += pf.token_count;
    }

    let mut cross_module_count = 0;
    for pair in merged_pairs {
        let norm_a = pair.file_a.replace('\\', "/");
        let norm_b = pair.file_b.replace('\\', "/");
        let parent_a = Path::new(&norm_a).parent().unwrap_or(Path::new(""));
        let parent_b = Path::new(&norm_b).parent().unwrap_or(Path::new(""));
        if parent_a != parent_b {
            cross_module_count += 1;
        }
    }

    let language_breakdown: Vec<LanguageStats> = lang_stats_map.into_values().collect();

    let norm_path_key =
        |p: &str| -> String { p.replace('\\', "/").trim_start_matches("./").to_string() };

    let mut file_duplicate_spans: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    for pair in merged_pairs {
        let norm_a = norm_path_key(&pair.file_a);
        let norm_b = norm_path_key(&pair.file_b);
        file_duplicate_spans
            .entry(norm_a)
            .or_default()
            .push((pair.start_line_a, pair.end_line_a));
        file_duplicate_spans
            .entry(norm_b)
            .or_default()
            .push((pair.start_line_b, pair.end_line_b));
    }

    let mut total_duplicated_tokens = 0;
    for pf in parsed_files.iter() {
        let norm_path = norm_path_key(&pf.path);
        if let Some(mut spans) = file_duplicate_spans.remove(&norm_path) {
            spans.sort_unstable_by_key(|s| s.0);
            let mut merged_spans: Vec<(usize, usize)> = Vec::new();
            for (start, end) in spans {
                if let Some(last) = merged_spans.last_mut()
                    && start <= last.1
                {
                    last.1 = last.1.max(end);
                    continue;
                }
                merged_spans.push((start, end));
            }

            let mut dup_count = 0;
            for span in &pf.token_spans {
                if merged_spans
                    .iter()
                    .any(|&(s, e)| span.line_start >= s && span.line_end <= e)
                {
                    dup_count += 1;
                }
            }
            total_duplicated_tokens += dup_count;
        }
    }

    let duplication_percentage = if total_tokens > 0 {
        ((total_duplicated_tokens as f64) / (total_tokens as f64) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let cross_module_ratio = if !merged_pairs.is_empty() {
        cross_module_count as f64 / merged_pairs.len() as f64
    } else {
        0.0
    };
    let duplication_weight = 1.5 * (1.0 + 0.3 * cross_module_ratio);
    let dry_health_score = (MAX_HEALTH_SCORE - duplication_percentage * duplication_weight)
        .clamp(MIN_HEALTH_SCORE, MAX_HEALTH_SCORE);

    let clone_clusters = crate::cluster::cluster_clone_pairs(merged_pairs);

    ScanScoringMetrics {
        language_breakdown,
        duplication_percentage,
        dry_health_score,
        clone_clusters,
    }
}
