use crate::ast::parser::parse_ast_tree;
use crate::types::{CloneType, LineSpan};
use std::collections::HashSet;
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

/// Extracts a sequence of AST node kinds (excluding comments) for an AST tree.
pub fn extract_ast_node_kinds(tree: &Tree) -> Vec<String> {
    let mut kinds = Vec::new();
    collect_node_kinds(tree.root_node(), &mut kinds);
    kinds
}

fn collect_node_kinds(node: Node, kinds: &mut Vec<String>) {
    if !node.kind().contains("comment") {
        kinds.push(node.kind().to_string());
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_node_kinds(child, kinds);
    }
}

/// Computes Longest Common Subsequence (LCS) similarity ratio between two AST node sequences (0.0 to 1.0).
pub fn calculate_sequence_similarity(a: &[String], b: &[String]) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    if a == b {
        return 1.0;
    }
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            if a[i] == b[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }
    let lcs = dp[n][m];
    (2.0 * lcs as f64) / ((n + m) as f64)
}

/// Classifies clone relationship and calculates similarity between two code fragments.
pub fn classify_ast_clone(
    content_a: &str,
    ext_a: &str,
    content_b: &str,
    ext_b: &str,
) -> (CloneType, f64) {
    // 1. Check exact text match (ignoring leading/trailing whitespace per line)
    let lines_a: Vec<&str> = content_a
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    let lines_b: Vec<&str> = content_b
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    if !lines_a.is_empty() && lines_a == lines_b {
        return (CloneType::Exact, 1.0);
    }

    // 2. Check AST analysis if both extensions have Tree-sitter parsers
    if let (Some(tree_a), Some(tree_b)) = (
        parse_ast_tree(content_a, ext_a),
        parse_ast_tree(content_b, ext_b),
    ) {
        let nodes_a = extract_ast_node_kinds(&tree_a);
        let nodes_b = extract_ast_node_kinds(&tree_b);
        let sim = calculate_sequence_similarity(&nodes_a, &nodes_b);

        if sim >= 0.999 || nodes_a == nodes_b {
            (CloneType::Renamed, 1.0)
        } else if sim >= 0.70 {
            (CloneType::NearMiss, (sim * 100.0).round() / 100.0)
        } else {
            // Check if subtrees share structural Merkle hashes (Type-4 Semantic)
            let subtrees_a = compute_ast_subtree_hashes(&tree_a, 3);
            let subtrees_b = compute_ast_subtree_hashes(&tree_b, 3);
            if !subtrees_a.is_empty() && !subtrees_b.is_empty() {
                let hashes_b: HashSet<_> = subtrees_b.iter().map(|s| &s.hash_hex).collect();
                let match_count = subtrees_a
                    .iter()
                    .filter(|s| hashes_b.contains(&s.hash_hex))
                    .count();
                let ratio =
                    (2.0 * match_count as f64) / ((subtrees_a.len() + subtrees_b.len()) as f64);
                if ratio >= 0.60 {
                    return (CloneType::Semantic, (ratio * 100.0).round() / 100.0);
                }
            }
            (CloneType::NearMiss, (sim * 100.0).round() / 100.0)
        }
    } else {
        // 3. Fallback for non-AST languages: compute line sequence LCS similarity
        let a_vec: Vec<String> = lines_a.iter().map(|s| s.to_string()).collect();
        let b_vec: Vec<String> = lines_b.iter().map(|s| s.to_string()).collect();
        let sim = calculate_sequence_similarity(&a_vec, &b_vec);
        let rounded_sim = (sim * 100.0).round() / 100.0;
        if sim >= 0.999 {
            (CloneType::Exact, 1.0)
        } else {
            (CloneType::NearMiss, rounded_sim)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_exact_clone_classification() {
        let code_a = "fn foo() { let x = 10; println!(\"{}\", x); }";
        let code_b = "fn foo() { let x = 10; println!(\"{}\", x); }";
        let (clone_type, sim) = classify_ast_clone(code_a, "rs", code_b, "rs");
        assert_eq!(clone_type, CloneType::Exact);
        assert_eq!(sim, 1.0);
    }

    #[test]
    fn test_renamed_clone_classification() {
        let code_a = "fn calculate_total(price: f64) -> f64 { price * 1.2 }";
        let code_b = "fn compute_sum(val: f64) -> f64 { val * 1.2 }";
        let (clone_type, sim) = classify_ast_clone(code_a, "rs", code_b, "rs");
        assert_eq!(clone_type, CloneType::Renamed);
        assert_eq!(sim, 1.0);
    }

    #[test]
    fn test_near_miss_clone_classification() {
        let code_a = r#"
            fn process_items(items: &[i32]) -> i32 {
                let mut sum = 0;
                for item in items {
                    sum += item;
                }
                sum
            }
        "#;
        let code_b = r#"
            fn process_items(items: &[i32]) -> i32 {
                let mut sum = 0;
                for item in items {
                    sum += item;
                    println!("item: {}", item);
                }
                sum
            }
        "#;
        let (clone_type, sim) = classify_ast_clone(code_a, "rs", code_b, "rs");
        assert_eq!(clone_type, CloneType::NearMiss);
        assert!(
            (0.70..1.0).contains(&sim),
            "Similarity should be in [0.70, 1.0)"
        );
    }

    #[test]
    fn test_ast_semantic_clone_classification() {
        let code_a = r#"
            fn wrapper_a(income: f64, extra: bool) -> f64 {
                if extra { println!("Extra active"); }
                if income > 50000.0 {
                    return income * 0.25;
                } else {
                    return income * 0.15;
                }
            }
        "#;
        let code_b = r#"
            fn wrapper_b(salary: f64, flag: bool, mode: i32) -> f64 {
                match mode { 1 => println!("mode 1"), _ => () };
                if flag { println!("Flag active"); }
                if salary > 50000.0 {
                    return salary * 0.25;
                } else {
                    return salary * 0.15;
                }
            }
        "#;
        let (clone_type, sim) = classify_ast_clone(code_a, "rs", code_b, "rs");
        assert!(
            clone_type == CloneType::Semantic || clone_type == CloneType::NearMiss,
            "Expected Semantic or NearMiss clone, got {:?}",
            clone_type
        );
        assert!(sim > 0.5);
    }
}
