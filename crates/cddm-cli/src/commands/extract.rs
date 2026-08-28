#![forbid(unsafe_code)]

use crate::types::ExtractArgs;
use cddm_core::{
    ExtractRequest, ExtractResult, ExtractTargetKind, ScanConfig, apply_shared_extraction,
    generate_shared_extraction, run_scan,
};
use std::path::Path;

/// Executes the CLI `cddm extract` command.
pub async fn run_extract_command(args: ExtractArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "\x1b[36m--> Analyzing codebase in '{}' for shared extraction...\x1b[0m",
        args.directory.display()
    );

    let target_kind = match args.crate_type.as_str() {
        "crate" => ExtractTargetKind::NewCrate,
        "module" => ExtractTargetKind::NewModule,
        "existing" => ExtractTargetKind::ExistingModule,
        _ => ExtractTargetKind::Auto,
    };

    let scan_config = ScanConfig {
        directory: args.directory.to_string_lossy().to_string(),
        min_tokens: args.min_tokens,
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

    let (progress_tx, _progress_rx) = tokio::sync::mpsc::channel(100);
    let cancel_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let scan_res = run_scan(scan_config, progress_tx, cancel_flag).await?;

    let occurrences = if let Some(cluster_id) = args.cluster {
        let found = scan_res
            .clone_clusters
            .iter()
            .find(|c| c.id == cluster_id)
            .ok_or_else(|| format!("Clone cluster #{} not found", cluster_id))?;
        found.occurrences.clone()
    } else if let Some(pair_id) = args.pair {
        let found = scan_res
            .clone_pairs
            .get(pair_id.saturating_sub(1))
            .ok_or_else(|| format!("Clone pair #{} not found", pair_id))?;
        vec![
            cddm_core::CloneLocation {
                file: found.file_a.clone(),
                start_line: found.start_line_a,
                end_line: found.end_line_a,
                author: None,
            },
            cddm_core::CloneLocation {
                file: found.file_b.clone(),
                start_line: found.start_line_b,
                end_line: found.end_line_b,
                author: None,
            },
        ]
    } else if let Some(first_cluster) = scan_res.clone_clusters.first() {
        println!(
            "\x1b[33mNo --pair or --cluster specified. Defaulting to largest cluster #{}\x1b[0m",
            first_cluster.id
        );
        first_cluster.occurrences.clone()
    } else if let Some(first_pair) = scan_res.clone_pairs.first() {
        println!("\x1b[33mNo --pair or --cluster specified. Defaulting to first clone pair\x1b[0m");
        vec![
            cddm_core::CloneLocation {
                file: first_pair.file_a.clone(),
                start_line: first_pair.start_line_a,
                end_line: first_pair.end_line_a,
                author: None,
            },
            cddm_core::CloneLocation {
                file: first_pair.file_b.clone(),
                start_line: first_pair.start_line_b,
                end_line: first_pair.end_line_b,
                author: None,
            },
        ]
    } else {
        return Err("No duplicate code clones found to extract".into());
    };

    let request = ExtractRequest {
        occurrences,
        target_path: args.target,
        custom_function_name: args.fn_name,
        target_kind,
        custom_parameter_names: None,
        generate_tests: args.generate_tests,
        generate_benchmarks: args.generate_benchmarks,
        dry_run: !args.apply || args.dry_run,
    };

    let result = if args.apply && !args.dry_run {
        apply_shared_extraction(Path::new("."), &request)?
    } else {
        generate_shared_extraction(Path::new("."), &request)?
    };

    print_extraction_summary(&result, args.apply && !args.dry_run);

    Ok(())
}

fn print_extraction_summary(result: &ExtractResult, applied: bool) {
    let mode_str = if applied {
        "COMMITTED"
    } else {
        "DRY-RUN PREVIEW"
    };
    println!(
        "\n\x1b[32m=== Shared Extraction Summary [{}] ===\x1b[0m",
        mode_str
    );
    println!(
        "  Helper Function:      \x1b[35m{}\x1b[0m",
        result.function_name
    );
    println!(
        "  Target Destination:   \x1b[36m{}\x1b[0m",
        result.target_path
    );
    println!("  Target Strategy:      {:?}", result.target_kind);
    println!(
        "  Lines of Code Saved:  \x1b[32m~{}\x1b[0m",
        result.total_lines_saved
    );
    println!("  Signature:            {}", result.helper_signature);

    if !result.generated_files.is_empty() {
        println!(
            "\n\x1b[36mGenerated Target Files ({})\x1b[0m:",
            result.generated_files.len()
        );
        for f in &result.generated_files {
            println!(
                "  [+] \x1b[32m{}\x1b[0m ({} bytes)",
                f.file_path,
                f.content.len()
            );
        }
    }

    if !result.test_files.is_empty() {
        println!(
            "\n\x1b[36mSynthesized Unit Tests ({})\x1b[0m:",
            result.test_files.len()
        );
        for t in &result.test_files {
            println!(
                "  [+] \x1b[32m{}\x1b[0m ({} bytes)",
                t.file_path,
                t.content.len()
            );
        }
    }

    if !result.benchmark_files.is_empty() {
        println!(
            "\n\x1b[36mSynthesized Micro-Benchmarks ({})\x1b[0m:",
            result.benchmark_files.len()
        );
        for b in &result.benchmark_files {
            println!(
                "  [+] \x1b[32m{}\x1b[0m ({} bytes)",
                b.file_path,
                b.content.len()
            );
        }
    }

    if !result.manifest_updates.is_empty() {
        println!(
            "\n\x1b[36mManifest Updates ({})\x1b[0m:",
            result.manifest_updates.len()
        );
        for m in &result.manifest_updates {
            println!(
                "  [*] \x1b[33m{}\x1b[0m -> added dependency '{}'",
                m.manifest_path, m.dependency_name
            );
        }
    }

    if !result.caller_rewrites.is_empty() {
        println!(
            "\n\x1b[36mCaller File Rewrites ({})\x1b[0m:",
            result.caller_rewrites.len()
        );
        for c in &result.caller_rewrites {
            let imp_str = c.injected_import.as_deref().unwrap_or("none");
            println!(
                "  [~] \x1b[35m{}\x1b[0m (Injected Import: \x1b[34m{}\x1b[0m)",
                c.file_path, imp_str
            );
        }
    }

    if !applied {
        println!(
            "\n\x1b[33m[TIP] Run with --apply to commit these changes and write files to \
             disk.\x1b[0m\n"
        );
    } else {
        println!("\n\x1b[32m[SUCCESS] Extraction applied successfully to workspace!\x1b[0m\n");
    }
}
