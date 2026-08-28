use super::*;
use crate::types::{CloneLocation, InferredParameter};
use tempfile::tempdir;

#[test]
fn test_generate_shared_extraction_new_crate_rust() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\n    \"crates/app_a\",\n    \"crates/app_b\",\n]\n",
    )
    .unwrap();

    let app_a = root.join("crates/app_a");
    fs::create_dir_all(app_a.join("src")).unwrap();
    fs::write(
        app_a.join("Cargo.toml"),
        "[package]\nname = \"app_a\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
    )
    .unwrap();
    fs::write(
        app_a.join("src/main.rs"),
        "fn main() {\n    let val = 42;\n    println!(\"{}\", val * 2);\n}\n",
    )
    .unwrap();

    let app_b = root.join("crates/app_b");
    fs::create_dir_all(app_b.join("src")).unwrap();
    fs::write(
        app_b.join("Cargo.toml"),
        "[package]\nname = \"app_b\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
    )
    .unwrap();
    fs::write(
        app_b.join("src/main.rs"),
        "fn run() {\n    let val = 42;\n    println!(\"{}\", val * 2);\n}\n",
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
                file: "crates/app_b/src/main.rs".to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
        ],
        target_path: "crates/shared_math".to_string(),
        custom_function_name: Some("compute_double".to_string()),
        target_kind: ExtractTargetKind::NewCrate,
        custom_parameter_names: None,
        generate_tests: false,
        dry_run: false,
    };

    let res = apply_shared_extraction(root, &req);
    assert!(res.is_ok(), "{:?}", res.err());
    let result = res.unwrap();

    assert_eq!(result.function_name, "compute_double");
    assert_eq!(result.generated_files.len(), 2);
    assert!(root.join("crates/shared_math/Cargo.toml").exists());
    assert!(root.join("crates/shared_math/src/lib.rs").exists());

    let lib_content = fs::read_to_string(root.join("crates/shared_math/src/lib.rs")).unwrap();
    assert!(lib_content.contains("pub fn compute_double()"));

    let app_a_cargo = fs::read_to_string(app_a.join("Cargo.toml")).unwrap();
    assert!(app_a_cargo.contains("shared_math = { path = \"../shared_math\" }"));
}

#[test]
fn test_generate_shared_extraction_module_typescript() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join("src/components")).unwrap();
    fs::write(
        root.join("src/components/A.ts"),
        "export function doA() {\n    const x = 100;\n    return x * 2;\n}\n",
    )
    .unwrap();
    fs::write(
        root.join("src/components/B.ts"),
        "export function doB() {\n    const x = 100;\n    return x * 2;\n}\n",
    )
    .unwrap();

    let req = ExtractRequest {
        occurrences: vec![
            CloneLocation {
                file: "src/components/A.ts".to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
            CloneLocation {
                file: "src/components/B.ts".to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
        ],
        target_path: "src/components/common_utils".to_string(),
        custom_function_name: Some("doubleValue".to_string()),
        target_kind: ExtractTargetKind::NewModule,
        custom_parameter_names: None,
        generate_tests: false,
        dry_run: false,
    };

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
    let dir = tempdir().unwrap();
    let root = dir.path();

    let svc_a = root.join("services/api");
    fs::create_dir_all(&svc_a).unwrap();
    fs::write(
        svc_a.join("pyproject.toml"),
        "[project]\nname = \"api\"\ndependencies = []\n",
    )
    .unwrap();
    fs::write(
        svc_a.join("views.py"),
        "def handle():\n    val = 10\n    return val * 5\n",
    )
    .unwrap();

    let svc_b = root.join("services/worker");
    fs::create_dir_all(&svc_b).unwrap();
    fs::write(
        svc_b.join("pyproject.toml"),
        "[project]\nname = \"worker\"\ndependencies = []\n",
    )
    .unwrap();
    fs::write(
        svc_b.join("tasks.py"),
        "def run():\n    val = 10\n    return val * 5\n",
    )
    .unwrap();

    let req = ExtractRequest {
        occurrences: vec![
            CloneLocation {
                file: "services/api/views.py".to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
            CloneLocation {
                file: "services/worker/tasks.py".to_string(),
                start_line: 2,
                end_line: 3,
                author: None,
            },
        ],
        target_path: "packages/math_utils".to_string(),
        custom_function_name: Some("calc_multiplier".to_string()),
        target_kind: ExtractTargetKind::NewCrate,
        custom_parameter_names: None,
        generate_tests: false,
        dry_run: false,
    };

    let res = apply_shared_extraction(root, &req);
    assert!(res.is_ok(), "{:?}", res.err());
    let result = res.unwrap();

    assert_eq!(result.function_name, "calc_multiplier");
    assert!(root.join("packages/math_utils/pyproject.toml").exists());
    assert!(root.join("packages/math_utils/__init__.py").exists());
}

#[test]
fn test_generate_shared_extraction_with_unit_tests() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/app_a\", \"crates/app_b\"]\n",
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
        generate_tests: true,
        dry_run: false,
    };

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
