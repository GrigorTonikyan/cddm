#![forbid(unsafe_code)]

use std::collections::HashMap;

use super::types::{DeadCodeItem, DeadCodeKind};
use crate::ast::parse_ast_tree;

/// Discovered function declaration metadata.
#[derive(Debug, Clone)]
struct FunctionMeta {
    name: String,
    file_path: String,
    line_start: usize,
    line_end: usize,
    token_count: usize,
    is_public: bool,
}

/// Analyze a set of source files statically to detect unreferenced functions and unreachable code blocks.
pub fn analyze_static_dead_code(
    files: &[(String, String, String)], // (relative_path, extension, source_content)
    min_tokens: usize,
) -> Vec<DeadCodeItem> {
    let mut dead_items = Vec::new();
    let mut functions = Vec::new();
    let mut symbol_references: HashMap<String, usize> = HashMap::new();
    let mut next_id = 1;

    // Phase 1: AST parsing, symbol indexing, unreachable block detection
    for (rel_path, ext, source) in files {
        if let Some(tree) = parse_ast_tree(source, ext) {
            let root = tree.root_node();

            // 1. Extract functions and definitions
            extract_functions_from_node(root, source, rel_path, &mut functions);

            // 2. Index all identifier references
            extract_identifiers_from_node(root, source, &mut symbol_references);

            // 3. Detect unreachable blocks within this file
            let mut unreachable_items = Vec::new();
            detect_unreachable_in_node(
                root,
                source,
                rel_path,
                &mut unreachable_items,
                &mut next_id,
            );
            dead_items.extend(unreachable_items);
        }
    }

    // Phase 2: Detect unreferenced private/internal functions
    for func in functions {
        if func.token_count < min_tokens {
            continue;
        }

        if is_standard_entrypoint(&func.name) {
            continue;
        }

        let ref_count = symbol_references.get(&func.name).copied().unwrap_or(0);
        // If symbol appears only once (at declaration) or zero times across the codebase
        if ref_count <= 1 {
            let lines_count = func.line_end.saturating_sub(func.line_start) + 1;
            let confidence = if func.is_public { 0.70 } else { 0.95 };

            dead_items.push(DeadCodeItem {
                id: next_id,
                file_path: func.file_path,
                symbol_name: func.name.clone(),
                kind: DeadCodeKind::UnreferencedFunction,
                line_start: func.line_start,
                line_end: func.line_end,
                token_count: func.token_count,
                estimated_lines_saved: lines_count,
                reason: format!(
                    "Function '{}' has {} references across the scanned codebase",
                    func.name, ref_count
                ),
                confidence,
            });
            next_id += 1;
        }
    }

    dead_items
}

fn extract_functions_from_node(
    node: tree_sitter::Node,
    source: &str,
    file_path: &str,
    out: &mut Vec<FunctionMeta>,
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

    if is_func && let Some(name) = get_function_name(node, source) {
        let line_start = node.start_position().row + 1;
        let line_end = node.end_position().row + 1;
        let byte_range = node.byte_range();
        let snippet = &source[byte_range.clone()];
        let token_count = snippet.split_whitespace().count();
        let is_public = is_node_public(node, source);

        out.push(FunctionMeta {
            name,
            file_path: file_path.to_string(),
            line_start,
            line_end,
            token_count,
            is_public,
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_functions_from_node(child, source, file_path, out);
    }
}

fn get_function_name(node: tree_sitter::Node, source: &str) -> Option<String> {
    if let Some(name_node) = node.child_by_field_name("name") {
        return Some(source[name_node.byte_range()].trim().to_string());
    }

    // Fallback: search immediate children for identifier
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" || child.kind() == "name" {
            return Some(source[child.byte_range()].trim().to_string());
        }
    }

    None
}

fn is_node_public(node: tree_sitter::Node, source: &str) -> bool {
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

fn extract_identifiers_from_node(
    node: tree_sitter::Node,
    source: &str,
    refs: &mut HashMap<String, usize>,
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
            *refs.entry(sym).or_insert(0) += 1;
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_identifiers_from_node(child, source, refs);
    }
}

fn detect_unreachable_in_node(
    node: tree_sitter::Node,
    source: &str,
    file_path: &str,
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
            if child_kind == "{" || child_kind == "}" {
                continue;
            }

            if terminated {
                let line_start = child.start_position().row + 1;
                let line_end = child.end_position().row + 1;
                let lines_count = line_end.saturating_sub(line_start) + 1;
                let snippet = &source[child.byte_range()];
                let tokens = snippet.split_whitespace().count();

                out.push(DeadCodeItem {
                    id: *next_id,
                    file_path: file_path.to_string(),
                    symbol_name: "<unreachable_statement>".to_string(),
                    kind: DeadCodeKind::UnreachableBlock,
                    line_start,
                    line_end,
                    token_count: tokens,
                    estimated_lines_saved: lines_count,
                    reason: "Statement follows unconditional termination in control flow block"
                        .to_string(),
                    confidence: 0.98,
                });
                *next_id += 1;
            }

            if is_terminal_statement(child, source) {
                terminated = true;
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        detect_unreachable_in_node(child, source, file_path, out, next_id);
    }
}

fn is_terminal_statement(node: tree_sitter::Node, source: &str) -> bool {
    let kind = node.kind();
    if matches!(
        kind,
        "return_statement"
            | "return_expression"
            | "break_statement"
            | "break_expression"
            | "continue_statement"
            | "throw_statement"
    ) {
        return true;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if matches!(
            child.kind(),
            "return_expression"
                | "return_statement"
                | "break_expression"
                | "break_statement"
                | "throw_statement"
        ) {
            return true;
        }
    }
    let text = source[node.byte_range()].trim();
    text.starts_with("return ")
        || text == "return;"
        || text.starts_with("break ")
        || text == "break;"
        || text.contains("panic!")
        || text.contains("std::process::exit")
        || text.contains("process.exit")
}

fn is_standard_entrypoint(name: &str) -> bool {
    matches!(
        name,
        "main"
            | "init"
            | "setup"
            | "teardown"
            | "test"
            | "new"
            | "default"
            | "run"
            | "execute"
            | "handler"
            | "start"
    ) || name.starts_with("test_")
        || name.ends_with("_test")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unreachable_statement_detection() {
        let code = r#"
fn compute() -> i32 {
    return 42;
    let unreachable_val = 100;
}
"#;
        let files = vec![(
            "src/compute.rs".to_string(),
            "rs".to_string(),
            code.to_string(),
        )];
        let items = analyze_static_dead_code(&files, 1);
        let unreach = items
            .iter()
            .find(|i| i.kind == DeadCodeKind::UnreachableBlock);
        assert!(unreach.is_some());
        assert_eq!(unreach.unwrap().line_start, 4);
    }

    #[test]
    fn test_unreferenced_function_detection() {
        let code = r#"
fn unused_helper_routine(x: i32) -> i32 {
    x * 2 + 10
}

fn main() {
    println!("Hello World");
}
"#;
        let files = vec![("src/app.rs".to_string(), "rs".to_string(), code.to_string())];
        let items = analyze_static_dead_code(&files, 2);
        let dead_func = items
            .iter()
            .find(|i| i.symbol_name == "unused_helper_routine");
        assert!(dead_func.is_some());
        assert_eq!(dead_func.unwrap().kind, DeadCodeKind::UnreferencedFunction);
    }
}
