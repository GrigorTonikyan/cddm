pub mod hasher;
pub mod import_resolver;
pub mod parser;
pub mod rewriter;
pub mod type_infer;

pub use hasher::{
    AstSubtreeHash, calculate_sequence_similarity, classify_ast_clone, compute_ast_subtree_hashes,
    extract_ast_node_kinds,
};
pub use import_resolver::{generate_import_statement, is_import_already_present};
pub use parser::{get_tree_sitter_language, parse_ast_tree};
pub use rewriter::{
    CloneSiteReplacement, rewrite_source_file, synthesize_helper_function_block,
    validate_ast_syntax,
};
pub use type_infer::{format_call_site, format_function_signature, infer_parameter_type};
