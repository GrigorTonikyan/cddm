#![forbid(unsafe_code)]

use super::actions::{HookAction, IgnoreAction, OutputFormat, PlatformChoice, RulesAction};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "cddm",
    author = "Grigor Tonikyan",
    version,
    about = "CDDM — Code De-Duplication Meister: High-Performance Polyglot Code Clone & \
             Modularity Analyzer",
    long_about = "CDDM analyzes codebases for duplicate code fragments, evaluates DRY health \
                  scores, and generates actionable structural reports."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan target directory for code duplication & DRY health score
    Scan {
        /// Directory path to scan (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Output report format (console, json, markdown, sarif)
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
        format: OutputFormat,

        /// Exit with non-zero status code if duplication percentage exceeds threshold (0-100)
        #[arg(long)]
        fail_threshold: Option<f64>,

        /// Specific language(s) to scan (e.g. Rust, TypeScript, Python)
        #[arg(short, long)]
        languages: Vec<String>,

        /// Glob patterns to ignore (e.g. node_modules, target)
        #[arg(short, long)]
        ignore: Vec<String>,

        /// Enable in-process git blame author & line age annotation
        #[arg(long, default_value_t = false)]
        git_blame: bool,

        /// Custom path for persistent redb cache database (default: .cddm/cache.db)
        #[arg(long)]
        cache_dir: Option<PathBuf>,

        /// Bypass persistent disk cache and force full re-scan
        #[arg(long, default_value_t = false)]
        no_cache: bool,

        /// Clear existing persistent cache database before scanning
        #[arg(long, default_value_t = false)]
        clear_cache: bool,

        /// Custom path to .cddmignore configuration file
        #[arg(long)]
        cddmignore: Option<PathBuf>,

        /// Automatically filter test files and test directories
        #[arg(long, default_value_t = false)]
        ignore_tests: bool,

        /// Automatically filter mock and fixture files
        #[arg(long, default_value_t = false)]
        ignore_mocks: bool,

        /// Automatically filter auto-generated files with generator headers
        #[arg(long, default_value_t = true)]
        ignore_generated: bool,

        /// Path to custom architectural policy rules (.cddmrules.toml)
        #[arg(long)]
        rules: Option<PathBuf>,

        /// Enforce architectural policy rules (exit code 1 on error-level violations)
        #[arg(long, default_value_t = false)]
        enforce_policies: bool,
    },

    /// Differential duplication scan comparing current changes against a Git base revision
    Diff {
        /// Base Git revision to compare against (e.g. main, origin/main, HEAD~1)
        base_ref: String,

        /// Target Git revision (default: working directory / HEAD)
        target_ref: Option<String>,

        /// Directory path of the Git repository to scan (default: current directory)
        #[arg(short, long, default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Output report format (console, json, markdown, sarif)
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
        format: OutputFormat,

        /// Exit with non-zero status code if duplication percentage exceeds threshold (0-100)
        #[arg(long)]
        fail_threshold: Option<f64>,

        /// Specific language(s) to scan
        #[arg(short, long)]
        languages: Vec<String>,

        /// Glob patterns to ignore
        #[arg(short, long)]
        ignore: Vec<String>,

        /// Enable in-process git blame author & line age annotation
        #[arg(long, default_value_t = false)]
        git_blame: bool,

        /// Custom path for persistent redb cache database (default: .cddm/cache.db)
        #[arg(long)]
        cache_dir: Option<PathBuf>,

        /// Bypass persistent disk cache and force full re-scan
        #[arg(long, default_value_t = false)]
        no_cache: bool,

        /// Custom path to .cddmignore configuration file
        #[arg(long)]
        cddmignore: Option<PathBuf>,

        /// Automatically filter test files and test directories
        #[arg(long, default_value_t = false)]
        ignore_tests: bool,

        /// Automatically filter mock and fixture files
        #[arg(long, default_value_t = false)]
        ignore_mocks: bool,

        /// Automatically filter auto-generated files with generator headers
        #[arg(long, default_value_t = true)]
        ignore_generated: bool,

        /// Path to custom architectural policy rules (.cddmrules.toml)
        #[arg(long)]
        rules: Option<PathBuf>,

        /// Enforce architectural policy rules (exit code 1 on error-level violations)
        #[arg(long, default_value_t = false)]
        enforce_policies: bool,
    },

    /// Synthesize automated refactoring suggestions for duplicate clone pairs
    Refactor {
        /// 1-based index of clone pair to refactor
        #[arg(short, long, default_value_t = 1)]
        pair: usize,

        /// 1-based index of clone cluster to refactor
        #[arg(short = 'c', long)]
        cluster: Option<usize>,

        /// Directory path to scan (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Output file path to write patch to (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Generate formatted markdown prompt for AI refactoring agents
        #[arg(long, default_value_t = false)]
        prompt: bool,

        /// Generate Tree-sitter AST-native code transformations
        #[arg(long, default_value_t = false)]
        ast: bool,

        /// Custom name for extracted function
        #[arg(long)]
        fn_name: Option<String>,

        /// Target module path for extracted helper
        #[arg(long)]
        target_module: Option<String>,

        /// Apply refactoring to dedicated Git branch
        #[arg(long)]
        apply_branch: Option<String>,

        /// Verify refactoring against test suite
        #[arg(long, default_value_t = false)]
        verify: bool,

        /// Custom test command for verification
        #[arg(long)]
        test_cmd: Option<String>,

        /// Specific language(s) to scan
        #[arg(short, long)]
        languages: Vec<String>,

        /// Glob patterns to ignore
        #[arg(short, long)]
        ignore: Vec<String>,
    },

    /// Launch interactive WebUI dashboard in browser
    Serve {
        /// Port to bind the WebUI HTTP and WebSocket server to
        #[arg(short, long, default_value_t = 3000)]
        port: u16,

        /// Automatically open WebUI in default web browser
        #[arg(short, long, default_value_t = false)]
        open: bool,
    },

    /// Watch directory and trigger continuous real-time clone analysis on file save
    Watch {
        /// Directory path to watch (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Specific language(s) to scan
        #[arg(short, long)]
        languages: Vec<String>,

        /// Glob patterns to ignore
        #[arg(short, long)]
        ignore: Vec<String>,

        /// Enable in-process git blame author & line age annotation
        #[arg(long, default_value_t = false)]
        git_blame: bool,

        /// Custom path for persistent redb cache database (default: .cddm/cache.db)
        #[arg(long)]
        cache_dir: Option<PathBuf>,

        /// Bypass persistent disk cache and force full re-scan
        #[arg(long, default_value_t = false)]
        no_cache: bool,

        /// Debounce delay in milliseconds before scanning on file changes
        #[arg(short, long, default_value_t = 500)]
        debounce_ms: u64,

        /// Exit with non-zero status code if duplication percentage exceeds threshold (0-100)
        #[arg(long)]
        fail_threshold: Option<f64>,
    },

    /// Run Language Server Protocol (LSP) server for live IDE diagnostic squiggles
    Lsp {
        /// Directory path to serve LSP for (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Minimum token count for clone detection
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,
    },

    /// Analyze historical duplication trends across Git commit history
    Trend {
        /// Directory path of Git repository (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Maximum number of historical commit snapshots to sample (default: 10)
        #[arg(short = 's', long, default_value_t = 10)]
        max_samples: usize,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Output report format (console, json, markdown)
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
        format: OutputFormat,
    },

    /// Manage local Git hooks (pre-commit / pre-push) for automated duplication gate enforcement
    Hook {
        /// Action to perform: install, uninstall, or status
        #[command(subcommand)]
        action: HookAction,
    },

    /// Manage .cddmignore rules and test path suppression matching
    Ignore {
        /// Subcommand action for suppression management
        #[command(subcommand)]
        action: IgnoreAction,
    },

    /// Manage architectural policy rules (.cddmrules.toml)
    Rules {
        /// Subcommand action for policy rule management
        #[command(subcommand)]
        action: RulesAction,
    },

    /// Generate turnkey CI/CD workflow configurations (GitHub Actions, GitLab CI, Azure Pipelines)
    Init {
        /// Target CI/CD platform: github, gitlab, or azure
        #[arg(value_enum, default_value_t = PlatformChoice::Github)]
        platform: PlatformChoice,

        /// Duplication percentage threshold to fail on (default: 15.0)
        #[arg(long, default_value_t = 15.0)]
        fail_threshold: f64,

        /// Minimum token count for clone detection (default: 50)
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Output file path (defaults to standard platform config file)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Write directly to disk (default: print to stdout unless --write or output specified)
        #[arg(short = 'w', long, default_value_t = false)]
        write: bool,
    },

    /// Generate formatted Markdown summary comment for Pull Requests / Merge Requests
    Comment {
        /// Directory path to scan (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Duplication percentage threshold to fail on (default: 15.0)
        #[arg(long, default_value_t = 15.0)]
        fail_threshold: f64,

        /// Target CI/CD platform format: github, gitlab, or azure
        #[arg(short, long, value_enum, default_value_t = PlatformChoice::Github)]
        platform: PlatformChoice,

        /// Output file path to write Markdown comment to (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
