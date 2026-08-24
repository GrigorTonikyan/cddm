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

/// Represents a single occurrence/location of duplicate code in a file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CloneLocation {
    /// File path of the code occurrence
    pub file: String,
    /// 1-based start line
    pub start_line: usize,
    /// 1-based end line
    pub end_line: usize,
    /// Optional author attribution
    pub author: Option<String>,
}

/// An N-way cluster (equivalence class) of code fragments sharing duplication across multiple locations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CloneCluster {
    /// 1-based cluster index
    pub id: usize,
    /// The structural clone classification of this cluster
    pub clone_type: CloneType,
    /// Maximum or representative token count of the cluster
    pub token_count: usize,
    /// Average or representative similarity score (0.0 to 1.0)
    pub similarity: f64,
    /// Unique structural hash identifying this clone cluster
    pub fragment_hash: String,
    /// List of occurrences / locations belonging to this cluster
    pub occurrences: Vec<CloneLocation>,
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
}

/// Default persistent cache database path.
pub const DEFAULT_CACHE_FILE: &str = ".cddm/cache.db";

/// Represents the status of a clone pair in a differential scan.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloneStatus {
    /// A newly introduced clone pair in the target branch/commit
    New,
    /// An existing pre-existing clone pair inherited from the base ref
    Legacy,
    /// A clone pair present in the base ref that has been refactored/resolved
    Resolved,
}

impl std::fmt::Display for CloneStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for CloneStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::New => "New",
            Self::Legacy => "Legacy",
            Self::Resolved => "Resolved",
        }
    }
}

/// A clone pair annotated with differential status relative to a base git ref.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DiffClonePair {
    /// The clone pair details
    pub clone_pair: ClonePair,
    /// Status relative to the base ref
    pub status: CloneStatus,
}

/// Summary metrics comparing target state to baseline git reference.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DiffSummary {
    /// The base git revision (e.g., "origin/main", "HEAD~1")
    pub base_ref: String,
    /// The target git revision or working tree description
    pub target_ref: String,
    /// DRY Health Score of the baseline revision
    pub base_dry_score: f64,
    /// DRY Health Score of the target revision
    pub target_dry_score: f64,
    /// Net DRY score delta (target_dry_score - base_dry_score)
    pub net_dry_delta: f64,
    /// Total number of modified/added files in the git delta
    pub total_changed_files: usize,
    /// Count of newly introduced clone pairs
    pub new_clones: usize,
    /// Count of legacy pre-existing clone pairs
    pub legacy_clones: usize,
    /// Count of resolved/eliminated clone pairs
    pub resolved_clones: usize,
}

/// The result of a differential git clone scan.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DiffScanResult {
    /// Unique identifier for this differential scan
    pub scan_id: String,
    /// Summary metrics comparing base and target
    pub summary: DiffSummary,
    /// List of clone pairs with differential statuses
    pub diff_clones: Vec<DiffClonePair>,
    /// How long the differential scan took in milliseconds
    pub duration_ms: u64,
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

/// A point-in-time duplication metrics snapshot for a Git commit.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TimelineSnapshot {
    /// Full commit hash (40-character hex string)
    pub commit_hash: String,
    /// Short commit hash (7-character hex string)
    pub short_hash: String,
    /// Commit author name
    pub author: String,
    /// Commit timestamp (Unix epoch seconds)
    pub commit_time: i64,
    /// Formatted commit date string (YYYY-MM-DD HH:MM:SS)
    pub formatted_date: String,
    /// Commit message (first line summary)
    pub message: String,
    /// Optional Git tag pointing to this commit
    pub tag: Option<String>,
    /// Total number of files analyzed
    pub total_files: usize,
    /// Total number of tokens analyzed
    pub total_tokens: usize,
    /// Total number of pairwise code clones
    pub total_clones: usize,
    /// Total number of clone clusters
    pub total_clusters: usize,
    /// Duplication percentage (0.0 to 100.0)
    pub duplication_percentage: f64,
    /// DRY Health Score (0.0 to 100.0)
    pub dry_health_score: f64,
}

