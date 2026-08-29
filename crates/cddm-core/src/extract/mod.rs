#![forbid(unsafe_code)]

pub mod bench_generator;
pub mod executor;
pub mod generator;
pub mod manifest;
pub mod rewriter;
pub mod test_generator;
pub mod types;

pub use bench_generator::generate_benchmark_files;
pub use executor::apply_extraction_to_workspace;
pub use generator::generate_extracted_target_files;
pub use manifest::update_workspace_manifests;
pub use rewriter::rewrite_caller_files;
pub use test_generator::generate_unit_test_files;
pub use types::*;

use std::path::Path;

/// Core coordinator to generate an automated shared crate or module extraction.
pub fn generate_shared_extraction(
    workspace_root: &Path,
    request: &ExtractRequest,
) -> Result<ExtractResult, String> {
    if request.occurrences.is_empty() {
        return Err("No occurrence locations provided for shared extraction".to_string());
    }

    let first_occ = &request.occurrences[0];
    let ext = Path::new(&first_occ.file)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("rs");

    // 1. Read code snippets per occurrence
    let site_snippets =
        crate::refactor::ast::load_occurrence_snippets(Some(workspace_root), &request.occurrences)?;

    // 2 & 3. Compute consensus invariant code and infer parameters
    let occ_pairs: Vec<(&crate::types::CloneLocation, &[String])> = site_snippets
        .iter()
        .map(|(occ, snip)| (*occ, snip.as_slice()))
        .collect();
    let (cluster_refactor, inferred_parameters) =
        crate::refactor::ast::collect_consensus_and_parameters(
            ext,
            "extract-preview",
            &occ_pairs,
            request.custom_parameter_names.as_deref(),
        );
    let common_body_lines = cluster_refactor.common_body_lines.clone();

    let default_fn_name = "extracted_shared_helper".to_string();
    let fn_name = request
        .custom_function_name
        .as_ref()
        .unwrap_or(&default_fn_name);

    // 4. Determine target kind if Auto
    let target_kind = match request.target_kind {
        ExtractTargetKind::Auto => {
            if request.target_path.starts_with("crates/")
                || request.target_path.starts_with("packages/")
            {
                ExtractTargetKind::NewCrate
            } else {
                ExtractTargetKind::NewModule
            }
        }
        kind => kind,
    };

    // 5. Generate target files
    let (helper_sig, generated_files) = generate_extracted_target_files(
        &request.target_path,
        target_kind,
        fn_name,
        &inferred_parameters,
        &common_body_lines,
        ext,
    );

    // 6. Update manifests if NewCrate
    let caller_files: Vec<String> = request.occurrences.iter().map(|o| o.file.clone()).collect();
    let target_crate_name = Path::new(&request.target_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("shared_utils");

    let manifest_updates = if target_kind == ExtractTargetKind::NewCrate {
        update_workspace_manifests(
            workspace_root,
            &request.target_path,
            target_crate_name,
            &caller_files,
            ext,
        )
    } else {
        Vec::new()
    };

    // 7. Rewrite caller occurrence files
    let (caller_rewrites, syntax_valid) = rewrite_caller_files(
        workspace_root,
        &request.occurrences,
        fn_name,
        &request.target_path,
        target_kind,
        ext,
    )?;

    // 8. Synthesize unit tests if requested
    let test_files = if request.generate_tests {
        generate_unit_test_files(
            &request.target_path,
            target_kind,
            fn_name,
            &inferred_parameters,
            ext,
        )
    } else {
        Vec::new()
    };

    // 9. Synthesize performance micro-benchmarks if requested
    let benchmark_files = if request.generate_benchmarks {
        generate_benchmark_files(
            &request.target_path,
            target_kind,
            fn_name,
            &inferred_parameters,
            ext,
        )
    } else {
        Vec::new()
    };

    let total_lines_saved = cluster_refactor.total_lines_saved;
    let message = format!(
        "Successfully planned shared extraction of '{}' to '{}' ({} files generated, {} tests \
         synthesized, {} benchmarks synthesized, {} manifests updated, {} callers rewritten, {} \
         lines saved)",
        fn_name,
        request.target_path,
        generated_files.len(),
        test_files.len(),
        benchmark_files.len(),
        manifest_updates.len(),
        caller_rewrites.len(),
        total_lines_saved
    );

    Ok(ExtractResult {
        function_name: fn_name.to_string(),
        target_path: request.target_path.clone(),
        target_kind,
        helper_signature: helper_sig,
        inferred_parameters,
        generated_files,
        test_files,
        benchmark_files,
        manifest_updates,
        caller_rewrites,
        total_lines_saved,
        syntax_valid,
        message,
    })
}

/// Executes and writes the shared extraction directly to the workspace filesystem.
pub fn apply_shared_extraction(
    workspace_root: &Path,
    request: &ExtractRequest,
) -> Result<ExtractResult, String> {
    let mut plan = generate_shared_extraction(workspace_root, request)?;
    if !request.dry_run {
        let written = apply_extraction_to_workspace(workspace_root, &plan, false)?;
        plan.message = format!(
            "Successfully applied shared extraction to workspace ({} file changes committed to \
             disk)",
            written
        );
    }
    Ok(plan)
}

#[cfg(test)]
mod tests;
