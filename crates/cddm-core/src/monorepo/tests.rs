use super::*;
use tempfile::tempdir;

#[test]
fn test_discover_workspaces_empty() {
    let dir = tempdir().unwrap();
    let ws = discover_workspaces(dir.path());
    assert_eq!(ws.len(), 1);
    assert_eq!(ws[0].path, ".");
    assert_eq!(ws[0].package_type, PKG_TYPE_ROOT_PROJECT);
}

#[test]
fn test_discover_workspaces_cargo() {
    let dir = tempdir().unwrap();
    let root_cargo = dir.path().join("Cargo.toml");
    fs::write(root_cargo, "[workspace]\nmembers = [\"crates/*\"]\n").unwrap();

    let crates_dir = dir.path().join("crates");
    fs::create_dir_all(&crates_dir).unwrap();

    let sub_a = crates_dir.join("sub-a");
    fs::create_dir_all(&sub_a).unwrap();
    fs::write(sub_a.join("Cargo.toml"), "[package]\nname = \"sub-a\"\n").unwrap();

    let sub_b = crates_dir.join("sub-b");
    fs::create_dir_all(&sub_b).unwrap();
    fs::write(sub_b.join("Cargo.toml"), "[package]\nname = \"sub-b\"\n").unwrap();

    let ws = discover_workspaces(dir.path());
    assert_eq!(ws.len(), 2);
    assert_eq!(ws[0].package_type, PKG_TYPE_RUST_CARGO);
    assert_eq!(ws[1].package_type, PKG_TYPE_RUST_CARGO);
}

#[test]
fn test_discover_workspaces_bun() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("bun.lock"), "").unwrap();

    let pkg_dir = dir.path().join("packages").join("web-app");
    fs::create_dir_all(&pkg_dir).unwrap();
    fs::write(pkg_dir.join("package.json"), "{\"name\":\"web-app\"}").unwrap();

    let ws = discover_workspaces(dir.path());
    assert_eq!(ws.len(), 1);
    assert_eq!(ws[0].name, "web-app");
    assert_eq!(ws[0].package_type, PKG_TYPE_JS_BUN);
    assert_eq!(ws[0].manifest_file, "package.json");
}

#[test]
fn test_discover_workspaces_pnpm() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("pnpm-workspace.yaml"),
        "packages:\n  - 'apps/*'\n",
    )
    .unwrap();

    let pkg_dir = dir.path().join("apps").join("client");
    fs::create_dir_all(&pkg_dir).unwrap();
    fs::write(pkg_dir.join("package.json"), "{\"name\":\"client\"}").unwrap();

    let ws = discover_workspaces(dir.path());
    assert_eq!(ws.len(), 1);
    assert_eq!(ws[0].name, "client");
    assert_eq!(ws[0].package_type, PKG_TYPE_JS_PNPM);
}

#[test]
fn test_discover_workspaces_python_uv() {
    let dir = tempdir().unwrap();
    let svc_dir = dir.path().join("services").join("ai-backend");
    fs::create_dir_all(&svc_dir).unwrap();
    fs::write(
        svc_dir.join("pyproject.toml"),
        "[project]\nname = \"ai-backend\"\n",
    )
    .unwrap();
    fs::write(svc_dir.join("uv.lock"), "").unwrap();

    let ws = discover_workspaces(dir.path());
    assert_eq!(ws.len(), 1);
    assert_eq!(ws[0].name, "ai-backend");
    assert_eq!(ws[0].package_type, PKG_TYPE_PYTHON_UV_POETRY);
    assert_eq!(ws[0].manifest_file, "pyproject.toml");
}

#[test]
fn test_discover_workspaces_python_requirements() {
    let dir = tempdir().unwrap();
    let svc_dir = dir.path().join("services").join("legacy-service");
    fs::create_dir_all(&svc_dir).unwrap();
    fs::write(svc_dir.join("requirements.txt"), "flask>=2.0\n").unwrap();

    let ws = discover_workspaces(dir.path());
    assert_eq!(ws.len(), 1);
    assert_eq!(ws[0].name, "legacy-service");
    assert_eq!(ws[0].package_type, PKG_TYPE_PYTHON_UV_POETRY);
    assert_eq!(ws[0].manifest_file, "requirements.txt");
}

#[test]
fn test_discover_workspaces_go() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("go.work"),
        "go 1.22\n\nuse (\n\t./services/auth\n)\n",
    )
    .unwrap();

    let auth_dir = dir.path().join("services").join("auth");
    fs::create_dir_all(&auth_dir).unwrap();
    fs::write(auth_dir.join("go.mod"), "module auth\n\ngo 1.22\n").unwrap();

    let ws = discover_workspaces(dir.path());
    assert_eq!(ws.len(), 1);
    assert_eq!(ws[0].name, "auth");
    assert_eq!(ws[0].package_type, PKG_TYPE_GO_WORKSPACE);
    assert_eq!(ws[0].manifest_file, "go.mod");
}

#[test]
fn test_discover_workspaces_polyglot() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("bunfig.toml"), "").unwrap();

    // 1. Rust crate
    let root_cargo = dir.path().join("Cargo.toml");
    fs::write(root_cargo, "[workspace]\nmembers = [\"crates/*\"]\n").unwrap();
    let crate_dir = dir.path().join("crates").join("core-engine");
    fs::create_dir_all(&crate_dir).unwrap();
    fs::write(
        crate_dir.join("Cargo.toml"),
        "[package]\nname=\"core-engine\"\n",
    )
    .unwrap();

    // 2. Bun package
    let bun_dir = dir.path().join("packages").join("frontend");
    fs::create_dir_all(&bun_dir).unwrap();
    fs::write(bun_dir.join("package.json"), "{\"name\":\"frontend\"}").unwrap();

    // 3. Python service
    let py_dir = dir.path().join("services").join("worker");
    fs::create_dir_all(&py_dir).unwrap();
    fs::write(
        py_dir.join("pyproject.toml"),
        "[project]\nname=\"worker\"\n",
    )
    .unwrap();

    let ws = discover_workspaces(dir.path());
    assert_eq!(ws.len(), 3);
    assert_eq!(ws[0].name, "core-engine");
    assert_eq!(ws[0].package_type, PKG_TYPE_RUST_CARGO);
    assert_eq!(ws[1].name, "frontend");
    assert_eq!(ws[1].package_type, PKG_TYPE_JS_BUN);
    assert_eq!(ws[2].name, "worker");
    assert_eq!(ws[2].package_type, PKG_TYPE_PYTHON_UV_POETRY);
}
