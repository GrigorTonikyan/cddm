#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

fn default_error_severity() -> PolicySeverity {
    PolicySeverity::Error
}

fn default_warning_severity() -> PolicySeverity {
    PolicySeverity::Warning
}

/// Severity level for an architectural policy rule violation.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PolicySeverity {
    /// Error-level violation causing quality gate failure
    Error,
    /// Warning-level violation flagged for review
    Warning,
    /// Informational note
    Info,
}

impl std::fmt::Display for PolicySeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for PolicySeverity {
    fn as_ref(&self) -> &str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

/// A boundary rule forbidding cross-package or cross-layer duplication.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct BoundaryRule {
    /// Unique identifier / name of the boundary rule
    pub name: String,
    /// Optional human-readable description
    pub description: Option<String>,
    /// Glob pattern representing the source module or layer (e.g. "src/domain/**")
    pub source: String,
    /// Glob patterns representing forbidden counterpart modules (e.g. ["src/presentation/**", "src/infra/**"])
    pub forbidden_targets: Vec<String>,
    /// Violation severity
    #[serde(default = "default_error_severity")]
    pub severity: PolicySeverity,
}

/// A zero-duplication zone rule forbidding any duplication within a sensitive path.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ZeroDuplicationRule {
    /// Unique identifier / name of the zero duplication rule
    pub name: String,
    /// Optional human-readable description
    pub description: Option<String>,
    /// Glob pattern representing the protected zone (e.g. "src/auth/**", "crates/cddm-core/src/crypto/**")
    pub pattern: String,
    /// Violation severity
    #[serde(default = "default_error_severity")]
    pub severity: PolicySeverity,
}

/// A limit rule constraining clone token size or cluster occurrence counts.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LimitRule {
    /// Unique identifier / name of the limit rule
    pub name: String,
    /// Optional human-readable description
    pub description: Option<String>,
    /// Glob pattern for paths subject to limits (e.g. "src/api/**")
    pub pattern: String,
    /// Maximum allowed token count per clone before triggering violation
    pub max_tokens: Option<usize>,
    /// Maximum allowed cluster occurrence count
    pub max_occurrences: Option<usize>,
    /// Violation severity
    #[serde(default = "default_warning_severity")]
    pub severity: PolicySeverity,
}

/// Complete architectural policies configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct PolicyConfig {
    /// Boundary rules preventing cross-layer duplication
    #[serde(default)]
    pub boundaries: Vec<BoundaryRule>,
    /// Zero duplication rules for sensitive subsystems
    #[serde(default)]
    pub zero_duplication: Vec<ZeroDuplicationRule>,
    /// Limit rules constraining clone sizes and cluster counts
    #[serde(default)]
    pub limits: Vec<LimitRule>,
    /// Raw `.cddmrules.toml` content if loaded from disk/string
    pub raw_toml: Option<String>,
}

/// An evaluated architectural policy violation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PolicyViolation {
    /// Name of the violated rule
    pub rule_name: String,
    /// Category of rule ("boundary", "zero_duplication", "limit")
    pub rule_type: String,
    /// Severity level
    pub severity: PolicySeverity,
    /// Explanatory message
    pub message: String,
    /// Primary offending file path
    pub file_a: String,
    /// Primary 1-based start line
    pub start_line_a: usize,
    /// Primary 1-based end line
    pub end_line_a: usize,
    /// Secondary / counterpart file path if applicable
    pub file_b: Option<String>,
    /// Secondary 1-based start line
    pub start_line_b: Option<usize>,
    /// Secondary 1-based end line
    pub end_line_b: Option<usize>,
    /// Associated cluster ID if applicable
    pub cluster_id: Option<usize>,
    /// Number of matching tokens in the violating fragment
    pub token_count: usize,
}

/// Aggregated result of evaluating architectural policies against a scan result.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PolicyEvaluationResult {
    /// Whether all policy assertions passed cleanly (no error-level violations)
    pub passed: bool,
    /// Total number of violations (errors + warnings + info)
    pub total_violations: usize,
    /// Number of error-level violations
    pub error_count: usize,
    /// Number of warning-level violations
    pub warning_count: usize,
    /// Number of info-level violations
    pub info_count: usize,
    /// Detailed list of all violations
    pub violations: Vec<PolicyViolation>,
}
