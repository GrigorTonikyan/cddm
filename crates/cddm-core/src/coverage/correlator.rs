#![forbid(unsafe_code)]

use std::collections::HashMap;

use super::parser::normalize_path;
use super::types::{
    CloneCoverageMetric, CoverageCorrelationSummary, CoverageReport, ExecutionTier,
};
use crate::types::ScanResult;

/// Correlate static clone pairs with runtime execution coverage data.
pub fn correlate_coverage(
    scan_result: &ScanResult,
    coverage: &CoverageReport,
) -> CoverageCorrelationSummary {
    let mut metrics = Vec::new();
    let mut dead_code_clones = 0;
    let mut test_gap_clones = 0;
    let mut hot_path_clones = 0;
    let mut total_runtime_hits = 0;
    let mut covered_clones_count = 0;

    for (idx, pair) in scan_result.clone_pairs.iter().enumerate() {
        let (hits_a, cov_a, total_a, pct_a) = evaluate_range_coverage(
            &coverage.file_line_hits,
            &pair.file_a,
            pair.start_line_a,
            pair.end_line_a,
        );

        let (hits_b, cov_b, total_b, pct_b) = evaluate_range_coverage(
            &coverage.file_line_hits,
            &pair.file_b,
            pair.start_line_b,
            pair.end_line_b,
        );

        let total_combined_hits = hits_a + hits_b;
        total_runtime_hits += total_combined_hits;

        let is_dead_code = hits_a == 0 && hits_b == 0;
        if is_dead_code {
            dead_code_clones += 1;
        }

        let has_test_gap = (hits_a > 0 && hits_b == 0) || (hits_b > 0 && hits_a == 0);
        if has_test_gap {
            test_gap_clones += 1;
        }

        if pct_a > 0.0 || pct_b > 0.0 {
            covered_clones_count += 1;
        }

        let execution_tier = if is_dead_code {
            ExecutionTier::Dead
        } else if total_combined_hits <= 10 {
            ExecutionTier::Cold
        } else if total_combined_hits <= 1_000 {
            ExecutionTier::Warm
        } else {
            hot_path_clones += 1;
            ExecutionTier::HotPath
        };

        let risk_score = calculate_risk_score(
            total_combined_hits,
            has_test_gap,
            is_dead_code,
            pair.token_count,
            pair.similarity,
        );

        metrics.push(CloneCoverageMetric {
            clone_pair_id: idx + 1,
            file_a: pair.file_a.clone(),
            start_line_a: pair.start_line_a,
            end_line_a: pair.end_line_a,
            hits_a,
            covered_lines_a: cov_a,
            total_lines_a: total_a,
            coverage_pct_a: pct_a,

            file_b: pair.file_b.clone(),
            start_line_b: pair.start_line_b,
            end_line_b: pair.end_line_b,
            hits_b,
            covered_lines_b: cov_b,
            total_lines_b: total_b,
            coverage_pct_b: pct_b,

            total_combined_hits,
            is_dead_code,
            has_test_gap,
            execution_tier,
            risk_score,
        });
    }

    metrics.sort_by_key(|b| std::cmp::Reverse(b.total_combined_hits));

    let overall_covered_clones_pct = if scan_result.clone_pairs.is_empty() {
        100.0
    } else {
        (covered_clones_count as f64 / scan_result.clone_pairs.len() as f64) * 100.0
    };

    CoverageCorrelationSummary {
        total_clone_pairs: scan_result.clone_pairs.len(),
        dead_code_clones,
        test_gap_clones,
        hot_path_clones,
        total_runtime_hits,
        overall_covered_clones_pct,
        metrics,
    }
}

/// Compute line coverage and execution hits for a specific file line range.
fn evaluate_range_coverage(
    file_line_hits: &HashMap<String, HashMap<usize, u64>>,
    file_path: &str,
    start_line: usize,
    end_line: usize,
) -> (u64, usize, usize, f64) {
    let norm_path = normalize_path(file_path);
    let total_lines = (end_line.saturating_sub(start_line) + 1).max(1);

    // Look for matching file key (exact or suffix)
    let hits_map = file_line_hits.get(&norm_path).or_else(|| {
        file_line_hits.iter().find_map(|(k, v)| {
            if k.ends_with(&norm_path) || norm_path.ends_with(k) {
                Some(v)
            } else {
                None
            }
        })
    });

    match hits_map {
        Some(map) => {
            let mut total_hits = 0;
            let mut covered_lines = 0;

            for line in start_line..=end_line {
                if let Some(&hits) = map.get(&line) {
                    total_hits += hits;
                    if hits > 0 {
                        covered_lines += 1;
                    }
                }
            }

            let pct = (covered_lines as f64 / total_lines as f64) * 100.0;
            (total_hits, covered_lines, total_lines, pct)
        }
        None => (0, 0, total_lines, 0.0),
    }
}

/// Heuristically calculate risk score (0.0 to 100.0) based on execution frequency and test divergence.
fn calculate_risk_score(
    combined_hits: u64,
    has_test_gap: bool,
    is_dead_code: bool,
    tokens: usize,
    similarity: f64,
) -> f64 {
    if is_dead_code {
        return 10.0;
    }

    let mut score: f64 = 30.0;

    // Hot path multiplier
    if combined_hits > 10_000 {
        score += 35.0;
    } else if combined_hits > 1_000 {
        score += 25.0;
    } else if combined_hits > 100 {
        score += 15.0;
    }

    // Divergence & test gap multiplier
    if has_test_gap {
        score += 25.0;
    }

    // Near-miss clone divergence risk
    if similarity < 0.95 {
        score += 10.0;
    }

    // Token length weight
    if tokens > 150 {
        score += 10.0;
    }

    score.min(100.0)
}
