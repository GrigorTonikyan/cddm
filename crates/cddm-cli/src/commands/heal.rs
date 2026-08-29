#![forbid(unsafe_code)]

use cddm_core::{
    AiProviderConfig, AiProviderKind, CloneLocation, HealRefactorRequest, ScanConfig,
    heal_cluster_refactor, run_scan,
};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::mpsc;

/// CLI arguments for the `cddm heal` command.
#[derive(Debug, Clone)]
pub struct HealCliArgs {
    pub directory: PathBuf,
    pub cluster: Option<usize>,
    pub pair: Option<usize>,
    pub provider_str: String,
    pub model: Option<String>,
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub max_iterations: usize,
    pub verify: bool,
    pub test_cmd: Option<String>,
    pub branch: Option<String>,
    pub fn_name: Option<String>,
    pub target_module: Option<String>,
    pub custom_instructions: Option<String>,
    pub min_tokens: usize,
}

/// Executes the CLI `cddm heal` command.
pub async fn run_heal_command(args: HealCliArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1b[36m--> Initializing CDDM AI Code Surgeon Healing Engine...\x1b[0m");

    let provider_kind = match args.provider_str.to_lowercase().as_str() {
        "gemini" => AiProviderKind::Gemini,
        "claude" => AiProviderKind::Claude,
        "openai" => AiProviderKind::OpenAi,
        "ollama" => AiProviderKind::Ollama,
        _ => AiProviderKind::Mock,
    };

    let (progress_tx, _progress_rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    let scan_config = ScanConfig {
        directory: args.directory.to_string_lossy().to_string(),
        min_tokens: args.min_tokens,
        languages: vec![],
        ignore_patterns: vec![],
        detect_type2: true,
        detect_type3: true,
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
    };

    println!("\x1b[33m--> Scanning workspace for target clone locations...\x1b[0m");
    let scan_res = run_scan(scan_config, progress_tx, cancel_flag).await?;

    let mut occurrences = Vec::new();

    if let Some(target_cluster_id) = args.cluster {
        if let Some(c) = scan_res
            .clone_clusters
            .iter()
            .find(|cl| cl.id == target_cluster_id)
        {
            occurrences = c.occurrences.clone();
            println!(
                "\x1b[32m--> Found Clone Cluster #{} with {} occurrence site(s)\x1b[0m",
                c.id,
                occurrences.len()
            );
        } else {
            return Err(format!(
                "Clone cluster #{} not found in scan results",
                target_cluster_id
            )
            .into());
        }
    } else if let Some(target_pair_idx) = args.pair {
        let pair_idx = target_pair_idx.saturating_sub(1);
        if pair_idx < scan_res.clone_pairs.len() {
            let p = &scan_res.clone_pairs[pair_idx];
            occurrences.push(CloneLocation {
                file: p.file_a.clone(),
                start_line: p.start_line_a,
                end_line: p.end_line_a,
                author: p.author_a.clone(),
            });
            occurrences.push(CloneLocation {
                file: p.file_b.clone(),
                start_line: p.start_line_b,
                end_line: p.end_line_b,
                author: p.author_b.clone(),
            });
            println!(
                "\x1b[32m--> Found Clone Pair #{}: '{}' <-> '{}'\x1b[0m",
                target_pair_idx, p.file_a, p.file_b
            );
        } else {
            return Err(
                format!("Clone pair #{} not found in scan results", target_pair_idx).into(),
            );
        }
    } else if !scan_res.clone_clusters.is_empty() {
        let c = &scan_res.clone_clusters[0];
        occurrences = c.occurrences.clone();
        println!(
            "\x1b[32m--> Defaulting to Top Clone Cluster #{} with {} occurrence(s)\x1b[0m",
            c.id,
            occurrences.len()
        );
    } else if !scan_res.clone_pairs.is_empty() {
        let p = &scan_res.clone_pairs[0];
        occurrences.push(CloneLocation {
            file: p.file_a.clone(),
            start_line: p.start_line_a,
            end_line: p.end_line_a,
            author: p.author_a.clone(),
        });
        occurrences.push(CloneLocation {
            file: p.file_b.clone(),
            start_line: p.start_line_b,
            end_line: p.end_line_b,
            author: p.author_b.clone(),
        });
        println!(
            "\x1b[32m--> Defaulting to Top Clone Pair #1: '{}' <-> '{}'\x1b[0m",
            p.file_a, p.file_b
        );
    } else {
        println!(
            "\x1b[32m[PASS] Zero code duplication detected in workspace! Nothing to heal.\x1b[0m"
        );
        return Ok(());
    }

    let req = HealRefactorRequest {
        cluster_id: args.cluster,
        pair_id: args.pair,
        occurrences,
        function_name: args.fn_name,
        target_module: args.target_module,
        custom_instructions: args.custom_instructions,
        provider_config: AiProviderConfig {
            provider: provider_kind,
            model: args.model,
            api_key: args.api_key,
            endpoint: args.endpoint,
            temperature: Some(0.2),
            timeout_secs: Some(60),
        },
        max_iterations: args.max_iterations,
        apply_branch: args.branch,
        verify: args.verify,
        test_cmd: args.test_cmd,
        workspace_root: Some(args.directory.clone()),
    };

    println!(
        "\x1b[35m--> Dispatching autonomous healing loop (Provider: {:?}, Max Iterations: \
         {})...\x1b[0m",
        req.provider_config.provider, req.max_iterations
    );

    let result = heal_cluster_refactor(&args.directory, &req).await?;

    println!("\n\x1b[36m=== Healing Session Summary ===\x1b[0m");
    for it in &result.iterations {
        let status = if it.test_passed {
            "\x1b[32m[PASS]\x1b[0m"
        } else if it.patch_applied {
            "\x1b[33m[APPLIED - TEST FAILED]\x1b[0m"
        } else {
            "\x1b[31m[PATCH FAILED]\x1b[0m"
        };
        println!("  Iteration {}: {}", it.iteration, status);
        if let Some(err) = &it.error_feedback {
            println!(
                "    \x1b[31mFeedback:\x1b[0m {}",
                err.lines().next().unwrap_or("")
            );
        }
    }

    if result.success {
        println!("\n\x1b[32m[SUCCESS] {}\x1b[0m", result.message);
        if let Some(b) = &result.branch_created {
            println!(
                "\x1b[36m--> Refactoring committed to Git branch: '{}'\x1b[0m",
                b
            );
        }
    } else {
        println!("\n\x1b[33m[WARNING] {}\x1b[0m", result.message);
    }

    Ok(())
}
