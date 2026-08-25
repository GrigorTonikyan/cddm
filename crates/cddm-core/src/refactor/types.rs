#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Refactoring strategies supported by the CDDM refactoring engine.
pub mod refactor_strategies {
    pub const EXTRACT_FUNCTION: &str = "extract_function";
    pub const PARAMETERIZE: &str = "parameterize";
}

/// Default function name prefix for synthesized helper abstractions.
pub const DEFAULT_HELPER_PREFIX: &str = "extracted_shared_helper";

/// Result of applying a synthesized refactoring patch to workspace files.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApplyPatchResult {
    pub success: bool,
    pub modified_files: Vec<String>,
    pub hunks_applied: usize,
    pub message: String,
}

/// A parsed hunk within a unified diff patch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub deleted_lines: Vec<String>,
    pub added_lines: Vec<String>,
}

/// A parsed file patch containing target path and list of hunks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedFilePatch {
    pub file_path: String,
    pub hunks: Vec<ParsedHunk>,
}

/// Represents a variable difference between two clone fragments.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParameterDifference {
    pub line_number_a: usize,
    pub line_number_b: usize,
    pub fragment_a_code: String,
    pub fragment_b_code: String,
}

/// Comprehensive deduplication and refactoring recommendation for a clone pair.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefactorSuggestion {
    pub suggested_function_name: String,
    pub strategy: String,
    pub common_body_lines: Vec<String>,
    pub parameter_differences: Vec<ParameterDifference>,
    pub target_module_hint: String,
    pub unified_patch: String,
    pub lines_saved: usize,
}

/// Describes refactoring transformation at a specific file site within a clone cluster.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClusterSiteRefactor {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub parameter_differences: Vec<ParameterDifference>,
    pub call_site_replacement: String,
}

/// Comprehensive deduplication and multi-site refactoring recommendation for an N-way clone cluster.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClusterRefactorSuggestion {
    pub cluster_id: String,
    pub suggested_function_name: String,
    pub strategy: String,
    pub common_body_lines: Vec<String>,
    pub target_module_hint: String,
    pub sites: Vec<ClusterSiteRefactor>,
    pub unified_patch: String,
    pub total_lines_saved: usize,
}
