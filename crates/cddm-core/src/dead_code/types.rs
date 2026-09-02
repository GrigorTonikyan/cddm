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

/// Configuration parameters for running dead clone cluster pruning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeadClonePruneConfig {
    pub directory: String,
    pub min_tokens: usize,
    pub dry_run: bool,
    pub safe_only: bool,
    pub confidence_threshold: f64,
    pub item_ids: Option<Vec<usize>>,
    pub languages: Option<Vec<String>>,
    pub ignore: Option<Vec<String>>,
}

impl Default for DeadClonePruneConfig {
    fn default() -> Self {
        Self {
            directory: ".".to_string(),
            min_tokens: 30,
            dry_run: false,
            safe_only: true,
            confidence_threshold: 0.90,
            item_ids: None,
            languages: None,
            ignore: None,
        }
    }
}

/// Status outcome for an individual pruned candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PruneActionStatus {
    Pruned,
    DryRunPruned,
    SkippedUnsafe,
    Failed,
}

impl PruneActionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PruneActionStatus::Pruned => "pruned",
            PruneActionStatus::DryRunPruned => "dry_run_pruned",
            PruneActionStatus::SkippedUnsafe => "skipped_unsafe",
            PruneActionStatus::Failed => "failed",
        }
    }
}

/// Record of a specific pruned dead clone item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrunedItem {
    pub id: usize,
    pub file_path: String,
    pub symbol_name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub lines_removed: usize,
    pub status: PruneActionStatus,
    pub confidence: f64,
    pub reason: String,
    pub diff_preview: Option<String>,
}

/// High-level result of executing dead clone cluster pruning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeadClonePruneResult {
    pub total_candidates: usize,
    pub pruned_items: usize,
    pub skipped_items: usize,
    pub total_lines_removed: usize,
    pub dry_run: bool,
    pub files_affected: Vec<String>,
    pub items: Vec<PrunedItem>,
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

    #[test]
    fn test_dead_clone_prune_config_default() {
        let config = DeadClonePruneConfig::default();
        assert_eq!(config.directory, ".");
        assert_eq!(config.min_tokens, 30);
        assert!(!config.dry_run);
        assert!(config.safe_only);
        assert_eq!(config.confidence_threshold, 0.90);
    }

    #[test]
    fn test_prune_action_status() {
        assert_eq!(PruneActionStatus::Pruned.as_str(), "pruned");
        assert_eq!(PruneActionStatus::DryRunPruned.as_str(), "dry_run_pruned");
        assert_eq!(PruneActionStatus::SkippedUnsafe.as_str(), "skipped_unsafe");
        assert_eq!(PruneActionStatus::Failed.as_str(), "failed");
    }
}
