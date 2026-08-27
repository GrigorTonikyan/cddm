#![forbid(unsafe_code)]

use super::common::{
    create_manifest_update, find_enclosing_manifest, insert_member_into_toml_array,
};
use crate::extract::types::ManifestUpdate;
use std::fs;
use std::path::Path;

pub fn update_package_json_root(
    workspace_root: &Path,
    new_crate_path: &str,
) -> Option<ManifestUpdate> {
    let root_pkg = workspace_root.join("package.json");
    if !root_pkg.exists() {
        return None;
    }
    let content = fs::read_to_string(&root_pkg).ok()?;
    if !content.contains("\"workspaces\"") {
        return None;
    }

    let rel_path = new_crate_path.replace('\\', "/");
    let entry = format!("\"{}\"", rel_path);

    if content.contains(&entry)
        || (rel_path.starts_with("packages/") && content.contains("\"packages/*\""))
    {
        return None;
    }

    let updated_content = insert_member_into_toml_array(&content, "\"workspaces\": [", &entry)?;

    let diff = format!(
        "--- a/package.json\n+++ b/package.json\n@@ workspaces @@\n+    {}",
        entry
    );

    Some(create_manifest_update(
        &root_pkg,
        workspace_root,
        new_crate_path,
        diff,
        updated_content,
    ))
}

pub fn update_caller_package_json(
    workspace_root: &Path,
    caller_file: &str,
    new_crate_name: &str,
) -> Option<ManifestUpdate> {
    let caller_path = workspace_root.join(caller_file);
    let manifest_path = find_enclosing_manifest(&caller_path, "package.json", workspace_root)?;
    let content = fs::read_to_string(&manifest_path).ok()?;

    if content.contains(&format!("\"{}\"", new_crate_name)) {
        return None;
    }

    let dep_entry = format!("    \"{}\": \"workspace:*\",", new_crate_name);
    let mut new_lines = Vec::new();
    let mut inserted = false;

    for line in content.lines() {
        if line.trim().starts_with("\"dependencies\": {") {
            new_lines.push(line.to_string());
            new_lines.push(dep_entry.clone());
            inserted = true;
            continue;
        }
        new_lines.push(line.to_string());
    }

    if !inserted {
        return None;
    }

    let updated_content = new_lines.join("\n") + "\n";
    let diff = format!(
        "--- a/package.json\n+++ b/package.json\n@@ dependencies @@\n+{}",
        dep_entry
    );

    Some(create_manifest_update(
        &manifest_path,
        workspace_root,
        new_crate_name,
        diff,
        updated_content,
    ))
}
