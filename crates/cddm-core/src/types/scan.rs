#![forbid(unsafe_code)]

use super::clone::{CloneCluster, ClonePair};
use super::policy::PolicyViolation;
use serde::{Deserialize, Serialize};

/// Default minimum token count required to classify a fragment as a clone.
pub const DEFAULT_MIN_TOKENS: usize = 50;

/// Default directory path when not specified.
pub const DEFAULT_DIRECTORY: &str = ".";

/// Default glob patterns excluded from scanning.
pub const DEFAULT_IGNORE_PATTERNS: &[&str] = &[
    "node_modules",
    "target",
    ".git",
    "dist",
    "build",
    ".logs",
    "packaging",
];

/// Minimum DRY health score bounds.
pub const MIN_HEALTH_SCORE: f64 = 0.0;

/// Maximum DRY health score bounds.
pub const MAX_HEALTH_SCORE: f64 = 100.0;

/// Default persistent cache database path.
pub const DEFAULT_CACHE_FILE: &str = ".cddm/cache.db";

/// Default architectural rules configuration path.
pub const DEFAULT_RULES_FILE: &str = ".cddmrules.toml";

/// Represents the execution phase of a duplication scan.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScanPhase {
    /// Discovering files and applying ignore/language filters
    Discovery,
    /// Parsing source code into normalized token streams
    Tokenization,
    /// Tree-sitter AST Merkle subtree parsing and structural hashing
    AstAnalysis,
    /// Winnowing rolling hash index construction
    Indexing,
    /// Pairwise clone matching and interval merging
    Merging,
    /// DRY health score and modularity index calculation
    Scoring,
    /// Scan completed successfully
    Complete,
    /// Scan was cancelled by user/signal
    Cancelled,
    /// Scan encountered an unrecoverable failure
    Failed,
}

impl std::fmt::Display for ScanPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for ScanPhase {
    fn as_ref(&self) -> &str {
        match self {
            Self::Discovery => "Discovery",
            Self::Tokenization => "Tokenization",
            Self::AstAnalysis => "AstAnalysis",
            Self::Indexing => "Indexing",
            Self::Merging => "Merging",
            Self::Scoring => "Scoring",
            Self::Complete => "Complete",
            Self::Cancelled => "Cancelled",
            Self::Failed => "Failed",
        }
    }
}

/// Represents the type of a token after normalization.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum NormalizedToken {
    /// An identifier (e.g., variable name, function name)
    Identifier,
    /// A string literal
    StringLiteral,
    /// A numeric literal
    NumericLiteral,
    /// A keyword, mapped to a unique `u16` ID
    Keyword(u16),
    /// Punctuation, mapped to a unique `u8` ID
    Punctuation(u8),
}

/// Represents the physical location of a token in the source file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LineSpan {
    /// 1-based start line
    pub line_start: usize,
    /// 1-based end line
    pub line_end: usize,
    /// 0-based byte offset from start of file
    pub byte_offset: usize,
}

/// Statistics about a specific programming language found during scan.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LanguageStats {
    /// Language name (e.g., "Rust", "TypeScript")
    pub language: String,
    /// Number of files parsed for this language
    pub files: usize,
    /// Total tokens parsed in this language
    pub tokens: usize,
    /// Number of clone instances involving this language
    pub clones: usize,
}

/// The final result of a code duplication scan.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ScanResult {
    /// Unique identifier for this scan run
    pub scan_id: String,
    /// Total number of files processed
    pub total_files: usize,
    /// Total number of tokens analyzed
    pub total_tokens: usize,
    /// Total number of clone pairs detected
    pub total_clones: usize,
    /// Total number of clone clusters (equivalence classes) detected
    pub total_clusters: usize,
    /// Percentage of total tokens that are duplicated
    pub duplication_percentage: f64,
    /// DRY Health Index Score (0.0 to 100.0) evaluating structural modularity
    pub dry_health_score: f64,
    /// The list of clone pairs found
    pub clone_pairs: Vec<ClonePair>,
    /// The list of N-way clone clusters found
    pub clone_clusters: Vec<CloneCluster>,
    /// How long the scan took in milliseconds
    pub duration_ms: u64,
    /// Statistics broken down by programming language
    pub language_breakdown: Vec<LanguageStats>,
    /// Architectural policy violations detected during scan
    #[serde(default)]
    pub policy_violations: Vec<PolicyViolation>,
}

/// Configuration options for running a code scan.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct ScanConfig {
    /// Root directory to scan (absolute or relative path)
    pub directory: String,
    /// Minimum number of tokens required to consider as a clone (default: 50)
    pub min_tokens: usize,
    /// List of language names to include. If empty, all supported languages are scanned.
    pub languages: Vec<String>,
    /// List of glob patterns for files/directories to ignore (e.g. "node_modules")
    pub ignore_patterns: Vec<String>,
    /// Whether to detect Type-2 clones (renamed identifiers/literals)
    pub detect_type2: bool,
    /// Whether to find clones within the exact same file
    pub scan_self: bool,
    /// Whether to annotate clone pairs with in-process git blame author information
    pub enable_git_blame: bool,
    /// Custom path for the persistent disk cache database (default: ".cddm/cache.db")
    pub cache_dir: Option<String>,
    /// Whether to use the persistent disk cache (default: true)
    pub enable_cache: bool,
    /// Custom path to .cddmignore file (default: None, loads from root directory if present)
    pub cddmignore_path: Option<String>,
    /// Automatically filter test files and test directories (default: false)
    pub ignore_tests: bool,
    /// Automatically filter mock and fixture files (default: false)
    pub ignore_mocks: bool,
    /// Automatically filter auto-generated files with generator headers (default: true)
    pub ignore_generated: bool,
    /// Custom path to .cddmrules.toml file (default: None, loads from root directory if present)
    pub rules_path: Option<String>,
    /// Enforce policy compliance and fail if violations exist (default: false)
    pub enforce_policies: bool,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            directory: DEFAULT_DIRECTORY.to_string(),
            min_tokens: DEFAULT_MIN_TOKENS,
            languages: Vec::new(),
            ignore_patterns: DEFAULT_IGNORE_PATTERNS
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            detect_type2: true,
            scan_self: true,
            enable_git_blame: false,
            cache_dir: None,
            enable_cache: true,
            cddmignore_path: None,
            ignore_tests: false,
            ignore_mocks: false,
            ignore_generated: true,
            rules_path: None,
            enforce_policies: false,
        }
    }
}

/// Represents an ongoing scan's progress event.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ScanProgress {
    /// ID of the scan
    pub scan_id: String,
    /// Current phase of execution
    pub phase: ScanPhase,
    /// Number of files processed so far in the current phase
    pub files_processed: usize,
    /// Total expected files in this phase
    pub total_files: usize,
    /// Overall progress (0.0 to 1.0)
    pub progress: f64,
    /// An informational message
    pub message: String,
}
