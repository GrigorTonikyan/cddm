pub mod ast;
pub mod blame;
pub mod cache;
pub mod cluster;
pub mod detector;
pub mod diff;
pub mod fingerprint;
pub mod grammar;
pub mod io;
pub mod refactor;
pub mod sarif;
pub mod simd;
pub mod tokenizer;
pub mod types;
pub mod watcher;

pub use cache::{CachedFileEntry, DiskFingerprintCache};
pub use cluster::cluster_clone_pairs;
pub use detector::run_scan;
pub use diff::{get_changed_files_between_refs, run_diff_scan};
pub use io::{FileSource, MMAP_THRESHOLD_BYTES, read_file_source};
pub use refactor::{
    ApplyPatchResult, ClusterRefactorSuggestion, ClusterSiteRefactor, ParameterDifference,
    RefactorSuggestion, analyze_clone_refactoring, analyze_cluster_refactoring,
    analyze_cluster_snippets_refactoring, analyze_snippets_refactoring, apply_patch_to_workspace,
    parse_unified_patch,
};
pub use sarif::{SarifReport, generate_sarif_json, generate_sarif_report};
pub use simd::compute_kgram_rolling_hashes;
pub use types::{
    CloneCluster, CloneLocation, ClonePair, CloneStatus, CloneType, DEFAULT_CACHE_FILE,
    DEFAULT_DIRECTORY, DEFAULT_IGNORE_PATTERNS, DEFAULT_MIN_TOKENS, DiffClonePair, DiffScanResult,
    DiffSummary, LanguageStats, LineSpan, MAX_HEALTH_SCORE, MIN_HEALTH_SCORE, NormalizedToken,
    ScanConfig, ScanPhase, ScanProgress, ScanResult,
};
pub use watcher::CddmWatcher;
