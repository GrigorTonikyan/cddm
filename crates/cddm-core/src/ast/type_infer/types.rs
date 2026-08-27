#![forbid(unsafe_code)]

/// Infers the language-specific type for a list of observed argument values.
pub fn infer_parameter_type(extension: &str, values: &[String]) -> String {
    let ext = extension.to_lowercase();
    if values.is_empty() {
        return default_type_for_ext(&ext);
    }

    let is_all_strings = values.iter().all(|v| {
        let trimmed = v.trim();
        (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
            || (trimmed.starts_with('`') && trimmed.ends_with('`'))
    });

    if is_all_strings {
        return match ext.as_str() {
            "rs" => "&str".to_string(),
            "ts" | "tsx" | "js" | "jsx" => "string".to_string(),
            "py" => "str".to_string(),
            "go" => "string".to_string(),
            "java" | "cs" | "kt" | "kts" | "scala" | "sc" | "swift" | "dart" => {
                "String".to_string()
            }
            "zig" | "zon" => "[]const u8".to_string(),
            "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" => "const char*".to_string(),
            "php" | "phtml" => "string".to_string(),
            _ => "string".to_string(),
        };
    }

    let is_all_integers = values.iter().all(|v| v.trim().parse::<i64>().is_ok());
    if is_all_integers {
        return match ext.as_str() {
            "rs" => "i64".to_string(),
            "ts" | "tsx" | "js" | "jsx" => "number".to_string(),
            "py" => "int".to_string(),
            "go" => "int".to_string(),
            "java" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" | "php" | "phtml" => {
                "int".to_string()
            }
            "cs" => "int".to_string(),
            "kt" | "kts" => "Int".to_string(),
            "scala" | "sc" => "Int".to_string(),
            "swift" => "Int".to_string(),
            "zig" | "zon" => "i64".to_string(),
            "dart" => "int".to_string(),
            _ => "int".to_string(),
        };
    }

    let is_all_floats = values.iter().all(|v| v.trim().parse::<f64>().is_ok());
    if is_all_floats {
        return match ext.as_str() {
            "rs" => "f64".to_string(),
            "ts" | "tsx" | "js" | "jsx" => "number".to_string(),
            "py" => "float".to_string(),
            "go" => "float64".to_string(),
            "java" | "cs" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" | "dart" => {
                "double".to_string()
            }
            "kt" | "kts" | "scala" | "sc" | "swift" => "Double".to_string(),
            "zig" | "zon" => "f64".to_string(),
            "php" | "phtml" => "float".to_string(),
            _ => "float".to_string(),
        };
    }

    let is_all_booleans = values.iter().all(|v| {
        let trimmed = v.trim().to_lowercase();
        trimmed == "true" || trimmed == "false"
    });
    if is_all_booleans {
        return match ext.as_str() {
            "rs" | "go" | "py" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" | "cs"
            | "zig" | "zon" => "bool".to_string(),
            "ts" | "tsx" | "js" | "jsx" => "boolean".to_string(),
            "java" | "dart" => "boolean".to_string(),
            "kt" | "kts" | "scala" | "sc" => "Boolean".to_string(),
            "swift" => "Bool".to_string(),
            "php" | "phtml" => "bool".to_string(),
            _ => "bool".to_string(),
        };
    }

    default_type_for_ext(&ext)
}

/// Infers the return type for an extracted helper function based on return expressions.
pub fn infer_return_type(extension: &str, return_exprs: &[String]) -> Option<String> {
    let ext = extension.to_lowercase();
    if return_exprs.is_empty() {
        return None;
    }

    let is_all_strings = return_exprs.iter().all(|v| {
        let trimmed = v.trim();
        (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
            || (trimmed.starts_with('`') && trimmed.ends_with('`'))
    });
    if is_all_strings {
        return Some(match ext.as_str() {
            "rs" => "String".to_string(),
            "ts" | "tsx" | "js" | "jsx" => "string".to_string(),
            "py" => "str".to_string(),
            "go" => "string".to_string(),
            "java" | "cs" | "kt" | "kts" | "scala" | "sc" | "swift" | "dart" => {
                "String".to_string()
            }
            "zig" | "zon" => "[]const u8".to_string(),
            "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" => "const char*".to_string(),
            "php" | "phtml" => "string".to_string(),
            _ => "string".to_string(),
        });
    }

    let is_all_integers = return_exprs.iter().all(|v| v.trim().parse::<i64>().is_ok());
    if is_all_integers {
        return Some(match ext.as_str() {
            "rs" => "i64".to_string(),
            "ts" | "tsx" | "js" | "jsx" => "number".to_string(),
            "py" => "int".to_string(),
            "go" => "int".to_string(),
            "java" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" | "php" | "phtml" => {
                "int".to_string()
            }
            "cs" => "int".to_string(),
            "kt" | "kts" => "Int".to_string(),
            "scala" | "sc" => "Int".to_string(),
            "swift" => "Int".to_string(),
            "zig" | "zon" => "i64".to_string(),
            "dart" => "int".to_string(),
            _ => "int".to_string(),
        });
    }

    let is_all_booleans = return_exprs.iter().all(|v| {
        let trimmed = v.trim().to_lowercase();
        trimmed == "true" || trimmed == "false"
    });
    if is_all_booleans {
        return Some(match ext.as_str() {
            "rs" | "go" | "py" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" | "cs"
            | "zig" | "zon" => "bool".to_string(),
            "ts" | "tsx" | "js" | "jsx" => "boolean".to_string(),
            "java" | "dart" => "boolean".to_string(),
            "kt" | "kts" | "scala" | "sc" => "Boolean".to_string(),
            "swift" => "Bool".to_string(),
            "php" | "phtml" => "bool".to_string(),
            _ => "bool".to_string(),
        });
    }

    Some(default_type_for_ext(&ext))
}

pub fn default_type_for_ext(ext: &str) -> String {
    match ext {
        "rs" => "&str".to_string(),
        "ts" | "tsx" => "any".to_string(),
        "js" | "jsx" | "rb" | "rake" | "ex" | "exs" | "lua" => "".to_string(),
        "py" => "Any".to_string(),
        "go" => "any".to_string(),
        "java" => "Object".to_string(),
        "cs" => "object".to_string(),
        "kt" | "kts" | "scala" | "sc" | "swift" => "Any".to_string(),
        "zig" | "zon" => "anytype".to_string(),
        "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" => "const void*".to_string(),
        "php" | "phtml" => "mixed".to_string(),
        "dart" => "dynamic".to_string(),
        _ => "auto".to_string(),
    }
}
