#![forbid(unsafe_code)]

pub mod cargo;
pub mod common;
pub mod go_mod;
pub mod jvm_dotnet;
pub mod package_json;
pub mod pyproject;

#[cfg(test)]
mod tests;

use crate::extract::types::ManifestUpdate;
use std::path::Path;

fn push_unique(updates: &mut Vec<ManifestUpdate>, update: ManifestUpdate) {
    if !updates
        .iter()
        .any(|u| u.manifest_path == update.manifest_path)
    {
        updates.push(update);
    }
}

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
            if let Some(root_update) =
                cargo::update_cargo_workspace_root(workspace_root, new_crate_path)
            {
                updates.push(root_update);
            }
            for caller in caller_files {
                if let Some(caller_update) = cargo::update_caller_cargo_toml(
                    workspace_root,
                    caller,
                    new_crate_path,
                    new_crate_name,
                ) {
                    push_unique(&mut updates, caller_update);
                }
            }
        }
        "ts" | "tsx" | "js" | "jsx" => {
            if let Some(root_update) =
                package_json::update_package_json_root(workspace_root, new_crate_path)
            {
                updates.push(root_update);
            }
            for caller in caller_files {
                if let Some(caller_update) =
                    package_json::update_caller_package_json(workspace_root, caller, new_crate_name)
                {
                    push_unique(&mut updates, caller_update);
                }
            }
        }
        "py" => {
            if let Some(root_update) =
                pyproject::update_pyproject_toml_root(workspace_root, new_crate_path)
            {
                updates.push(root_update);
            }
            for caller in caller_files {
                if let Some(caller_update) = pyproject::update_caller_pyproject_toml(
                    workspace_root,
                    caller,
                    new_crate_path,
                    new_crate_name,
                ) {
                    push_unique(&mut updates, caller_update);
                }
            }
        }
        "go" => {
            if let Some(root_update) = go_mod::update_go_work_root(workspace_root, new_crate_path) {
                updates.push(root_update);
            }
            for caller in caller_files {
                if let Some(caller_update) = go_mod::update_caller_go_mod(
                    workspace_root,
                    caller,
                    new_crate_path,
                    new_crate_name,
                ) {
                    push_unique(&mut updates, caller_update);
                }
            }
        }
        "java" => {
            for caller in caller_files {
                if let Some(caller_update) =
                    jvm_dotnet::update_caller_pom_xml(workspace_root, caller, new_crate_name)
                {
                    push_unique(&mut updates, caller_update);
                } else if let Some(caller_update) =
                    jvm_dotnet::update_caller_build_gradle(workspace_root, caller, new_crate_name)
                {
                    push_unique(&mut updates, caller_update);
                }
            }
        }
        "cs" => {
            for caller in caller_files {
                if let Some(caller_update) = jvm_dotnet::update_caller_csproj(
                    workspace_root,
                    caller,
                    new_crate_path,
                    new_crate_name,
                ) {
                    push_unique(&mut updates, caller_update);
                }
            }
        }
        _ => {}
    }

    updates
}
