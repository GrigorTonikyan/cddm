#![forbid(unsafe_code)]

pub mod extractor;
pub mod graph;
#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::path::Path;

use super::types::{
    CrossPackageReachabilitySummary, DeadCodeItem, DeadCodeKind, ReachabilityStatus,
    SymbolReachability,
};
use crate::ast::parse_ast_tree;
use crate::monorepo::discover_workspaces;
use extractor::{
    collect_node_call_names, detect_unreachable_blocks, extract_package_functions,
    extract_package_identifiers,
};
use graph::{
    PackageFunctionMeta, SymbolCallSite, compute_transitive_callers, resolve_file_package,
};

/// Traces cross-repository and cross-package call-graph reachability across a polyglot workspace.
pub fn trace_cross_package_reachability(
    files: &[(String, String, String)],
    root_dir: &str,
    min_tokens: usize,
) -> (Vec<DeadCodeItem>, CrossPackageReachabilitySummary) {
    let root_path = Path::new(root_dir);
    let workspaces = discover_workspaces(root_path);

    let package_names: Vec<String> = if workspaces.is_empty() {
        vec!["root".to_string()]
    } else {
        workspaces.iter().map(|w| w.name.clone()).collect()
    };

    let mut functions: Vec<PackageFunctionMeta> = Vec::new();
    let mut call_sites: Vec<SymbolCallSite> = Vec::new();
    let mut symbol_pkg_refs: HashMap<String, HashSet<String>> = HashMap::new();
    let mut symbol_file_refs: HashMap<String, HashSet<String>> = HashMap::new();
    let mut symbol_total_counts: HashMap<String, usize> = HashMap::new();
    let mut unreachable_items: Vec<DeadCodeItem> = Vec::new();
    let mut next_id = 1;

    for (rel_path, ext, source) in files {
        let pkg_name = resolve_file_package(rel_path, &workspaces);

        if let Some(tree) = parse_ast_tree(source, ext) {
            let root = tree.root_node();

            extract_package_functions(root, source, rel_path, &pkg_name, &mut functions);

            let mut index = extractor::SymbolReferenceIndex {
                call_sites: &mut call_sites,
                pkg_refs: &mut symbol_pkg_refs,
                file_refs: &mut symbol_file_refs,
                total_counts: &mut symbol_total_counts,
            };

            extract_package_identifiers(root, source, rel_path, &pkg_name, &mut index);

            detect_unreachable_blocks(
                root,
                source,
                rel_path,
                &pkg_name,
                &mut unreachable_items,
                &mut next_id,
            );
        }
    }

    let mut direct_call_graph: HashMap<String, HashSet<String>> = HashMap::new();

    for func in &functions {
        let caller_name = func.name.clone();
        for (rel_path, ext, source) in files {
            if rel_path == &func.file_path
                && let Some(tree) = parse_ast_tree(source, ext)
            {
                let mut body_symbols = HashSet::new();
                collect_node_call_names(tree.root_node(), source, &mut body_symbols);
                for callee in body_symbols {
                    if callee != caller_name {
                        direct_call_graph
                            .entry(callee)
                            .or_default()
                            .insert(caller_name.clone());
                    }
                }
            }
        }
    }

    let mut dead_items = unreachable_items;
    let mut symbol_traces = Vec::new();
    let mut live_cross_pkg_count = 0;
    let mut live_internal_count = 0;
    let mut unused_export_count = 0;
    let mut dead_internal_count = 0;
    let mut total_cross_pkg_calls = 0;

    for func in &functions {
        if is_standard_entrypoint(&func.name) {
            continue;
        }

        let total_refs = symbol_total_counts.get(&func.name).copied().unwrap_or(0);
        let referencing_pkgs = symbol_pkg_refs.get(&func.name).cloned().unwrap_or_default();
        let referencing_files = symbol_file_refs
            .get(&func.name)
            .cloned()
            .unwrap_or_default();

        let mut other_pkgs: Vec<String> = referencing_pkgs
            .iter()
            .filter(|p| *p != &func.package_name)
            .cloned()
            .collect();
        other_pkgs.sort();

        let direct_callers: Vec<String> = direct_call_graph
            .get(&func.name)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default();

        let transitive_callers = compute_transitive_callers(&func.name, &direct_call_graph);

        let is_cross_package_called = !other_pkgs.is_empty();
        if is_cross_package_called {
            total_cross_pkg_calls += other_pkgs.len();
        }

        let status = if is_cross_package_called {
            live_cross_pkg_count += 1;
            ReachabilityStatus::LiveCrossPackage
        } else if total_refs > 1 || referencing_files.len() > 1 {
            live_internal_count += 1;
            ReachabilityStatus::LiveInternal
        } else if func.is_exported {
            unused_export_count += 1;
            ReachabilityStatus::UnusedExport
        } else {
            dead_internal_count += 1;
            ReachabilityStatus::DeadInternal
        };

        let trace = SymbolReachability {
            symbol_name: func.name.clone(),
            declaring_package: func.package_name.clone(),
            declaring_file: func.file_path.clone(),
            is_exported: func.is_exported,
            status,
            direct_callers,
            transitive_callers,
            caller_packages: other_pkgs.clone(),
            total_references: total_refs,
        };
        symbol_traces.push(trace);

        if (status == ReachabilityStatus::DeadInternal
            || status == ReachabilityStatus::UnusedExport)
            && func.token_count >= min_tokens
        {
            let lines_count = func.line_end.saturating_sub(func.line_start) + 1;
            let (confidence, reason) = if func.is_exported {
                (
                    0.90,
                    format!(
                        "Exported in package '{}' but 0 imports found across sibling workspace packages",
                        func.package_name
                    ),
                )
            } else {
                (
                    0.95,
                    format!(
                        "Internal function in package '{}' has {} references",
                        func.package_name, total_refs
                    ),
                )
            };

            dead_items.push(DeadCodeItem {
                id: next_id,
                file_path: func.file_path.clone(),
                symbol_name: func.name.clone(),
                kind: DeadCodeKind::UnreferencedFunction,
                line_start: func.line_start,
                line_end: func.line_end,
                token_count: func.token_count,
                estimated_lines_saved: lines_count,
                reason,
                confidence,
                package_name: Some(func.package_name.clone()),
                is_exported: func.is_exported,
                cross_package_callers: other_pkgs,
            });
            next_id += 1;
        }
    }

    let summary = CrossPackageReachabilitySummary {
        total_packages: package_names.len(),
        packages: package_names,
        live_cross_package_symbols: live_cross_pkg_count,
        live_internal_symbols: live_internal_count,
        unused_exported_symbols: unused_export_count,
        dead_internal_symbols: dead_internal_count,
        total_cross_package_calls: total_cross_pkg_calls,
        symbol_traces,
    };

    (dead_items, summary)
}

fn is_standard_entrypoint(name: &str) -> bool {
    matches!(
        name,
        "main"
            | "default"
            | "init"
            | "setup"
            | "run"
            | "render"
            | "App"
            | "Root"
            | "handle"
            | "handler"
            | "new"
            | "default_config"
    )
}
