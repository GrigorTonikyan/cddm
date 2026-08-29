#![forbid(unsafe_code)]

use super::actions::{OutputFormat, PlatformChoice};
use clap::Args;
use std::path::PathBuf;

/// CLI Arguments for `cddm serve`
#[derive(Args, Debug, Clone)]
pub struct ServeArgs {
    /// Port to bind the WebUI HTTP and WebSocket server to
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,

    /// Automatically open WebUI in default web browser
    #[arg(short, long, default_value_t = false)]
    pub open: bool,
}

/// CLI Arguments for `cddm watch`
#[derive(Args, Debug, Clone)]
pub struct WatchArgs {
    /// Directory path to watch (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Minimum token count to consider as duplicate clone
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,

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

    /// Debounce delay in milliseconds before scanning on file changes
    #[arg(short, long, default_value_t = 500)]
    pub debounce_ms: u64,

    /// Exit with non-zero status code if duplication percentage exceeds threshold (0-100)
    #[arg(long)]
    pub fail_threshold: Option<f64>,

    /// Optionally start embedded WebUI Studio server on specified port (default: 3000)
    #[arg(short = 's', long, num_args = 0..=1, default_missing_value = "3000")]
    pub serve: Option<u16>,

    /// Automatically open WebUI in browser when --serve is enabled
    #[arg(short, long, default_value_t = false)]
    pub open: bool,

    /// Output report format (console, json, markdown, ndjson)
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
    pub format: OutputFormat,

    /// Detect cross-language semantic clones across different programming languages
    #[arg(long, default_value_t = false)]
    pub cross_language: bool,
}

/// CLI Arguments for `cddm lsp`
#[derive(Args, Debug, Clone)]
pub struct LspArgs {
    /// Directory path to serve LSP for (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Minimum token count for clone detection
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,
}

/// CLI Arguments for `cddm trend`
#[derive(Args, Debug, Clone)]
pub struct TrendArgs {
    /// Directory path of Git repository (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Maximum number of historical commit snapshots to sample (default: 10)
    #[arg(short = 's', long, default_value_t = 10)]
    pub max_samples: usize,

    /// Minimum token count to consider as duplicate clone
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,

    /// Output report format (console, json, markdown)
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
    pub format: OutputFormat,
}

/// CLI Arguments for `cddm init`
#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    /// Target CI/CD platform: github, gitlab, or azure
    #[arg(value_enum, default_value_t = PlatformChoice::Github)]
    pub platform: PlatformChoice,

    /// Duplication percentage threshold to fail on (default: 15.0)
    #[arg(long, default_value_t = 15.0)]
    pub fail_threshold: f64,

    /// Minimum token count for clone detection (default: 50)
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,

    /// Output file path (defaults to standard platform config file)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Write directly to disk (default: print to stdout unless --write or output specified)
    #[arg(short = 'w', long, default_value_t = false)]
    pub write: bool,
}

/// CLI Arguments for `cddm comment`
#[derive(Args, Debug, Clone)]
pub struct CommentArgs {
    /// Directory path to scan (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Minimum token count to consider as duplicate clone
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,

    /// Duplication percentage threshold to fail on (default: 15.0)
    #[arg(long, default_value_t = 15.0)]
    pub fail_threshold: f64,

    /// Target CI/CD platform format: github, gitlab, or azure
    #[arg(short, long, value_enum, default_value_t = PlatformChoice::Github)]
    pub platform: PlatformChoice,

    /// Output file path to write Markdown comment to (default: stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

/// CLI Arguments for `cddm monorepo`
#[derive(Args, Debug, Clone)]
pub struct MonorepoArgs {
    /// Root directory of monorepo (default: current directory)
    #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    /// Minimum token count to consider as duplicate clone
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,
}

/// CLI Arguments for `cddm tui`
#[derive(Args, Debug, Clone)]
pub struct TuiArgs {
    /// Directory path to scan (default: current directory)
    pub directory: Option<PathBuf>,

    /// Minimum token count to consider as duplicate clone
    #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
    pub min_tokens: usize,

    /// Enable live watch mode for real-time rescanning on file changes
    #[arg(short, long, default_value_t = false)]
    pub watch: bool,

    /// Exit with non-zero status code if duplication percentage exceeds threshold (0-100)
    #[arg(long)]
    pub fail_threshold: Option<f64>,

    /// Specific language(s) to scan
    #[arg(short, long)]
    pub languages: Vec<String>,

    /// Glob patterns to ignore
    #[arg(short, long)]
    pub ignore: Vec<String>,
}
