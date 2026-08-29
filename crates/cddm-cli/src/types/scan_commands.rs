#![forbid(unsafe_code)]

use super::actions::OutputFormat;
use clap::Args;
use std::path::PathBuf;

/// CLI Arguments for `cddm scan`
#[derive(Args, Debug, Clone)]
pub struct ScanArgs {
    /// Directory path to scan (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Minimum token count to consider as duplicate clone
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,

    /// Output report format (console, json, markdown, sarif)
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
    pub format: OutputFormat,

    /// Exit with non-zero status code if duplication percentage exceeds threshold (0-100)
    #[arg(long)]
    pub fail_threshold: Option<f64>,

    /// Specific language(s) to scan (e.g. Rust, TypeScript, Python)
    #[arg(short, long)]
    pub languages: Vec<String>,

    /// Glob patterns to ignore (e.g. node_modules, target)
    #[arg(short, long)]
    pub ignore: Vec<String>,

    /// Enable in-process git blame author & line age annotation
    #[arg(long, default_value_t = false)]
    pub git_blame: bool,

    /// Custom path for persistent redb cache database (default: .cddm/cache.db)
    #[arg(long)]
    pub cache_dir: Option<PathBuf>,

    /// Bypass persistent disk cache and force full re-scan
    #[arg(long, default_value_t = false)]
    pub no_cache: bool,

    /// Clear existing persistent cache database before scanning
    #[arg(long, default_value_t = false)]
    pub clear_cache: bool,

    /// Custom path to .cddmignore configuration file
    #[arg(long)]
    pub cddmignore: Option<PathBuf>,

    /// Automatically filter test files and test directories (default: true)
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub ignore_tests: bool,

    /// Include test files and test directories in scan
    #[arg(long, default_value_t = false)]
    pub no_ignore_tests: bool,

    /// Automatically filter mock and fixture files (default: true)
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub ignore_mocks: bool,

    /// Include mock and fixture files in scan
    #[arg(long, default_value_t = false)]
    pub no_ignore_mocks: bool,

    /// Automatically filter auto-generated files with generator headers
    #[arg(long, default_value_t = true)]
    pub ignore_generated: bool,

    /// Path to custom architectural policy rules (.cddmrules.toml)
    #[arg(long)]
    pub rules: Option<PathBuf>,

    /// Enforce architectural policy rules (exit code 1 on error-level violations)
    #[arg(long, default_value_t = false)]
    pub enforce_policies: bool,

    /// Detect cross-language semantic clones across different programming languages (default: true)
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub cross_language: bool,

    /// Disable cross-language semantic clone detection
    #[arg(long, default_value_t = false)]
    pub no_cross_language: bool,

    /// Disable Type-3 near-miss modified statement clone detection
    #[arg(long, default_value_t = false)]
    pub no_type3: bool,

    /// Maximum number of parallel worker threads to utilize (default: all logical cores)
    #[arg(short = 'j', long)]
    pub threads: Option<usize>,
}

/// CLI Arguments for `cddm diff`
#[derive(Args, Debug, Clone)]
pub struct DiffArgs {
    /// Base Git revision to compare against (e.g. main, origin/main, HEAD~1)
    pub base_ref: String,

    /// Target Git revision (default: working directory / HEAD)
    pub target_ref: Option<String>,

    /// Directory path of the Git repository to scan (default: current directory)
    #[arg(short, long, default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Minimum token count to consider as duplicate clone
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,

    /// Output report format (console, json, markdown, sarif)
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
    pub format: OutputFormat,

    /// Exit with non-zero status code if duplication percentage exceeds threshold (0-100)
    #[arg(long)]
    pub fail_threshold: Option<f64>,

    /// Specific language(s) to scan
    #[arg(short, long)]
    pub languages: Vec<String>,

    /// Glob patterns to ignore
    #[arg(short, long)]
    pub ignore: Vec<String>,

    /// Enable in-process git blame author & line age annotation
    #[arg(long, default_value_t = false)]
    pub git_blame: bool,

    /// Custom path for persistent redb cache database (default: .cddm/cache.db)
    #[arg(long)]
    pub cache_dir: Option<PathBuf>,

    /// Bypass persistent disk cache and force full re-scan
    #[arg(long, default_value_t = false)]
    pub no_cache: bool,

    /// Custom path to .cddmignore configuration file
    #[arg(long)]
    pub cddmignore: Option<PathBuf>,

    /// Automatically filter test files and test directories
    #[arg(long, default_value_t = false)]
    pub ignore_tests: bool,

    /// Automatically filter mock and fixture files
    #[arg(long, default_value_t = false)]
    pub ignore_mocks: bool,

