#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported coverage report file formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CoverageFormat {
    Auto,
    Lcov,
    Cobertura,
    Istanbul,
}

/// Normalized in-memory representation of line hit execution data across files.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageReport {
    /// Mapping of normalized file path -> (line number -> hit count).
    pub file_line_hits: HashMap<String, HashMap<usize, u64>>,
    /// Total execution hits across all parsed files.
    pub total_hits: u64,
    /// Total instrumentation lines recorded.
    pub total_instrumented_lines: usize,
}

/// Execution tier indicating frequency and importance in runtime traces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionTier {
    /// 0 executions - candidate for dead code removal.
    Dead,
    /// 1 - 10 executions.
    Cold,
    /// 11 - 1,000 executions.
    Warm,
    /// > 1,000 executions - critical hot path where duplication bugs have high impact.
    HotPath,
}

/// Correlated coverage metrics for an individual clone pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloneCoverageMetric {
    pub clone_pair_id: usize,
    pub file_a: String,
    pub start_line_a: usize,
    pub end_line_a: usize,
    pub hits_a: u64,
    pub covered_lines_a: usize,
    pub total_lines_a: usize,
    pub coverage_pct_a: f64,

    pub file_b: String,
    pub start_line_b: usize,
    pub end_line_b: usize,
    pub hits_b: u64,
    pub covered_lines_b: usize,
    pub total_lines_b: usize,
    pub coverage_pct_b: f64,

    pub total_combined_hits: u64,
    pub is_dead_code: bool,
    pub has_test_gap: bool,
    pub execution_tier: ExecutionTier,
    pub risk_score: f64,
}

/// High-level summary of runtime coverage correlation against codebase duplicates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoverageCorrelationSummary {
    pub total_clone_pairs: usize,
    pub dead_code_clones: usize,
    pub test_gap_clones: usize,
    pub hot_path_clones: usize,
    pub total_runtime_hits: u64,
    pub overall_covered_clones_pct: f64,
    pub metrics: Vec<CloneCoverageMetric>,
}
