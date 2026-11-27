#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::PolicySeverity;

/// Repository-wide scan quality thresholds and detection toggles under `[scan]` in `.cddmrules.toml`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScanPolicy {
    /// Maximum allowed duplication percentage (0.0 - 100.0)
    #[serde(default)]
    pub max_duplication_pct: Option<f64>,
    /// Minimum required DRY Health Score (0.0 - 100.0)
    #[serde(default)]
    pub min_dry_score: Option<f64>,
    /// Override for Type-3 near-miss clone detection during `cddm rules check`
    #[serde(default)]
    pub detect_type3: Option<bool>,
    /// Override for Type-4 semantic AST/CFG clone detection during `cddm rules check`
    #[serde(default)]
    pub detect_type4: Option<bool>,
    /// Override for excluding test files during `cddm rules check`
    #[serde(default)]
    pub ignore_tests: Option<bool>,
    /// Override for excluding mock/fixture files during `cddm rules check`
    #[serde(default)]
    pub ignore_mocks: Option<bool>,
    /// Violation severity when quality thresholds fail (default: Error)
    #[serde(default)]
    pub severity: Option<PolicySeverity>,
    /// Optional actionable remediation advice
    #[serde(default)]
    pub remediation: Option<String>,
}

/// Dead-code quality gate configuration under `[dead_code]` in `.cddmrules.toml`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeadCodePolicy {
    /// Fail quality gate if any dead code items are found (equivalent to `max_items = 0`)
    #[serde(default)]
    pub fail_on_findings: Option<bool>,
    /// Maximum allowed dead code findings before triggering a policy violation
    #[serde(default)]
    pub max_items: Option<usize>,
    /// Minimum token threshold for dead code detection
    #[serde(default)]
    pub min_tokens: Option<usize>,
    /// Restrict dead code analysis to static AST & symbol analysis only
    #[serde(default)]
    pub static_only: Option<bool>,
    /// Violation severity when dead code threshold fails (default: Error)
    #[serde(default)]
    pub severity: Option<PolicySeverity>,
    /// Optional actionable remediation advice
    #[serde(default)]
    pub remediation: Option<String>,
}

/// Evaluation outcome of a dead-code failure gate.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DeadCodeGateResult {
    /// Whether `total_dead_items <= max_items`
    pub passed: bool,
    /// Total number of detected dead code items
    pub total_dead_items: usize,
    /// Maximum permitted dead code items for this gate
    pub max_items: usize,
    /// Human-readable gate status summary
    pub message: String,
}
