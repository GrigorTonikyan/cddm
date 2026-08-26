#![forbid(unsafe_code)]

use super::types::*;
use axum::{extract::Json, http::StatusCode};
use cddm_core::{
    ApplyRefactorBranchRequest, ApplyRefactorBranchResult, AstRewriteResult,
    ClusterRefactorSuggestion, RefactorSandboxRequest, RefactorSandboxResult, RefactorSuggestion,
    VerifyRefactorRequest, VerifyRefactorResult, analyze_clone_refactoring,
    analyze_cluster_refactoring, apply_cluster_refactor_branch, generate_ai_refactor_prompt,
    generate_ast_cluster_refactor, preview_cluster_refactor, verify_refactor_test_suite,
};
use std::path::Path;

pub async fn refactor_handler(
    Json(req): Json<RefactorRequest>,
) -> Result<Json<RefactorSuggestion>, (StatusCode, String)> {
    match analyze_clone_refactoring(
        &req.file_a,
        (req.start_line_a, req.end_line_a),
        &req.file_b,
        (req.start_line_b, req.end_line_b),
    ) {
        Ok(suggestion) => Ok(Json(suggestion)),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

pub async fn refactor_cluster_handler(
    Json(req): Json<ClusterRefactorRequest>,
) -> Result<Json<ClusterRefactorSuggestion>, (StatusCode, String)> {
    match analyze_cluster_refactoring(&req.cluster_id, &req.occurrences) {
        Ok(suggestion) => Ok(Json(suggestion)),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

pub async fn refactor_sandbox_handler(
    Json(req): Json<RefactorSandboxRequest>,
) -> Result<Json<RefactorSandboxResult>, (StatusCode, String)> {
    match preview_cluster_refactor(
        &req.occurrences,
        req.custom_function_name.as_deref(),
        req.target_module_path.as_deref(),
        req.custom_parameter_names.as_deref(),
    ) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

pub async fn refactor_apply_branch_handler(
    Json(req): Json<ApplyRefactorBranchRequest>,
) -> Result<Json<ApplyRefactorBranchResult>, (StatusCode, String)> {
    match apply_cluster_refactor_branch(
        Path::new("."),
        &req.patch,
        req.branch_name.as_deref(),
        req.create_branch,
    ) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

pub async fn refactor_ai_prompt_handler(
    Json(req): Json<cddm_core::AiRefactorPromptRequest>,
) -> Result<Json<AiPromptResponse>, (StatusCode, String)> {
    let prompt = generate_ai_refactor_prompt(&req);
    Ok(Json(AiPromptResponse { prompt }))
}

pub async fn refactor_ast_handler(
    Json(payload): Json<RefactorSandboxRequest>,
) -> Result<Json<AstRewriteResult>, (StatusCode, String)> {
    generate_ast_cluster_refactor(
        &payload.occurrences,
        payload.custom_function_name.as_deref(),
        payload.target_module_path.as_deref(),
        payload.custom_parameter_names.as_deref(),
    )
    .map(Json)
    .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

pub async fn refactor_verify_handler(
    Json(payload): Json<VerifyRefactorRequest>,
) -> Result<Json<VerifyRefactorResult>, (StatusCode, String)> {
    let dir = Path::new(&payload.directory);
    verify_refactor_test_suite(
        dir,
        payload.test_command.as_deref(),
        payload.branch_name.as_deref(),
        payload.timeout_seconds,
    )
    .map(Json)
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

pub async fn refactor_heal_handler(
    Json(req): Json<cddm_core::HealRefactorRequest>,
) -> Result<Json<cddm_core::HealRefactorResult>, (StatusCode, String)> {
    let dir = req
        .workspace_root
        .clone()
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    match cddm_core::heal_cluster_refactor(&dir, &req).await {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

pub async fn cache_export_handler(
    Json(req): Json<CacheExportRequest>,
) -> Result<Json<cddm_core::CachePackSummary>, (StatusCode, String)> {
    let db_path = req
        .cache_dir
        .unwrap_or_else(|| std::path::PathBuf::from(".cddm/cache.db"));
    let out_path = req
        .output_pack_path
        .unwrap_or_else(|| std::path::PathBuf::from("cddm-cache.cddmpack"));
    cddm_core::export_cache_pack(&db_path, &out_path)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

pub async fn cache_import_handler(
    Json(req): Json<CacheImportRequest>,
) -> Result<Json<cddm_core::CachePackSummary>, (StatusCode, String)> {
    let target_dir = req
        .target_cache_dir
        .unwrap_or_else(|| std::path::PathBuf::from(".cddm"));
    cddm_core::import_cache_pack(&req.pack_file, &target_dir)
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

pub async fn monorepo_handler(
    Json(req): Json<MonorepoScanRequest>,
) -> Result<Json<cddm_core::MonorepoScanSummary>, (StatusCode, String)> {
    let dir = req
        .directory
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let config = cddm_core::ScanConfig {
        directory: dir.to_string_lossy().to_string(),
        min_tokens: req.min_tokens.unwrap_or(50),
        languages: vec![],
        ignore_patterns: vec![],
        detect_type2: true,
        scan_self: false,
        enable_git_blame: false,
        cache_dir: None,
        enable_cache: false,
        cddmignore_path: None,
        ignore_tests: false,
        ignore_mocks: false,
        ignore_generated: true,
        rules_path: None,
        enforce_policies: false,
        cross_language: false,
    };
    cddm_core::run_monorepo_scan(&dir, &config)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
