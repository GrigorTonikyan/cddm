#![forbid(unsafe_code)]

use clap::{Args, Subcommand};
use std::path::PathBuf;

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

    /// Model identifier name (e.g. gemini-1.5-pro, claude-3-5-sonnet, gpt-4o, llama3)
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
