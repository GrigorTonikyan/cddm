#![forbid(unsafe_code)]

use clap::{Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum IgnoreAction {
    /// Initialize a standard, well-documented .cddmignore configuration template
    Init {
        /// Target directory path to create .cddmignore in (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Overwrite existing .cddmignore file if present
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },

    /// Test whether a specific file path or line number is ignored by suppression rules
    Check {
        /// Target file path to check
        path: PathBuf,

        /// Optional 1-based line number to check for inline suppression directives
        #[arg(short, long)]
        line: Option<usize>,

        /// Path to custom .cddmignore file
        #[arg(long)]
        cddmignore: Option<PathBuf>,

        /// Check with test file suppression enabled
        #[arg(long, default_value_t = false)]
        ignore_tests: bool,

        /// Check with mock file suppression enabled
        #[arg(long, default_value_t = false)]
        ignore_mocks: bool,

        /// Check with generated file suppression enabled
        #[arg(long, default_value_t = true)]
        ignore_generated: bool,
    },
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum RulesAction {
    /// Initialize a starter .cddmrules.toml configuration template
    Init {
        /// Target output file path (default: .cddmrules.toml)
        #[arg(short, long, default_value = cddm_core::DEFAULT_RULES_FILE)]
        output: PathBuf,

        /// Overwrite existing file if present
        #[arg(short, long, default_value_t = false)]
        force: bool,

        /// Write directly to disk (default: true)
        #[arg(long, default_value_t = true)]
        write: bool,
    },

    /// Evaluate architectural policy rules against codebase
    Check {
        /// Target directory to scan (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Custom path to .cddmrules.toml file
        #[arg(short, long)]
        rules: Option<PathBuf>,

        /// Minimum token count for clone detection
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Output report format (console, json, markdown)
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
        format: OutputFormat,

        /// Exit with non-zero code if any policy violations exist
        #[arg(long, default_value_t = false)]
        enforce_policies: bool,
    },
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum HookAction {
    /// Install a Git hook enforcing code duplication thresholds
    Install {
        /// Hook type to install (pre-commit or pre-push)
        #[arg(short = 't', long, default_value = "pre-commit")]
        hook_type: String,

        /// Duplication percentage threshold to fail on (default: 15.0)
        #[arg(long, default_value_t = 15.0)]
        fail_threshold: f64,

        /// Minimum token count for clone detection (default: 50)
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Repository root directory path (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,
    },
    /// Uninstall an existing CDDM Git hook
    Uninstall {
        /// Hook type to remove (pre-commit or pre-push)
        #[arg(short = 't', long, default_value = "pre-commit")]
        hook_type: String,

        /// Repository root directory path (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,
    },
    /// Check current installation status of Git hooks
    Status {
        /// Repository root directory path (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlatformChoice {
    Gitea,
    Github,
    Gitlab,
    Azure,
}

impl std::fmt::Display for PlatformChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gitea => write!(f, "Gitea Actions"),
            Self::Github => write!(f, "GitHub Actions"),
            Self::Gitlab => write!(f, "GitLab CI"),
            Self::Azure => write!(f, "Azure Pipelines"),
        }
    }
}

impl From<PlatformChoice> for cddm_core::WorkflowPlatform {
    fn from(choice: PlatformChoice) -> Self {
        match choice {
            PlatformChoice::Gitea => cddm_core::WorkflowPlatform::Gitea,
            PlatformChoice::Github => cddm_core::WorkflowPlatform::GitHub,
            PlatformChoice::Gitlab => cddm_core::WorkflowPlatform::GitLab,
            PlatformChoice::Azure => cddm_core::WorkflowPlatform::Azure,
        }
    }
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum CacheAction {
    /// Export persistent cache database to a portable .cddmpack archive
    Export {
        /// Custom path to cache database (default: .cddm/cache.db)
        #[arg(long)]
        cache_dir: Option<PathBuf>,

        /// Output pack archive file path (default: cddm-cache.cddmpack)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Import a portable .cddmpack archive into persistent cache database
    Import {
        /// Path to .cddmpack archive file to import
        pack_file: PathBuf,

        /// Target cache directory to populate (default: .cddm)
        #[arg(long)]
        target_dir: Option<PathBuf>,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Console,
    Json,
    Markdown,
    Sarif,
    Ndjson,
}
