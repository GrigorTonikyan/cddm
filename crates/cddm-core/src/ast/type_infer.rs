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
            "java" | "cs" => "string".to_string(),
            "c" | "cpp" | "h" | "hpp" => "const char*".to_string(),
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
            "java" | "cs" => "int".to_string(),
            "c" | "cpp" | "h" | "hpp" => "int".to_string(),
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
            "java" | "cs" => "double".to_string(),
            "c" | "cpp" | "h" | "hpp" => "double".to_string(),
            _ => "float".to_string(),
        };
    }

    let is_all_booleans = values.iter().all(|v| {
        let trimmed = v.trim().to_lowercase();
        trimmed == "true" || trimmed == "false"
    });
    if is_all_booleans {
        return match ext.as_str() {
            "rs" | "go" | "py" => "bool".to_string(),
            "ts" | "tsx" | "js" | "jsx" => "boolean".to_string(),
            "java" | "cs" | "c" | "cpp" => "bool".to_string(),
            _ => "bool".to_string(),
        };
    }

    default_type_for_ext(&ext)
}

fn default_type_for_ext(ext: &str) -> String {
    match ext {
        "rs" => "&str".to_string(),
        "ts" | "tsx" => "any".to_string(),
        "js" | "jsx" => "".to_string(),
        "py" => "Any".to_string(),
        "go" => "any".to_string(),
        "java" => "Object".to_string(),
        "cs" => "object".to_string(),
        "c" | "cpp" => "const void*".to_string(),
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
        "c" | "cpp" | "h" | "hpp" => {
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
        "rs" | "c" | "cpp" | "java" => format!("{}{}({});", indent, function_name, args_joined),
        "ts" | "tsx" | "js" | "jsx" => format!("{}{}({});", indent, function_name, args_joined),
        "py" => format!("{}{}({})", indent, function_name, args_joined),
        "go" => {
            let pascal = to_pascal_case(function_name);
            format!("{}{}({})", indent, pascal, args_joined)
        }
        "cs" => {
            let pascal = to_pascal_case(function_name);
            format!("{}{}({});", indent, pascal, args_joined)
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
    }

    #[test]
    fn test_infer_integer_type() {
        let vals = vec!["42".to_string(), "100".to_string()];
        assert_eq!(infer_parameter_type("rs", &vals), "i64");
        assert_eq!(infer_parameter_type("ts", &vals), "number");
        assert_eq!(infer_parameter_type("py", &vals), "int");
        assert_eq!(infer_parameter_type("go", &vals), "int");
    }

    #[test]
    fn test_infer_boolean_type() {
        let vals = vec!["true".to_string(), "false".to_string()];
        assert_eq!(infer_parameter_type("rs", &vals), "bool");
        assert_eq!(infer_parameter_type("ts", &vals), "boolean");
        assert_eq!(infer_parameter_type("py", &vals), "bool");
    }

    #[test]
    fn test_format_function_signatures() {
        let params = vec![
            InferredParameter {
                name: "label".to_string(),
                inferred_type: "&str".to_string(),
                original_values: vec!["\"a\"".to_string()],
            },
            InferredParameter {
                name: "count".to_string(),
                inferred_type: "i64".to_string(),
                original_values: vec!["10".to_string()],
            },
        ];

        let rust_sig = format_function_signature("rs", "process_item", &params);
        assert_eq!(rust_sig, "pub fn process_item(label: &str, count: i64)");

        let ts_params = vec![
            InferredParameter {
                name: "label".to_string(),
                inferred_type: "string".to_string(),
                original_values: vec![],
            },
            InferredParameter {
                name: "count".to_string(),
                inferred_type: "number".to_string(),
                original_values: vec![],
            },
        ];
        let ts_sig = format_function_signature("ts", "processItem", &ts_params);
        assert_eq!(
            ts_sig,
            "export function processItem(label: string, count: number): void"
        );

        let py_sig = format_function_signature("py", "process_item", &ts_params);
        assert_eq!(
            py_sig,
            "def process_item(label: string, count: number) -> None:"
        );
    }

    #[test]
    fn test_format_call_sites() {
        let args = vec!["\"data\"".to_string(), "42".to_string()];
        assert_eq!(
            format_call_site("rs", "helper", &args, "    "),
            "    helper(\"data\", 42);"
        );
        assert_eq!(
            format_call_site("py", "helper", &args, "  "),
            "  helper(\"data\", 42)"
        );
        assert_eq!(
            format_call_site("go", "extract_val", &args, "\t"),
            "\tExtractVal(\"data\", 42)"
        );
    }
}
