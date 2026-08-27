#![forbid(unsafe_code)]

use super::common::{compute_relative_path, create_manifest_update, find_enclosing_manifest};
use crate::extract::types::ManifestUpdate;
use std::fs;
use std::path::Path;

pub fn update_caller_pom_xml(
    workspace_root: &Path,
    caller_file: &str,
    new_crate_name: &str,
) -> Option<ManifestUpdate> {
    let caller_path = workspace_root.join(caller_file);
    let manifest_path = find_enclosing_manifest(&caller_path, "pom.xml", workspace_root)?;
    let content = fs::read_to_string(&manifest_path).ok()?;

    let artifact_id = new_crate_name.to_lowercase();
    if content.contains(&format!("<artifactId>{}</artifactId>", artifact_id)) {
        return None;
    }

    let dep_block = format!(
        "        <dependency>\n            <groupId>com.cddm.shared</groupId>\n            \
         <artifactId>{}</artifactId>\n            <version>0.1.0</version>\n        </dependency>",
        artifact_id
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
    let diff = format!(
        "--- a/pom.xml\n+++ b/pom.xml\n@@ <dependencies> @@\n+{}",
        dep_block
    );

    Some(create_manifest_update(
        &manifest_path,
        workspace_root,
        &artifact_id,
        diff,
        updated_content,
    ))
}

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
    let diff = format!(
        "--- a/build.gradle\n+++ b/build.gradle\n@@ dependencies @@\n+{}",
        dep_line
    );

    Some(create_manifest_update(
        &manifest_path,
        workspace_root,
        &project_name,
        diff,
        updated_content,
    ))
}

pub fn update_caller_csproj(
    workspace_root: &Path,
    caller_file: &str,
    new_crate_path: &str,
    new_crate_name: &str,
) -> Option<ManifestUpdate> {
    let caller_path = workspace_root.join(caller_file);
    let caller_dir = if caller_path.is_file() {
        caller_path.parent()?
    } else {
        &caller_path
    };

    let mut current = caller_dir;
    let mut csproj_path = None;
    while current >= workspace_root {
        if let Ok(entries) = fs::read_dir(current) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|s| s.to_str()) == Some("csproj") {
                    csproj_path = Some(p);
                    break;
                }
            }
        }
        if csproj_path.is_some() {
            break;
        }
        current = current.parent()?;
    }

    let manifest_path = csproj_path?;
    let content = fs::read_to_string(&manifest_path).ok()?;

    let target_abs = workspace_root.join(new_crate_path);
    let target_proj = format!("{}/{}.csproj", target_abs.display(), new_crate_name);
    let target_proj_path = Path::new(&target_proj);

    let manifest_dir = manifest_path.parent()?;
    let rel_to_target = compute_relative_path(manifest_dir, target_proj_path);

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
    let diff = format!(
        "--- a/project.csproj\n+++ b/project.csproj\n@@ ProjectReference @@\n+    \
         <ProjectReference Include=\"{}\" />",
        rel_to_target
    );

    Some(create_manifest_update(
        &manifest_path,
        workspace_root,
        new_crate_name,
        diff,
        updated_content,
    ))
}
