#![forbid(unsafe_code)]

use super::common::{
    create_manifest_update, insert_member_into_toml_array, resolve_caller_manifest_and_rel_path,
};
use crate::extract::types::ManifestUpdate;
use std::fs;
use std::path::Path;

pub fn update_cargo_workspace_root(
    workspace_root: &Path,
    new_crate_path: &str,
) -> Option<ManifestUpdate> {
    let root_cargo = workspace_root.join("Cargo.toml");
    if !root_cargo.exists() {
        return None;
    }
    let content = fs::read_to_string(&root_cargo).ok()?;
    if !content.contains("[workspace]") {
        return None;
    }

    let rel_path = new_crate_path.replace('\\', "/");
    let member_entry = format!("\"{}\"", rel_path);

    if content.contains(&member_entry)
        || (rel_path.starts_with("crates/") && content.contains("\"crates/*\""))
        || (rel_path.starts_with("packages/") && content.contains("\"packages/*\""))
    {
        return None;
    }

    let updated_content = insert_member_into_toml_array(&content, "members = [", &member_entry)?;

    let diff = format!(
        "--- a/Cargo.toml\n+++ b/Cargo.toml\n@@ workspace.members @@\n+    {}",
        member_entry
    );

    Some(create_manifest_update(
        &root_cargo,
        workspace_root,
        new_crate_path,
        diff,
        updated_content,
    ))
}

pub fn update_caller_cargo_toml(
    workspace_root: &Path,
    caller_file: &str,
    new_crate_path: &str,
    new_crate_name: &str,
) -> Option<ManifestUpdate> {
    let (manifest_path, content, rel_to_target) = resolve_caller_manifest_and_rel_path(
        workspace_root,
        caller_file,
        "Cargo.toml",
        new_crate_path,
    )?;

    let crate_ident = new_crate_name.replace('-', "_");
    if content.contains(&format!("{} =", crate_ident))
        || content.contains(&format!("\"{}\" =", crate_ident))
        || content.contains(&format!("{} =", new_crate_name))
    {
        return None;
    }

    let dep_line = format!("{} = {{ path = \"{}\" }}", crate_ident, rel_to_target);

    let mut new_lines = Vec::new();
    let mut inserted = false;
    let mut in_deps = false;

    for line in content.lines() {
        if line.trim() == "[dependencies]" {
            in_deps = true;
            new_lines.push(line.to_string());
            new_lines.push(dep_line.clone());
            inserted = true;
            continue;
        }

        if in_deps && line.trim().starts_with('[') {
            in_deps = false;
        }

        new_lines.push(line.to_string());
    }

    if !inserted {
        new_lines.push("\n[dependencies]".to_string());
        new_lines.push(dep_line.clone());
    }

    let updated_content = new_lines.join("\n") + "\n";
    let diff = format!(
        "--- a/Cargo.toml\n+++ b/Cargo.toml\n@@ [dependencies] @@\n+{}",
        dep_line
    );

    Some(create_manifest_update(
        &manifest_path,
        workspace_root,
        &crate_ident,
        diff,
        updated_content,
    ))
}
