#![forbid(unsafe_code)]

use super::*;
use std::fs;
use tempfile::tempdir;

struct ManifestTestFixture<'a> {
    root_file: &'a str,
    root_content: &'a str,
    caller_subpath: &'a str,
    caller_file: &'a str,
    caller_content: &'a str,
    caller_source: &'a str,
    shared_path: &'a str,
    shared_name: &'a str,
    ext: &'a str,
}

fn execute_manifest_fixture(fixture: ManifestTestFixture) -> Vec<ManifestUpdate> {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join(fixture.root_file), fixture.root_content).unwrap();

    let caller_dir = root.join(fixture.caller_subpath);
    fs::create_dir_all(&caller_dir).unwrap();
    fs::write(caller_dir.join(fixture.caller_file), fixture.caller_content).unwrap();
    fs::write(caller_dir.join(fixture.caller_source), "/* test */").unwrap();

    let source_path = format!("{}/{}", fixture.caller_subpath, fixture.caller_source);
    update_workspace_manifests(
        root,
        fixture.shared_path,
        fixture.shared_name,
        &[source_path],
        fixture.ext,
    )
}

fn assert_manifest_updates(
    updates: &[ManifestUpdate],
    root_path: &str,
    root_contains: &str,
    caller_path: &str,
    caller_contains: &str,
) {
    assert_eq!(updates.len(), 2);
    assert_eq!(updates[0].manifest_path, root_path);
    assert!(updates[0].updated_content.contains(root_contains));
    assert_eq!(updates[1].manifest_path, caller_path);
    assert!(updates[1].updated_content.contains(caller_contains));
}

#[test]
fn test_cargo_manifest_updates() {
    let updates = execute_manifest_fixture(ManifestTestFixture {
        root_file: "Cargo.toml",
        root_content: "[workspace]\nmembers = [\"crates/a\"]\n",
        caller_subpath: "crates/b",
        caller_file: "Cargo.toml",
        caller_content: "[package]\nname = \"b\"\nversion = \"0.1.0\"\n[dependencies]\nserde = \
                         \"1.0\"\n",
        caller_source: "src.rs",
        shared_path: "crates/shared_utils",
        shared_name: "shared_utils",
        ext: "rs",
    });

    assert_manifest_updates(
        &updates,
        "Cargo.toml",
        "\"crates/shared_utils\"",
        "crates/b/Cargo.toml",
        "shared_utils = { path = \"../shared_utils\" }",
    );
}

#[test]
fn test_package_json_manifest_updates() {
    let updates = execute_manifest_fixture(ManifestTestFixture {
        root_file: "package.json",
        root_content: "{\n  \"name\": \"root\",\n  \"workspaces\": [\"packages/a\"]\n}\n",
        caller_subpath: "packages/b",
        caller_file: "package.json",
        caller_content: "{\n  \"name\": \"b\",\n  \"dependencies\": {\"react\": \"^19.0.0\"}\n}\n",
        caller_source: "index.ts",
        shared_path: "packages/shared-utils",
        shared_name: "shared-utils",
        ext: "ts",
    });

    assert_manifest_updates(
        &updates,
        "package.json",
        "\"packages/shared-utils\"",
        "packages/b/package.json",
        "\"shared-utils\": \"workspace:*\"",
    );
}

#[test]
fn test_pyproject_toml_manifest_updates() {
    let updates = execute_manifest_fixture(ManifestTestFixture {
        root_file: "pyproject.toml",
        root_content: "[tool.uv.workspace]\nmembers = [\"services/api\"]\n",
        caller_subpath: "services/worker",
        caller_file: "pyproject.toml",
        caller_content: "[project]\nname = \"worker\"\ndependencies = [\"requests>=2.0\"]\n",
        caller_source: "worker.py",
        shared_path: "packages/common-utils",
        shared_name: "common-utils",
        ext: "py",
    });

    assert_manifest_updates(
        &updates,
        "pyproject.toml",
        "\"packages/common-utils\"",
        "services/worker/pyproject.toml",
        "common-utils @ file://",
    );
}

#[test]
fn test_go_mod_manifest_updates() {
    let updates = execute_manifest_fixture(ManifestTestFixture {
        root_file: "go.work",
        root_content: "go 1.23.0\n\nuse (\n\t./services/api\n)\n",
        caller_subpath: "services/worker",
        caller_file: "go.mod",
        caller_content: "module worker\n\ngo 1.23.0\n",
        caller_source: "main.go",
        shared_path: "packages/common",
        shared_name: "common",
        ext: "go",
    });

    assert_manifest_updates(
        &updates,
        "go.work",
        "./packages/common",
        "services/worker/go.mod",
        "require common v0.0.0",
    );
    assert!(
        updates[1]
            .updated_content
            .contains("replace common => ../../packages/common")
    );
}
