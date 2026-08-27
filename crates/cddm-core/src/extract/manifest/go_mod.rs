#![forbid(unsafe_code)]

use super::common::{create_manifest_update, resolve_caller_manifest_and_rel_path};
use crate::extract::types::ManifestUpdate;
use std::fs;
use std::path::Path;

pub fn update_go_work_root(workspace_root: &Path, new_crate_path: &str) -> Option<ManifestUpdate> {
    let root_gowork = workspace_root.join("go.work");
    if !root_gowork.exists() {
        return None;
    }
    let content = fs::read_to_string(&root_gowork).ok()?;
    let rel_path = new_crate_path.replace('\\', "/");
    let use_entry = format!("./{}", rel_path.trim_start_matches("./"));

    if content.contains(&use_entry) {
        return None;
    }

    let mut new_lines = Vec::new();
    let mut in_use = false;
    let mut added = false;

    for line in content.lines() {
        if line.trim().starts_with("use (") {
            in_use = true;
            new_lines.push(line.to_string());
            new_lines.push(format!("\t{}", use_entry));
            added = true;
            continue;
        }
        if in_use && line.trim() == ")" {
            in_use = false;
        }
        new_lines.push(line.to_string());
    }

    if !added {
        new_lines.push("\nuse (".to_string());
        new_lines.push(format!("\t{}", use_entry));
        new_lines.push(")".to_string());
    }

    let updated_content = new_lines.join("\n") + "\n";
    let diff = format!("--- a/go.work\n+++ b/go.work\n@@ use @@\n+\t{}", use_entry);

    Some(create_manifest_update(
        &root_gowork,
        workspace_root,
        new_crate_path,
        diff,
        updated_content,
    ))
}

pub fn update_caller_go_mod(
    workspace_root: &Path,
    caller_file: &str,
    new_crate_path: &str,
    new_crate_name: &str,
) -> Option<ManifestUpdate> {
    let (manifest_path, content, rel_to_target) = resolve_caller_manifest_and_rel_path(
        workspace_root,
        caller_file,
        "go.mod",
        new_crate_path,
    )?;

    let module_ident = new_crate_name.to_lowercase();
    if content.contains(&format!("require {}", module_ident))
        || content.contains(&format!("replace {}", module_ident))
    {
        return None;
    }

    let mut new_lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let req_line = format!("require {} v0.0.0", module_ident);
    new_lines.push(format!("\n{}", req_line));
    new_lines.push(format!("replace {} => {}", module_ident, rel_to_target));

    let diff = format!(
        "--- a/go.mod\n+++ b/go.mod\n@@ require/replace @@\n+require {} v0.0.0\n+replace {} => {}",
        module_ident, module_ident, rel_to_target
    );

    let updated_content = new_lines.join("\n") + "\n";

    Some(create_manifest_update(
        &manifest_path,
        workspace_root,
        &module_ident,
        diff,
        updated_content,
    ))
}
