#![forbid(unsafe_code)]

use super::*;
use crate::types::InferredParameter;

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
fn test_infer_return_types() {
    let int_vals = vec!["42".to_string(), "100".to_string()];
    assert_eq!(infer_return_type("rs", &int_vals), Some("i64".to_string()));
    assert_eq!(infer_return_type("py", &int_vals), Some("int".to_string()));
    assert_eq!(infer_return_type("go", &int_vals), Some("int".to_string()));
    assert_eq!(
        infer_return_type("ts", &int_vals),
        Some("number".to_string())
    );

    let str_vals = vec!["\"a\"".to_string()];
    assert_eq!(
        infer_return_type("rs", &str_vals),
        Some("String".to_string())
    );
    assert_eq!(infer_return_type("py", &str_vals), Some("str".to_string()));
    assert_eq!(
        infer_return_type("java", &str_vals),
        Some("String".to_string())
    );
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

    let py_sig_ret = format_function_signature_with_return("py", "calculate", &params, Some("int"));
    assert_eq!(
        py_sig_ret,
        "def calculate(label: String, count: Int) -> int:"
    );

    let rs_sig_ret = format_function_signature_with_return("rs", "calculate", &params, Some("i64"));
    assert_eq!(
        rs_sig_ret,
        "pub fn calculate(label: String, count: Int) -> i64"
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
