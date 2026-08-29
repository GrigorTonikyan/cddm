#![forbid(unsafe_code)]

use crate::extract::{CallerRewrite, ExtractedFile, ManifestUpdate};
use serde::{Deserialize, Serialize};

/// Configuration for an Organization Federation Hub.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HubConfig {
    pub name: String,
    pub repositories: Vec<HubRepoConfig>,
    #[serde(default = "default_min_tokens")]
    pub min_tokens: usize,
    #[serde(default = "default_fail_threshold")]
    pub fail_threshold: f64,
    #[serde(default)]
    pub ignore_patterns: Vec<String>,
}

fn default_min_tokens() -> usize {
    50
}

fn default_fail_threshold() -> f64 {
    crate::types::DEFAULT_FAIL_THRESHOLD
}

/// Configuration for an individual member repository in the Federation Hub.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HubRepoConfig {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub branch: Option<String>,
}

/// A specific duplicate occurrence within a member repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossRepoOccurrence {
    pub repo_name: String,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    #[serde(default)]
    pub snippet: Option<String>,
}

/// A pairwise clone relationship spanning either intra-repo or cross-repo sites.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossRepoClonePair {
    pub id: usize,
    pub repo_a: String,
    pub file_a: String,
    pub lines_a: (usize, usize),
    pub repo_b: String,
    pub file_b: String,
    pub lines_b: (usize, usize),
    pub tokens: usize,
    pub similarity: f64,
    pub clone_type: String,
}

/// An N-way clone cluster spanning multiple member repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossRepoCluster {
    pub id: usize,
    pub repos: Vec<String>,
    pub occurrences: Vec<CrossRepoOccurrence>,
    pub token_count: usize,
    pub similarity: f64,
    pub suggested_package: String,
}

/// Metric tracking duplicate volume between two distinct repositories.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoDuplicationMetric {
    pub repo_a: String,
    pub repo_b: String,
    pub shared_clones: usize,
    pub shared_tokens: usize,
}

/// Aggregated summary of an organization-wide Federation Hub scan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HubScanSummary {
    pub hub_name: String,
    pub total_repos: usize,
    pub repos: Vec<HubRepoConfig>,
    pub total_files: usize,
    pub total_tokens: usize,
    pub total_clones: usize,
    pub cross_repo_clones: usize,
    pub cross_repo_clusters: usize,
    pub organization_dry_score: f64,
    pub cross_repo_duplication_pct: f64,
    pub duplication_matrix: Vec<RepoDuplicationMetric>,
    pub clusters: Vec<CrossRepoCluster>,
    pub top_cross_repo_pairs: Vec<CrossRepoClonePair>,
}

/// Request to extract a cross-repo cluster into a standalone shared package.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HubExtractRequest {
    pub hub_config: Option<HubConfig>,
    pub cluster_id: usize,
    pub target_package_name: String,
    pub package_type: String,
    pub target_dir: String,
    #[serde(default)]
    pub dry_run: bool,
}

/// PR update specification for an individual member repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HubRepoUpdate {
    pub repo_name: String,
    pub repo_path: String,
    pub manifest_updates: Vec<ManifestUpdate>,
    pub caller_rewrites: Vec<CallerRewrite>,
    pub patch_diff: String,
}

/// Result of synthesizing a standalone cross-repo shared package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HubExtractResult {
    pub package_name: String,
    pub package_type: String,
    pub target_dir: String,
    pub generated_files: Vec<ExtractedFile>,
    pub repo_updates: Vec<HubRepoUpdate>,
    pub lines_saved: usize,
    pub repos_updated: usize,
}
