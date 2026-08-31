#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Classification of dead code items detected in the codebase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeadCodeKind {
    /// Unreferenced private or internal function with 0 callers
    UnreferencedFunction,
    /// Code block structurally unreachable (after return, throw, exit, panic)
    UnreachableBlock,
    /// Duplicate code clone fragment with zero runtime executions or callers
    DeadClone,
    /// Function present in codebase but never exercised in test coverage
    UncoveredFunction,
    /// Dead or invariant conditional branch
    DeadBranch,
}

impl DeadCodeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeadCodeKind::UnreferencedFunction => "unreferenced_function",
            DeadCodeKind::UnreachableBlock => "unreachable_block",
            DeadCodeKind::DeadClone => "dead_clone",
            DeadCodeKind::UncoveredFunction => "uncovered_function",
            DeadCodeKind::DeadBranch => "dead_branch",
        }
    }

    pub fn display_label(&self) -> &'static str {
        match self {
            DeadCodeKind::UnreferencedFunction => "Unreferenced Function",
            DeadCodeKind::UnreachableBlock => "Unreachable Block",
            DeadCodeKind::DeadClone => "Dead Duplicate Clone",
            DeadCodeKind::UncoveredFunction => "Uncovered Function",
            DeadCodeKind::DeadBranch => "Dead Branch",
        }
    }
}

/// A specific dead code entity detected by static analysis or coverage telemetry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeadCodeItem {
    pub id: usize,
    pub file_path: String,
    pub symbol_name: String,
    pub kind: DeadCodeKind,
    pub line_start: usize,
    pub line_end: usize,
    pub token_count: usize,
    pub estimated_lines_saved: usize,
    pub reason: String,
    pub confidence: f64,
}

/// High-level summary of dead code analysis across a codebase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeadCodeSummary {
    pub total_dead_items: usize,
    pub dead_functions: usize,
    pub unreachable_blocks: usize,
    pub dead_clones: usize,
    pub uncovered_items: usize,
    pub total_dead_lines: usize,
    pub estimated_savings_pct: f64,
    pub items: Vec<DeadCodeItem>,
}

/// Configuration parameters for running dead code detection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeadCodeConfig {
    pub directory: String,
    pub min_tokens: usize,
    pub static_only: bool,
    pub report_path: Option<String>,
    pub report_content: Option<String>,
    pub languages: Option<Vec<String>>,
    pub ignore: Option<Vec<String>>,
}

impl Default for DeadCodeConfig {
    fn default() -> Self {
        Self {
            directory: ".".to_string(),
            min_tokens: 30,
            static_only: false,
            report_path: None,
            report_content: None,
            languages: None,
            ignore: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dead_code_kind_labels() {
        assert_eq!(
            DeadCodeKind::UnreferencedFunction.as_str(),
            "unreferenced_function"
        );
        assert_eq!(
            DeadCodeKind::UnreferencedFunction.display_label(),
            "Unreferenced Function"
        );
        assert_eq!(DeadCodeKind::UnreachableBlock.as_str(), "unreachable_block");
        assert_eq!(DeadCodeKind::DeadClone.as_str(), "dead_clone");
    }

    #[test]
    fn test_dead_code_summary_default() {
        let summary = DeadCodeSummary::default();
        assert_eq!(summary.total_dead_items, 0);
        assert_eq!(summary.total_dead_lines, 0);
        assert!(summary.items.is_empty());
    }
}
