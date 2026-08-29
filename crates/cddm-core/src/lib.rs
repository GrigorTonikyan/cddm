pub mod ai;
pub mod ai_prompt;
pub mod ast;
pub mod blame;
pub mod cache;
pub mod cluster;
pub mod coverage;
pub mod detector;
pub mod diff;
pub mod extract;
pub mod fingerprint;
pub mod grammar;
pub mod hub;
pub mod io;
pub mod monorepo;
pub mod neural;
pub mod overlap;
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
pub use coverage::{
    CloneCoverageMetric, CoverageCorrelationSummary, CoverageFormat, CoverageReport, ExecutionTier,
    correlate_coverage, load_coverage_report, normalize_path, parse_coverage_data, parse_lcov,
};
pub use detector::run_scan;
pub use diff::{get_changed_files_between_refs, run_diff_scan};
pub use extract::{
    CallerRewrite, ExtractRequest, ExtractResult, ExtractTargetKind, ExtractedFile, ManifestUpdate,
    apply_extraction_to_workspace, apply_shared_extraction, generate_extracted_target_files,
    generate_shared_extraction, update_workspace_manifests,
};
pub use hub::{
    CrossRepoClonePair, CrossRepoCluster, CrossRepoOccurrence, DEFAULT_HUB_CONFIG_FILE, HubConfig,
    HubExtractRequest, HubExtractResult, HubRepoConfig, HubRepoUpdate, HubScanSummary,
    RepoDuplicationMetric, build_adhoc_hub_config, generate_default_hub_config,
    generate_hub_extraction, load_hub_config, run_hub_scan,
};
pub use io::{FileSource, MMAP_THRESHOLD_BYTES, read_file_source};
pub use monorepo::{
    MonorepoScanSummary, MonorepoWorkspace, discover_workspaces, run_monorepo_scan,
};
pub use neural::{
    CodeEmbeddingVector, EquivalenceConfidence, NeuralClonePair, NeuralCodeEmbedder,
    NeuralEmbeddingConfig, NeuralMatcher, NeuralScanResult, SubwordTokenizer,
    compare_code_embeddings, compute_code_embedding, scan_neural_clones,
};
pub use overlap::{
    EcosystemAlgorithm, OverlapMatch, OverlapScanResult, RecommendedLibrary,
    get_canonical_algorithms, scan_workspace_overlap,
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
pub use watcher::{CddmWatcher, WatchDeltaReport, WatchFileEvent};
pub use workflow::{
    generate_azure_pipelines, generate_github_workflow, generate_gitlab_ci, get_hook_status,
    install_git_hook, uninstall_git_hook,
};
