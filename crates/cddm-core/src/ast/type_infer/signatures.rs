#![forbid(unsafe_code)]

use crate::types::InferredParameter;

/// Generates a language-specific function signature header.
pub fn format_function_signature(
    extension: &str,
    function_name: &str,
    parameters: &[InferredParameter],
) -> String {
    format_function_signature_with_return(extension, function_name, parameters, None)
}

fn join_name_colon_type(params: &[InferredParameter]) -> String {
    params
        .iter()
        .map(|p| format!("{}: {}", p.name, p.inferred_type))
        .collect::<Vec<_>>()
        .join(", ")
}

fn join_type_space_name(params: &[InferredParameter]) -> String {
    params
        .iter()
        .map(|p| format!("{} {}", p.inferred_type, p.name))
        .collect::<Vec<_>>()
        .join(", ")
}

fn join_name_only(params: &[InferredParameter]) -> String {
    params
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Generates a language-specific function signature header with an optional return type.
pub fn format_function_signature_with_return(
    extension: &str,
    function_name: &str,
    parameters: &[InferredParameter],
    return_type: Option<&str>,
) -> String {
    let ext = extension.to_lowercase();
    match ext.as_str() {
        "rs" => {
            let params = join_name_colon_type(parameters);
            if let Some(ret) = return_type {
                format!("pub fn {}({}) -> {}", function_name, params, ret)
            } else {
                format!("pub fn {}({})", function_name, params)
            }
        }
        "ts" | "tsx" => {
            let params = join_name_colon_type(parameters);
            let ret = return_type.unwrap_or("void");
            format!("export function {}({}): {}", function_name, params, ret)
        }
        "js" | "jsx" => {
            let params = join_name_only(parameters);
            format!("export function {}({})", function_name, params)
        }
        "py" => {
            let params = join_name_colon_type(parameters);
            let ret = return_type.unwrap_or("None");
            format!("def {}({}) -> {}:", function_name, params, ret)
        }
        "go" => {
            let pascal_name = to_pascal_case(function_name);
            let params = parameters
                .iter()
                .map(|p| format!("{} {}", p.name, p.inferred_type))
                .collect::<Vec<_>>()
                .join(", ");
            if let Some(ret) = return_type {
                format!("func {}({}) {}", pascal_name, params, ret)
            } else {
                format!("func {}({})", pascal_name, params)
            }
        }
        "java" => {
            let params = join_type_space_name(parameters);
            let ret = return_type.unwrap_or("void");
            format!("public static {} {}({})", ret, function_name, params)
        }
        "cs" => {
            let pascal_name = to_pascal_case(function_name);
            let params = join_type_space_name(parameters);
            let ret = return_type.unwrap_or("void");
            format!("public static {} {}({})", ret, pascal_name, params)
        }
        "kt" | "kts" => {
            let params = join_name_colon_type(parameters);
            if let Some(ret) = return_type {
                format!("fun {}({}): {}", function_name, params, ret)
            } else {
                format!("fun {}({})", function_name, params)
            }
        }
        "scala" | "sc" => {
            let params = join_name_colon_type(parameters);
            let ret = return_type.unwrap_or("Unit");
            format!("def {}({}): {} =", function_name, params, ret)
        }
        "swift" => {
            let params = join_name_colon_type(parameters);
            if let Some(ret) = return_type {
                format!("public func {}({}) -> {}", function_name, params, ret)
            } else {
                format!("public func {}({})", function_name, params)
            }
        }
        "zig" | "zon" => {
            let params = join_name_colon_type(parameters);
            let ret = return_type.unwrap_or("void");
            format!("pub fn {}({}) {}", function_name, params, ret)
        }
        "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" => {
            let params = join_type_space_name(parameters);
            let ret = return_type.unwrap_or("void");
            format!("{} {}({})", ret, function_name, params)
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
            let ret = return_type.unwrap_or("void");
            format!("function {}({}): {}", function_name, params, ret)
        }
        "rb" | "rake" => {
            let snake_name = to_snake_case(function_name);
            let params = join_name_only(parameters);
            format!("def {}({})", snake_name, params)
        }
        "ex" | "exs" => {
            let snake_name = to_snake_case(function_name);
            let params = join_name_only(parameters);
            format!("def {}({}) do", snake_name, params)
        }
        "lua" => {
            let snake_name = to_snake_case(function_name);
            let params = join_name_only(parameters);
            format!("local function {}({})", snake_name, params)
        }
        "dart" => {
            let params = join_type_space_name(parameters);
            let ret = return_type.unwrap_or("void");
            format!("{} {}({})", ret, function_name, params)
        }
        _ => {
            let params = join_name_only(parameters);
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

pub fn to_pascal_case(s: &str) -> String {
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

pub fn to_snake_case(s: &str) -> String {
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
