#![forbid(unsafe_code)]

use super::types::*;
use axum::{
    extract::Json,
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
};
use cddm_core::{
    ApplyRefactorBranchRequest, ApplyRefactorBranchResult, AstRewriteResult,
    ClusterRefactorSuggestion, RefactorSandboxRequest, RefactorSandboxResult, RefactorSuggestion,
    VerifyRefactorRequest, VerifyRefactorResult, analyze_clone_refactoring,
    analyze_cluster_refactoring, apply_cluster_refactor_branch, generate_ai_refactor_prompt,
    generate_ast_cluster_refactor, preview_cluster_refactor, verify_refactor_test_suite,
};
use std::convert::Infallible;
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

pub async fn refactor_stream_handler(
    Json(req): Json<RefactorStreamRequest>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let locs = vec![
        cddm_core::CloneLocation {
            file: req.file_a.clone(),
            start_line: req.start_line_a,
            end_line: req.end_line_a,
            author: None,
        },
        cddm_core::CloneLocation {
            file: req.file_b.clone(),
            start_line: req.start_line_b,
            end_line: req.end_line_b,
            author: None,
        },
    ];

    let suggestion = analyze_clone_refactoring(
        &req.file_a,
        (req.start_line_a, req.end_line_a),
        &req.file_b,
        (req.start_line_b, req.end_line_b),
    );

    let prompt_req = match &suggestion {
        Ok(sug) => cddm_core::AiRefactorPromptRequest {
            clone_type: cddm_core::CloneType::Exact,
            similarity: 1.0,
            token_count: 50,
            lines_saved_est: sug.lines_saved,
            function_name: sug.suggested_function_name.clone(),
            target_module: sug.target_module_hint.clone(),
            occurrences: cddm_core::occurrences_to_ai_context(&locs),
            invariant_body: sug.common_body_lines.join("\n"),
            parameters: sug
                .parameter_differences
                .iter()
                .map(|p| p.fragment_a_code.clone())
                .collect(),
            context_slices: None,
            custom_instructions: None,
        },
        Err(_) => cddm_core::AiRefactorPromptRequest {
            clone_type: cddm_core::CloneType::Exact,
            similarity: 1.0,
            token_count: 50,
            lines_saved_est: 10,
            function_name: "deduplicated_function".to_string(),
            target_module: "shared".to_string(),
            occurrences: cddm_core::occurrences_to_ai_context(&locs),
            invariant_body: String::new(),
            parameters: vec![],
            context_slices: None,
            custom_instructions: None,
        },
    };

    let prompt = generate_ai_refactor_prompt(&prompt_req);

    let provider_kind = match req.provider.as_deref().map(str::to_lowercase).as_deref() {
        Some("gemini") => cddm_core::AiProviderKind::Gemini,
        Some("claude") => cddm_core::AiProviderKind::Claude,
        Some("openai") => cddm_core::AiProviderKind::OpenAi,
        Some("ollama") => cddm_core::AiProviderKind::Ollama,
        Some("custom") => cddm_core::AiProviderKind::Custom,
        Some("mock") => cddm_core::AiProviderKind::Mock,
        _ => cddm_core::AiProviderKind::Mock,
    };

    let provider_config = cddm_core::AiProviderConfig {
        provider: provider_kind,
        model: req.model,
        endpoint: req.endpoint,
        api_key: req.api_key,
        temperature: req.temperature,
        timeout_secs: Some(60),
    };

    let ai_provider = cddm_core::create_ai_provider(&provider_config);

    let (tx, rx) = tokio::sync::mpsc::channel(64);
    tokio::spawn(async move {
        match ai_provider.stream_prompt(&prompt).await {
            Ok(mut stream) => {
                use tokio_stream::StreamExt;
                while let Some(chunk_res) = stream.next().await {
                    match chunk_res {
                        Ok(chunk) => {
                            let data = serde_json::json!({ "chunk": chunk }).to_string();
                            if tx.send(Ok(Event::default().data(data))).await.is_err() {
                                return;
                            }
                        }
                        Err(e) => {
                            let data = serde_json::json!({ "error": e }).to_string();
                            let _ = tx.send(Ok(Event::default().data(data))).await;
                            return;
                        }
                    }
                }
                let data = serde_json::json!({ "done": true }).to_string();
                let _ = tx.send(Ok(Event::default().data(data))).await;
            }
            Err(e) => {
                let data = serde_json::json!({ "error": e }).to_string();
                let _ = tx.send(Ok(Event::default().data(data))).await;
            }
        }
    });

    let sse_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Sse::new(sse_stream).keep_alive(KeepAlive::default())
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
        .unwrap_or_else(|| cddm_core::resolve_default_cache_path(std::path::Path::new(".")));
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
        .unwrap_or_else(|| cddm_core::find_workspace_root(std::path::Path::new(".")).join(".cddm"));
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
        detect_type3: true,
        detect_type4: true,
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
        threads: None,
        in_tree_cache: false,
        include_ignored: false,
    };
    cddm_core::run_monorepo_scan(&dir, &config)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
