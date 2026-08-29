#![forbid(unsafe_code)]

use super::actions::{CacheAction, HookAction, IgnoreAction, RulesAction};
use super::commands::{
    CommentArgs, CoverageArgs, DiffArgs, ExtractArgs, HealArgs, HubArgs, InitArgs, LspArgs,
    MonorepoArgs, OverlapArgs, RefactorArgs, ScanArgs, SemanticArgs, ServeArgs, TrendArgs, TuiArgs,
    WatchArgs,
};
use clap::{Parser, Subcommand};

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
    Scan(ScanArgs),

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
