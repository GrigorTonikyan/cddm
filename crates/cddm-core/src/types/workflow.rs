#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

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