    /// Automatically filter auto-generated files with generator headers
    #[arg(long, default_value_t = true)]
    pub ignore_generated: bool,

    /// Path to custom architectural policy rules (.cddmrules.toml)
    #[arg(long)]
    pub rules: Option<PathBuf>,

    /// Enforce architectural policy rules (exit code 1 on error-level violations)
    #[arg(long, default_value_t = false)]
    pub enforce_policies: bool,

    /// Detect cross-language semantic clones across different programming languages
    #[arg(long, default_value_t = false)]
    pub cross_language: bool,

    /// Multi-branch clone drift matrix comparison across multiple revisions (comma-separated list)
    #[arg(long, value_delimiter = ',')]
    pub matrix: Vec<String>,
}

/// CLI Arguments for `cddm semantic`
#[derive(Args, Debug, Clone)]
pub struct SemanticArgs {
    /// Directory path to scan (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Minimum hybrid similarity threshold (0.0 to 1.0, default: 0.70)
    #[arg(short, long, default_value_t = 0.70)]
    pub threshold: f64,

    /// Minimum token count to consider as duplicate clone
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,

    /// Output report format (console, json, markdown)
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
    pub format: OutputFormat,

    /// Specific language(s) to scan
    #[arg(short, long)]
    pub languages: Vec<String>,

    /// Glob patterns to ignore
    #[arg(short, long)]
    pub ignore: Vec<String>,

    /// Enable in-process dense neural code embedding equivalence scan
    #[arg(long)]
    pub neural: bool,

    /// Minimum cosine similarity threshold for neural matching (default: 0.85)
    #[arg(long, default_value_t = 0.85)]
    pub neural_threshold: f32,

    /// Maximum number of parallel worker threads to utilize (default: all logical cores)
    #[arg(short = 'j', long)]
    pub threads: Option<usize>,
}

/// CLI Arguments for `cddm refactor`
#[derive(Args, Debug, Clone)]
pub struct RefactorArgs {
    /// 1-based index of clone pair to refactor
    #[arg(short, long, default_value_t = 1)]
    pub pair: usize,

    /// 1-based index of clone cluster to refactor
    #[arg(short = 'c', long)]
    pub cluster: Option<usize>,

    /// Directory path to scan (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Minimum token count to consider as duplicate clone
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,

    /// Output file path to write patch to (default: stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Generate formatted markdown prompt for AI refactoring agents
    #[arg(long, default_value_t = false)]
    pub prompt: bool,

    /// Generate Tree-sitter AST-native code transformations
    #[arg(long, default_value_t = false)]
    pub ast: bool,

    /// Custom name for extracted function
    #[arg(long)]
    pub fn_name: Option<String>,

    /// Target module path for extracted helper
    #[arg(long)]
    pub target_module: Option<String>,

    /// Apply refactoring to dedicated Git branch
    #[arg(long)]
    pub apply_branch: Option<String>,

    /// Verify refactoring against test suite
    #[arg(long, default_value_t = false)]
    pub verify: bool,

    /// Custom test command for verification
    #[arg(long)]
    pub test_cmd: Option<String>,

    /// Specific language(s) to scan
    #[arg(short, long)]
    pub languages: Vec<String>,

    /// Glob patterns to ignore
    #[arg(short, long)]
    pub ignore: Vec<String>,
}

/// CLI Arguments for `cddm extract`
#[derive(Args, Debug, Clone)]
pub struct ExtractArgs {
    /// 1-based index of clone pair to extract
    #[arg(short, long)]
    pub pair: Option<usize>,

    /// 1-based index of clone cluster to extract
    #[arg(short = 'c', long)]
    pub cluster: Option<usize>,

    /// Directory path to scan (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Target destination path (e.g. `crates/shared_utils` or `src/common/utils.rs`)
    #[arg(short, long, default_value = "crates/shared_utils")]
    pub target: String,

    /// Custom extracted helper function name
    #[arg(long)]
    pub fn_name: Option<String>,

    /// Packaging strategy: auto, crate, module, existing
    #[arg(long, default_value = "auto")]
    pub crate_type: String,

    /// Perform a dry-run preview without modifying files
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Commit and apply generated files, manifest updates, and caller rewrites
    #[arg(long, default_value_t = false)]
    pub apply: bool,

    /// Automatically synthesize unit tests for the extracted helper
    #[arg(long, default_value_t = false)]
    pub generate_tests: bool,

    /// Automatically synthesize performance micro-benchmarks for the extracted helper
    #[arg(long, visible_alias = "bench", default_value_t = false)]
    pub generate_benchmarks: bool,

    /// Minimum token count for clone detection
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,
}
