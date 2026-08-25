#![forbid(unsafe_code)]

use super::clone::ClonePair;
use serde::{Deserialize, Serialize};

/// Represents the status of a clone pair in a differential scan.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloneStatus {
    /// A newly introduced clone pair in the target branch/commit
    New,
    /// An existing pre-existing clone pair inherited from the base ref
    Legacy,
    /// A clone pair present in the base ref that has been refactored/resolved
    Resolved,
}

impl std::fmt::Display for CloneStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for CloneStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::New => "New",
            Self::Legacy => "Legacy",
            Self::Resolved => "Resolved",
        }
    }
}

/// A clone pair annotated with differential status relative to a base git ref.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DiffClonePair {
    /// The clone pair details
    pub clone_pair: ClonePair,
    /// Status relative to the base ref
    pub status: CloneStatus,
}

/// Summary metrics comparing target state to baseline git reference.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DiffSummary {
    /// The base git revision (e.g., "origin/main", "HEAD~1")
    pub base_ref: String,
    /// The target git revision or working tree description
    pub target_ref: String,
    /// DRY Health Score of the baseline revision
    pub base_dry_score: f64,
    /// DRY Health Score of the target revision
    pub target_dry_score: f64,
    /// Net DRY score delta (target_dry_score - base_dry_score)
    pub net_dry_delta: f64,
    /// Total number of modified/added files in the git delta
    pub total_changed_files: usize,
    /// Count of newly introduced clone pairs
    pub new_clones: usize,
    /// Count of legacy pre-existing clone pairs
    pub legacy_clones: usize,
    /// Count of resolved/eliminated clone pairs
    pub resolved_clones: usize,
}

/// The result of a differential git clone scan.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DiffScanResult {
    /// Unique identifier for this differential scan
    pub scan_id: String,
    /// Summary metrics comparing base and target
    pub summary: DiffSummary,
    /// List of clone pairs with differential statuses
    pub diff_clones: Vec<DiffClonePair>,
    /// How long the differential scan took in milliseconds
    pub duration_ms: u64,
}
