#![forbid(unsafe_code)]

use super::import_resolver::{generate_import_statement, is_import_already_present};
use super::parser::parse_ast_tree;
use super::type_infer::{format_call_site, format_function_signature};
use crate::types::{AstRewrittenFile, InferredParameter};

/// Synthesizes a standalone helper function code block from common invariant lines.
pub fn synthesize_helper_function_block(
    extension: &str,
    function_name: &str,
    parameters: &[InferredParameter],
    common_body_lines: &[String],
    base_indent: &str,
) -> String {
    let ext = extension.to_lowercase();
    let sig = format_function_signature(&ext, function_name, parameters);
    let body_indent = format!("{}    ", base_indent);

    let mut out = String::new();
    match ext.as_str() {
        "py" => {
            out.push_str(&format!("{}{}\n", base_indent, sig));
            if common_body_lines.is_empty() {
                out.push_str(&format!("{}pass\n", body_indent));
            } else {
                for line in common_body_lines {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        out.push_str(&format!("{}{}\n", body_indent, trimmed));
                    }
                }
            }
        }
        "rb" | "rake" => {
            out.push_str(&format!("{}{}\n", base_indent, sig));
            for line in common_body_lines {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    out.push_str(&format!("{}{}\n", body_indent, trimmed));
                }
            }
            out.push_str(&format!("{}end\n", base_indent));
        }
        "ex" | "exs" => {
            out.push_str(&format!("{}{}\n", base_indent, sig));
            for line in common_body_lines {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    out.push_str(&format!("{}{}\n", body_indent, trimmed));
                }
            }
            out.push_str(&format!("{}end\n", base_indent));
        }
        "lua" => {
            out.push_str(&format!("{}{}\n", base_indent, sig));
            for line in common_body_lines {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    out.push_str(&format!("{}{}\n", body_indent, trimmed));
                }
            }
            out.push_str(&format!("{}end\n", base_indent));
        }
        "go" => {
            out.push_str(&format!("{}{} {{\n", base_indent, sig));
            for line in common_body_lines {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    out.push_str(&format!("{}\t{}\n", base_indent, trimmed));
                }
            }
            out.push_str(&format!("{}}}\n", base_indent));
        }
        _ => {
            out.push_str(&format!("{}{} {{\n", base_indent, sig));
            for line in common_body_lines {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    out.push_str(&format!("{}{}\n", body_indent, trimmed));
                }
            }
            out.push_str(&format!("{}}}\n", base_indent));
        }
    }

    out
}

/// Represents a single replacement site to substitute in a source file.
#[derive(Debug, Clone)]
pub struct CloneSiteReplacement {
    pub start_line: usize,
    pub end_line: usize,
    pub arguments: Vec<String>,
}

/// Rewrites a source file by replacing specified clone sites with helper function calls.
pub fn rewrite_source_file(
    file_path: &str,
    source_code: &str,
    extension: &str,
    helper_name: &str,
    target_module: Option<&str>,
    mut sites: Vec<CloneSiteReplacement>,
) -> AstRewrittenFile {
    let original_lines: Vec<String> = source_code.lines().map(|s| s.to_string()).collect();
    let original_line_count = original_lines.len();

    // Sort sites in descending order of start_line to avoid index shifts
    sites.sort_by_key(|b| std::cmp::Reverse(b.start_line));

    let mut current_lines = original_lines.clone();
    let mut call_sites_count = 0;

    for site in &sites {
        if site.start_line == 0 || site.start_line > current_lines.len() {
            continue;
        }

        let start_idx = site.start_line.saturating_sub(1);
        let end_idx = site.end_line.min(current_lines.len());
        if start_idx >= end_idx {
            continue;
        }

        // Determine indentation from the first line of the site
        let first_line = &current_lines[start_idx];
        let indent = first_line
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();

        let call_replacement = format_call_site(extension, helper_name, &site.arguments, &indent);
        current_lines.splice(start_idx..end_idx, vec![call_replacement]);
        call_sites_count += 1;
    }

    let mut imports_added = Vec::new();
    if let Some(target) = target_module
        && let Some(import_stmt) =
            generate_import_statement(file_path, target, helper_name, extension)
        && !is_import_already_present(&current_lines, &import_stmt)
    {
        // Insert import near the top of the file after preambles/packages
        let mut insert_idx = 0;
        while insert_idx < current_lines.len() {
            let line = current_lines[insert_idx].trim();
            if line.starts_with("//")
                || line.starts_with("/*")
                || line.starts_with('*')
                || line.starts_with("#!")
                || line.starts_with("<?php")
                || line.starts_with("#pragma")
                || line.starts_with("package ")
            {
                insert_idx += 1;
            } else {
                break;
            }
        }
        current_lines.insert(insert_idx, import_stmt.clone());
        imports_added.push(import_stmt);
    }

    let has_crlf = source_code.contains("\r\n");
    let line_ending = if has_crlf { "\r\n" } else { "\n" };
    let mut rewritten_source = current_lines.join(line_ending);
    if source_code.ends_with('\n') || source_code.ends_with("\r\n") {
        rewritten_source.push_str(line_ending);
    }

    let new_line_count = current_lines.len();

    AstRewrittenFile {
        file_path: file_path.to_string(),
        original_line_count,
        new_line_count,
        call_sites_count,
        rewritten_source,
        imports_added,
    }
}

