pub mod hasher;
pub mod parser;

pub use hasher::{
    AstSubtreeHash, calculate_sequence_similarity, classify_ast_clone, compute_ast_subtree_hashes,
    extract_ast_node_kinds,
};
pub use parser::{get_tree_sitter_language, parse_ast_tree};
