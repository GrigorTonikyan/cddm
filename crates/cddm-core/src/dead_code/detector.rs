#![forbid(unsafe_code)]

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::mpsc;

use super::static_analyzer::analyze_static_dead_code;
use super::types::{DeadCodeConfig, DeadCodeItem, DeadCodeKind, DeadCodeSummary};
use crate::coverage::{CoverageFormat, load_coverage_report, parse_coverage_data};
use crate::detector::discovery::{discover_candidate_files, init_suppression_engine};
use crate::detector::run_scan;
use crate::error::CddmError;
use crate::io::read_file_source;
use crate::types::ScanConfig;

/// Execute comprehensive polyglot dead code detection across the specified workspace.
pub async fn run_dead_code_detection(config: DeadCodeConfig) -> Result<DeadCodeSummary, CddmError> {
    tracing::info!(
        directory = %config.directory,
        min_tokens = config.min_tokens,
        static_only = config.static_only,
        "Initiating CDDM polyglot dead code detection..."
    );

    let scan_config = ScanConfig {
        directory: config.directory.clone(),
        min_tokens: config.min_tokens,
        languages: config.languages.clone().unwrap_or_default(),
        ignore_patterns: config.ignore.clone().unwrap_or_default(),
        ..Default::default()
    };

    let suppression_engine = init_suppression_engine(&scan_config);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    let file_paths = discover_candidate_files(&scan_config, &suppression_engine, &cancel_flag)
        .map_err(CddmError::General)?;

    tracing::debug!(
        "Discovered {} candidate files for dead code analysis",
        file_paths.len()
    );

    let mut files_content = Vec::with_capacity(file_paths.len());
    let mut total_codebase_lines = 0;

    for path in &file_paths {
        if let Ok(source) = read_file_source(path) {
            let rel_path = path.to_string_lossy().to_string();
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();
            let content_str = source.as_str().to_string();
            total_codebase_lines += content_str.lines().count();
            files_content.push((rel_path, ext, content_str));
        }
    }

    // 1. Static Dead Code Analysis (Unreferenced Functions & Unreachable Blocks)
    let mut items = analyze_static_dead_code(&files_content, config.min_tokens);
    let mut next_id = items.len() + 1;

    // 2. Dead Duplicate Clones Detection
    let (tx, _rx) = mpsc::channel(100);
    if let Ok(scan_result) = run_scan(scan_config.clone(), tx, cancel_flag.clone()).await {
        let coverage_opt = load_optional_coverage(&config);

        for pair in &scan_result.clone_pairs {
            let is_dead = if let Some(ref cov) = coverage_opt {
                let hits_a = cov
                    .file_line_hits
                    .get(&pair.file_a)
                    .and_then(|m| m.get(&pair.start_line_a))
                    .copied()
                    .unwrap_or(0);
                let hits_b = cov
                    .file_line_hits
                    .get(&pair.file_b)
                    .and_then(|m| m.get(&pair.start_line_b))
                    .copied()
                    .unwrap_or(0);
                hits_a == 0 && hits_b == 0
            } else {
                pair.token_count >= config.min_tokens * 2 && pair.similarity >= 0.98
            };

            if is_dead {
                let lines_saved = (pair.end_line_a.saturating_sub(pair.start_line_a) + 1).max(1);
                items.push(DeadCodeItem {
                    id: next_id,
                    file_path: pair.file_a.clone(),
                    symbol_name: format!("ClonePair#{}:{}", pair.start_line_a, pair.end_line_a),
                    kind: DeadCodeKind::DeadClone,
                    line_start: pair.start_line_a,
                    line_end: pair.end_line_a,
                    token_count: pair.token_count,
                    estimated_lines_saved: lines_saved,
                    reason: format!(
                        "Duplicate clone pair with zero runtime hits against '{}'",
                        pair.file_b
                    ),
                    confidence: 0.92,
                });
                next_id += 1;
            }
        }
    }

    // 3. Optional Runtime Test Coverage Dead Items
    if !config.static_only
        && let Some(coverage) = load_optional_coverage(&config)
    {
        for (file_key, lines_map) in &coverage.file_line_hits {
            let mut zero_run_start: Option<usize> = None;
            let mut zero_run_len = 0;

            let mut sorted_lines: Vec<_> = lines_map.iter().collect();
            sorted_lines.sort_by_key(|(line, _)| *line);

            for (&line, &hits) in sorted_lines {
                if hits == 0 {
                    if zero_run_start.is_none() {
                        zero_run_start = Some(line);
                    }
                    zero_run_len += 1;
                } else if let Some(start) = zero_run_start {
                    if zero_run_len >= 5 {
                        items.push(DeadCodeItem {
                            id: next_id,
                            file_path: file_key.clone(),
                            symbol_name: format!("<uncovered_block_{start}_{}>", line - 1),
                            kind: DeadCodeKind::UncoveredFunction,
                            line_start: start,
                            line_end: line - 1,
                            token_count: zero_run_len * 4,
                            estimated_lines_saved: zero_run_len,
                            reason: format!(
                                "Consecutive {zero_run_len} instrumented lines with 0 runtime \
                                 executions in test suite"
                            ),
                            confidence: 0.90,
                        });
                        next_id += 1;
                    }
                    zero_run_start = None;
                    zero_run_len = 0;
                }
            }
        }
    }

    // 4. Compute Aggregate Statistics
    let mut dead_functions = 0;
    let mut unreachable_blocks = 0;
    let mut dead_clones = 0;
    let mut uncovered_items = 0;
    let mut total_dead_lines = 0;

    for item in &items {
        total_dead_lines += item.estimated_lines_saved;
        match item.kind {
            DeadCodeKind::UnreferencedFunction => dead_functions += 1,
            DeadCodeKind::UnreachableBlock => unreachable_blocks += 1,
            DeadCodeKind::DeadClone => dead_clones += 1,
            DeadCodeKind::UncoveredFunction => uncovered_items += 1,
            DeadCodeKind::DeadBranch => unreachable_blocks += 1,
        }
    }

    let estimated_savings_pct = if total_codebase_lines > 0 {
        ((total_dead_lines as f64 / total_codebase_lines as f64) * 100.0).min(100.0)
    } else {
        0.0
    };

    let summary = DeadCodeSummary {
        total_dead_items: items.len(),
        dead_functions,
        unreachable_blocks,
        dead_clones,
        uncovered_items,
        total_dead_lines,
        estimated_savings_pct,
        items,
    };

    tracing::info!(
        total_dead_items = summary.total_dead_items,
        total_dead_lines = summary.total_dead_lines,
        "Dead code detection completed successfully"
    );

    Ok(summary)
}

fn load_optional_coverage(config: &DeadCodeConfig) -> Option<crate::coverage::CoverageReport> {
    if let Some(ref content) = config.report_content {
        return parse_coverage_data(content, CoverageFormat::Auto).ok();
    }
    if let Some(ref path_str) = config.report_path {
        let p = Path::new(path_str);
        if p.exists() {
            return load_coverage_report(p, CoverageFormat::Auto).ok();
        }
    }
    let default_lcov = Path::new(&config.directory).join("lcov.info");
    if default_lcov.exists() {
        return load_coverage_report(&default_lcov, CoverageFormat::Auto).ok();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dead_code_detection_runner() {
        let config = DeadCodeConfig {
            directory: ".".to_string(),
            min_tokens: 100,
            static_only: true,
            ..Default::default()
        };

        let res = run_dead_code_detection(config).await;
        assert!(res.is_ok());
        let summary = res.unwrap();
        assert!(summary.estimated_savings_pct >= 0.0);
    }
}
