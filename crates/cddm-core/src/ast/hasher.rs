use crate::types::LineSpan;
use tree_sitter::{Node, Tree};

/// Represents an AST Subtree Hash for Type 3 / Type 4 clone matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstSubtreeHash {
    /// Blake3 256-bit Merkle subtree hash hex string
    pub hash_hex: String,
    /// Line span in source file
    pub span: LineSpan,
    /// Root node kind (e.g. `function_item`, `if_expression`)
    pub node_kind: String,
    /// AST subtree depth
    pub depth: usize,
}

/// Recursively computes top-down Blake3 Merkle hashes for all subtrees with depth >= min_depth.
pub fn compute_ast_subtree_hashes(tree: &Tree, min_depth: usize) -> Vec<AstSubtreeHash> {
    let mut results = Vec::new();
    let root = tree.root_node();
    visit_node(root, 0, min_depth, &mut results);
    results
}

fn visit_node(
    node: Node,
    _depth: usize,
    min_depth: usize,
    results: &mut Vec<AstSubtreeHash>,
) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(node.kind().as_bytes());

    let mut children_hashes = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        // Skip comment nodes
        if child.kind().contains("comment") {
            continue;
        }
        let child_hash = visit_node(child, _depth + 1, min_depth, results);
        children_hashes.push(child_hash);
    }

    for child_h in &children_hashes {
        hasher.update(child_h.as_bytes());
    }

    let hash_hex = hasher.finalize().to_hex().to_string();

    let node_depth = get_subtree_depth(node);
    if node_depth >= min_depth
        && !node.kind().contains("comment")
        && !node.kind().contains("string")
    {
        let start_pos = node.start_position();
        let end_pos = node.end_position();

        results.push(AstSubtreeHash {
            hash_hex: hash_hex.clone(),
            span: LineSpan {
                line_start: start_pos.row + 1,
                line_end: end_pos.row + 1,
                byte_offset: node.start_byte(),
            },
            node_kind: node.kind().to_string(),
            depth: node_depth,
        });
    }

    hash_hex
}

fn get_subtree_depth(node: Node) -> usize {
    let mut cursor = node.walk();
    let mut max_child_depth = 0;
    for child in node.children(&mut cursor) {
        max_child_depth = max_child_depth.max(get_subtree_depth(child));
    }
    1 + max_child_depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::parser::parse_ast_tree;

    #[test]
    fn test_ast_subtree_hashing() {
        let code = r#"
            fn calculate_tax(income: f64) -> f64 {
                if income > 50000.0 {
                    return income * 0.25;
                } else {
                    return income * 0.15;
                }
            }
        "#;

        let tree = parse_ast_tree(code, "rs").expect("Failed to parse Rust AST");
        let subtrees = compute_ast_subtree_hashes(&tree, 2);
        assert!(
            !subtrees.is_empty(),
            "Should extract subtrees of depth >= 2"
        );
    }
}
