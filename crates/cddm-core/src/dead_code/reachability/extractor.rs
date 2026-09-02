#![forbid(unsafe_code)]

use super::graph::{PackageFunctionMeta, SymbolCallSite};
use crate::dead_code::types::{DeadCodeItem, DeadCodeKind};
use std::collections::{HashMap, HashSet};

pub fn extract_package_functions(
    node: tree_sitter::Node,
    source: &str,
    file_path: &str,
    package_name: &str,
    out: &mut Vec<PackageFunctionMeta>,
) {
    let kind = node.kind();
    let is_func = matches!(
        kind,
        "function_item"
            | "function_declaration"
            | "function_definition"
            | "method_definition"
            | "method_declaration"
            | "func_declaration"
    );

    if is_func && let Some(name) = get_node_identifier(node, source) {
        let line_start = node.start_position().row + 1;
        let line_end = node.end_position().row + 1;
        let byte_range = node.byte_range();
        let snippet = &source[byte_range];
        let token_count = snippet.split_whitespace().count();
        let is_exported = is_exported_symbol(node, source);

        out.push(PackageFunctionMeta {
            name,
            file_path: file_path.to_string(),
            package_name: package_name.to_string(),
            line_start,
            line_end,
            token_count,
            is_exported,
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_package_functions(child, source, file_path, package_name, out);
    }
}

/// Mutable reference collection for symbol usage indexing.
#[derive(Debug)]
pub struct SymbolReferenceIndex<'a> {
    pub call_sites: &'a mut Vec<SymbolCallSite>,
    pub pkg_refs: &'a mut HashMap<String, HashSet<String>>,
    pub file_refs: &'a mut HashMap<String, HashSet<String>>,
    pub total_counts: &'a mut HashMap<String, usize>,
}

pub fn extract_package_identifiers(
    node: tree_sitter::Node,
    source: &str,
    file_path: &str,
    package_name: &str,
    index: &mut SymbolReferenceIndex<'_>,
) {
    let kind = node.kind();
    if matches!(
        kind,
        "identifier"
            | "type_identifier"
            | "field_identifier"
            | "property_identifier"
            | "shorthand_property_identifier"
    ) {
        let sym = source[node.byte_range()].trim().to_string();
        if !sym.is_empty() {
            *index.total_counts.entry(sym.clone()).or_insert(0) += 1;
            index
                .pkg_refs
                .entry(sym.clone())
                .or_default()
                .insert(package_name.to_string());
            index
                .file_refs
                .entry(sym.clone())
                .or_default()
                .insert(file_path.to_string());
            index.call_sites.push(SymbolCallSite {
                symbol: sym,
                file_path: file_path.to_string(),
                package_name: package_name.to_string(),
            });
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_package_identifiers(child, source, file_path, package_name, index);
    }
}

pub fn collect_node_call_names(node: tree_sitter::Node, source: &str, out: &mut HashSet<String>) {
    let kind = node.kind();
    if matches!(kind, "call_expression" | "method_call_expression")
        && let Some(name_node) = node
            .child_by_field_name("function")
            .or_else(|| node.child_by_field_name("name"))
    {
        let name = source[name_node.byte_range()].trim().to_string();
        if !name.is_empty() {
            out.insert(name);
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_node_call_names(child, source, out);
    }
}

pub fn detect_unreachable_blocks(
    node: tree_sitter::Node,
    source: &str,
    file_path: &str,
    package_name: &str,
    out: &mut Vec<DeadCodeItem>,
    next_id: &mut usize,
) {
    let kind = node.kind();
    if matches!(
        kind,
        "block" | "statement_block" | "compound_statement" | "block_body"
    ) {
        let mut terminated = false;
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let child_kind = child.kind();
            if terminated {
                let line_start = child.start_position().row + 1;
                let line_end = child.end_position().row + 1;
                let snippet = &source[child.byte_range()];
                let token_count = snippet.split_whitespace().count().max(1);
                let lines_saved = line_end.saturating_sub(line_start) + 1;

                out.push(DeadCodeItem {
                    id: *next_id,
                    file_path: file_path.to_string(),
                    symbol_name: "<unreachable_statement>".to_string(),
                    kind: DeadCodeKind::UnreachableBlock,
                    line_start,
                    line_end,
                    token_count,
                    estimated_lines_saved: lines_saved,
                    reason: "Statement follows unconditional termination in control flow block"
                        .to_string(),
                    confidence: 0.98,
                    package_name: Some(package_name.to_string()),
                    is_exported: false,
                    cross_package_callers: Vec::new(),
                });
                *next_id += 1;
                break;
            }

            if is_terminating_statement(child_kind, child, source) {
                terminated = true;
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        detect_unreachable_blocks(child, source, file_path, package_name, out, next_id);
    }
}

fn is_terminating_statement(kind: &str, node: tree_sitter::Node, source: &str) -> bool {
    matches!(
        kind,
        "return_statement" | "throw_statement" | "break_statement" | "continue_statement"
    ) || (kind == "expression_statement" && {
        let text = &source[node.byte_range()];
        text.contains("panic!") || text.contains("exit(") || text.contains("process.exit")
    })
}

fn get_node_identifier(node: tree_sitter::Node, source: &str) -> Option<String> {
    if let Some(name_node) = node.child_by_field_name("name") {
        return Some(source[name_node.byte_range()].trim().to_string());
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" || child.kind() == "name" {
            return Some(source[child.byte_range()].trim().to_string());
        }
    }
    None
}

fn is_exported_symbol(node: tree_sitter::Node, source: &str) -> bool {
    if let Some(vis) = node.child_by_field_name("visibility") {
        return source[vis.byte_range()].contains("pub");
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let text = &source[child.byte_range()];
        if text == "pub" || text == "export" || text == "public" {
            return true;
        }
    }
    false
}
