#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Represents the type of clone detected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CloneType {
    /// Exact identical text (Type 1 clone).
    Exact,
    /// Identical structure but identifiers/literals are renamed (Type 2 clone).
    Renamed,
    /// Near-miss clones with added/deleted statements (Type 3 clone).
    NearMiss,
    /// Semantic AST clones with structurally identical logic (Type 4 clone).
    Semantic,
}

/// Represents a single occurrence/location of duplicate code in a file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CloneLocation {
    /// File path of the code occurrence
    pub file: String,
    /// 1-based start line
    pub start_line: usize,
    /// 1-based end line
    pub end_line: usize,
    /// Optional author attribution
    pub author: Option<String>,
}

/// An N-way cluster (equivalence class) of code fragments sharing duplication across multiple locations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CloneCluster {
    /// 1-based cluster index
    pub id: usize,
    /// The structural clone classification of this cluster
    pub clone_type: CloneType,
    /// Maximum or representative token count of the cluster
    pub token_count: usize,
    /// Average or representative similarity score (0.0 to 1.0)
    pub similarity: f64,
    /// Unique structural hash identifying this clone cluster
    pub fragment_hash: String,
    /// List of occurrences / locations belonging to this cluster
    pub occurrences: Vec<CloneLocation>,
}

/// A matched pair of code fragments indicating a code clone.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClonePair {
    /// File path of the first fragment
    pub file_a: String,
    /// Start line in the first file
    pub start_line_a: usize,
    /// End line in the first file
    pub end_line_a: usize,
    /// File path of the second fragment
    pub file_b: String,
    /// Start line in the second file
    pub start_line_b: usize,
    /// End line in the second file
    pub end_line_b: usize,
    /// Number of matching tokens in this clone
    pub token_count: usize,
    /// Similarity percentage (0.0 to 1.0)
    pub similarity: f64,
    /// Unique hash identifying the structure of this clone
    pub fragment_hash: String,
    /// The type of the clone
    pub clone_type: CloneType,
    /// Optional author attribution for fragment A
    pub author_a: Option<String>,
    /// Optional author attribution for fragment B
    pub author_b: Option<String>,
}

/// Canonical sort and deduplication of clone pairs.
pub fn deduplicate_clone_pairs(pairs: &mut Vec<ClonePair>) {
    pairs.sort_by(|a, b| {
        a.file_a
            .cmp(&b.file_a)
            .then(a.file_b.cmp(&b.file_b))
            .then(a.start_line_a.cmp(&b.start_line_a))
            .then(a.start_line_b.cmp(&b.start_line_b))
            .then(a.end_line_a.cmp(&b.end_line_a))
            .then(a.end_line_b.cmp(&b.end_line_b))
    });
    pairs.dedup_by(|a, b| {
        a.file_a == b.file_a
            && a.file_b == b.file_b
            && a.start_line_a == b.start_line_a
            && a.end_line_a == b.end_line_a
            && a.start_line_b == b.start_line_b
            && a.end_line_b == b.end_line_b
    });
}
