#![forbid(unsafe_code)]

pub mod detector;
pub mod pruner;
pub mod reachability;
pub mod static_analyzer;
pub mod types;

pub use detector::run_dead_code_detection;
pub use pruner::prune_dead_clone_clusters;
pub use reachability::trace_cross_package_reachability;
pub use static_analyzer::analyze_static_dead_code;
pub use types::{
    CrossPackageReachabilitySummary, DeadClonePruneConfig, DeadClonePruneResult, DeadCodeConfig,
    DeadCodeItem, DeadCodeKind, DeadCodeSummary, PruneActionStatus, PrunedItem, ReachabilityStatus,
    SymbolReachability,
};
