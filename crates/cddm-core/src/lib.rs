pub mod ai_prompt;
pub mod ast;
pub mod blame;
pub mod cache;
pub mod cluster;
pub mod detector;
pub mod diff;
pub mod fingerprint;
pub mod grammar;
pub mod io;
pub mod pr_comment;
pub mod refactor;
pub mod sarif;
pub mod simd;
pub mod suppression;
pub mod timeline;
pub mod tokenizer;
pub mod types;
pub mod watcher;
pub mod workflow;

pub use ai_prompt::{AiOccurrenceContext, AiRefactorPromptRequest, generate_ai_refactor_prompt};
pub use cache::{CachedFileEntry, DiskFingerprintCache};
pub use cluster::cluster_clone_pairs;
pub use detector::run_scan;
pub use diff::{get_changed_files_between_refs, run_diff_scan};
pub use io::{FileSource, MMAP_THRESHOLD_BYTES, read_file_source};
pub use pr_comment::generate_pr_markdown_comment;
pub use refactor::{
    ApplyPatchResult, ClusterRefactorSuggestion, ClusterSiteRefactor, ParameterDifference,
    RefactorSuggestion, analyze_clone_refactoring, analyze_cluster_refactoring,
    analyze_cluster_snippets_refactoring, analyze_snippets_refactoring,
    apply_cluster_refactor_branch, apply_patch_to_workspace, parse_unified_patch,
    preview_cluster_refactor,
};
pub use sarif::{SarifReport, generate_sarif_json, generate_sarif_report};
pub use simd::compute_kgram_rolling_hashes;
pub use suppression::SuppressionEngine;
pub use timeline::collect_git_timeline;
pub use types::{
    ApplyRefactorBranchRequest, ApplyRefactorBranchResult, CloneCluster, CloneLocation, ClonePair,
    CloneStatus, CloneType, DEFAULT_CACHE_FILE, DEFAULT_DIRECTORY, DEFAULT_IGNORE_PATTERNS,
    DEFAULT_MIN_TOKENS, DiffClonePair, DiffScanResult, DiffSummary, FileChurnMetric, HookStatus,
    LanguageStats, LineSpan, MAX_HEALTH_SCORE, MIN_HEALTH_SCORE, NormalizedToken,
    RefactorSandboxRequest, RefactorSandboxResult, ScanConfig, ScanPhase, ScanProgress, ScanResult,
    SuppressionConfig, SuppressionDirective, SuppressionRule, TimelineSnapshot, TimelineTrend,
    WorkflowPlatform,
};
pub use watcher::CddmWatcher;
pub use workflow::{
    generate_azure_pipelines, generate_github_workflow, generate_gitlab_ci, get_hook_status,
    install_git_hook, uninstall_git_hook,
};
