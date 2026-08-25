#![forbid(unsafe_code)]

use crate::formatters::{
    print_ast_refactor_recommendation, print_cluster_refactor_recommendation,
    print_refactor_recommendation,
};
use cddm_core::{
    AiOccurrenceContext, AiRefactorPromptRequest, CloneLocation, CloneType, LineSpan, ScanConfig,
    analyze_clone_refactoring, analyze_cluster_refactoring, apply_cluster_refactor_branch,
    generate_ai_refactor_prompt, generate_ast_cluster_refactor, run_scan,
    verify_refactor_test_suite,
};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

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
                occurrences: occurrences
                    .iter()
                    .map(|occ| {
                        let snippet = fs::read_to_string(&occ.file).unwrap_or_default();
                        let lines: Vec<&str> = snippet.lines().collect();
                        let sub = if occ.start_line > 0 && occ.start_line <= lines.len() {
                            let end = occ.end_line.min(lines.len());
                            lines[occ.start_line - 1..end].join("\n")
                        } else {
                            String::new()
                        };
                        AiOccurrenceContext {
                            path: occ.file.clone(),
                            span: LineSpan {
                                line_start: occ.start_line,
                                line_end: occ.end_line,
                                byte_offset: 0,
                            },
                            snippet: sub,
                        }
                    })
                    .collect(),
                invariant_body: ast_res.helper_function_code.clone(),
                parameters: ast_res
                    .inferred_parameters
                    .iter()
                    .map(|p| format!("{}: {}", p.name, p.inferred_type))
                    .collect(),
                custom_instructions: None,
            };
            let prompt_text = generate_ai_refactor_prompt(&prompt_req);
            println!("{}", prompt_text);
            if let Some(out_path) = output {
                fs::write(&out_path, &prompt_text)?;
                println!(
                    "\nAI refactoring prompt written to '{}'.",
                    out_path.display()
                );
            }
        } else {
            print_ast_refactor_recommendation(cluster, &ast_res);

            if let Some(out_path) = output {
                fs::write(&out_path, &ast_res.unified_patch)?;
                println!(
                    "\nAST-native unified patch written to '{}'.",
                    out_path.display()
                );
            }
        }
    } else if let Some(c_idx) = cluster {
        if result.clone_clusters.is_empty() {
            println!("No duplicate code clone clusters found to refactor.");
            return Ok(());
        }

        let target_idx = if c_idx > 0 && c_idx <= result.clone_clusters.len() {
            c_idx - 1
        } else {
            eprintln!(
                "Warning: Specified cluster index {} out of range (total: {}); defaulting to 1.",
                c_idx,
                result.clone_clusters.len()
            );
            0
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
                occurrences: selected_cluster
                    .occurrences
                    .iter()
                    .map(|occ| {
                        let snippet = fs::read_to_string(&occ.file).unwrap_or_default();
                        let lines: Vec<&str> = snippet.lines().collect();
                        let sub = if occ.start_line > 0 && occ.start_line <= lines.len() {
                            let end = occ.end_line.min(lines.len());
                            lines[occ.start_line - 1..end].join("\n")
                        } else {
                            String::new()
                        };
                        AiOccurrenceContext {
                            path: occ.file.clone(),
                            span: LineSpan {
                                line_start: occ.start_line,
                                line_end: occ.end_line,
                                byte_offset: 0,
                            },
                            snippet: sub,
                        }
                    })
                    .collect(),
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
                custom_instructions: None,
            };
            let prompt_text = generate_ai_refactor_prompt(&prompt_req);
            println!("{}", prompt_text);
            if let Some(out_path) = output {
                fs::write(&out_path, &prompt_text)?;
                println!(
                    "\nAI refactoring prompt written to '{}'.",
                    out_path.display()
                );
            }
        } else {
            print_cluster_refactor_recommendation(selected_cluster, &suggestion);

            if let Some(out_path) = output {
                fs::write(&out_path, &suggestion.unified_patch)?;
                println!(
                    "\nMulti-site unified patch written to '{}'.",
                    out_path.display()
                );
            }
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
            let snippet_a = fs::read_to_string(&selected.file_a).unwrap_or_default();
            let lines_a: Vec<&str> = snippet_a.lines().collect();
            let sub_a = if selected.start_line_a > 0 && selected.start_line_a <= lines_a.len() {
                let end = selected.end_line_a.min(lines_a.len());
                lines_a[selected.start_line_a - 1..end].join("\n")
            } else {
                String::new()
            };
            let snippet_b = fs::read_to_string(&selected.file_b).unwrap_or_default();
            let lines_b: Vec<&str> = snippet_b.lines().collect();
            let sub_b = if selected.start_line_b > 0 && selected.start_line_b <= lines_b.len() {
                let end = selected.end_line_b.min(lines_b.len());
                lines_b[selected.start_line_b - 1..end].join("\n")
            } else {
                String::new()
            };

            let prompt_req = AiRefactorPromptRequest {
                clone_type: selected.clone_type.clone(),
                similarity: selected.similarity,
                token_count: selected.token_count,
                lines_saved_est: suggestion.lines_saved,
                function_name: suggestion.suggested_function_name.clone(),
                target_module: suggestion.target_module_hint.clone(),
                occurrences: vec![
                    AiOccurrenceContext {
                        path: selected.file_a.clone(),
                        span: LineSpan {
                            line_start: selected.start_line_a,
                            line_end: selected.end_line_a,
                            byte_offset: 0,
                        },
                        snippet: sub_a,
                    },
                    AiOccurrenceContext {
                        path: selected.file_b.clone(),
                        span: LineSpan {
                            line_start: selected.start_line_b,
                            line_end: selected.end_line_b,
                            byte_offset: 0,
                        },
                        snippet: sub_b,
                    },
                ],
                invariant_body: suggestion.common_body_lines.join("\n"),
                parameters: suggestion
                    .parameter_differences
                    .iter()
                    .map(|p| p.fragment_a_code.clone())
                    .collect(),
                custom_instructions: None,
            };
            let prompt_text = generate_ai_refactor_prompt(&prompt_req);
            println!("{}", prompt_text);
            if let Some(out_path) = output {
                fs::write(&out_path, &prompt_text)?;
                println!(
                    "\nAI refactoring prompt written to '{}'.",
                    out_path.display()
                );
            }
        } else {
            print_refactor_recommendation(selected, &suggestion);

            if let Some(out_path) = output {
                fs::write(&out_path, &suggestion.unified_patch)?;
                println!("\nUnified patch written to '{}'.", out_path.display());
            }
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
