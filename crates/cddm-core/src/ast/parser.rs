use tree_sitter::{Language, Parser, Tree};

/// Returns tree-sitter Language for supported extensions.
pub fn get_tree_sitter_language(extension: &str) -> Option<Language> {
    match extension.to_lowercase().as_str() {
        "rs" => Some(tree_sitter_rust::LANGUAGE.into()),
        "ts" | "tsx" => Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
        "js" | "jsx" | "cjs" | "mjs" => Some(tree_sitter_javascript::LANGUAGE.into()),
        "py" => Some(tree_sitter_python::LANGUAGE.into()),
        "go" => Some(tree_sitter_go::LANGUAGE.into()),
        "c" | "h" => Some(tree_sitter_c::LANGUAGE.into()),
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" | "c++" | "h++" => {
            Some(tree_sitter_cpp::LANGUAGE.into())
        }
        "java" => Some(tree_sitter_java::LANGUAGE.into()),
        "cs" => Some(tree_sitter_c_sharp::LANGUAGE.into()),
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

    #[test]
    fn test_parse_javascript_ast() {
        let code = "function multiply(x, y) { return x * y; }";
        let tree = parse_ast_tree(code, "js").expect("JavaScript AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "program");
    }

    #[test]
    fn test_parse_python_ast() {
        let code = "def greet(name):\n    return f'Hello, {name}'\n";
        let tree = parse_ast_tree(code, "py").expect("Python AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "module");
    }

    #[test]
    fn test_parse_go_ast() {
        let code = "package main\n\nfunc calculate(x int) int {\n\treturn x * 2\n}\n";
        let tree = parse_ast_tree(code, "go").expect("Go AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
    }

    #[test]
    fn test_parse_c_ast() {
        let code = "int factorial(int n) { if (n <= 1) return 1; return n * factorial(n - 1); }";
        let tree = parse_ast_tree(code, "c").expect("C AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "translation_unit");
    }

    #[test]
    fn test_parse_cpp_ast() {
        let code = "namespace utils { template<typename T> T max_val(T a, T b) { return a > b ? a \
                    : b; } }";
        let tree = parse_ast_tree(code, "cpp").expect("C++ AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "translation_unit");
    }

    #[test]
    fn test_parse_java_ast() {
        let code = "public class Calculator { public int sum(int a, int b) { return a + b; } }";
        let tree = parse_ast_tree(code, "java").expect("Java AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "program");
    }

    #[test]
    fn test_parse_c_sharp_ast() {
        let code = "namespace App { public class Greeter { public string SayHi() => \"Hi\"; } }";
        let tree = parse_ast_tree(code, "cs").expect("C# AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "compilation_unit");
    }
}
