pub mod ai;
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
pub mod monorepo;
pub mod policy;
pub mod pr_comment;
pub mod refactor;
pub mod sarif;
pub mod semantic_graph;
pub mod simd;
pub mod suppression;
pub mod timeline;
pub mod tokenizer;
pub mod types;
pub mod watcher;
pub mod workflow;

pub use ai::{
    AiProvider, AiProviderConfig, AiProviderKind, HealIterationLog, HealRefactorRequest,
    HealRefactorResult, create_ai_provider, heal_cluster_refactor,
};
pub use ai_prompt::{AiOccurrenceContext, AiRefactorPromptRequest, generate_ai_refactor_prompt};
pub use cache::pack::{
    CACHE_PACK_VERSION, CachePackManifest, CachePackSummary, export_cache_pack, import_cache_pack,
};
pub use cache::{CachedFileEntry, DiskFingerprintCache};
pub use cluster::cluster_clone_pairs;
pub use detector::run_scan;
pub use diff::{get_changed_files_between_refs, run_diff_scan};
pub use io::{FileSource, MMAP_THRESHOLD_BYTES, read_file_source};
pub use monorepo::{
    MonorepoScanSummary, MonorepoWorkspace, discover_workspaces, run_monorepo_scan,
};
pub use policy::PolicyEngine;
pub use pr_comment::generate_pr_markdown_comment;
pub use refactor::{
    ApplyPatchResult, ClusterRefactorSuggestion, ClusterSiteRefactor, ParameterDifference,
    RefactorSuggestion, analyze_clone_refactoring, analyze_cluster_refactoring,
    analyze_cluster_snippets_refactoring, analyze_snippets_refactoring,
    apply_cluster_refactor_branch, apply_patch_to_workspace, generate_ast_cluster_refactor,
    parse_unified_patch, preview_cluster_refactor, verify_refactor_test_suite,
};
pub use sarif::{SarifReport, generate_sarif_json, generate_sarif_report};
pub use semantic_graph::{
    CfgEdge, CfgEdgeType, CfgNode, CfgNodeType, ControlFlowGraph, CrossLanguageClonePair,
    HybridSimilarity, PdgEdge, PdgEdgeKind, ProgramDependenceGraph, SemanticCloneMatch,
    SemanticComparisonResponse, build_pdg_from_cfg, calculate_embedding_similarity,
    calculate_graph_similarity, compute_hybrid_similarity, compute_weisfeiler_lehman_hash,
    extract_cfgs_from_source, scan_cross_language_workspace,
};
pub use simd::compute_kgram_rolling_hashes;
pub use suppression::SuppressionEngine;
pub use timeline::collect_git_timeline;
pub use types::{
    ApplyRefactorBranchRequest, ApplyRefactorBranchResult, AstRewriteResult, AstRewrittenFile,
    BoundaryRule, CloneCluster, CloneLocation, ClonePair, CloneStatus, CloneType,
    DEFAULT_CACHE_FILE, DEFAULT_DIRECTORY, DEFAULT_IGNORE_PATTERNS, DEFAULT_MIN_TOKENS,
    DEFAULT_RULES_FILE, DiffClonePair, DiffScanResult, DiffSummary, FileChurnMetric, HookStatus,
    InferredParameter, LanguageStats, LimitRule, LineSpan, MAX_HEALTH_SCORE, MIN_HEALTH_SCORE,
    NormalizedToken, PolicyConfig, PolicyEvaluationResult, PolicySeverity, PolicyViolation,
    RefactorSandboxRequest, RefactorSandboxResult, ScanConfig, ScanPhase, ScanProgress, ScanResult,
    SuppressionConfig, SuppressionDirective, SuppressionRule, TimelineSnapshot, TimelineTrend,
    VerifyRefactorRequest, VerifyRefactorResult, WorkflowPlatform, ZeroDuplicationRule,
};
pub use watcher::CddmWatcher;
pub use workflow::{
    generate_azure_pipelines, generate_github_workflow, generate_gitlab_ci, get_hook_status,
    install_git_hook, uninstall_git_hook,
};
