#![forbid(unsafe_code)]

use super::common::{
    create_manifest_update, insert_member_into_toml_array, resolve_caller_manifest_and_rel_path,
};
use crate::extract::types::ManifestUpdate;
use std::fs;
use std::path::Path;

pub fn update_pyproject_toml_root(
    workspace_root: &Path,
    new_crate_path: &str,
) -> Option<ManifestUpdate> {
    let root_pyproject = workspace_root.join("pyproject.toml");
    if !root_pyproject.exists() {
        return None;
    }
    let content = fs::read_to_string(&root_pyproject).ok()?;
    let rel_path = new_crate_path.replace('\\', "/");
    let entry = format!("\"{}\"", rel_path);

    if content.contains(&entry) {
        return None;
    }

    if content.contains("[tool.uv.workspace]") || content.contains("members = [") {
        let updated_content = insert_member_into_toml_array(&content, "members = [", &entry)?;
        let diff = format!(
            "--- a/pyproject.toml\n+++ b/pyproject.toml\n@@ workspace.members @@\n+    {}",
            entry
        );
        return Some(create_manifest_update(
            &root_pyproject,
            workspace_root,
            new_crate_path,
            diff,
            updated_content,
        ));
    }

    None
}

pub fn update_caller_pyproject_toml(
    workspace_root: &Path,
    caller_file: &str,
    new_crate_path: &str,
    new_crate_name: &str,
) -> Option<ManifestUpdate> {
    let (manifest_path, content, rel_to_target) = resolve_caller_manifest_and_rel_path(
        workspace_root,
        caller_file,
        "pyproject.toml",
        new_crate_path,
    )?;

    let pkg_name = new_crate_name.replace('_', "-");
    if content.contains(&format!("\"{}\"", pkg_name))
        || content.contains(&format!("{} =", pkg_name))
    {
        return None;
    }

    let mut new_lines = Vec::new();
    let mut inserted = false;

    if content.contains("[tool.poetry.dependencies]") {
        let dep_line = format!(
            "{} = {{ path = \"{}\", develop = true }}",
            pkg_name, rel_to_target
        );
        for line in content.lines() {
            new_lines.push(line.to_string());
            if line.trim() == "[tool.poetry.dependencies]" {
                new_lines.push(dep_line.clone());
                inserted = true;
            }
        }
    } else if content.contains("dependencies = [") {
        let dep_line = format!("    \"{} @ file://{}\",", pkg_name, rel_to_target);
        for line in content.lines() {
            new_lines.push(line.to_string());
            if line.trim().starts_with("dependencies = [") {
                new_lines.push(dep_line.clone());
                inserted = true;
            }
        }
    } else {
        new_lines = content.lines().map(|s| s.to_string()).collect();
        new_lines.push("\n[project]".to_string());
        new_lines.push(format!(
            "dependencies = [\"{} @ file://{}\"]",
            pkg_name, rel_to_target
        ));
        inserted = true;
    }

    if !inserted {
        return None;
    }

    let updated_content = new_lines.join("\n") + "\n";
    let diff = format!(
        "--- a/pyproject.toml\n+++ b/pyproject.toml\n@@ dependencies @@\n+    {}",
        pkg_name
    );

    Some(create_manifest_update(
        &manifest_path,
        workspace_root,
        &pkg_name,
        diff,
        updated_content,
    ))
}
