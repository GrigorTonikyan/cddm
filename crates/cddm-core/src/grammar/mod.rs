#![forbid(unsafe_code)]

pub mod languages_core;
pub mod languages_polyglot;
pub mod registry;
pub mod types;

pub use languages_core::CORE_LANGUAGES;
pub use languages_polyglot::POLYGLOT_LANGUAGES;
pub use registry::{SUPPORTED_LANGUAGES, get_grammar_for_path};
pub use types::{LanguageGrammar, make_c_style_grammar};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_get_grammar_for_path() {
        let rs_path = Path::new("src/main.rs");
        let ts_path = Path::new("app/index.ts");
        let py_path = Path::new("script.py");
        let go_path = Path::new("server.go");
        let css_path = Path::new("style.css");
        let unknown_path = Path::new("file.xyz");
        let no_ext_path = Path::new("Makefile");

        assert_eq!(get_grammar_for_path(rs_path).unwrap().name, "Rust");
        assert_eq!(get_grammar_for_path(ts_path).unwrap().name, "TypeScript");
        assert_eq!(get_grammar_for_path(py_path).unwrap().name, "Python");
        assert_eq!(get_grammar_for_path(go_path).unwrap().name, "Go");
        assert_eq!(get_grammar_for_path(css_path).unwrap().name, "CSS");
        assert_eq!(
            get_grammar_for_path(Path::new("main.zig")).unwrap().name,
            "Zig"
        );
        assert_eq!(
            get_grammar_for_path(Path::new("App.scala")).unwrap().name,
            "Scala"
        );
        assert_eq!(
            get_grammar_for_path(Path::new("module.ex")).unwrap().name,
            "Elixir"
        );
        assert_eq!(
            get_grammar_for_path(Path::new("schema.sql")).unwrap().name,
            "SQL"
        );
        assert_eq!(
            get_grammar_for_path(Path::new("Dockerfile")).unwrap().name,
            "Dockerfile"
        );
        assert_eq!(
            get_grammar_for_path(Path::new("service.dockerfile"))
                .unwrap()
                .name,
            "Dockerfile"
        );

        assert!(get_grammar_for_path(unknown_path).is_none());
        assert!(get_grammar_for_path(no_ext_path).is_none());
    }

    #[test]
    fn test_grammar_properties() {
        let rs_grammar = get_grammar_for_path(Path::new("test.rs")).unwrap();
        assert!(rs_grammar.keywords.contains(&"fn"));
        assert_eq!(rs_grammar.line_comment, "//");
        assert_eq!(rs_grammar.block_comment, Some(("/*", "*/")));

        let py_grammar = get_grammar_for_path(Path::new("test.py")).unwrap();
        assert!(py_grammar.keywords.contains(&"def"));
        assert_eq!(py_grammar.line_comment, "#");
        assert_eq!(py_grammar.block_comment, None);

        let zig_grammar = get_grammar_for_path(Path::new("test.zig")).unwrap();
        assert!(zig_grammar.keywords.contains(&"comptime"));

        let scala_grammar = get_grammar_for_path(Path::new("test.scala")).unwrap();
        assert!(scala_grammar.keywords.contains(&"trait"));

        let elixir_grammar = get_grammar_for_path(Path::new("test.ex")).unwrap();
        assert!(elixir_grammar.keywords.contains(&"defmodule"));
    }

    #[test]
    fn test_all_supported_extensions() {
        for grammar in SUPPORTED_LANGUAGES {
            for ext in grammar.extensions {
                let filename = format!("test.{}", ext);
                let path = Path::new(&filename);
                let found = get_grammar_for_path(path).unwrap();
                assert_eq!(found.name, grammar.name);
            }
        }
    }

    #[test]
    fn test_supported_language_count() {
        assert!(SUPPORTED_LANGUAGES.len() >= 20);
    }
}
