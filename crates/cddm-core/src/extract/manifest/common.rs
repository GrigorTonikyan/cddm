#![forbid(unsafe_code)]

use crate::extract::types::ManifestUpdate;
use std::path::{Path, PathBuf};

pub fn find_enclosing_manifest(
    start_path: &Path,
    manifest_name: &str,
    workspace_root: &Path,
) -> Option<PathBuf> {
    let mut current = if start_path.is_file() {
        start_path.parent()?
    } else {
        start_path
    };

    while current >= workspace_root {
        let candidate = current.join(manifest_name);
        if candidate.exists() && candidate != workspace_root.join(manifest_name) {
            return Some(candidate);
        }
        current = current.parent()?;
    }

    None
}

pub fn compute_relative_path(from: &Path, to: &Path) -> String {
    let from_comps: Vec<_> = from.components().collect();
    let to_comps: Vec<_> = to.components().collect();

    let mut common_len = 0;
    while common_len < from_comps.len()
        && common_len < to_comps.len()
        && from_comps[common_len] == to_comps[common_len]
    {
        common_len += 1;
    }

    let up_count = from_comps.len() - common_len;
    let mut rel = PathBuf::new();
    for _ in 0..up_count {
        rel.push("..");
    }
    for comp in &to_comps[common_len..] {
        rel.push(comp);
    }

    if rel.as_os_str().is_empty() {
        ".".to_string()
    } else {
        rel.to_string_lossy().replace('\\', "/")
    }
}

pub fn resolve_caller_manifest_content(
    workspace_root: &Path,
    caller_file: &str,
    manifest_name: &str,
) -> Option<(PathBuf, String)> {
    let caller_path = workspace_root.join(caller_file);
    let manifest_path = find_enclosing_manifest(&caller_path, manifest_name, workspace_root)?;
    let content = std::fs::read_to_string(&manifest_path).ok()?;
    Some((manifest_path, content))
}

pub fn resolve_caller_manifest_and_rel_path(
    workspace_root: &Path,
    caller_file: &str,
    manifest_name: &str,
    new_crate_path: &str,
) -> Option<(PathBuf, String, String)> {
    let (manifest_path, content) =
        resolve_caller_manifest_content(workspace_root, caller_file, manifest_name)?;
    let manifest_dir = manifest_path.parent()?;
    let target_abs = workspace_root.join(new_crate_path);
    let rel_to_target = compute_relative_path(manifest_dir, &target_abs);
    Some((manifest_path, content, rel_to_target))
}

pub fn insert_member_into_toml_array(
    content: &str,
    array_key: &str,
    member_entry: &str,
) -> Option<String> {
    let mut new_lines = Vec::new();
    let mut in_array = false;
    let mut added = false;

    for line in content.lines() {
        if line.trim().starts_with(array_key) {
            in_array = true;
            if line.contains(']') {
                let modified = line.replace(']', &format!(", {}]", member_entry));
                new_lines.push(modified);
                added = true;
                in_array = false;
                continue;
            }
        }

        if in_array && line.trim().starts_with(']') && !added {
            new_lines.push(format!("    {},", member_entry));
            added = true;
            in_array = false;
        }

        new_lines.push(line.to_string());
    }

    if added {
        Some(new_lines.join("\n") + "\n")
    } else {
        None
    }
}

pub fn create_manifest_update(
    manifest_path: &Path,
    workspace_root: &Path,
    dep_name: &str,
    diff: String,
    updated_content: String,
) -> ManifestUpdate {
    let rel_manifest = manifest_path
        .strip_prefix(workspace_root)
        .unwrap_or(manifest_path)
        .to_string_lossy()
        .replace('\\', "/");

    ManifestUpdate {
        manifest_path: rel_manifest,
        dependency_name: dep_name.to_string(),
        diff_preview: diff,
        updated_content,
    }
}

pub fn update_root_workspace_manifest(
    workspace_root: &Path,
    manifest_filename: &str,
    workspace_marker: &str,
    array_key: &str,
    new_crate_path: &str,
) -> Option<ManifestUpdate> {
    let root_manifest = workspace_root.join(manifest_filename);
    if !root_manifest.exists() {
        return None;
    }
    let content = std::fs::read_to_string(&root_manifest).ok()?;
    if !content.contains(workspace_marker) {
        return None;
    }

    let rel_path = new_crate_path.replace('\\', "/");
    let member_entry = format!("\"{}\"", rel_path);

    if content.contains(&member_entry) {
        return None;
    }

    // Check if the path is dynamically covered by any wildcard glob pattern (e.g. "crates/*", "packages/*", "libs/*")
    if let Some((prefix, _)) = rel_path.split_once('/') {
        let glob_double = format!("\"{}/*\"", prefix);
        let glob_single = format!("'{}/*'", prefix);
        if content.contains(&glob_double) || content.contains(&glob_single) {
            return None;
        }
    }

    let updated_content = insert_member_into_toml_array(&content, array_key, &member_entry)?;

    let section_name = array_key
        .trim_end_matches(" = [")
        .trim_end_matches(": [")
        .trim_matches('"');
    let diff = format!(
        "--- a/{}\n+++ b/{}\n@@ {} @@\n+    {}",
        manifest_filename, manifest_filename, section_name, member_entry
    );

    Some(create_manifest_update(
        &root_manifest,
        workspace_root,
        new_crate_path,
        diff,
        updated_content,
    ))
}
