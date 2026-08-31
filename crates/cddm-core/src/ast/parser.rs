use std::cell::RefCell;
use std::collections::HashMap;
use tree_sitter::{InputEdit, Language, Parser, Point, Tree};

thread_local! {
    static PARSER_CACHE: RefCell<HashMap<&'static str, Parser>> = RefCell::new(HashMap::new());
}

/// Returns a canonical static key for an extension to maximize parser cache hits across aliases.
pub fn get_canonical_lang_key(extension: &str) -> Option<&'static str> {
    match extension.to_lowercase().as_str() {
        "rs" => Some("rust"),
        "ts" | "tsx" => Some("typescript"),
        "js" | "jsx" | "cjs" | "mjs" => Some("javascript"),
        "py" => Some("python"),
        "go" => Some("go"),
        "c" | "h" => Some("c"),
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" | "c++" | "h++" => Some("cpp"),
        "java" => Some("java"),
        "cs" => Some("c_sharp"),
        "rb" | "rake" | "gemspec" => Some("ruby"),
        "php" | "phtml" => Some("php"),
        "swift" => Some("swift"),
        "sh" | "bash" => Some("bash"),
        "lua" => Some("lua"),
        "json" => Some("json"),
        "html" | "htm" => Some("html"),
        "kt" | "kts" => Some("kotlin"),
        "zig" | "zon" => Some("zig"),
        "scala" | "sc" => Some("scala"),
        "ex" | "exs" => Some("elixir"),
        "sql" | "dsql" => Some("sql"),
        "dockerfile" | "containerfile" => Some("containerfile"),
        _ => None,
    }
}

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
        "rb" | "rake" | "gemspec" => Some(tree_sitter_ruby::LANGUAGE.into()),
        "php" | "phtml" => Some(tree_sitter_php::LANGUAGE_PHP.into()),
        "swift" => Some(tree_sitter_swift::LANGUAGE.into()),
        "sh" | "bash" => Some(tree_sitter_bash::LANGUAGE.into()),
        "lua" => Some(tree_sitter_lua::LANGUAGE.into()),
        "json" => Some(tree_sitter_json::LANGUAGE.into()),
        "html" | "htm" => Some(tree_sitter_html::LANGUAGE.into()),
        "kt" | "kts" => Some(tree_sitter_kotlin_ng::LANGUAGE.into()),
        "zig" | "zon" => Some(tree_sitter_zig::LANGUAGE.into()),
        "scala" | "sc" => Some(tree_sitter_scala::LANGUAGE.into()),
        "ex" | "exs" => Some(tree_sitter_elixir::LANGUAGE.into()),
        "sql" | "dsql" => Some(tree_sitter_sequel::LANGUAGE.into()),
        "dockerfile" | "containerfile" => Some(tree_sitter_containerfile::LANGUAGE.into()),
        _ => None,
    }
}

/// Parses source string into a tree-sitter Tree using a thread-local parser cache.
pub fn parse_ast_tree(source: &str, extension: &str) -> Option<Tree> {
    let key = get_canonical_lang_key(extension)?;
    let lang = get_tree_sitter_language(extension)?;

    PARSER_CACHE.with(|cache| {
        let mut map = cache.borrow_mut();
        let parser = map.entry(key).or_insert_with(|| {
            let mut p = Parser::new();
            let _ = p.set_language(&lang);
            p
        });
        parser.parse(source, None)
    })
}

/// Incrementally re-parses source string into a tree-sitter Tree using an existing edited Tree.
pub fn parse_ast_tree_incremental(source: &str, extension: &str, old_tree: &Tree) -> Option<Tree> {
    let key = get_canonical_lang_key(extension)?;
    let lang = get_tree_sitter_language(extension)?;

    PARSER_CACHE.with(|cache| {
        let mut map = cache.borrow_mut();
        let parser = map.entry(key).or_insert_with(|| {
            let mut p = Parser::new();
            let _ = p.set_language(&lang);
            p
        });
        parser.parse(source, Some(old_tree))
    })
}

