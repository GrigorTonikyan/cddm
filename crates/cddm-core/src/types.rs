use serde::{Deserialize, Serialize};

/// Default minimum token count required to classify a fragment as a clone.
pub const DEFAULT_MIN_TOKENS: usize = 50;

/// Default directory path when not specified.
pub const DEFAULT_DIRECTORY: &str = ".";

/// Default glob patterns excluded from scanning.
pub const DEFAULT_IGNORE_PATTERNS: &[&str] =
    &["node_modules", "target", ".git", "dist", "build", ".logs"];

/// Minimum DRY health score bounds.
pub const MIN_HEALTH_SCORE: f64 = 0.0;

/// Maximum DRY health score bounds.
pub const MAX_HEALTH_SCORE: f64 = 100.0;

/// Represents the execution phase of a duplication scan.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScanPhase {
    /// Discovering files and applying ignore/language filters
    Discovery,
    /// Parsing source code into normalized token streams
    Tokenization,
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

/// Represents the type of clone detected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CloneType {
    /// Exact identical text (Type 1 clone).
    Exact,
    /// Identical structure but identifiers/literals are renamed (Type 2 clone).
    Renamed,
    /// Near-miss clones with added/deleted statements (Type 3 clone).
    NearMiss,
    /// Semantic AST clones with structurally identical logic (Type 4 clone).
    Semantic,
}

/// A matched pair of code fragments indicating a code clone.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClonePair {
    /// File path of the first fragment
    pub file_a: String,
    /// Start line in the first file
    pub start_line_a: usize,
    /// End line in the first file
    pub end_line_a: usize,
    /// File path of the second fragment
    pub file_b: String,
    /// Start line in the second file
    pub start_line_b: usize,
    /// End line in the second file
    pub end_line_b: usize,
    /// Number of matching tokens in this clone
    pub token_count: usize,
    /// Similarity percentage (0.0 to 1.0)
    pub similarity: f64,
    /// Unique hash identifying the structure of this clone
    pub fragment_hash: String,
    /// The type of the clone
    pub clone_type: CloneType,
    /// Optional author attribution for fragment A
    pub author_a: Option<String>,
    /// Optional author attribution for fragment B
    pub author_b: Option<String>,
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
    /// Percentage of total tokens that are duplicated
    pub duplication_percentage: f64,
    /// DRY Health Index Score (0.0 to 100.0) evaluating structural modularity
    pub dry_health_score: f64,
    /// The list of clone pairs found
    pub clone_pairs: Vec<ClonePair>,
    /// How long the scan took in milliseconds
    pub duration_ms: u64,
    /// Statistics broken down by programming language
    pub language_breakdown: Vec<LanguageStats>,
}

/// Configuration options for running a code scan.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_config_default() {
        let config = ScanConfig::default();
        assert_eq!(config.directory, DEFAULT_DIRECTORY);
        assert_eq!(config.min_tokens, DEFAULT_MIN_TOKENS);
        assert!(config.languages.is_empty());
        assert_eq!(config.ignore_patterns.len(), DEFAULT_IGNORE_PATTERNS.len());
        assert!(config.detect_type2);
        assert!(config.scan_self);
        assert!(!config.enable_git_blame);
    }

    #[test]
    fn test_scan_phase_serde() {
        let phases = [
            ScanPhase::Discovery,
            ScanPhase::Tokenization,
            ScanPhase::Indexing,
            ScanPhase::Merging,
            ScanPhase::Scoring,
            ScanPhase::Complete,
            ScanPhase::Cancelled,
            ScanPhase::Failed,
        ];
        for phase in phases {
            let serialized = serde_json::to_string(&phase).unwrap();
            let deserialized: ScanPhase = serde_json::from_str(&serialized).unwrap();
            assert_eq!(phase, deserialized);
            assert_eq!(phase.to_string(), phase.as_ref());
        }
    }

    #[test]
    fn test_scan_config_serde_roundtrip() {
        let config = ScanConfig::default();
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: ScanConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_clone_type_serde_variants() {
        let variants = [
            CloneType::Exact,
            CloneType::Renamed,
            CloneType::NearMiss,
            CloneType::Semantic,
        ];
        for variant in variants {
            let serialized = serde_json::to_string(&variant).unwrap();
            let deserialized: CloneType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn test_scan_result_serde_roundtrip() {
        let result = ScanResult {
            scan_id: "test-id".to_string(),
            total_files: 10,
            total_tokens: 1000,
            total_clones: 5,
            duplication_percentage: 2.5,
            dry_health_score: 95.0,
            clone_pairs: vec![ClonePair {
                file_a: "a.rs".to_string(),
                start_line_a: 1,
                end_line_a: 10,
                file_b: "b.rs".to_string(),
                start_line_b: 2,
                end_line_b: 11,
                token_count: 50,
                similarity: 1.0,
                fragment_hash: "hash".to_string(),
                clone_type: CloneType::Exact,
                author_a: None,
                author_b: None,
            }],
            duration_ms: 100,
            language_breakdown: vec![LanguageStats {
                language: "Rust".to_string(),
                files: 10,
                tokens: 1000,
                clones: 5,
            }],
        };
        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: ScanResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(result, deserialized);
    }

    #[test]
    fn test_line_span_equality() {
        let span1 = LineSpan {
            line_start: 1,
            line_end: 2,
            byte_offset: 0,
        };
        let span2 = LineSpan {
            line_start: 1,
            line_end: 2,
            byte_offset: 0,
        };
        let span3 = LineSpan {
            line_start: 1,
            line_end: 3,
            byte_offset: 0,
        };
        assert_eq!(span1, span2);
        assert_ne!(span1, span3);
    }
}
