#![forbid(unsafe_code)]

use super::common::{
    create_manifest_update, find_enclosing_manifest, update_root_workspace_manifest,
};
use crate::extract::types::ManifestUpdate;
use std::fs;
use std::path::Path;

pub fn update_package_json_root(
    workspace_root: &Path,
    new_crate_path: &str,
) -> Option<ManifestUpdate> {
    update_root_workspace_manifest(
        workspace_root,
        "package.json",
        "\"workspaces\"",
        "\"workspaces\": [",
        new_crate_path,
    )
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