/// Metric tracking file modification frequency and duplicate clone association.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileChurnMetric {
    /// Relative file path
    pub file_path: String,
    /// Number of times modified across timeline
    pub commit_count: usize,
    /// Active clone count in current snapshot
    pub clone_count: usize,
}

/// Aggregated historical duplication trend across Git history.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TimelineTrend {
    /// Chronological list of sampled commit snapshots (oldest to newest)
    pub snapshots: Vec<TimelineSnapshot>,
    /// Initial DRY health score (from oldest sampled commit)
    pub initial_score: f64,
    /// Current DRY health score (from newest sampled commit)
    pub current_score: f64,
    /// Net DRY health score delta (current_score - initial_score)
    pub score_delta: f64,
    /// Net duplication percentage delta (current_dup - initial_dup)
    pub duplication_delta: f64,
    /// Top file churn hotspots
    pub churn_hotspots: Vec<FileChurnMetric>,
}

/// Target CI/CD workflow platform for automated script generation.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkflowPlatform {
    GitHub,
    GitLab,
    Azure,
}

impl std::fmt::Display for WorkflowPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GitHub => write!(f, "GitHub Actions"),
            Self::GitLab => write!(f, "GitLab CI"),
            Self::Azure => write!(f, "Azure Pipelines"),
        }
    }
}

/// Status of local Git pre-commit and pre-push hooks.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HookStatus {
    /// Whether a pre-commit hook is active
    pub pre_commit_installed: bool,
    /// Whether a pre-push hook is active
    pub pre_push_installed: bool,
    /// Path to the Git hooks directory
    pub hooks_dir: String,
}

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

/// Request payload for synthesizing customized cluster refactoring preview in the sandbox.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RefactorSandboxRequest {
    /// Target 1-based cluster ID
    pub cluster_id: Option<usize>,
    /// Explicit list of occurrences to refactor
    pub occurrences: Vec<CloneLocation>,
    /// Custom function name to extract (e.g. "validate_user_credentials")
    pub custom_function_name: Option<String>,
    /// Target destination module/file path for extracted helper
    pub target_module_path: Option<String>,
    /// Custom parameter names to use in extracted function signature
    pub custom_parameter_names: Option<Vec<String>>,
}

/// Structured result of customized refactoring synthesis in the sandbox.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RefactorSandboxResult {
    /// Cluster ID if applicable
    pub cluster_id: Option<usize>,
    /// Name of the extracted helper function
    pub function_name: String,
    /// Target file path where extracted function resides
    pub target_module_path: String,
    /// Unified diff patch
    pub unified_patch: String,
    /// Total lines saved across all sites
    pub total_lines_saved: usize,
    /// Number of call sites refactored
    pub sites_count: usize,
    /// List of affected distinct file paths
    pub affected_files: Vec<String>,
}

/// Request payload for applying refactoring patch directly with Git branch creation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApplyRefactorBranchRequest {
    /// Unified patch content to apply
    pub patch: String,
    /// Name of the git branch to create (e.g. "cddm/refactor-cluster-1")
    pub branch_name: Option<String>,
    /// Commit message if committing
    pub commit_message: Option<String>,
    /// Whether to create a dedicated Git branch before applying
    pub create_branch: bool,
}

