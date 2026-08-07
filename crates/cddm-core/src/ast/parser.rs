use tree_sitter::{Language, Parser, Tree};

/// Returns tree-sitter Language for supported extensions.
pub fn get_tree_sitter_language(extension: &str) -> Option<Language> {
    match extension.to_lowercase().as_str() {
        "rs" => Some(tree_sitter_rust::LANGUAGE.into()),
        "ts" | "tsx" => Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
        "js" | "jsx" | "cjs" | "mjs" => Some(tree_sitter_javascript::LANGUAGE.into()),
        "py" => Some(tree_sitter_python::LANGUAGE.into()),
        _ => None,
    }
}

/// Parses source string into a tree-sitter Tree.
pub fn parse_ast_tree(source: &str, extension: &str) -> Option<Tree> {
    let lang = get_tree_sitter_language(extension)?;
    let mut parser = Parser::new();
    parser.set_language(&lang).ok()?;
    parser.parse(source, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rust_ast() {
        let code = "fn main() { let x = 42; }";
        let tree = parse_ast_tree(code, "rs").expect("Rust AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(root.child_count() > 0);
    }

    #[test]
    fn test_parse_typescript_ast() {
        let code = "const add = (a: number, b: number): number => a + b;";
        let tree = parse_ast_tree(code, "ts").expect("TypeScript AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "program");
    }
}
