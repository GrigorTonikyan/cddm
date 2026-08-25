#![forbid(unsafe_code)]

use cddm_core::{
    AstRewriteResult, CloneCluster, ClonePair, ClusterRefactorSuggestion, RefactorSuggestion,
};

pub fn print_refactor_recommendation(selected: &ClonePair, suggestion: &RefactorSuggestion) {
    println!("\n=== CDDM Automated Refactoring Advisor ===");
    println!(
        "{:<24} {}:{}-{}",
        "Target Fragment A:", selected.file_a, selected.start_line_a, selected.end_line_a
    );
    println!(
        "{:<24} {}:{}-{}",
        "Target Fragment B:", selected.file_b, selected.start_line_b, selected.end_line_b
    );
    println!("{:<24} {}", "Refactoring Strategy:", suggestion.strategy);
    println!(
        "{:<24} {}",
        "Suggested Helper:", suggestion.suggested_function_name
    );
    println!("{:<24} {}", "Target Module:", suggestion.target_module_hint);
    println!(
        "{:<24} {}",
        "Estimated Lines Saved:", suggestion.lines_saved
    );
    println!("\n--- Generated Unified Patch Preview ---\n");
    println!("{}", suggestion.unified_patch);
}

pub fn print_cluster_refactor_recommendation(
    cluster: &CloneCluster,
    suggestion: &ClusterRefactorSuggestion,
) {
    println!("\n=== CDDM — Multi-Site Cluster Refactoring Recommendation ===");
    println!("{:<24} Cluster #{}", "Cluster Target:", cluster.id);
    println!("{:<24} {:?}", "Clone Classification:", cluster.clone_type);
    println!(
        "{:<24} {} locations",
        "Total Occurrences:",
        cluster.occurrences.len()
    );
    println!("{:<24} {}", "Refactoring Strategy:", suggestion.strategy);
    println!(
        "{:<24} {}",
        "Suggested Helper:", suggestion.suggested_function_name
    );
    println!("{:<24} {}", "Target Module:", suggestion.target_module_hint);
    println!(
        "{:<24} {}",
        "Total Lines Saved:", suggestion.total_lines_saved
    );
    println!("\n--- Occurrence Sites ---");
    for (i, site) in suggestion.sites.iter().enumerate() {
        println!(
            "  Site {}: {}:{}-{}",
            i + 1,
            site.file,
            site.start_line,
            site.end_line
        );
    }
    println!("\n--- Generated Multi-File Unified Patch Preview ---\n");
    println!("{}", suggestion.unified_patch);
}

pub fn print_ast_refactor_recommendation(cluster_id: Option<usize>, result: &AstRewriteResult) {
    println!("\n=== CDDM — AST-Native Tree-sitter Refactoring Transformation ===");
    if let Some(cid) = cluster_id {
        println!("{:<24} Cluster #{}", "Cluster Target:", cid);
    }
    println!("{:<24} {}", "Extracted Helper:", result.function_name);
    println!("{:<24} {}", "Helper Signature:", result.helper_signature);
    println!("{:<24} {}", "Target Module:", result.target_module_path);
    println!(
        "{:<24} {} lines",
        "Total Lines Saved:", result.total_lines_saved
    );
    println!(
        "{:<24} {}",
        "Syntax Validated:",
        if result.syntax_valid {
            "[PASS]"
        } else {
            "[FAIL]"
        }
    );
    println!(
        "{:<24} {} files",
        "Rewritten Files:",
        result.rewritten_files.len()
    );

    if !result.inferred_parameters.is_empty() {
        println!("\n--- Inferred Parameters ---");
        for (i, param) in result.inferred_parameters.iter().enumerate() {
            println!(
                "  Param {}: {} ({})",
                i + 1,
                param.name,
                param.inferred_type
            );
        }
    }

    println!("\n--- Synthesized Helper Implementation ---\n");
    println!("{}", result.helper_function_code);

    println!("--- Transformed Source Files ---");
    for file in &result.rewritten_files {
        println!(
            "  File: {} ({} -> {} lines, {} call sites replaced)",
            file.file_path, file.original_line_count, file.new_line_count, file.call_sites_count
        );
        for imp in &file.imports_added {
            println!("    + Added Import: {}", imp);
        }
    }
}
