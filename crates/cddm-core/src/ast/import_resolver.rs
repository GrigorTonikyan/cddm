#![forbid(unsafe_code)]

use std::path::Path;

/// Generates an import statement for a caller file importing a helper from target_module.
pub fn generate_import_statement(
    caller_file: &str,
    target_module: &str,
    function_name: &str,
    extension: &str,
) -> Option<String> {
    if caller_file == target_module {
        return None;
    }

    let ext = extension.to_lowercase();
    let caller_path = Path::new(caller_file);
    let target_path = Path::new(target_module);

    let module_stem = if target_path.file_name().and_then(|s| s.to_str()) == Some("__init__.py") {
        target_path
            .parent()
            .and_then(|p| p.file_stem())
            .and_then(|s| s.to_str())
            .unwrap_or("shared_utils")
    } else {
        target_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("helper")
    };

    match ext.as_str() {
        "rs" => {
            if caller_path.parent() == target_path.parent() {
                Some(format!("use super::{}::{};", module_stem, function_name))
            } else {
                Some(format!("use crate::{}::{};", module_stem, function_name))
            }
        }
        "ts" | "tsx" | "js" | "jsx" => {
            let rel_import = if caller_path.parent() == target_path.parent() {
                format!("./{}", module_stem)
            } else {
                format!("../{}", module_stem)
            };
            Some(format!(
                "import {{ {} }} from \"{}\";",
                function_name, rel_import
            ))
        }
        "py" => {
            if caller_path.parent() == target_path.parent() {
                Some(format!("from .{} import {}", module_stem, function_name))
            } else {
                Some(format!("from {} import {}", module_stem, function_name))
            }
        }
        "go" => {
            let pkg_name = target_path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
                .unwrap_or("common");
            Some(format!("import \"{}\"", pkg_name))
        }
        "java" => {
            let pascal_stem = to_pascal_case(module_stem);
            Some(format!("import static {}.{};", pascal_stem, function_name))
        }
        "kt" | "kts" | "scala" | "sc" => Some(format!("import {}.{}", module_stem, function_name)),
        "cs" => {
            let pascal_ns = to_pascal_case(module_stem);
            Some(format!("using {};", pascal_ns))
        }
        "c" | "h" => Some(format!("#include \"{}.h\"", module_stem)),
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Some(format!("#include \"{}.hpp\"", module_stem)),
        "php" | "phtml" => {
            let pascal_stem = to_pascal_case(module_stem);
            Some(format!("use App\\Utils\\{};", pascal_stem))
        }
        "rb" | "rake" => Some(format!("require_relative '{}'", module_stem)),
        "swift" => {
            let pascal_stem = to_pascal_case(module_stem);
            Some(format!("import {}", pascal_stem))
        }
        "zig" | "zon" => Some(format!(
            "const {} = @import(\"{}.zig\");",
            module_stem, module_stem
        )),
        "ex" | "exs" => {
            let pascal_stem = to_pascal_case(module_stem);
            Some(format!("import {}", pascal_stem))
        }
        "dart" => Some(format!("import '{}.dart';", module_stem)),
        _ => None,
    }
}

/// Checks if an import statement is already present in source lines.
pub fn is_import_already_present(source_lines: &[String], import_statement: &str) -> bool {
    let clean_import = import_statement.trim();
    source_lines.iter().any(|line| line.trim() == clean_import)
}

fn to_pascal_case(s: &str) -> String {
    let mut res = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            res.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            res.push(c);
        }
    }
    if res.is_empty() {
        "Helper".to_string()
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_file_no_import() {
        assert_eq!(
            generate_import_statement("src/utils.rs", "src/utils.rs", "helper", "rs"),
            None
        );
    }

    #[test]
    fn test_rust_import_generation() {
        let import = generate_import_statement("src/handlers.rs", "src/utils.rs", "helper", "rs");
        assert_eq!(import, Some("use super::utils::helper;".to_string()));
    }

    #[test]
    fn test_ts_import_generation() {
        let import = generate_import_statement(
            "src/components/A.tsx",
            "src/components/utils.ts",
            "formatData",
            "ts",
        );
        assert_eq!(
            import,
            Some("import { formatData } from \"./utils\";".to_string())
        );
    }

    #[test]
    fn test_python_import_generation() {
        let import = generate_import_statement(
            "app/worker.py",
            "packages/shared_utils/__init__.py",
            "calculate_tax",
            "py",
        );
        assert_eq!(
            import,
            Some("from shared_utils import calculate_tax".to_string())
        );
    }

    #[test]
    fn test_polyglot_import_generation() {
        assert_eq!(
            generate_import_statement("src/Main.java", "src/MathUtils.java", "sum", "java"),
            Some("import static MathUtils.sum;".to_string())
        );
        assert_eq!(
            generate_import_statement("src/Main.kt", "src/utils.kt", "calculate", "kt"),
            Some("import utils.calculate".to_string())
        );
        assert_eq!(
            generate_import_statement("src/Program.cs", "src/CoreUtils.cs", "Compute", "cs"),
            Some("using CoreUtils;".to_string())
        );
        assert_eq!(
            generate_import_statement("src/main.c", "src/math_helper.c", "factorial", "c"),
            Some("#include \"math_helper.h\"".to_string())
        );
        assert_eq!(
            generate_import_statement("src/main.cpp", "src/math_helper.cpp", "factorial", "cpp"),
            Some("#include \"math_helper.hpp\"".to_string())
        );
        assert_eq!(
            generate_import_statement("app.rb", "lib/string_utils.rb", "format_text", "rb"),
            Some("require_relative 'string_utils'".to_string())
        );
        assert_eq!(
            generate_import_statement("main.zig", "helpers.zig", "compute", "zig"),
            Some("const helpers = @import(\"helpers.zig\");".to_string())
        );
        assert_eq!(
            generate_import_statement("app.swift", "Utility.swift", "log", "swift"),
            Some("import Utility".to_string())
        );
    }

    #[test]
    fn test_is_import_already_present() {
        let lines = vec![
            "use super::utils::helper;".to_string(),
            "fn main() {}".to_string(),
        ];
        assert!(is_import_already_present(
            &lines,
            "use super::utils::helper;"
        ));
        assert!(!is_import_already_present(
            &lines,
            "use super::other::helper;"
        ));
    }
}