/// Validates whether source code parses into a valid AST without syntax errors.
pub fn validate_ast_syntax(source: &str, extension: &str) -> bool {
    if let Some(tree) = parse_ast_tree(source, extension) {
        !tree.root_node().has_error()
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthesize_helper_function_rust() {
        let params = vec![InferredParameter {
            name: "val".to_string(),
            inferred_type: "i64".to_string(),
            original_values: vec!["42".to_string()],
        }];
        let body = vec![
            "let squared = val * val;".to_string(),
            "println!(\"{}\", squared);".to_string(),
        ];

        let code = synthesize_helper_function_block("rs", "calc_square", &params, &body, "");
        assert!(code.contains("pub fn calc_square(val: i64) {"));
        assert!(code.contains("    let squared = val * val;"));
        assert!(code.contains("    println!(\"{}\", squared);"));
    }

    #[test]
    fn test_synthesize_helper_function_python() {
        let params = vec![InferredParameter {
            name: "name".to_string(),
            inferred_type: "str".to_string(),
            original_values: vec![],
        }];
        let body = vec!["print(f'Hello, {name}')".to_string()];

        let code = synthesize_helper_function_block("py", "greet", &params, &body, "");
        assert!(code.contains("def greet(name: str) -> None:"));
        assert!(code.contains("    print(f'Hello, {name}')"));
    }

    #[test]
    fn test_synthesize_helper_function_ruby() {
        let params = vec![InferredParameter {
            name: "item".to_string(),
            inferred_type: "".to_string(),
            original_values: vec![],
        }];
        let body = vec!["puts item".to_string()];

        let code = synthesize_helper_function_block("rb", "print_item", &params, &body, "");
        assert!(code.contains("def print_item(item)"));
        assert!(code.contains("    puts item"));
        assert!(code.contains("end"));
    }

    #[test]
    fn test_synthesize_helper_function_go() {
        let params = vec![InferredParameter {
            name: "x".to_string(),
            inferred_type: "int".to_string(),
            original_values: vec!["10".to_string()],
        }];
        let body = vec!["return x * 2".to_string()];

        let code = synthesize_helper_function_block("go", "double_val", &params, &body, "");
        assert!(code.contains("func DoubleVal(x int) {"));
        assert!(code.contains("\treturn x * 2"));
    }

    #[test]
    fn test_rewrite_source_file_rust() {
        let source = r#"fn main() {
    let a = 1;
    let b = 2;
    println!("{}", a + b);
}
"#;
        let sites = vec![CloneSiteReplacement {
            start_line: 2,
            end_line: 4,
            arguments: vec!["1".to_string(), "2".to_string()],
        }];

        let result = rewrite_source_file(
            "src/main.rs",
            source,
            "rs",
            "extracted_helper",
            Some("src/utils.rs"),
            sites,
        );

        assert_eq!(result.call_sites_count, 1);
        assert!(
            result
                .rewritten_source
                .contains("use super::utils::extracted_helper;")
        );
        assert!(
            result
                .rewritten_source
                .contains("    extracted_helper(1, 2);")
        );
    }

    #[test]
    fn test_validate_ast_syntax() {
        assert!(validate_ast_syntax("fn main() { let x = 10; }", "rs"));
        assert!(validate_ast_syntax("const x: number = 10;", "ts"));
        assert!(validate_ast_syntax("package main\nfunc main() {}", "go"));
        assert!(validate_ast_syntax(
            "public class A { public void run() {} }",
            "java"
        ));
    }
}
