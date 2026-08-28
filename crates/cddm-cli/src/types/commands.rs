#![forbid(unsafe_code)]

use clap::Args;
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
