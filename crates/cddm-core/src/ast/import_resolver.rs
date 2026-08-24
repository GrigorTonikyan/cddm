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
    match ext.as_str() {
        "rs" => {
            let caller_path = Path::new(caller_file);
            let target_path = Path::new(target_module);
            let module_stem = target_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("helper");

            if caller_path.parent() == target_path.parent() {
                Some(format!("use super::{}::{};", module_stem, function_name))
            } else {
                Some(format!("use crate::{}::{};", module_stem, function_name))
            }
        }
        "ts" | "tsx" | "js" | "jsx" => {
            let caller_path = Path::new(caller_file);
            let target_path = Path::new(target_module);
            let target_stem = target_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("helper");

            let rel_import = if caller_path.parent() == target_path.parent() {
                format!("./{}", target_stem)
            } else {
                format!("../{}", target_stem)
            };
            Some(format!(
                "import {{ {} }} from \"{}\";",
                function_name, rel_import
            ))
        }
        "py" => {
            let target_path = Path::new(target_module);
            let module_stem = target_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("helper");
            Some(format!("from .{} import {}", module_stem, function_name))
        }
        "go" => {
            let target_path = Path::new(target_module);
            let pkg_name = target_path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
                .unwrap_or("common");
            Some(format!("import \"{}\"", pkg_name))
        }
        _ => None,
    }
}

/// Checks if an import statement is already present in source lines.
pub fn is_import_already_present(source_lines: &[String], import_statement: &str) -> bool {
    let clean_import = import_statement.trim();
    source_lines.iter().any(|line| line.trim() == clean_import)
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