/// Applies an edit to a Tree in preparation for incremental re-parsing.
pub fn apply_tree_edit(
    tree: &mut Tree,
    start_byte: usize,
    old_end_byte: usize,
    new_end_byte: usize,
    start_point: (usize, usize),
    old_end_point: (usize, usize),
    new_end_point: (usize, usize),
) {
    let edit = InputEdit {
        start_byte,
        old_end_byte,
        new_end_byte,
        start_position: Point {
            row: start_point.0,
            column: start_point.1,
        },
        old_end_position: Point {
            row: old_end_point.0,
            column: old_end_point.1,
        },
        new_end_position: Point {
            row: new_end_point.0,
            column: new_end_point.1,
        },
    };
    tree.edit(&edit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rust_ast_incremental() {
        let original_code = "fn calculate() -> i32 { 42 }";
        let mut tree = parse_ast_tree(original_code, "rs").expect("Initial parse failed");

        let modified_code = "fn calculate(x: i32) -> i32 { x + 42 }";
        // Edit parameter list "(x: i32)" inserted at byte 13
        apply_tree_edit(&mut tree, 13, 13, 20, (0, 13), (0, 13), (0, 20));
        let updated_tree = parse_ast_tree_incremental(modified_code, "rs", &tree)
            .expect("Incremental parse failed");
        let root = updated_tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(root.to_sexp().contains("parameters"));
    }

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

    #[test]
    fn test_parse_ruby_ast() {
        let code = "def calculate_area(width, height)\n  width * height\nend";
        let tree = parse_ast_tree(code, "rb").expect("Ruby AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "program");
    }

    #[test]
    fn test_parse_php_ast() {
        let code = "<?php\nfunction formatName(string $first, string $last): string {\n    return \
                    $first . ' ' . $last;\n}\n";
        let tree = parse_ast_tree(code, "php").expect("PHP AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "program");
    }

    #[test]
    fn test_parse_swift_ast() {
        let code = "func computeScore(base: Int, multiplier: Int) -> Int {\n    return base * \
                    multiplier\n}\n";
        let tree = parse_ast_tree(code, "swift").expect("Swift AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
    }

    #[test]
    fn test_parse_bash_ast() {
        let code = "#!/bin/bash\nfunction deploy() {\n  echo 'Deploying artifact...'\n  tar -czf \
                    dist.tar.gz ./dist\n}\n";
        let tree = parse_ast_tree(code, "sh").expect("Bash AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "program");
    }

    #[test]
    fn test_parse_lua_ast() {
        let code = "function factorial(n)\n  if n == 0 then return 1 else return n * factorial(n \
                    - 1) end\nend";
        let tree = parse_ast_tree(code, "lua").expect("Lua AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "chunk");
    }

    #[test]
    fn test_parse_json_ast() {
        let code = "{\"name\": \"cddm\", \"version\": \"1.5.0\", \"active\": true}";
        let tree = parse_ast_tree(code, "json").expect("JSON AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "document");
    }

    #[test]
    fn test_parse_html_ast() {
        let code = "<!DOCTYPE html><html><head><title>CDDM</title></head><body><h1>Hello</h1></\
                    body></html>";
        let tree = parse_ast_tree(code, "html").expect("HTML AST parsing failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "document");
    }

    #[test]
    fn test_parse_kotlin_ast() {
        let code = "package app\n\nfun main(args: Array<String>) {\n    val msg = \"Hello \
                    Kotlin\"\n    println(msg)\n}\n";
        let tree = parse_ast_tree(code, "kt").expect("Kotlin AST parsing failed");
        let root = tree.root_node();
        assert!(root.child_count() > 0);
    }

    #[test]
    fn test_parse_zig_ast() {
        let code = "const std = @import(\"std\");\npub fn main() void {\n    const stdout = \
                    std.io.getStdOut().writer();\n}\n";
        let tree = parse_ast_tree(code, "zig").expect("Zig AST parsing failed");
        let root = tree.root_node();
        assert!(root.child_count() > 0);
    }

    #[test]
    fn test_parse_scala_ast() {
        let code = "object Main {\n  def main(args: Array[String]): Unit = {\n    val greeting = \
                    \"Hello Scala\"\n    println(greeting)\n  }\n}\n";
        let tree = parse_ast_tree(code, "scala").expect("Scala AST parsing failed");
        let root = tree.root_node();
        assert!(root.child_count() > 0);
    }

    #[test]
    fn test_parse_elixir_ast() {
        let code = "defmodule Math do\n  def double(x) do\n    x * 2\n  end\nend\n";
        let tree = parse_ast_tree(code, "ex").expect("Elixir AST parsing failed");
        let root = tree.root_node();
        assert!(root.child_count() > 0);
    }

    #[test]
    fn test_parse_sql_ast() {
        let code = "SELECT u.id, u.name, COUNT(o.id) as order_count FROM users u LEFT JOIN orders \
                    o ON u.id = o.user_id WHERE u.active = 1 GROUP BY u.id, u.name ORDER BY \
                    order_count DESC;";
        let tree = parse_ast_tree(code, "sql").expect("SQL AST parsing failed");
        let root = tree.root_node();
        assert!(root.child_count() > 0);
    }

    #[test]
    fn test_parse_dockerfile_ast() {
        let code = "FROM rust:1.85 as builder\nWORKDIR /app\nCOPY . .\nRUN cargo build \
                    --release\nCMD [\"./target/release/cddm\"]\n";
        let tree = parse_ast_tree(code, "dockerfile").expect("Dockerfile AST parsing failed");
        let root = tree.root_node();
        assert!(root.child_count() > 0);
    }

    #[test]
    fn test_parser_cache_reuse_and_aliases() {
        let code1 = "const a = 1;";
        let code2 = "const b = 2;";
        let tree1 = parse_ast_tree(code1, "ts").expect("TS parse 1");
        let tree2 = parse_ast_tree(code2, "tsx").expect("TSX parse 2");
        assert_eq!(tree1.root_node().kind(), "program");
        assert_eq!(tree2.root_node().kind(), "program");
        assert_eq!(get_canonical_lang_key("ts"), Some("typescript"));
        assert_eq!(get_canonical_lang_key("tsx"), Some("typescript"));
        assert_eq!(get_canonical_lang_key("unknown"), None);
    }
}
