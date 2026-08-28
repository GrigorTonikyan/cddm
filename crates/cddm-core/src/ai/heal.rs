#![forbid(unsafe_code)]

use super::provider::create_ai_provider;
use super::types::{HealIterationLog, HealRefactorRequest, HealRefactorResult};
use crate::ai_prompt::{AiOccurrenceContext, AiRefactorPromptRequest, generate_ai_refactor_prompt};
use crate::refactor::{
    apply_cluster_refactor_branch, apply_patch_to_workspace_dir, verify_refactor_test_suite,
};
use crate::types::{CloneType, LineSpan};
use std::path::Path;

/// Executes the closed-loop autonomous AI healing refactoring loop.
pub async fn heal_cluster_refactor(
    workspace_root: &Path,
    req: &HealRefactorRequest,
) -> Result<HealRefactorResult, String> {
    let provider = create_ai_provider(&req.provider_config);
    let max_iters = req.max_iterations.clamp(1, 10);

    let mut iterations = Vec::new();
    let mut current_patch = String::new();
    let mut modified_files = Vec::new();
    let mut success = false;

    let occurrences_ctx: Vec<AiOccurrenceContext> = req
        .occurrences
        .iter()
        .map(|occ| {
            let full_path = workspace_root.join(&occ.file);
            let snippet = if let Ok(content) = std::fs::read_to_string(&full_path) {
                let lines: Vec<&str> = content.lines().collect();
                if occ.start_line > 0 && occ.start_line <= lines.len() {
                    let end = occ.end_line.min(lines.len());
                    lines[occ.start_line - 1..end].join("\n")
                } else {
                    String::new()
                }
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
                snippet,
            }
        })
        .collect();

    let fn_name = req
        .function_name
        .clone()
        .unwrap_or_else(|| "extracted_shared_helper".to_string());
    let target_mod = req
        .target_module
        .clone()
        .unwrap_or_else(|| "src/utils.rs".to_string());

    let mut context_slices = Vec::new();
    for occ in &req.occurrences {
        let abs_path = workspace_root.join(&occ.file);
        if let Ok(src) = std::fs::read_to_string(&abs_path) {
            let cfgs = crate::semantic_graph::extract_cfgs_from_source(&occ.file, &src, "rust");
            for cfg in cfgs {
                if occ.start_line >= cfg.line_start && occ.end_line <= cfg.line_end {
                    let pdg = crate::semantic_graph::build_pdg_from_cfg(cfg);
                    let slice = crate::semantic_graph::extract_context_slice(
                        &pdg,
                        occ.start_line,
                        occ.end_line,
                    );
                    context_slices.push(slice);
                    break;
                }
            }
        }
    }

    let initial_prompt_req = AiRefactorPromptRequest {
        clone_type: CloneType::Renamed,
        similarity: 0.95,
        token_count: 100,
        lines_saved_est: occurrences_ctx.len() * 10,
        function_name: fn_name.clone(),
        target_module: target_mod.clone(),
        occurrences: occurrences_ctx.clone(),
        invariant_body: String::new(),
        parameters: Vec::new(),
        context_slices: if context_slices.is_empty() {
            None
        } else {
            Some(context_slices)
        },
        custom_instructions: req.custom_instructions.clone(),
    };

    let mut current_prompt = generate_ai_refactor_prompt(&initial_prompt_req);

    for iter in 1..=max_iters {
        let ai_response = provider.complete_prompt(&current_prompt).await?;
        let extracted_patch = extract_patch_from_response(&ai_response);
        current_patch = extracted_patch.clone();

        let mut patch_applied = false;
        let mut test_passed = false;
        let mut error_feedback = None;

        if !extracted_patch.trim().is_empty()
            && let Ok(apply_res) =
                apply_patch_to_workspace_dir(workspace_root, &extracted_patch, false)
        {
            patch_applied = apply_res.success;
            modified_files = apply_res.modified_files;
        }

        if patch_applied && req.verify {
            let verify_res =
                verify_refactor_test_suite(workspace_root, req.test_cmd.as_deref(), None, Some(30));

            match verify_res {
                Ok(v) if v.success => {
                    test_passed = true;
                    success = true;
                }
                Ok(v) => {
                    let err_msg = format!(
                        "Test failure (exit code {}):\n{}\n{}",
                        v.exit_code, v.stderr_snippet, v.stdout_snippet
                    );
                    error_feedback = Some(err_msg.clone());
                    current_prompt = format!(
                        "{}\n\n[PREVIOUS ATTEMPT FAILED TESTS]\n{}\nPlease analyze the failure \
                         and generate an improved unified diff patch that fixes the error and \
                         passes the test suite.",
                        current_prompt, err_msg
                    );
                }
                Err(e) => {
                    let err_msg = format!("Verification runner error: {}", e);
                    error_feedback = Some(err_msg.clone());
                    current_prompt = format!(
                        "{}\n\n[VERIFICATION ERROR]\n{}\nPlease fix the patch syntax and ensure \
                         it applies cleanly.",
                        current_prompt, err_msg
                    );
                }
            }
        } else if patch_applied {
            success = true;
        } else {
            let err_msg = "Patch application failed. The unified diff could not be cleanly \
                           applied to target files."
                .to_string();
            error_feedback = Some(err_msg.clone());
            current_prompt = format!(
                "{}\n\n[PATCH APPLICATION ERROR]\n{}\nPlease provide valid unified diff patch \
                 syntax with exact line context matching the target files.",
                current_prompt, err_msg
            );
        }

        iterations.push(HealIterationLog {
            iteration: iter,
            prompt: current_prompt.clone(),
            response_patch: extracted_patch,
            patch_applied,
            test_passed,
            error_feedback,
        });

        if success {
            break;
        }
    }

    let mut branch_created = None;
    if success
        && let Some(branch_name) = &req.apply_branch
        && let Ok(branch_res) =
            apply_cluster_refactor_branch(workspace_root, &current_patch, Some(branch_name), true)
    {
        branch_created = branch_res.branch_created;
    }

    let message = if success {
        format!(
            "Autonomous AI refactor healing succeeded in {} iteration(s)",
            iterations.len()
        )
    } else {
        format!(
            "Autonomous AI refactor healing concluded after {} iteration(s) without full test \
             verification",
            iterations.len()
        )
    };

    Ok(HealRefactorResult {
        success,
        iterations_run: iterations.len(),
        final_patch: current_patch,
        modified_files,
        branch_created,
        iterations,
        message,
    })
}

/// Helper function to extract a clean unified diff patch from markdown or raw AI output.
pub fn extract_patch_from_response(response: &str) -> String {
    let trimmed = response.trim();
    if let Some(start) = trimmed.find("```diff") {
        let sub = &trimmed[start + 7..];
        if let Some(end) = sub.find("```") {
            return sub[..end].trim().to_string();
        }
    } else if let Some(start) = trimmed.find("```patch") {
        let sub = &trimmed[start + 8..];
        if let Some(end) = sub.find("```") {
            return sub[..end].trim().to_string();
        }
    } else if let Some(start) = trimmed.find("```") {
        let sub = &trimmed[start + 3..];
        let after_lang = if let Some(newline) = sub.find('\n') {
            &sub[newline + 1..]
        } else {
            sub
        };
        if let Some(end) = after_lang.find("```") {
            return after_lang[..end].trim().to_string();
        }
    }

    trimmed.to_string()
}
