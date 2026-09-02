#![forbid(unsafe_code)]

pub mod detector;
pub mod pruner;
pub mod static_analyzer;
pub mod types;

pub use detector::run_dead_code_detection;
pub use pruner::prune_dead_clone_clusters;
pub use static_analyzer::analyze_static_dead_code;
pub use types::{
    DeadClonePruneConfig, DeadClonePruneResult, DeadCodeConfig, DeadCodeItem, DeadCodeKind,
    DeadCodeSummary, PruneActionStatus, PrunedItem,
};
