#![forbid(unsafe_code)]

use super::actions::{CacheAction, HookAction, IgnoreAction, RulesAction};
use super::commands::{
    CommentArgs, CoverageArgs, DeadCodeArgs, DiffArgs, ExtractArgs, HealArgs, HubArgs, InitArgs,
    LspArgs, MonorepoArgs, OverlapArgs, PruneArgs, RefactorArgs, ScanArgs, SemanticArgs, ServeArgs,
    TrendArgs, TuiArgs, WatchArgs,
};
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
    /// Enable verbose debug logging output (-v for debug, -vv for trace)
    #[arg(short = 'v', long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Suppress all non-error output and diagnostics
    #[arg(short = 'q', long, global = true)]
    pub quiet: bool,

    /// Explicitly set the logging verbosity level (trace, debug, info, warn, error, off)
    #[arg(long, global = true, env = "CDDM_LOG_LEVEL")]
    pub log_level: Option<String>,

    /// Write structured logs to a dedicated log file
    #[arg(long, global = true, env = "CDDM_LOG_FILE")]
    pub log_file: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan target directory for code duplication & DRY health score
    Scan(ScanArgs),

    /// Detect unreferenced functions, unreachable blocks, and dead code duplicates
    #[command(alias = "dead")]
    DeadCode(DeadCodeArgs),

    /// Automatically prune unreachable dead clone clusters and unreferenced code
    Prune(PruneArgs),

    /// Differential duplication scan comparing current changes against a Git base revision
    Diff(DiffArgs),

    /// Analyze cross-language semantic clones & Weisfeiler-Lehman graph isomorphisms
    Semantic(SemanticArgs),

    /// Synthesize automated refactoring suggestions for duplicate clone pairs
    Refactor(RefactorArgs),

    /// Extract duplicate code into a standalone shared crate or module
    Extract(ExtractArgs),

    /// Launch interactive WebUI dashboard in browser
    Serve(ServeArgs),

    /// Watch directory and trigger continuous real-time clone analysis on file save
    Watch(WatchArgs),

    /// Run Language Server Protocol (LSP) server for live IDE diagnostic squiggles
    Lsp(LspArgs),

    /// Analyze historical duplication trends across Git commit history
    Trend(TrendArgs),

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
    Init(InitArgs),

    /// Generate formatted Markdown summary comment for Pull Requests / Merge Requests
    Comment(CommentArgs),

    /// Autonomous AI Code Surgeon refactoring with closed-loop test healing
    Heal(HealArgs),

    /// Manage persistent fingerprint cache and export/import .cddmpack archives
    Cache {
        /// Action to perform: export or import
        #[command(subcommand)]
        action: CacheAction,
    },

    /// Discover and scan monorepos with multi-workspace packages
    Monorepo(MonorepoArgs),

    /// Launch interactive Terminal UI (TUI) Studio dashboard
    Tui(TuiArgs),

    /// Detect reimplemented ecosystem library algorithms and suggest standard packages
    Overlap(OverlapArgs),

    /// Manage and scan multi-repository Organization Federation Hub (.cddmhub.toml)
    Hub(HubArgs),

    /// Dynamic runtime execution & coverage-aware de-duplication analysis
    Coverage(CoverageArgs),
}
