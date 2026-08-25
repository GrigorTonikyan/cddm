#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// A point-in-time duplication metrics snapshot for a Git commit.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TimelineSnapshot {
    /// Full commit hash (40-character hex string)
    pub commit_hash: String,
    /// Short commit hash (7-character hex string)
    pub short_hash: String,
    /// Commit author name
    pub author: String,
    /// Commit timestamp (Unix epoch seconds)
    pub commit_time: i64,
    /// Formatted commit date string (YYYY-MM-DD HH:MM:SS)
    pub formatted_date: String,
    /// Commit message (first line summary)
    pub message: String,
    /// Optional Git tag pointing to this commit
    pub tag: Option<String>,
    /// Total number of files analyzed
    pub total_files: usize,
    /// Total number of tokens analyzed
    pub total_tokens: usize,
    /// Total number of pairwise code clones
    pub total_clones: usize,
    /// Total number of clone clusters
    pub total_clusters: usize,
    /// Duplication percentage (0.0 to 100.0)
    pub duplication_percentage: f64,
    /// DRY Health Score (0.0 to 100.0)
    pub dry_health_score: f64,
}

/// Metric tracking file modification frequency and duplicate clone association.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileChurnMetric {
    /// Relative file path
    pub file_path: String,
    /// Number of times modified across timeline
    pub commit_count: usize,
    /// Active clone count in current snapshot
    pub clone_count: usize,
}

/// Aggregated historical duplication trend across Git history.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TimelineTrend {
    /// Chronological list of sampled commit snapshots (oldest to newest)
    pub snapshots: Vec<TimelineSnapshot>,
    /// Initial DRY health score (from oldest sampled commit)
    pub initial_score: f64,
    /// Current DRY health score (from newest sampled commit)
    pub current_score: f64,
    /// Net DRY health score delta (current_score - initial_score)
    pub score_delta: f64,
    /// Net duplication percentage delta (current_dup - initial_dup)
    pub duplication_delta: f64,
    /// Top file churn hotspots
    pub churn_hotspots: Vec<FileChurnMetric>,
}
