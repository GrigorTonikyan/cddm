#![forbid(unsafe_code)]

use crate::types::InferredParameter;

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

fn default_type_for_ext(ext: &str) -> String {
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

/// Generates a language-specific function signature header.
pub fn format_function_signature(
    extension: &str,
    function_name: &str,
    parameters: &[InferredParameter],
) -> String {
    let ext = extension.to_lowercase();
    match ext.as_str() {
        "rs" => {
            let params = parameters
                .iter()
                .map(|p| format!("{}: {}", p.name, p.inferred_type))
                .collect::<Vec<_>>()
                .join(", ");
            format!("pub fn {}({})", function_name, params)
        }
        "ts" | "tsx" => {
            let params = parameters
                .iter()
                .map(|p| format!("{}: {}", p.name, p.inferred_type))
                .collect::<Vec<_>>()
                .join(", ");
            format!("export function {}({}): void", function_name, params)
        }
        "js" | "jsx" => {
            let params = parameters
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ");
            format!("export function {}({})", function_name, params)
        }
        "py" => {
            let params = parameters
                .iter()
                .map(|p| format!("{}: {}", p.name, p.inferred_type))
                .collect::<Vec<_>>()
                .join(", ");
            format!("def {}({}) -> None:", function_name, params)
        }
        "go" => {
            let pascal_name = to_pascal_case(function_name);
            let params = parameters
                .iter()
                .map(|p| format!("{} {}", p.name, p.inferred_type))
                .collect::<Vec<_>>()
                .join(", ");
            format!("func {}({})", pascal_name, params)
        }
        "java" => {
            let params = parameters
                .iter()
                .map(|p| format!("{} {}", p.inferred_type, p.name))
                .collect::<Vec<_>>()
                .join(", ");
            format!("public static void {}({})", function_name, params)
        }
        "cs" => {
            let pascal_name = to_pascal_case(function_name);
            let params = parameters
                .iter()
                .map(|p| format!("{} {}", p.inferred_type, p.name))
                .collect::<Vec<_>>()
                .join(", ");
            format!("public static void {}({})", pascal_name, params)
        }
        "kt" | "kts" => {
            let params = parameters
                .iter()
                .map(|p| format!("{}: {}", p.name, p.inferred_type))
                .collect::<Vec<_>>()
                .join(", ");
            format!("fun {}({})", function_name, params)
        }
        "scala" | "sc" => {
            let params = parameters
                .iter()
                .map(|p| format!("{}: {}", p.name, p.inferred_type))
                .collect::<Vec<_>>()
                .join(", ");
            format!("def {}({}): Unit =", function_name, params)
        }
        "swift" => {
            let params = parameters
                .iter()
                .map(|p| format!("{}: {}", p.name, p.inferred_type))
                .collect::<Vec<_>>()
                .join(", ");
            format!("public func {}({})", function_name, params)
        }
        "zig" | "zon" => {
            let params = parameters
                .iter()
                .map(|p| format!("{}: {}", p.name, p.inferred_type))
                .collect::<Vec<_>>()
                .join(", ");
            format!("pub fn {}({}) void", function_name, params)
        }
        "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" => {
            let params = parameters
                .iter()
                .map(|p| format!("{} {}", p.inferred_type, p.name))
                .collect::<Vec<_>>()
                .join(", ");
            format!("void {}({})", function_name, params)
        }
        "php" | "phtml" => {
            let params = parameters
                .iter()
                .map(|p| {
                    if p.inferred_type.is_empty() || p.inferred_type == "mixed" {
                        format!("${}", p.name)
                    } else {
                        format!("{} ${}", p.inferred_type, p.name)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("function {}({}): void", function_name, params)
        }
        "rb" | "rake" => {
            let snake_name = to_snake_case(function_name);
            let params = parameters
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ");
            format!("def {}({})", snake_name, params)
        }
        "ex" | "exs" => {
            let snake_name = to_snake_case(function_name);
            let params = parameters
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ");
            format!("def {}({}) do", snake_name, params)
        }
        "lua" => {
            let snake_name = to_snake_case(function_name);
            let params = parameters
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ");
            format!("local function {}({})", snake_name, params)
        }
        "dart" => {
            let params = parameters
                .iter()
                .map(|p| format!("{} {}", p.inferred_type, p.name))
                .collect::<Vec<_>>()
                .join(", ");
            format!("void {}({})", function_name, params)
        }
        _ => {
            let params = parameters
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ");
            format!("function {}({})", function_name, params)
        }
    }
}

/// Formats a function call invocation expression.
pub fn format_call_site(
    extension: &str,
    function_name: &str,
    arguments: &[String],
    indent: &str,
) -> String {
    let ext = extension.to_lowercase();
    let args_joined = arguments.join(", ");
    match ext.as_str() {
        "rs" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" | "java" | "ts" | "tsx" | "js"
        | "jsx" | "php" | "phtml" | "dart" => {
            format!("{}{}({});", indent, function_name, args_joined)
        }
        "py" | "kt" | "kts" | "scala" | "sc" | "swift" => {
            format!("{}{}({})", indent, function_name, args_joined)
        }
        "go" => {
            let pascal = to_pascal_case(function_name);
            format!("{}{}({})", indent, pascal, args_joined)
        }
        "cs" => {
            let pascal = to_pascal_case(function_name);
            format!("{}{}({});", indent, pascal, args_joined)
        }
        "zig" | "zon" => format!("{}{}({});", indent, function_name, args_joined),
        "rb" | "rake" | "ex" | "exs" | "lua" => {
            let snake = to_snake_case(function_name);
            format!("{}{}({})", indent, snake, args_joined)
        }
        _ => format!("{}{}({});", indent, function_name, args_joined),
    }
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

fn to_snake_case(s: &str) -> String {
    let mut res = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !res.ends_with('_') {
                res.push('_');
            }
            res.extend(c.to_lowercase());
        } else if c == '-' || c == ' ' {
            if !res.ends_with('_') {
                res.push('_');
            }
        } else {
            res.push(c);
        }
    }
    if res.is_empty() {
        "helper".to_string()
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_string_type() {
        let vals = vec!["\"hello\"".to_string(), "\"world\"".to_string()];
        assert_eq!(infer_parameter_type("rs", &vals), "&str");
        assert_eq!(infer_parameter_type("ts", &vals), "string");
        assert_eq!(infer_parameter_type("py", &vals), "str");
        assert_eq!(infer_parameter_type("go", &vals), "string");
        assert_eq!(infer_parameter_type("java", &vals), "String");
        assert_eq!(infer_parameter_type("kt", &vals), "String");
        assert_eq!(infer_parameter_type("zig", &vals), "[]const u8");
        assert_eq!(infer_parameter_type("c", &vals), "const char*");
    }

    #[test]
    fn test_infer_integer_type() {
        let vals = vec!["42".to_string(), "100".to_string()];
        assert_eq!(infer_parameter_type("rs", &vals), "i64");
        assert_eq!(infer_parameter_type("ts", &vals), "number");
        assert_eq!(infer_parameter_type("py", &vals), "int");
        assert_eq!(infer_parameter_type("go", &vals), "int");
        assert_eq!(infer_parameter_type("kt", &vals), "Int");
        assert_eq!(infer_parameter_type("zig", &vals), "i64");
        assert_eq!(infer_parameter_type("swift", &vals), "Int");
    }

    #[test]
    fn test_infer_boolean_type() {
        let vals = vec!["true".to_string(), "false".to_string()];
        assert_eq!(infer_parameter_type("rs", &vals), "bool");
        assert_eq!(infer_parameter_type("ts", &vals), "boolean");
        assert_eq!(infer_parameter_type("py", &vals), "bool");
        assert_eq!(infer_parameter_type("kt", &vals), "Boolean");
        assert_eq!(infer_parameter_type("swift", &vals), "Bool");
    }

    #[test]
    fn test_format_function_signatures_polyglot() {
        let params = vec![
            InferredParameter {
                name: "label".to_string(),
                inferred_type: "String".to_string(),
                original_values: vec!["\"a\"".to_string()],
            },
            InferredParameter {
                name: "count".to_string(),
                inferred_type: "Int".to_string(),
                original_values: vec!["10".to_string()],
            },
        ];

        let kt_sig = format_function_signature("kt", "processItem", &params);
        assert_eq!(kt_sig, "fun processItem(label: String, count: Int)");

        let scala_sig = format_function_signature("scala", "processItem", &params);
        assert_eq!(
            scala_sig,
            "def processItem(label: String, count: Int): Unit ="
        );

        let swift_sig = format_function_signature("swift", "processItem", &params);
        assert_eq!(
            swift_sig,
            "public func processItem(label: String, count: Int)"
        );

        let zig_sig = format_function_signature("zig", "processItem", &params);
        assert_eq!(
            zig_sig,
            "pub fn processItem(label: String, count: Int) void"
        );

        let rb_sig = format_function_signature("rb", "processItem", &params);
        assert_eq!(rb_sig, "def process_item(label, count)");

        let ex_sig = format_function_signature("ex", "processItem", &params);
        assert_eq!(ex_sig, "def process_item(label, count) do");

        let php_sig = format_function_signature("php", "processItem", &params);
        assert_eq!(
            php_sig,
            "function processItem(String $label, Int $count): void"
        );
    }

    #[test]
    fn test_format_call_sites_polyglot() {
        let args = vec!["\"data\"".to_string(), "42".to_string()];
        assert_eq!(
            format_call_site("kt", "helper", &args, "    "),
            "    helper(\"data\", 42)"
        );
        assert_eq!(
            format_call_site("zig", "helper", &args, "    "),
            "    helper(\"data\", 42);"
        );
        assert_eq!(
            format_call_site("swift", "helper", &args, "    "),
            "    helper(\"data\", 42)"
        );
        assert_eq!(
            format_call_site("rb", "process_items", &args, "  "),
            "  process_items(\"data\", 42)"
        );
        assert_eq!(
            format_call_site("php", "helper", &args, "    "),
            "    helper(\"data\", 42);"
        );
    }
}
