#![forbid(unsafe_code)]

pub use super::scan_commands::{DiffArgs, ExtractArgs, RefactorArgs, ScanArgs, SemanticArgs};
pub use super::service_commands::{
    CommentArgs, InitArgs, LspArgs, MonorepoArgs, ServeArgs, TrendArgs, TuiArgs, WatchArgs,
};
use clap::{Args, Subcommand};
use std::path::PathBuf;

/// CLI Arguments for `cddm heal`
#[derive(Args, Debug, Clone)]
pub struct HealArgs {
    /// Directory path to scan (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Target clone cluster index to heal
    #[arg(short = 'c', long)]
    pub cluster: Option<usize>,

    /// Target clone pair index to heal
    #[arg(short = 'p', long)]
    pub pair: Option<usize>,

    /// AI Provider backend (gemini, claude, openai, ollama, mock)
    #[arg(long, default_value = "mock")]
    pub provider: String,

    /// Model identifier name (e.g. gemini-2.5-pro, claude-3-7-sonnet, gpt-4.5-preview, qwen2.5-coder)
    #[arg(long)]
    pub model: Option<String>,

    /// Secret API key for authentication
    #[arg(long)]
    pub api_key: Option<String>,

    /// Custom endpoint URL (e.g. http://localhost:11434 for Ollama)
    #[arg(long)]
    pub endpoint: Option<String>,

    /// Maximum healing repair iterations
    #[arg(short = 'i', long, default_value_t = 3)]
    pub max_iterations: usize,

    /// Verify refactoring against test suite
    #[arg(long, default_value_t = true)]
    pub verify: bool,

    /// Custom test command (e.g. "cargo test", "bun test")
    #[arg(long)]
    pub test_cmd: Option<String>,

    /// Apply passing refactoring to dedicated Git branch
    #[arg(long)]
    pub branch: Option<String>,

    /// Custom extracted function name
    #[arg(long)]
    pub fn_name: Option<String>,

    /// Target module path for helper function
    #[arg(long)]
    pub target_module: Option<String>,

    /// Custom instructions or architectural constraints for the AI
    #[arg(long)]
    pub custom_instructions: Option<String>,

    /// Minimum token count for clone detection
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,
}

/// CLI Arguments for `cddm overlap`
#[derive(Args, Debug, Clone)]
pub struct OverlapArgs {
    /// Directory path to scan (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Confidence threshold for library overlap detection (0.0 to 1.0)
    #[arg(short, long, default_value_t = 0.3)]
    pub threshold: f64,

    /// Output format (console, json, markdown)
    #[arg(short, long, default_value = "console")]
    pub format: String,
}

/// Subcommands for `cddm hub`
#[derive(Subcommand, Debug, Clone)]
pub enum HubSubcommand {
    /// Initialize a new .cddmhub.toml configuration template
    Init {
        /// Custom configuration file path (default: .cddmhub.toml)
        #[arg(short, long, default_value = cddm_core::DEFAULT_HUB_CONFIG_FILE)]
        config: PathBuf,
        /// Organization or hub name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Scan organization federation repositories for cross-repository duplication
    Scan {
        /// Configuration file path or repository directories to scan
        #[arg(default_values_t = [String::from(cddm_core::DEFAULT_HUB_CONFIG_FILE)])]
        targets: Vec<String>,
        /// Output format (console, json, markdown)
        #[arg(short, long, default_value = "console")]
        format: String,
        /// Minimum token count
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,
    },
    /// Extract a cross-repository duplicate cluster into a standalone shared package
    Extract {
        /// Configuration file path (default: .cddmhub.toml)
        #[arg(short, long, default_value = cddm_core::DEFAULT_HUB_CONFIG_FILE)]
        config: PathBuf,
        /// Cluster index to extract
        #[arg(short = 'c', long, default_value_t = 1)]
        cluster: usize,
        /// Target package name (e.g. @org/shared-utils or cddm-shared-common)
        #[arg(short = 'n', long, default_value = "@org/shared-extracted")]
        pkg_name: String,
        /// Target package ecosystem (npm, cargo, pypi, go)
        #[arg(short = 't', long, default_value = "npm")]
        pkg_type: String,
        /// Destination directory path for the new standalone package
        #[arg(short = 'd', long, default_value = "./packages/shared-extracted")]
        target_dir: String,
        /// Dry run preview without writing changes to disk
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
}

/// CLI Arguments for `cddm hub`
#[derive(Args, Debug, Clone)]
pub struct HubArgs {
    #[command(subcommand)]
    pub action: HubSubcommand,
}

/// CLI Arguments for `cddm coverage`
#[derive(Args, Debug, Clone)]
pub struct CoverageArgs {
    /// Path to coverage tracefile (e.g. lcov.info, coverage.xml, coverage-final.json)
    #[arg(short, long)]
    pub report: PathBuf,

    /// Directory path to scan (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Minimum token count to consider as duplicate clone
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,

    /// Output report format (console, json, markdown)
    #[arg(short, long, default_value = "console")]
    pub format: String,

    /// Show only dead code duplicates (0 runtime executions across all sites)
    #[arg(long, default_value_t = false)]
    pub dead_code_only: bool,

    /// Filter clones by minimum combined runtime execution hits
    #[arg(long, default_value_t = 0)]
    pub min_hits: u64,

    /// Filter clones exceeding this risk score threshold (0-100)
    #[arg(long)]
    pub risk_threshold: Option<f64>,
}

/// CLI Arguments for `cddm dead-code`
#[derive(Args, Debug, Clone)]
pub struct DeadCodeArgs {
    /// Directory path to scan (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Minimum token count threshold for dead code items
    #[arg(short, long, default_value_t = 30)]
    pub min_tokens: usize,

    /// Output report format (console, json, markdown, sarif)
    #[arg(short, long, default_value = "console")]
    pub format: String,

    /// Restrict analysis to static AST & symbol analysis only
    #[arg(long, default_value_t = false)]
    pub static_only: bool,

    /// Path to optional coverage report file (e.g. lcov.info, coverage.xml)
    #[arg(short, long)]
    pub coverage: Option<PathBuf>,

    /// Filter by target programming languages
    #[arg(short, long, value_delimiter = ',')]
    pub languages: Option<Vec<String>>,

    /// Custom file or path ignore patterns
    #[arg(short, long, value_delimiter = ',')]
    pub ignore: Option<Vec<String>>,
}

/// CLI Arguments for `cddm prune`
#[derive(Args, Debug, Clone)]
pub struct PruneArgs {
    /// Directory path to scan (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Dry run preview without modifying files on disk
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Only prune dead clones meeting strict closed-loop safety verification (default: true)
    #[arg(long, default_value_t = true)]
    pub safe_only: bool,

    /// Confidence threshold for safe removal (0.0 to 1.0)
    #[arg(short, long, default_value_t = 0.90)]
    pub threshold: f64,

    /// Minimum token count threshold for dead clone items
    #[arg(short, long, default_value_t = 30)]
    pub min_tokens: usize,

    /// Output report format (console, json, markdown, sarif)
    #[arg(short, long, default_value = "console")]
    pub format: String,

    /// Filter by target programming languages
    #[arg(short, long, value_delimiter = ',')]
    pub languages: Option<Vec<String>>,

    /// Custom file or path ignore patterns
    #[arg(short, long, value_delimiter = ',')]
    pub ignore: Option<Vec<String>>,
}
