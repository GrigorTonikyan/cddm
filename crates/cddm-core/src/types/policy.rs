#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

fn default_error_severity() -> PolicySeverity {
    PolicySeverity::Error
}

fn default_warning_severity() -> PolicySeverity {
    PolicySeverity::Warning
}

fn default_pattern_type() -> String {
    "regex".to_string()
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

/// A rule forbidding specific code patterns via regex or Tree-sitter AST queries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProhibitedPatternRule {
    /// Unique identifier / name of the rule
    pub name: String,
    /// Optional human-readable description
    pub description: Option<String>,
    /// Regex pattern or Tree-sitter AST query
    pub pattern: String,
    /// Pattern kind: "regex" (default) or "ast"
    #[serde(default = "default_pattern_type")]
    pub pattern_type: String,
    /// Glob patterns for files to include
    #[serde(default)]
    pub include: Vec<String>,
    /// Glob patterns for files to exclude
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Violation severity
    #[serde(default = "default_error_severity")]
    pub severity: PolicySeverity,
    /// Actionable remediation advice
    #[serde(default)]
    pub remediation: Option<String>,
}

/// A rule constraining token count per file or across a set of files matching a glob.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TokenBudgetRule {
    /// Unique identifier / name of the token budget rule
    pub name: String,
    /// Optional human-readable description
    pub description: Option<String>,
    /// Glob pattern representing the target files (e.g. ".agents/rules/*.md")
    pub pattern: String,
    /// Maximum allowed tokens per individual file
    #[serde(default)]
    pub max_file_tokens: Option<usize>,
    /// Maximum allowed combined tokens across all matching files
    #[serde(default)]
    pub max_total_tokens: Option<usize>,
    /// Violation severity
    #[serde(default = "default_error_severity")]
    pub severity: PolicySeverity,
}

/// A rule forbidding specific files, conflicting tooling, or legacy lockfiles.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ForbiddenFileRule {
    /// Unique identifier / name of the forbidden file rule
    pub name: String,
    /// Optional human-readable description
    pub description: Option<String>,
    /// Single glob pattern (e.g. ".eslintrc*", "yarn.lock")
    #[serde(default)]
    pub pattern: Option<String>,
    /// Multiple glob patterns
    #[serde(default)]
    pub patterns: Vec<String>,
    /// Violation severity
    #[serde(default = "default_error_severity")]
    pub severity: PolicySeverity,
    /// Actionable remediation advice
    #[serde(default)]
    pub remediation: Option<String>,
}

/// A rule asserting that constants, versions, or configuration values stay synchronized across files.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SyncRule {
    /// Unique identifier / name of the sync rule
    pub name: String,
    /// Optional human-readable description
    pub description: Option<String>,
    /// Source file path where authoritative value is defined (e.g. "package.json", "Cargo.toml")
    pub source_file: String,
    /// Key path in source JSON or TOML (e.g. "version", "package.version")
    #[serde(default)]
    pub source_key: Option<String>,
    /// Regex pattern with a capture group to extract source value
    #[serde(default)]
    pub source_regex: Option<String>,
    /// Target file path(s) to check (e.g. ["src/constants.ts", "webui/package.json"])
    #[serde(default)]
    pub target_files: Vec<String>,
    /// Single target file fallback if target_files is empty
    #[serde(default)]
    pub target_file: Option<String>,
    /// Key path in target JSON or TOML (if target is JSON/TOML)
    #[serde(default)]
    pub target_key: Option<String>,
    /// Target regex pattern with a capture group (e.g. r#"APP_VERSION\s*=\s*["']([^"']+)["']"#)
    #[serde(default)]
    pub target_regex: Option<String>,
    /// Violation severity
    #[serde(default = "default_error_severity")]
    pub severity: PolicySeverity,
    /// Actionable remediation advice
    #[serde(default)]
    pub remediation: Option<String>,
}

/// A policy verifying that framework routes discovered on disk are documented in project Markdown files.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RouteDocsPolicy {
    /// Optional unique identifier / name of the route docs policy
    #[serde(default)]
    pub name: Option<String>,
    /// Framework type: "nextjs", "astro", "axum", or "auto"
    #[serde(default)]
    pub framework: Option<String>,
    /// Root directory containing route definitions (e.g. "src/app", "src/pages")
    pub routes_dir: String,
    /// Primary documentation file path (e.g. "README.md")
    #[serde(default)]
    pub doc_file: Option<String>,
    /// Multiple documentation file paths
    #[serde(default)]
    pub doc_files: Vec<String>,
    /// Routes or glob patterns to ignore (e.g. ["api/**", "_components/**"])
    #[serde(default)]
    pub ignore_routes: Vec<String>,
    /// Violation severity
    #[serde(default = "default_error_severity")]
    pub severity: PolicySeverity,
    /// Actionable remediation advice
    #[serde(default)]
    pub remediation: Option<String>,
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
    /// Prohibited pattern rules (regex and AST queries)
    #[serde(default)]
    pub prohibited_patterns: Vec<ProhibitedPatternRule>,
    /// Built-in zero-emoji sensor flag
    #[serde(default)]
    pub zero_emojis: Option<bool>,
    /// Token budget rules constraining file and total token limits
    #[serde(default)]
    pub token_budgets: Vec<TokenBudgetRule>,
    /// Forbidden files and artifacts rules
    #[serde(default)]
    pub forbidden_files: Vec<ForbiddenFileRule>,
    /// SSoT constant and version synchronization rules
    #[serde(default)]
    pub sync_rules: Vec<SyncRule>,
    /// Route documentation completeness policy
    #[serde(default)]
    pub route_docs_sync: Option<RouteDocsPolicy>,
    /// Multiple route documentation completeness policies
    #[serde(default)]
    pub route_docs: Vec<RouteDocsPolicy>,
    /// Raw `.cddmrules.toml` content if loaded from disk/string
    pub raw_toml: Option<String>,
}

/// An evaluated architectural policy violation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PolicyViolation {
    /// Name of the violated rule
    pub rule_name: String,
    /// Category of rule ("boundary", "zero_duplication", "limit", "prohibited_pattern", "zero_emojis", "token_budget")
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
    /// Code/text snippet where violation occurred
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    /// Suggested remediation guidance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
}

impl Default for PolicyViolation {
    fn default() -> Self {
        Self {
            rule_name: String::new(),
            rule_type: String::new(),
            severity: PolicySeverity::Error,
            message: String::new(),
            file_a: String::new(),
            start_line_a: 1,
            end_line_a: 1,
            file_b: None,
            start_line_b: None,
            end_line_b: None,
            cluster_id: None,
            token_count: 0,
            snippet: None,
            remediation: None,
        }
    }
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
