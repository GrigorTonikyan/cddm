#![forbid(unsafe_code)]

use super::types::ManifestUpdate;
use std::fs;
use std::path::{Path, PathBuf};

/// Updates workspace root and caller package manifests to link the newly extracted crate or package.
pub fn update_workspace_manifests(
    workspace_root: &Path,
    new_crate_path: &str,
    new_crate_name: &str,
    caller_files: &[String],
    ext: &str,
) -> Vec<ManifestUpdate> {
    let mut updates = Vec::new();
    let norm_ext = ext.to_lowercase();

    match norm_ext.as_str() {
        "rs" => {
            // Update root Cargo.toml if workspace
            if let Some(root_update) = update_cargo_workspace_root(workspace_root, new_crate_path) {
                updates.push(root_update);
            }
            // Update caller Cargo.toml files
            for caller in caller_files {
                if let Some(caller_update) =
                    update_caller_cargo_toml(workspace_root, caller, new_crate_path, new_crate_name)
                {
                    let already_has = updates
                        .iter()
                        .any(|u| u.manifest_path == caller_update.manifest_path);
                    if !already_has {
                        updates.push(caller_update);
                    }
                }
            }
        }
        "ts" | "tsx" | "js" | "jsx" => {
            // Update root package.json if workspace
            if let Some(root_update) = update_package_json_root(workspace_root, new_crate_path) {
                updates.push(root_update);
            }
            // Update caller package.json files
            for caller in caller_files {
                if let Some(caller_update) =
                    update_caller_package_json(workspace_root, caller, new_crate_name)
                {
                    let already_has = updates
                        .iter()
                        .any(|u| u.manifest_path == caller_update.manifest_path);
                    if !already_has {
                        updates.push(caller_update);
                    }
                }
            }
        }
        _ => {}
    }

    updates
}

fn update_cargo_workspace_root(
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

    // If already covered by wildcard or explicit entry
    if content.contains(&member_entry)
        || (rel_path.starts_with("crates/") && content.contains("\"crates/*\""))
        || (rel_path.starts_with("packages/") && content.contains("\"packages/*\""))
    {
        return None;
    }

    let mut new_lines = Vec::new();
    let mut in_members = false;
    let mut added = false;

    for line in content.lines() {
        if line.trim().starts_with("members = [") {
            in_members = true;
            if line.contains(']') {
                // Inline array: members = ["crates/*"] -> members = ["crates/*", "new_crate"]
                let modified = line.replace(']', &format!(", {}]", member_entry));
                new_lines.push(modified);
                added = true;
                in_members = false;
                continue;
            }
        }

        if in_members && line.trim().starts_with(']') && !added {
            new_lines.push(format!("    {},", member_entry));
            added = true;
            in_members = false;
        }

        new_lines.push(line.to_string());
    }

    if !added {
        return None;
    }

    let updated_content = new_lines.join("\n") + "\n";
    let diff = format!(
        "--- a/Cargo.toml\n+++ b/Cargo.toml\n@@ workspace.members @@\n+    {}",
        member_entry
    );

    Some(ManifestUpdate {
        manifest_path: "Cargo.toml".to_string(),
        dependency_name: new_crate_path.to_string(),
        diff_preview: diff,
        updated_content,
    })
}

fn update_caller_cargo_toml(
    workspace_root: &Path,
    caller_file: &str,
    new_crate_path: &str,
    new_crate_name: &str,
) -> Option<ManifestUpdate> {
    let caller_path = workspace_root.join(caller_file);
    let manifest_path = find_enclosing_manifest(&caller_path, "Cargo.toml", workspace_root)?;
    let content = fs::read_to_string(&manifest_path).ok()?;

    let crate_ident = new_crate_name.replace('-', "_");
    if content.contains(&format!("{} =", crate_ident))
        || content.contains(&format!("\"{}\" =", crate_ident))
        || content.contains(&format!("{} =", new_crate_name))
    {
        return None;
    }

    let manifest_dir = manifest_path.parent()?;
    let target_abs = workspace_root.join(new_crate_path);
    let rel_to_target = compute_relative_path(manifest_dir, &target_abs);

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

    let rel_manifest = manifest_path
        .strip_prefix(workspace_root)
        .unwrap_or(&manifest_path)
        .to_string_lossy()
        .replace('\\', "/");

    let updated_content = new_lines.join("\n") + "\n";
    let diff = format!(
        "--- a/{}\n+++ b/{}\n@@ [dependencies] @@\n+{}",
        rel_manifest, rel_manifest, dep_line
    );

    Some(ManifestUpdate {
        manifest_path: rel_manifest,
        dependency_name: crate_ident,
        diff_preview: diff,
        updated_content,
    })
}

fn update_package_json_root(workspace_root: &Path, new_crate_path: &str) -> Option<ManifestUpdate> {
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

    let mut new_lines = Vec::new();
    let mut in_workspaces = false;
    let mut added = false;

    for line in content.lines() {
        if line.trim().starts_with("\"workspaces\": [") {
            in_workspaces = true;
            if line.contains(']') {
                let modified = line.replace(']', &format!(", {}]", entry));
                new_lines.push(modified);
                added = true;
                in_workspaces = false;
                continue;
            }
        }

        if in_workspaces && line.trim().starts_with(']') && !added {
            new_lines.push(format!("    {},", entry));
            added = true;
            in_workspaces = false;
        }

        new_lines.push(line.to_string());
    }

    if !added {
        return None;
    }

    let updated_content = new_lines.join("\n") + "\n";
    let diff = format!(
        "--- a/package.json\n+++ b/package.json\n@@ workspaces @@\n+    {}",
        entry
    );

    Some(ManifestUpdate {
        manifest_path: "package.json".to_string(),
        dependency_name: new_crate_path.to_string(),
        diff_preview: diff,
        updated_content,
    })
}

fn update_caller_package_json(
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

    let rel_manifest = manifest_path
        .strip_prefix(workspace_root)
        .unwrap_or(&manifest_path)
        .to_string_lossy()
        .replace('\\', "/");

    let updated_content = new_lines.join("\n") + "\n";
    let diff = format!(
        "--- a/{}\n+++ b/{}\n@@ dependencies @@\n+{}",
        rel_manifest, rel_manifest, dep_entry
    );

    Some(ManifestUpdate {
        manifest_path: rel_manifest,
        dependency_name: new_crate_name.to_string(),
        diff_preview: diff,
        updated_content,
    })
}

fn find_enclosing_manifest(
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

fn compute_relative_path(from: &Path, to: &Path) -> String {
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