/// Result of applying refactoring patch with Git branch creation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ApplyRefactorBranchResult {
    /// Whether patch applied successfully
    pub success: bool,
    /// Name of the branch created, if applicable
    pub branch_created: Option<String>,
    /// List of modified file paths
    pub modified_files: Vec<String>,
    /// Total hunks applied
    pub hunks_applied: usize,
    /// Informational message
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

    fn assert_serde_roundtrip<
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    >(
        val: &T,
    ) {
        let json = serde_json::to_string(val).expect("Serialization failed");
        let recovered: T = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(val, &recovered);
    }

    #[test]
    fn test_scan_phase_serde() {
        let phases = [
            ScanPhase::Discovery,
            ScanPhase::Tokenization,
            ScanPhase::AstAnalysis,
            ScanPhase::Indexing,
            ScanPhase::Merging,
            ScanPhase::Scoring,
            ScanPhase::Complete,
            ScanPhase::Cancelled,
            ScanPhase::Failed,
        ];
        for phase in phases {
            assert_serde_roundtrip(&phase);
            assert_eq!(phase.to_string(), phase.as_ref());
        }
    }

    #[test]
    fn test_scan_config_serde_roundtrip() {
        assert_serde_roundtrip(&ScanConfig::default());
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
            assert_serde_roundtrip(&variant);
        }
    }

    #[test]
    fn test_scan_result_serde_roundtrip() {
        let result = ScanResult {
            scan_id: "test-id".to_string(),
            total_files: 10,
            total_tokens: 1000,
            total_clones: 5,
            total_clusters: 1,
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
            clone_clusters: vec![CloneCluster {
                id: 1,
                clone_type: CloneType::Exact,
                token_count: 50,
                similarity: 1.0,
                fragment_hash: "hash".to_string(),
                occurrences: vec![
                    CloneLocation {
                        file: "a.rs".to_string(),
                        start_line: 1,
                        end_line: 10,
                        author: None,
                    },
                    CloneLocation {
                        file: "b.rs".to_string(),
                        start_line: 2,
                        end_line: 11,
                        author: None,
                    },
                ],
            }],
            duration_ms: 100,
            language_breakdown: vec![LanguageStats {
                language: "Rust".to_string(),
                files: 10,
                tokens: 1000,
                clones: 5,
            }],
        };
        assert_serde_roundtrip(&result);
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

    #[test]
    fn test_clone_status_display_and_serde() {
        let statuses = [CloneStatus::New, CloneStatus::Legacy, CloneStatus::Resolved];
        for status in statuses {
            assert_serde_roundtrip(&status);
            assert_eq!(status.to_string(), status.as_ref());
        }
    }

    #[test]
    fn test_diff_scan_result_serde_roundtrip() {
        let diff_result = DiffScanResult {
            scan_id: "diff-test-id".to_string(),
            summary: DiffSummary {
                base_ref: "main".to_string(),
                target_ref: "feature/refactor".to_string(),
                base_dry_score: 92.5,
                target_dry_score: 96.0,
                net_dry_delta: 3.5,
                total_changed_files: 3,
                new_clones: 0,
                legacy_clones: 2,
                resolved_clones: 1,
            },
            diff_clones: vec![DiffClonePair {
                clone_pair: ClonePair {
                    file_a: "src/a.rs".to_string(),
                    start_line_a: 10,
                    end_line_a: 20,
                    file_b: "src/b.rs".to_string(),
                    start_line_b: 30,
                    end_line_b: 40,
                    token_count: 60,
                    similarity: 1.0,
                    fragment_hash: "hash_diff".to_string(),
                    clone_type: CloneType::Exact,
                    author_a: None,
                    author_b: None,
                },
                status: CloneStatus::Legacy,
            }],
            duration_ms: 45,
        };
        assert_serde_roundtrip(&diff_result);
    }

    #[test]
    fn test_timeline_serde_roundtrip() {
        let snapshot = TimelineSnapshot {
            commit_hash: "0123456789abcdef0123456789abcdef01234567".to_string(),
            short_hash: "0123456".to_string(),
            author: "Grigor Tonikyan".to_string(),
            commit_time: 1700000000,
            formatted_date: "2026-08-24 12:00:00".to_string(),
            message: "feat: add timeline".to_string(),
            tag: Some("v1.3.0".to_string()),
            total_files: 100,
            total_tokens: 50000,
            total_clones: 12,
            total_clusters: 4,
            duplication_percentage: 5.2,
            dry_health_score: 92.4,
        };
        assert_serde_roundtrip(&snapshot);

        let trend = TimelineTrend {
            snapshots: vec![snapshot],
            initial_score: 88.0,
            current_score: 92.4,
            score_delta: 4.4,
            duplication_delta: -2.1,
            churn_hotspots: vec![FileChurnMetric {
                file_path: "src/main.rs".to_string(),
                commit_count: 5,
                clone_count: 2,
            }],
        };
        assert_serde_roundtrip(&trend);
    }

    #[test]
    fn test_workflow_platform_display_and_serde() {
        let platforms = [
            WorkflowPlatform::GitHub,
            WorkflowPlatform::GitLab,
            WorkflowPlatform::Azure,
        ];
        for platform in platforms {
            assert_serde_roundtrip(&platform);
        }
        assert_eq!(WorkflowPlatform::GitHub.to_string(), "GitHub Actions");
        assert_eq!(WorkflowPlatform::GitLab.to_string(), "GitLab CI");
        assert_eq!(WorkflowPlatform::Azure.to_string(), "Azure Pipelines");
    }

    #[test]
    fn test_hook_status_serde() {
        let hook_status = HookStatus {
            pre_commit_installed: true,
            pre_push_installed: false,
            hooks_dir: ".git/hooks".to_string(),
        };
        assert_serde_roundtrip(&hook_status);
    }

    #[test]
    fn test_suppression_types_serde() {
        let rule = SuppressionRule {
            pattern: "tests/**".to_string(),
            rule_type: "threshold".to_string(),
            min_tokens: Some(100),
            ignored_clone_types: vec![CloneType::Exact],
            line_number: 12,
        };
        assert_serde_roundtrip(&rule);

        let directive = SuppressionDirective {
            file_path: "src/auth.rs".to_string(),
            start_line: 10,
            end_line: 25,
            directive_type: "ignore_block".to_string(),
            reason: Some("Intentional duplicate API mock".to_string()),
        };
        assert_serde_roundtrip(&directive);

        let config = SuppressionConfig {
            rules: vec![rule],
            ignore_tests: true,
            ignore_mocks: true,
            ignore_generated: true,
            raw_cddmignore: Some("# .cddmignore\ntests/**\n".to_string()),
        };
        assert_serde_roundtrip(&config);
    }

    #[test]
    fn test_refactor_sandbox_types_serde() {
        let req = RefactorSandboxRequest {
            cluster_id: Some(1),
            occurrences: vec![CloneLocation {
                file: "src/a.rs".to_string(),
                start_line: 10,
                end_line: 20,
                author: None,
            }],
            custom_function_name: Some("custom_validate".to_string()),
            target_module_path: Some("src/utils.rs".to_string()),
            custom_parameter_names: Some(vec!["arg1".to_string(), "arg2".to_string()]),
        };
        assert_serde_roundtrip(&req);

        let res = RefactorSandboxResult {
            cluster_id: Some(1),
            function_name: "custom_validate".to_string(),
            target_module_path: "src/utils.rs".to_string(),
            unified_patch: "--- a/src/a.rs\n+++ b/src/a.rs\n...".to_string(),
            total_lines_saved: 25,
            sites_count: 2,
            affected_files: vec!["src/a.rs".to_string(), "src/b.rs".to_string()],
        };
        assert_serde_roundtrip(&res);

        let branch_res = ApplyRefactorBranchResult {
            success: true,
            branch_created: Some("cddm/refactor-cluster-1".to_string()),
            modified_files: vec!["src/a.rs".to_string(), "src/utils.rs".to_string()],
            hunks_applied: 2,
            message: "Refactor applied to branch cddm/refactor-cluster-1".to_string(),
        };
        assert_serde_roundtrip(&branch_res);
    }
}
