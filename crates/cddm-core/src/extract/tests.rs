use super::*;
use crate::types::{CloneLocation, InferredParameter};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_generate_shared_extraction_new_crate_rust() {
    let (dir, mut req) = setup_dummy_rust_app(false, false);
    let root = dir.path();
    req.target_path = "crates/shared_math".to_string();
    req.custom_function_name = Some("compute_double".to_string());

    let res = apply_shared_extraction(root, &req);
    assert!(res.is_ok(), "{:?}", res.err());
    let result = res.unwrap();

    assert_eq!(result.function_name, "compute_double");
    assert_eq!(result.generated_files.len(), 2);
    assert!(root.join("crates/shared_math/Cargo.toml").exists());
    assert!(root.join("crates/shared_math/src/lib.rs").exists());

    let lib_content = fs::read_to_string(root.join("crates/shared_math/src/lib.rs")).unwrap();
    assert!(lib_content.contains("pub fn compute_double()"));

    let app_a_cargo = fs::read_to_string(root.join("crates/app_a/Cargo.toml")).unwrap();
    assert!(app_a_cargo.contains("shared_math = { path = \"../shared_math\" }"));
}

fn setup_two_file_extract_fixture(
    rel_path_a: &str,
    code_a: &str,
    rel_path_b: &str,
    code_b: &str,
    target_path: &str,
    target_kind: ExtractTargetKind,
    fn_name: &str,
) -> (tempfile::TempDir, ExtractRequest) {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let file_a = root.join(rel_path_a);
    let file_b = root.join(rel_path_b);
    fs::create_dir_all(file_a.parent().unwrap()).unwrap();
    fs::create_dir_all(file_b.parent().unwrap()).unwrap();
    fs::write(&file_a, code_a).unwrap();
    fs::write(&file_b, code_b).unwrap();

    let req = ExtractRequest {
        occurrences: vec![
            CloneLocation {
                file: rel_path_a.to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
            CloneLocation {
                file: rel_path_b.to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
        ],
        target_path: target_path.to_string(),
        custom_function_name: Some(fn_name.to_string()),
        target_kind,
        custom_parameter_names: None,
        generate_tests: false,
        generate_benchmarks: false,
        dry_run: false,
    };
    (dir, req)
}

#[test]
fn test_generate_shared_extraction_module_typescript() {
    let (dir, req) = setup_two_file_extract_fixture(
        "src/components/A.ts",
        "export function doA() {\n    const x = 100;\n    return x * 2;\n}\n",
        "src/components/B.ts",
        "export function doB() {\n    const x = 100;\n    return x * 2;\n}\n",
        "src/components/common_utils",
        ExtractTargetKind::NewModule,
        "doubleValue",
    );
    let root = dir.path();

    let res = apply_shared_extraction(root, &req);
    assert!(res.is_ok(), "{:?}", res.err());
    let result = res.unwrap();

    assert_eq!(result.function_name, "doubleValue");
    assert!(root.join("src/components/common_utils.ts").exists());

    let mod_content = fs::read_to_string(root.join("src/components/common_utils.ts")).unwrap();
    assert!(mod_content.contains("export function doubleValue()"));
}

#[test]
fn test_generate_shared_extraction_python_package() {
    let (dir, req) = setup_two_file_extract_fixture(
        "services/api/views.py",
        "def handle():\n    val = 10\n    return val * 5\n",
        "services/worker/tasks.py",
        "def run():\n    val = 10\n    return val * 5\n",
        "packages/math_utils",
        ExtractTargetKind::NewCrate,
        "calc_multiplier",
    );
    let root = dir.path();

    let res = apply_shared_extraction(root, &req);
    assert!(res.is_ok(), "{:?}", res.err());
    let result = res.unwrap();

    assert_eq!(result.function_name, "calc_multiplier");
    assert!(root.join("packages/math_utils/pyproject.toml").exists());
    assert!(root.join("packages/math_utils/__init__.py").exists());
}

fn setup_dummy_rust_app(
    generate_tests: bool,
    generate_benchmarks: bool,
) -> (tempfile::TempDir, ExtractRequest) {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/app_a\"]\n",
    )
    .unwrap();

    let app_a = root.join("crates/app_a/src");
    fs::create_dir_all(&app_a).unwrap();
    fs::write(
        root.join("crates/app_a/Cargo.toml"),
        "[package]\nname = \"app_a\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    fs::write(
        app_a.join("main.rs"),
        "fn main() {\n    let val = 10;\n    println!(\"{}\", val);\n}\n",
    )
    .unwrap();

    let req = ExtractRequest {
        occurrences: vec![
            CloneLocation {
                file: "crates/app_a/src/main.rs".to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
            CloneLocation {
                file: "crates/app_a/src/main.rs".to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
        ],
        target_path: "crates/shared_tools".to_string(),
        custom_function_name: Some("process_val".to_string()),
        target_kind: ExtractTargetKind::NewCrate,
        custom_parameter_names: None,
        generate_tests,
        generate_benchmarks,
        dry_run: false,
    };

    (dir, req)
}

#[test]
fn test_generate_shared_extraction_with_unit_tests() {
    let (dir, req) = setup_dummy_rust_app(true, false);
    let root = dir.path();

    let res = apply_shared_extraction(root, &req);
    assert!(res.is_ok(), "{:?}", res.err());
    let result = res.unwrap();

    assert_eq!(result.test_files.len(), 1);
    assert!(
        root.join("crates/shared_tools/tests/process_val_test.rs")
            .exists()
    );
}

fn check_generated_test(
    path: &str,
    kind: ExtractTargetKind,
    func: &str,
    param_name: &str,
    param_type: &str,
    param_val: &str,
    lang: &str,
) -> ExtractedFile {
    let params = vec![InferredParameter {
        name: param_name.to_string(),
        inferred_type: param_type.to_string(),
        original_values: vec![param_val.to_string()],
    }];
    let mut tests = test_generator::generate_unit_test_files(path, kind, func, &params, lang);
    assert_eq!(tests.len(), 1);
    tests.remove(0)
}

#[test]
fn test_generate_rust_unit_tests() {
    let t = check_generated_test(
        "crates/math_utils",
        ExtractTargetKind::NewCrate,
        "compute_double",
        "val",
        "i32",
        "100",
        "rs",
    );
    assert_eq!(
        t.file_path,
        "crates/math_utils/tests/compute_double_test.rs"
    );
    assert!(t.content.contains("fn test_compute_double_execution()"));
    assert!(t.content.contains("compute_double(100);"));
}

#[test]
fn test_generate_typescript_unit_tests() {
    let t = check_generated_test(
        "packages/list_helpers",
        ExtractTargetKind::NewCrate,
        "processItems",
        "items",
        "string[]",
        "[\"a\", \"b\"]",
        "ts",
    );
    assert_eq!(
        t.file_path,
        "packages/list_helpers/src/process_items.test.ts"
    );
    assert!(
        t.content
            .contains("import { describe, expect, it } from \"vitest\";")
    );
    assert!(
        t.content
            .contains("import { processItems } from \"./index\";")
    );
}

#[test]
fn test_generate_python_unit_tests() {
    let t = check_generated_test(
        "services/common_math",
        ExtractTargetKind::NewCrate,
        "calculate_total",
        "multiplier",
        "int",
        "5",
        "py",
    );
    assert_eq!(
        t.file_path,
        "services/common_math/tests/test_calculate_total.py"
    );
    assert!(t.content.contains("import pytest"));
    assert!(t.content.contains("def test_calculate_total_execution():"));
}

#[test]
fn test_generate_shared_extraction_with_benchmarks() {
    let (dir, req) = setup_dummy_rust_app(true, true);
    let root = dir.path();

    let res = apply_shared_extraction(root, &req);
    assert!(res.is_ok(), "{:?}", res.err());
    let result = res.unwrap();

    assert_eq!(result.benchmark_files.len(), 1);
    assert!(
        root.join("crates/shared_tools/benches/process_val_bench.rs")
            .exists()
    );
}

#[test]
fn test_generate_python_target_files() {
    let params = vec![InferredParameter {
        name: "x".to_string(),
        inferred_type: "int".to_string(),
        original_values: vec!["10".to_string()],
    }];
    let body = vec!["print(x)".to_string()];

    let (sig, files) = generate_extracted_target_files(
        "packages/data_helpers",
        ExtractTargetKind::NewCrate,
        "process",
        &params,
        &body,
        "py",
    );

    assert_eq!(sig, "def process(x: int) -> None:");
    assert_eq!(files.len(), 2);
    assert_eq!(files[0].file_path, "packages/data_helpers/pyproject.toml");
    assert_eq!(files[1].file_path, "packages/data_helpers/__init__.py");
    assert!(files[1].content.contains("def process(x: int) -> None:"));
}

#[test]
fn test_generate_java_and_csharp_target_files() {
    let params = vec![InferredParameter {
        name: "msg".to_string(),
        inferred_type: "String".to_string(),
        original_values: vec!["\"test\"".to_string()],
    }];
    let body = vec!["System.out.println(msg);".to_string()];

    let (sig_j, files_j) = generate_extracted_target_files(
        "packages/core-utils",
        ExtractTargetKind::NewCrate,
        "logMessage",
        &params,
        &body,
        "java",
    );
    assert_eq!(sig_j, "public static void logMessage(String msg)");
    assert_eq!(files_j.len(), 2);

    let (sig_c, files_c) = generate_extracted_target_files(
        "packages/math_helpers",
        ExtractTargetKind::NewCrate,
        "compute",
        &params,
        &body,
        "cs",
    );
    assert_eq!(sig_c, "public static void Compute(String msg)");
    assert_eq!(files_c.len(), 2);
}
