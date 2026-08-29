#![forbid(unsafe_code)]

use crate::formatters::{
    print_ast_refactor_recommendation, print_cluster_refactor_recommendation,
    print_refactor_recommendation,
};
use cddm_core::{
    AiRefactorPromptRequest, CloneLocation, CloneType, ScanConfig, analyze_clone_refactoring,
    analyze_cluster_refactoring, apply_cluster_refactor_branch, generate_ai_refactor_prompt,
    generate_ast_cluster_refactor, run_scan, verify_refactor_test_suite,
};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

fn write_output_file(
    output: Option<&PathBuf>,
    content: &str,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(out_path) = output {
        fs::write(out_path, content)?;
        println!("\n{label} written to '{}'.", out_path.display());
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn run_refactor_command(
    pair: usize,
    cluster: Option<usize>,
    directory: PathBuf,
    min_tokens: usize,
    output: Option<PathBuf>,
    prompt: bool,
    ast: bool,
    fn_name: Option<String>,
    target_module: Option<String>,
    apply_branch: Option<String>,
    verify: bool,
    test_cmd: Option<String>,
    languages: Vec<String>,
    ignore: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = ScanConfig {
        directory: directory.to_string_lossy().to_string(),
        min_tokens,
        languages,
        ignore_patterns: if ignore.is_empty() {
            ScanConfig::default().ignore_patterns
        } else {
            ignore
        },
        detect_type2: true,
        scan_self: true,
        enable_git_blame: false,
        cache_dir: None,
        enable_cache: true,
        cddmignore_path: None,
        ignore_tests: false,
        ignore_mocks: false,
        ignore_generated: true,
        rules_path: None,
        enforce_policies: false,
        cross_language: false,
        threads: None,
    };

    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    let result = run_scan(config, tx, cancel_flag).await?;
    let patch_to_apply: String;

    if ast {
        let occurrences = if let Some(c_idx) = cluster {
            if result.clone_clusters.is_empty() {
                println!("No duplicate code clone clusters found to refactor.");
                return Ok(());
            }
            let target_idx = if c_idx > 0 && c_idx <= result.clone_clusters.len() {
                c_idx - 1
            } else {
                0
            };
            result.clone_clusters[target_idx].occurrences.clone()
        } else {
            if result.clone_pairs.is_empty() {
                println!("No duplicate code clone pairs found to refactor.");
                return Ok(());
            }
            let target_idx = if pair > 0 && pair <= result.clone_pairs.len() {
                pair - 1
            } else {
                0
            };
            let selected = &result.clone_pairs[target_idx];
            vec![
                CloneLocation {
                    file: selected.file_a.clone(),
                    start_line: selected.start_line_a,
                    end_line: selected.end_line_a,
                    author: selected.author_a.clone(),
                },
                CloneLocation {
                    file: selected.file_b.clone(),
                    start_line: selected.start_line_b,
                    end_line: selected.end_line_b,
                    author: selected.author_b.clone(),
                },
            ]
        };

        let ast_res = generate_ast_cluster_refactor(
            &occurrences,
            fn_name.as_deref(),
            target_module.as_deref(),
            None,
        )?;

        patch_to_apply = ast_res.unified_patch.clone();

        if prompt {
            let prompt_req = AiRefactorPromptRequest {
                clone_type: CloneType::Exact,
                similarity: 1.0,
                token_count: 100,
                lines_saved_est: ast_res.total_lines_saved,
                function_name: ast_res.function_name.clone(),
                target_module: ast_res.target_module_path.clone(),
                occurrences: cddm_core::occurrences_to_ai_context(&occurrences),
                invariant_body: ast_res.helper_function_code.clone(),
                parameters: ast_res
                    .inferred_parameters
                    .iter()
                    .map(|p| format!("{}: {}", p.name, p.inferred_type))
                    .collect(),
                context_slices: None,
                custom_instructions: None,
            };
            let prompt_text = generate_ai_refactor_prompt(&prompt_req);
            println!("{}", prompt_text);
            write_output_file(output.as_ref(), &prompt_text, "AI refactoring prompt")?;
        } else {
            print_ast_refactor_recommendation(cluster, &ast_res);
            write_output_file(
                output.as_ref(),
                &ast_res.unified_patch,
                "AST-native unified patch",
            )?;
        }
    } else if let Some(c_idx) = cluster {
        if result.clone_clusters.is_empty() {
            println!("No duplicate code clone clusters found to refactor.");
            return Ok(());
        }

        let target_idx = if c_idx > 0 && c_idx <= result.clone_clusters.len() {
            c_idx - 1
        } else {
            println!(
                "Error: Cluster index {} out of bounds (1..{})",
                c_idx,
                result.clone_clusters.len()
            );
            return Ok(());
        };

        let selected_cluster = &result.clone_clusters[target_idx];
        let suggestion = analyze_cluster_refactoring(
            &selected_cluster.id.to_string(),
            &selected_cluster.occurrences,
        )?;
        patch_to_apply = suggestion.unified_patch.clone();

        if prompt {
            let prompt_req = AiRefactorPromptRequest {
                clone_type: selected_cluster.clone_type.clone(),
                similarity: selected_cluster.similarity,
                token_count: selected_cluster.token_count,
                lines_saved_est: suggestion.total_lines_saved,
                function_name: suggestion.suggested_function_name.clone(),
                target_module: suggestion.target_module_hint.clone(),
                occurrences: cddm_core::occurrences_to_ai_context(&selected_cluster.occurrences),
                invariant_body: suggestion.common_body_lines.join("\n"),
                parameters: suggestion
                    .sites
                    .iter()
                    .flat_map(|s| {
                        s.parameter_differences
                            .iter()
                            .map(|p| p.fragment_a_code.clone())
                    })
                    .collect(),
                context_slices: None,
                custom_instructions: None,
            };
            let prompt_text = generate_ai_refactor_prompt(&prompt_req);
            println!("{}", prompt_text);
            write_output_file(output.as_ref(), &prompt_text, "AI refactoring prompt")?;
        } else {
            print_cluster_refactor_recommendation(selected_cluster, &suggestion);
            write_output_file(
                output.as_ref(),
                &suggestion.unified_patch,
                "Multi-site unified patch",
            )?;
        }
    } else {
        if result.clone_pairs.is_empty() {
            println!("No duplicate code clone pairs found to refactor.");
            return Ok(());
        }

        let target_idx = if pair > 0 && pair <= result.clone_pairs.len() {
            pair - 1
        } else {
            eprintln!(
                "Warning: Specified pair index {} out of range (total: {}); defaulting to 1.",
                pair,
                result.clone_pairs.len()
            );
            0
        };

        let selected = &result.clone_pairs[target_idx];
        let suggestion = analyze_clone_refactoring(
            &selected.file_a,
            (selected.start_line_a, selected.end_line_a),
            &selected.file_b,
            (selected.start_line_b, selected.end_line_b),
        )?;
        patch_to_apply = suggestion.unified_patch.clone();

        if prompt {
            let locs = vec![
                CloneLocation {
                    file: selected.file_a.clone(),
                    start_line: selected.start_line_a,
                    end_line: selected.end_line_a,
                    author: selected.author_a.clone(),
                },
                CloneLocation {
                    file: selected.file_b.clone(),
                    start_line: selected.start_line_b,
                    end_line: selected.end_line_b,
                    author: selected.author_b.clone(),
                },
            ];

            let prompt_req = AiRefactorPromptRequest {
                clone_type: selected.clone_type.clone(),
                similarity: selected.similarity,
                token_count: selected.token_count,
                lines_saved_est: suggestion.lines_saved,
                function_name: suggestion.suggested_function_name.clone(),
                target_module: suggestion.target_module_hint.clone(),
                occurrences: cddm_core::occurrences_to_ai_context(&locs),
                invariant_body: suggestion.common_body_lines.join("\n"),
                parameters: suggestion
                    .parameter_differences
                    .iter()
                    .map(|p| p.fragment_a_code.clone())
                    .collect(),
                context_slices: None,
                custom_instructions: None,
            };
            let prompt_text = generate_ai_refactor_prompt(&prompt_req);
            println!("{}", prompt_text);
            write_output_file(output.as_ref(), &prompt_text, "AI refactoring prompt")?;
        } else {
            print_refactor_recommendation(selected, &suggestion);
            write_output_file(output.as_ref(), &suggestion.unified_patch, "Unified patch")?;
        }
    }

    if let Some(branch_name) = apply_branch
        && !patch_to_apply.is_empty()
    {
        match apply_cluster_refactor_branch(&directory, &patch_to_apply, Some(&branch_name), true) {
            Ok(res) => {
                println!(
                    "\n[PASS] Refactoring patch applied to branch '{}':",
                    branch_name
                );
                println!("  Modified files ({}):", res.modified_files.len());
                for f in &res.modified_files {
                    println!("    - {}", f);
                }
            }
            Err(e) => {
                eprintln!(
                    "\n[ERROR] Failed to apply refactoring to branch '{}': {}",
                    branch_name, e
                );
            }
        }
    }

    if verify {
        println!("\n=== CDDM — Closed-Loop Test Suite Verification ===");
        match verify_refactor_test_suite(&directory, test_cmd.as_deref(), None, None) {
            Ok(v_res) => {
                if v_res.success {
                    println!(
                        "[PASS] {} (Exit Code: 0, Duration: {}ms)",
                        v_res.command_executed, v_res.duration_ms
                    );
                } else {
                    println!(
                        "[FAIL] {} (Exit Code: {}, Duration: {}ms)",
                        v_res.command_executed, v_res.exit_code, v_res.duration_ms
                    );
                    if !v_res.stderr_snippet.is_empty() {
                        println!("\n--- Stderr Output ---\n{}", v_res.stderr_snippet);
                    } else if !v_res.stdout_snippet.is_empty() {
                        println!("\n--- Stdout Output ---\n{}", v_res.stdout_snippet);
                    }
                }
            }
            Err(e) => {
                eprintln!("[ERROR] Test verification failed to execute: {}", e);
            }
        }
    }

    Ok(())
}
