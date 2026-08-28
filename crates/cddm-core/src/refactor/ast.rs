#![forbid(unsafe_code)]

use super::consensus::analyze_cluster_snippets_refactoring;
use super::types::DEFAULT_HELPER_PREFIX;
use crate::types::{AstRewriteResult, CloneLocation, InferredParameter};
use std::fs;
use std::path::Path;

/// Generates an AST-native refactoring transformation across multiple occurrence files.
pub fn generate_ast_cluster_refactor(
    occurrences: &[CloneLocation],
    custom_function_name: Option<&str>,
    target_module_path: Option<&str>,
    custom_parameter_names: Option<&[String]>,
) -> Result<AstRewriteResult, String> {
    if occurrences.is_empty() {
        return Err("No occurrences provided for AST refactoring".to_string());
    }

    let fn_name = custom_function_name.unwrap_or(DEFAULT_HELPER_PREFIX);
    let target_path = target_module_path.unwrap_or(&occurrences[0].file);

    let ext = Path::new(&occurrences[0].file)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("rs");

    // Group occurrences by file
    let mut files_map: std::collections::BTreeMap<String, Vec<&CloneLocation>> =
        std::collections::BTreeMap::new();
    for occ in occurrences {
        files_map.entry(occ.file.clone()).or_default().push(occ);
    }

    // Step 1: Read code snippets per occurrence
    let mut site_snippets = Vec::new();
    for occ in occurrences {
        let p = Path::new(&occ.file);
        if !p.exists() {
            return Err(format!("Occurrence file '{}' does not exist", occ.file));
        }
        let content = fs::read_to_string(p)
            .map_err(|e| format!("Failed to read occurrence file '{}': {}", occ.file, e))?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let start_idx = occ.start_line.saturating_sub(1);
        let end_idx = occ.end_line.min(lines.len());
        if start_idx < end_idx {
            site_snippets.push((occ, lines[start_idx..end_idx].to_vec()));
        }
    }

    if site_snippets.is_empty() {
        return Err("Failed to extract code snippets from occurrences".to_string());
    }

    // Step 2 & 3: Compute common invariant lines and infer parameters
    let occ_pairs: Vec<(&CloneLocation, &[String])> = site_snippets
        .iter()
        .map(|(occ, snip)| (*occ, snip.as_slice()))
        .collect();
    let (cluster_refactor, inferred_parameters) =
        collect_consensus_and_parameters(ext, "ast-preview", &occ_pairs, custom_parameter_names);
    let common_body_lines = cluster_refactor.common_body_lines.clone();

    // Step 4: Synthesize helper function code
    let helper_sig =
        crate::ast::type_infer::format_function_signature(ext, fn_name, &inferred_parameters);
    let helper_function_code = crate::ast::rewriter::synthesize_helper_function_block(
        ext,
        fn_name,
        &inferred_parameters,
        &common_body_lines,
        "",
    );

    // Step 5: Rewrite each file
    let mut rewritten_files = Vec::new();
    let mut syntax_valid = true;

    for (file_path, occs) in files_map {
        let path = Path::new(&file_path);
        let raw_content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

        let mut replacements = Vec::new();
        for occ in occs {
            let site_refactor = cluster_refactor
                .sites
                .iter()
                .find(|s| s.file == occ.file && s.start_line == occ.start_line);

            let arguments = if let Some(site) = site_refactor {
                site.parameter_differences
                    .iter()
                    .map(|p| p.fragment_a_code.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                Vec::new()
            };

            replacements.push(crate::ast::rewriter::CloneSiteReplacement {
                start_line: occ.start_line,
                end_line: occ.end_line,
                arguments,
            });
        }

        let rewritten = crate::ast::rewriter::rewrite_source_file(
            &file_path,
            &raw_content,
            ext,
            fn_name,
            Some(target_path),
            replacements,
        );

        if !crate::ast::rewriter::validate_ast_syntax(&rewritten.rewritten_source, ext) {
            syntax_valid = false;
        }

        rewritten_files.push(rewritten);
    }

    let total_lines_saved = cluster_refactor.total_lines_saved;
    let mut patch = cluster_refactor.unified_patch;
    if let Some(name) = custom_function_name {
        patch = patch.replace(DEFAULT_HELPER_PREFIX, name);
    }

    Ok(AstRewriteResult {
        cluster_id: None,
        function_name: fn_name.to_string(),
        target_module_path: target_path.to_string(),
        helper_signature: helper_sig,
        helper_function_code,
        inferred_parameters,
        rewritten_files,
        unified_patch: patch,
        total_lines_saved,
        syntax_valid,
    })
}

pub(crate) fn collect_consensus_and_parameters(
    ext: &str,
    cluster_name: &str,
    site_snippets: &[(&CloneLocation, &[String])],
    custom_parameter_names: Option<&[String]>,
) -> (
    super::types::ClusterRefactorSuggestion,
    Vec<InferredParameter>,
) {
    let cluster_refactor = analyze_cluster_snippets_refactoring(cluster_name, site_snippets);

    let mut inferred_parameters = Vec::new();
    let mut param_index = 0;

    let mut param_diff_groups: Vec<Vec<String>> = Vec::new();
    for site in &cluster_refactor.sites {
        for (i, diff) in site.parameter_differences.iter().enumerate() {
            if i >= param_diff_groups.len() {
                param_diff_groups.push(Vec::new());
            }
            if !diff.fragment_a_code.is_empty() {
                param_diff_groups[i].push(diff.fragment_a_code.clone());
            }
        }
    }

    for (i, vals) in param_diff_groups.iter().enumerate() {
        let name = if let Some(custom_names) = custom_parameter_names
            && i < custom_names.len()
        {
            custom_names[i].clone()
        } else {
            param_index += 1;
            format!("param_{}", param_index)
        };
        let inferred_type = crate::ast::type_infer::infer_parameter_type(ext, vals);
        inferred_parameters.push(InferredParameter {
            name,
            inferred_type,
            original_values: vals.clone(),
        });
    }

    (cluster_refactor, inferred_parameters)
}
