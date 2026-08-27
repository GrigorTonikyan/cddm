#![forbid(unsafe_code)]

use super::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cargo_manifest_updates() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let root_cargo = r#"[workspace]
members = [
    "crates/a",
]
"#;
    fs::write(root.join("Cargo.toml"), root_cargo).unwrap();

    let caller_dir = root.join("crates/b");
    fs::create_dir_all(&caller_dir).unwrap();
    let caller_cargo = r#"[package]
name = "b"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#;
    fs::write(caller_dir.join("Cargo.toml"), caller_cargo).unwrap();
    fs::write(caller_dir.join("src.rs"), "fn foo() {}").unwrap();

    let updates = update_workspace_manifests(
        root,
        "crates/shared_utils",
        "shared_utils",
        &["crates/b/src.rs".to_string()],
        "rs",
    );

    assert_eq!(updates.len(), 2);
    assert_eq!(updates[0].manifest_path, "Cargo.toml");
    assert!(
        updates[0]
            .updated_content
            .contains("\"crates/shared_utils\"")
    );
    assert_eq!(updates[1].manifest_path, "crates/b/Cargo.toml");
    assert!(
        updates[1]
            .updated_content
            .contains("shared_utils = { path = \"../shared_utils\" }")
    );
}

#[test]
fn test_package_json_manifest_updates() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let root_pkg = r#"{
  "name": "root",
  "workspaces": [
    "packages/a"
  ]
}
"#;
    fs::write(root.join("package.json"), root_pkg).unwrap();

    let caller_dir = root.join("packages/b");
    fs::create_dir_all(&caller_dir).unwrap();
    let caller_pkg = r#"{
  "name": "b",
  "dependencies": {
    "react": "^19.0.0"
  }
}
"#;
    fs::write(caller_dir.join("package.json"), caller_pkg).unwrap();
    fs::write(caller_dir.join("index.ts"), "export const x = 1;").unwrap();

    let updates = update_workspace_manifests(
        root,
        "packages/shared-utils",
        "shared-utils",
        &["packages/b/index.ts".to_string()],
        "ts",
    );

    assert_eq!(updates.len(), 2);
    assert_eq!(updates[0].manifest_path, "package.json");
    assert!(
        updates[0]
            .updated_content
            .contains("\"packages/shared-utils\"")
    );
    assert_eq!(updates[1].manifest_path, "packages/b/package.json");
    assert!(
        updates[1]
            .updated_content
            .contains("\"shared-utils\": \"workspace:*\"")
    );
}

#[test]
fn test_pyproject_toml_manifest_updates() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let root_pyproject = r#"[tool.uv.workspace]
members = [
    "services/api",
]
"#;
    fs::write(root.join("pyproject.toml"), root_pyproject).unwrap();

    let caller_dir = root.join("services/worker");
    fs::create_dir_all(&caller_dir).unwrap();
    let caller_pyproject = r#"[project]
name = "worker"
dependencies = [
    "requests>=2.0",
]
"#;
    fs::write(caller_dir.join("pyproject.toml"), caller_pyproject).unwrap();
    fs::write(caller_dir.join("worker.py"), "def run(): pass").unwrap();

    let updates = update_workspace_manifests(
        root,
        "packages/common-utils",
        "common-utils",
        &["services/worker/worker.py".to_string()],
        "py",
    );

    assert_eq!(updates.len(), 2);
    assert_eq!(updates[0].manifest_path, "pyproject.toml");
    assert!(
        updates[0]
            .updated_content
            .contains("\"packages/common-utils\"")
    );
    assert_eq!(updates[1].manifest_path, "services/worker/pyproject.toml");
    assert!(
        updates[1]
            .updated_content
            .contains("common-utils @ file://")
    );
}

#[test]
fn test_go_mod_manifest_updates() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let root_gowork = r#"go 1.23.0

use (
	./services/api
)
"#;
    fs::write(root.join("go.work"), root_gowork).unwrap();

    let caller_dir = root.join("services/worker");
    fs::create_dir_all(&caller_dir).unwrap();
    let caller_gomod = r#"module worker

go 1.23.0
"#;
    fs::write(caller_dir.join("go.mod"), caller_gomod).unwrap();
    fs::write(caller_dir.join("main.go"), "package main").unwrap();

    let updates = update_workspace_manifests(
        root,
        "packages/common",
        "common",
        &["services/worker/main.go".to_string()],
        "go",
    );

    assert_eq!(updates.len(), 2);
    assert_eq!(updates[0].manifest_path, "go.work");
    assert!(updates[0].updated_content.contains("./packages/common"));
    assert_eq!(updates[1].manifest_path, "services/worker/go.mod");
    assert!(updates[1].updated_content.contains("require common v0.0.0"));
    assert!(
        updates[1]
            .updated_content
            .contains("replace common => ../../packages/common")
    );
}
