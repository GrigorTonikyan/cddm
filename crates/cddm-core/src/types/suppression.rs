#![forbid(unsafe_code)]

use super::clone::CloneType;
use serde::{Deserialize, Serialize};

/// A parsed suppression rule from .cddmignore or programmatic configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SuppressionRule {
    /// Glob path pattern (e.g. "tests/**", "*.generated.ts")
    pub pattern: String,
    /// Type of suppression: "ignore", "threshold", or "type_filter"
    pub rule_type: String,
    /// Custom minimum token threshold if rule_type == "threshold"
    pub min_tokens: Option<usize>,
    /// List of excluded clone types if rule_type == "type_filter"
    pub ignored_clone_types: Vec<CloneType>,
    /// Line number in .cddmignore where rule was defined (0 if programmatic)
    pub line_number: usize,
}

/// An inline suppression directive parsed from source code comments or attributes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SuppressionDirective {
    /// File path containing the directive
    pub file_path: String,
    /// 1-based start line of the suppressed span
    pub start_line: usize,
    /// 1-based end line of the suppressed span
    pub end_line: usize,
    /// Directive classification: "ignore_line", "ignore_block", "attribute"
    pub directive_type: String,
    /// Optional reason/comment
    pub reason: Option<String>,
}

/// Complete active suppression configuration and rules.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SuppressionConfig {
    /// List of active .cddmignore rules
    pub rules: Vec<SuppressionRule>,
    /// Whether test files are automatically filtered
    pub ignore_tests: bool,
    /// Whether mock/fixture files are automatically filtered
    pub ignore_mocks: bool,
    /// Whether generated files are automatically filtered
    pub ignore_generated: bool,
    /// Raw .cddmignore file contents if loaded from disk
    pub raw_cddmignore: Option<String>,
}
