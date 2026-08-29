#![forbid(unsafe_code)]

use super::common::{
    compute_relative_path, create_manifest_update, find_enclosing_manifest,
    resolve_caller_manifest_content,
};
use crate::extract::types::ManifestUpdate;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_MAVEN_GROUP_ID: &str = "com.cddm.shared";
const DEFAULT_MAVEN_VERSION: &str = "0.1.0";

/// Finds the nearest enclosing project file by file extension (e.g. `csproj`).
pub fn find_enclosing_project_file(
    start_path: &Path,
    extension: &str,
    workspace_root: &Path,
) -> Option<PathBuf> {
    let mut current = if start_path.is_file() {
        start_path.parent()?
    } else {
        start_path
    };

    while current >= workspace_root {
        if let Ok(entries) = fs::read_dir(current) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|s| s.to_str()) == Some(extension) {
                    return Some(p);
                }
            }
        }
        current = current.parent()?;
    }
    None
}

/// Injects a Maven `<dependency>` entry into a caller's `pom.xml`.
pub fn update_caller_pom_xml(
    workspace_root: &Path,
    caller_file: &str,
    new_crate_name: &str,
) -> Option<ManifestUpdate> {
    let (manifest_path, content) =
        resolve_caller_manifest_content(workspace_root, caller_file, "pom.xml")?;

    let artifact_id = new_crate_name.to_lowercase();
    if content.contains(&format!("<artifactId>{}</artifactId>", artifact_id)) {
        return None;
    }

    let dep_block = format!(
        "        <dependency>\n            <groupId>{}</groupId>\n            \
         <artifactId>{}</artifactId>\n            <version>{}</version>\n        </dependency>",
        DEFAULT_MAVEN_GROUP_ID, artifact_id, DEFAULT_MAVEN_VERSION
    );

    let mut new_lines = Vec::new();
    let mut in_dependencies = false;
    let mut inserted = false;

    for line in content.lines() {
        if line.contains("<dependencies>") {
            in_dependencies = true;
            new_lines.push(line.to_string());
            new_lines.push(dep_block.clone());
            inserted = true;
            continue;
        }
        if in_dependencies && line.contains("</dependencies>") {
            in_dependencies = false;
        }
        new_lines.push(line.to_string());
    }

    if !inserted {
        new_lines.push("\n  <dependencies>".to_string());
        new_lines.push(dep_block.clone());
        new_lines.push("  </dependencies>".to_string());
    }

    let updated_content = new_lines.join("\n") + "\n";
    let manifest_filename = manifest_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("pom.xml");
    let diff = format!(
        "--- a/{}\n+++ b/{}\n@@ <dependencies> @@\n+{}",
        manifest_filename, manifest_filename, dep_block
    );

    Some(create_manifest_update(
        &manifest_path,
        workspace_root,
        &artifact_id,
        diff,
        updated_content,
    ))
}

/// Injects a Gradle `implementation project(':name')` entry into a caller's `build.gradle` or `build.gradle.kts`.
pub fn update_caller_build_gradle(
    workspace_root: &Path,
    caller_file: &str,
    new_crate_name: &str,
) -> Option<ManifestUpdate> {
    let caller_path = workspace_root.join(caller_file);
    let manifest_path = find_enclosing_manifest(&caller_path, "build.gradle", workspace_root)
        .or_else(|| find_enclosing_manifest(&caller_path, "build.gradle.kts", workspace_root))?;
    let content = fs::read_to_string(&manifest_path).ok()?;

    let project_name = new_crate_name.to_lowercase();
    if content.contains(&format!("project(':{}')", project_name))
        || content.contains(&format!("project(\":{}\")", project_name))
    {
        return None;
    }

    let dep_line = format!("    implementation project(':{}')", project_name);
    let mut new_lines = Vec::new();
    let mut inserted = false;

    for line in content.lines() {
        new_lines.push(line.to_string());
        if line.trim().starts_with("dependencies {") {
            new_lines.push(dep_line.clone());
            inserted = true;
        }
    }

    if !inserted {
        new_lines.push("\ndependencies {".to_string());
        new_lines.push(dep_line.clone());
        new_lines.push("}".to_string());
    }

    let updated_content = new_lines.join("\n") + "\n";
    let manifest_filename = manifest_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("build.gradle");
    let diff = format!(
        "--- a/{}\n+++ b/{}\n@@ dependencies @@\n+{}",
        manifest_filename, manifest_filename, dep_line
    );

    Some(create_manifest_update(
        &manifest_path,
        workspace_root,
        &project_name,
        diff,
        updated_content,
    ))
}

/// Injects a .NET `<ProjectReference>` entry into a caller's `.csproj`.
pub fn update_caller_csproj(
    workspace_root: &Path,
    caller_file: &str,
    new_crate_path: &str,
    new_crate_name: &str,
) -> Option<ManifestUpdate> {
    let caller_path = workspace_root.join(caller_file);
    let manifest_path = find_enclosing_project_file(&caller_path, "csproj", workspace_root)?;
    let content = fs::read_to_string(&manifest_path).ok()?;

    let target_proj = workspace_root
        .join(new_crate_path)
        .join(format!("{}.csproj", new_crate_name));

    let manifest_dir = manifest_path.parent()?;
    let rel_to_target = compute_relative_path(manifest_dir, &target_proj);

    if content.contains(&rel_to_target) || content.contains(new_crate_name) {
        return None;
    }

    let proj_ref_line = format!(
        "  <ItemGroup>\n    <ProjectReference Include=\"{}\" />\n  </ItemGroup>",
        rel_to_target
    );

    let mut new_lines = Vec::new();
    let mut inserted = false;

    for line in content.lines() {
        if line.contains("</Project>") && !inserted {
            new_lines.push(proj_ref_line.clone());
            inserted = true;
        }
        new_lines.push(line.to_string());
    }

    if !inserted {
        new_lines.push(proj_ref_line.clone());
    }

    let updated_content = new_lines.join("\n") + "\n";
    let manifest_filename = manifest_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("project.csproj");
    let diff = format!(
        "--- a/{}\n+++ b/{}\n@@ ProjectReference @@\n+    <ProjectReference Include=\"{}\" />",
        manifest_filename, manifest_filename, rel_to_target
    );

    Some(create_manifest_update(
        &manifest_path,
        workspace_root,
        new_crate_name,
        diff,
        updated_content,
    ))
}
