#![forbid(unsafe_code)]

pub mod executor;
pub mod generator;
pub mod manifest;
pub mod rewriter;
pub mod test_generator;
pub mod types;

pub use executor::apply_extraction_to_workspace;
pub use generator::generate_extracted_target_files;
pub use manifest::update_workspace_manifests;
pub use rewriter::rewrite_caller_files;
pub use test_generator::generate_unit_test_files;
pub use types::*;

use crate::refactor::consensus::analyze_cluster_snippets_refactoring;
use crate::types::InferredParameter;
use std::fs;
use std::path::Path;

/// Core coordinator to generate an automated shared crate or module extraction.
pub fn generate_shared_extraction(
    workspace_root: &Path,
    request: &ExtractRequest,
) -> Result<ExtractResult, String> {
    if request.occurrences.is_empty() {
        return Err("No occurrence locations provided for shared extraction".to_string());
    }

    let first_occ = &request.occurrences[0];
    let ext = Path::new(&first_occ.file)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("rs");

    // 1. Read occurrence snippets
    let mut site_snippets = Vec::new();
    for occ in &request.occurrences {
        let abs_path = workspace_root.join(&occ.file);
        if !abs_path.exists() {
            return Err(format!("Occurrence file '{}' does not exist", occ.file));
        }
        let content = fs::read_to_string(&abs_path)
            .map_err(|e| format!("Failed to read occurrence file '{}': {}", occ.file, e))?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let start_idx = occ.start_line.saturating_sub(1);
        let end_idx = occ.end_line.min(lines.len());
        if start_idx < end_idx {
            site_snippets.push((occ, lines[start_idx..end_idx].to_vec()));
        }
    }

    if site_snippets.is_empty() {
        return Err("No valid code snippets extracted from occurrences".to_string());
    }

    // 2. Compute consensus invariant code
    let occ_pairs: Vec<(&crate::types::CloneLocation, &[String])> = site_snippets
        .iter()
        .map(|(occ, snip)| (*occ, snip.as_slice()))
        .collect();
    let cluster_refactor = analyze_cluster_snippets_refactoring("extract-preview", &occ_pairs);
    let common_body_lines = cluster_refactor.common_body_lines;

    // 3. Infer parameters
    let mut inferred_parameters = Vec::new();
    let mut param_index = 0;
    let mut param_diff_groups: Vec<Vec<String>> = Vec::new();

    for site in &cluster_refactor.sites {
        for (i, diff) in site.parameter_differences.iter().enumerate() {
            if i >= param_diff_groups.len() {
                param_diff_groups.push(Vec::new());
            }
            if !diff.fragment_a_code.is_empty() {
                param_diff_groups[i].push(diff.fragment_a_code.clone());
            }
        }
    }

    for (i, vals) in param_diff_groups.iter().enumerate() {
        let name = if let Some(custom_names) = &request.custom_parameter_names
            && i < custom_names.len()
        {
            custom_names[i].clone()
        } else {
            param_index += 1;
            format!("param_{}", param_index)
        };
        let inferred_type = crate::ast::type_infer::infer_parameter_type(ext, vals);
        inferred_parameters.push(InferredParameter {
            name,
            inferred_type,
            original_values: vals.clone(),
        });
    }

    let default_fn_name = "extracted_shared_helper".to_string();
    let fn_name = request
        .custom_function_name
        .as_ref()
        .unwrap_or(&default_fn_name);

    // 4. Determine target kind if Auto
    let target_kind = match request.target_kind {
        ExtractTargetKind::Auto => {
            if request.target_path.starts_with("crates/")
                || request.target_path.starts_with("packages/")
            {
                ExtractTargetKind::NewCrate
            } else {
                ExtractTargetKind::NewModule
            }
        }
        kind => kind,
    };

    // 5. Generate target files
    let (helper_sig, generated_files) = generate_extracted_target_files(
        &request.target_path,
        target_kind,
        fn_name,
        &inferred_parameters,
        &common_body_lines,
        ext,
    );

    // 6. Update manifests if NewCrate
    let caller_files: Vec<String> = request.occurrences.iter().map(|o| o.file.clone()).collect();
    let target_crate_name = Path::new(&request.target_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("shared_utils");

    let manifest_updates = if target_kind == ExtractTargetKind::NewCrate {
        update_workspace_manifests(
            workspace_root,
            &request.target_path,
            target_crate_name,
            &caller_files,
            ext,
        )
    } else {
        Vec::new()
    };

    // 7. Rewrite caller occurrence files
    let (caller_rewrites, syntax_valid) = rewrite_caller_files(
        workspace_root,
        &request.occurrences,
        fn_name,
        &request.target_path,
        target_kind,
        ext,
    )?;

    // 8. Synthesize unit tests if requested
    let test_files = if request.generate_tests {
        generate_unit_test_files(
            &request.target_path,
            target_kind,
            fn_name,
            &inferred_parameters,
            ext,
        )
    } else {
        Vec::new()
    };

    let total_lines_saved = cluster_refactor.total_lines_saved;
    let message = format!(
        "Successfully planned shared extraction of '{}' to '{}' ({} files generated, {} tests \
         synthesized, {} manifests updated, {} callers rewritten, {} lines saved)",
        fn_name,
        request.target_path,
        generated_files.len(),
        test_files.len(),
        manifest_updates.len(),
        caller_rewrites.len(),
        total_lines_saved
    );

    Ok(ExtractResult {
        function_name: fn_name.to_string(),
        target_path: request.target_path.clone(),
        target_kind,
        helper_signature: helper_sig,
        inferred_parameters,
        generated_files,
        test_files,
        manifest_updates,
        caller_rewrites,
        total_lines_saved,
        syntax_valid,
        message,
    })
}

/// Executes and writes the shared extraction directly to the workspace filesystem.
pub fn apply_shared_extraction(
    workspace_root: &Path,
    request: &ExtractRequest,
) -> Result<ExtractResult, String> {
    let mut plan = generate_shared_extraction(workspace_root, request)?;
    if !request.dry_run {
        let written = apply_extraction_to_workspace(workspace_root, &plan, false)?;
        plan.message = format!(
            "Successfully applied shared extraction to workspace ({} file changes committed to \
             disk)",
            written
        );
    }
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CloneLocation;
    use tempfile::tempdir;

    #[test]
    fn test_generate_shared_extraction_new_crate_rust() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::write(
            root.join("Cargo.toml"),
            "[workspace]\nmembers = [\n    \"crates/app_a\",\n    \"crates/app_b\",\n]\n",
        )
        .unwrap();

        let app_a = root.join("crates/app_a");
        fs::create_dir_all(app_a.join("src")).unwrap();
        fs::write(
            app_a.join("Cargo.toml"),
            "[package]\nname = \"app_a\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
        )
        .unwrap();
        fs::write(
            app_a.join("src/main.rs"),
            "fn main() {\n    let val = 42;\n    println!(\"{}\", val * 2);\n}\n",
        )
        .unwrap();

        let app_b = root.join("crates/app_b");
        fs::create_dir_all(app_b.join("src")).unwrap();
        fs::write(
            app_b.join("Cargo.toml"),
            "[package]\nname = \"app_b\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
        )
        .unwrap();
        fs::write(
            app_b.join("src/main.rs"),
            "fn run() {\n    let val = 42;\n    println!(\"{}\", val * 2);\n}\n",
        )
        .unwrap();

        let req = ExtractRequest {
            occurrences: vec![
                CloneLocation {
                    file: "crates/app_a/src/main.rs".to_string(),
                    start_line: 2,
                    end_line: 3,
                    author: None,
                },
                CloneLocation {
                    file: "crates/app_b/src/main.rs".to_string(),
                    start_line: 2,
                    end_line: 3,
                    author: None,
                },
            ],
            target_path: "crates/shared_math".to_string(),
            custom_function_name: Some("compute_double".to_string()),
            target_kind: ExtractTargetKind::NewCrate,
            custom_parameter_names: None,
            generate_tests: false,
            dry_run: false,
        };

        let res = apply_shared_extraction(root, &req);
        assert!(res.is_ok(), "{:?}", res.err());
        let result = res.unwrap();

        assert_eq!(result.function_name, "compute_double");
        assert_eq!(result.generated_files.len(), 2);
        assert!(root.join("crates/shared_math/Cargo.toml").exists());
        assert!(root.join("crates/shared_math/src/lib.rs").exists());

        let lib_content = fs::read_to_string(root.join("crates/shared_math/src/lib.rs")).unwrap();
        assert!(lib_content.contains("pub fn compute_double()"));

        let app_a_cargo = fs::read_to_string(app_a.join("Cargo.toml")).unwrap();
        assert!(app_a_cargo.contains("shared_math = { path = \"../shared_math\" }"));
    }

    #[test]
    fn test_generate_shared_extraction_module_typescript() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::create_dir_all(root.join("src/components")).unwrap();
        fs::write(
            root.join("src/components/A.ts"),
            "export function doA() {\n    const x = 100;\n    return x * 2;\n}\n",
        )
        .unwrap();
        fs::write(
            root.join("src/components/B.ts"),
            "export function doB() {\n    const x = 100;\n    return x * 2;\n}\n",
        )
        .unwrap();

        let req = ExtractRequest {
            occurrences: vec![
                CloneLocation {
                    file: "src/components/A.ts".to_string(),
                    start_line: 2,
                    end_line: 3,
                    author: None,
                },
                CloneLocation {
                    file: "src/components/B.ts".to_string(),
                    start_line: 2,
                    end_line: 3,
                    author: None,
                },
            ],
            target_path: "src/components/common_utils".to_string(),
            custom_function_name: Some("doubleValue".to_string()),
            target_kind: ExtractTargetKind::NewModule,
            custom_parameter_names: None,
            generate_tests: false,
            dry_run: false,
        };

        let res = apply_shared_extraction(root, &req);
        assert!(res.is_ok(), "{:?}", res.err());
        let result = res.unwrap();

        assert_eq!(result.function_name, "doubleValue");
        assert!(root.join("src/components/common_utils.ts").exists());

        let mod_content = fs::read_to_string(root.join("src/components/common_utils.ts")).unwrap();
        assert!(mod_content.contains("export function doubleValue()"));
    }

    #[test]
    fn test_generate_shared_extraction_python_package() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let svc_a = root.join("services/api");
        fs::create_dir_all(&svc_a).unwrap();
        fs::write(
            svc_a.join("pyproject.toml"),
            "[project]\nname = \"api\"\ndependencies = []\n",
        )
        .unwrap();
        fs::write(
            svc_a.join("views.py"),
            "def handle():\n    val = 10\n    return val * 5\n",
        )
        .unwrap();

        let svc_b = root.join("services/worker");
        fs::create_dir_all(&svc_b).unwrap();
        fs::write(
            svc_b.join("pyproject.toml"),
            "[project]\nname = \"worker\"\ndependencies = []\n",
        )
        .unwrap();
        fs::write(
            svc_b.join("tasks.py"),
            "def run():\n    val = 10\n    return val * 5\n",
        )
        .unwrap();

        let req = ExtractRequest {
            occurrences: vec![
                CloneLocation {
                    file: "services/api/views.py".to_string(),
                    start_line: 2,
                    end_line: 3,
                    author: None,
                },
                CloneLocation {
                    file: "services/worker/tasks.py".to_string(),
                    start_line: 2,
                    end_line: 3,
                    author: None,
                },
            ],
            target_path: "packages/math_utils".to_string(),
            custom_function_name: Some("calc_multiplier".to_string()),
            target_kind: ExtractTargetKind::NewCrate,
            custom_parameter_names: None,
            generate_tests: false,
            dry_run: false,
        };

        let res = apply_shared_extraction(root, &req);
        assert!(res.is_ok(), "{:?}", res.err());
        let result = res.unwrap();

        assert_eq!(result.function_name, "calc_multiplier");
        assert!(root.join("packages/math_utils/pyproject.toml").exists());
        assert!(root.join("packages/math_utils/__init__.py").exists());
    }

    #[test]
    fn test_generate_shared_extraction_with_unit_tests() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::write(
            root.join("Cargo.toml"),
            "[workspace]\nmembers = [\"crates/app_a\", \"crates/app_b\"]\n",
        )
        .unwrap();

        let app_a = root.join("crates/app_a/src");
        fs::create_dir_all(&app_a).unwrap();
        fs::write(
            root.join("crates/app_a/Cargo.toml"),
            "[package]\nname = \"app_a\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        fs::write(
            app_a.join("main.rs"),
            "fn main() {\n    let val = 10;\n    println!(\"{}\", val);\n}\n",
        )
        .unwrap();

        let req = ExtractRequest {
            occurrences: vec![
                CloneLocation {
                    file: "crates/app_a/src/main.rs".to_string(),
                    start_line: 2,
                    end_line: 3,
                    author: None,
                },
                CloneLocation {
                    file: "crates/app_a/src/main.rs".to_string(),
                    start_line: 2,
                    end_line: 3,
                    author: None,
                },
            ],
            target_path: "crates/shared_tools".to_string(),
            custom_function_name: Some("process_val".to_string()),
            target_kind: ExtractTargetKind::NewCrate,
            custom_parameter_names: None,
            generate_tests: true,
            dry_run: false,
        };

        let res = apply_shared_extraction(root, &req);
        assert!(res.is_ok(), "{:?}", res.err());
        let result = res.unwrap();

        assert_eq!(result.test_files.len(), 1);
        assert!(
            root.join("crates/shared_tools/tests/process_val_test.rs")
                .exists()
        );
    }
}
