#![forbid(unsafe_code)]

use crate::cache::find_workspace_root;
use crate::grammar::get_grammar_for_path;
use crate::policy::PolicyEngine;
use crate::suppression::SuppressionEngine;
use crate::types::{DEFAULT_RULES_FILE, ScanConfig};
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn init_suppression_engine(config: &ScanConfig) -> SuppressionEngine {
    if let Some(path_str) = &config.cddmignore_path {
        SuppressionEngine::from_file(
            Path::new(path_str),
            config.ignore_tests,
            config.ignore_mocks,
            config.ignore_generated,
        )
        .unwrap_or_else(|_| SuppressionEngine::default_engine())
    } else {
        let dir_path = Path::new(&config.directory);
        let root_cddmignore = dir_path.join(".cddmignore");
        let ws_cddmignore = find_workspace_root(dir_path).join(".cddmignore");
        let cddmignore_file = if root_cddmignore.exists() {
            Some(root_cddmignore)
        } else if ws_cddmignore.exists() {
            Some(ws_cddmignore)
        } else {
            None
        };

        if let Some(path) = cddmignore_file {
            SuppressionEngine::from_file(
                &path,
                config.ignore_tests,
                config.ignore_mocks,
                config.ignore_generated,
            )
            .unwrap_or_else(|_| SuppressionEngine::default_engine())
        } else {
            SuppressionEngine::new(crate::types::SuppressionConfig {
                rules: Vec::new(),
                ignore_tests: config.ignore_tests,
                ignore_mocks: config.ignore_mocks,
                ignore_generated: config.ignore_generated,
                raw_cddmignore: None,
            })
            .unwrap_or_else(|_| SuppressionEngine::default_engine())
        }
    }
}

pub fn init_policy_engine(config: &ScanConfig) -> PolicyEngine {
    if let Some(path_str) = &config.rules_path {
        PolicyEngine::from_file(Path::new(path_str)).unwrap_or_else(|_| PolicyEngine::empty())
    } else {
        let dir_path = Path::new(&config.directory);
        let root_rules = dir_path.join(DEFAULT_RULES_FILE);
        let ws_rules = find_workspace_root(dir_path).join(DEFAULT_RULES_FILE);
        let rules_file = if root_rules.exists() {
            Some(root_rules)
        } else if ws_rules.exists() {
            Some(ws_rules)
        } else {
            None
        };

        if let Some(path) = rules_file {
            PolicyEngine::from_file(&path).unwrap_or_else(|_| PolicyEngine::empty())
        } else {
            PolicyEngine::empty()
        }
    }
}

pub fn discover_candidate_files(
    config: &ScanConfig,
    suppression_engine: &SuppressionEngine,
    cancel_flag: &Arc<AtomicBool>,
) -> Result<Vec<PathBuf>, String> {
    let respect_ignore = !config.include_ignored;
    let mut builder = WalkBuilder::new(&config.directory);

    builder
        .hidden(!config.include_ignored)
        .parents(true)
        .git_ignore(respect_ignore)
        .git_global(respect_ignore)
        .git_exclude(respect_ignore)
        .ignore(respect_ignore);

    if respect_ignore {
        builder.add_custom_ignore_filename(".gitignore");

        let dir_path = Path::new(&config.directory);
        let root_ignore = dir_path.join(".gitignore");
        let ws_root = find_workspace_root(dir_path);
        let ws_ignore = ws_root.join(".gitignore");

        if root_ignore.exists() {
            let _ = builder.add_ignore(root_ignore);
        } else if ws_ignore.exists() {
            let _ = builder.add_ignore(ws_ignore);
        }
    }

    let mut files_to_process = Vec::new();

    for result in builder.build() {
        if cancel_flag.load(Ordering::Relaxed) {
            return Err("Scan cancelled".to_string());
        }
        if let Ok(entry) = result
            && entry.path().is_file()
            && let Some(grammar) = get_grammar_for_path(entry.path())
            && (config.languages.is_empty() || config.languages.contains(&grammar.name.to_string()))
        {
            let path_str = entry.path().to_string_lossy().to_string();
            let mut ignored = false;
            for pat in &config.ignore_patterns {
                if path_str.contains(pat) {
                    ignored = true;
                    break;
                }
            }
            if !ignored && !suppression_engine.is_path_ignored(entry.path(), None) {
                files_to_process.push(entry.path().to_path_buf());
            }
        }
    }

    Ok(files_to_process)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_native_gitignore_traversal_and_override() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        // Create .gitignore
        fs::write(root.join(".gitignore"), "ignored.rs\nbuild_output/\n").unwrap();

        // Create files
        fs::write(root.join("main.rs"), "fn main() { println!(\"hello\"); }").unwrap();
        fs::write(
            root.join("ignored.rs"),
            "fn ignored() { println!(\"ignore\"); }",
        )
        .unwrap();

        let build_dir = root.join("build_output");
        fs::create_dir_all(&build_dir).unwrap();
        fs::write(build_dir.join("out.rs"), "fn out() { println!(\"out\"); }").unwrap();

        let cancel_flag = Arc::new(AtomicBool::new(false));
        let suppression = SuppressionEngine::default_engine();

        // 1. Default: respect .gitignore
        let default_config = ScanConfig {
            directory: root.to_string_lossy().to_string(),
            include_ignored: false,
            ignore_patterns: vec![],
            ..Default::default()
        };
        let discovered =
            discover_candidate_files(&default_config, &suppression, &cancel_flag).unwrap();
        let filenames: Vec<String> = discovered
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(filenames.contains(&"main.rs".to_string()));
        assert!(!filenames.contains(&"ignored.rs".to_string()));
        assert!(!filenames.contains(&"out.rs".to_string()));

        // 2. Override: include_ignored = true
        let include_config = ScanConfig {
            directory: root.to_string_lossy().to_string(),
            include_ignored: true,
            ignore_patterns: vec![],
            ..Default::default()
        };
        let discovered_all =
            discover_candidate_files(&include_config, &suppression, &cancel_flag).unwrap();
        let all_filenames: Vec<String> = discovered_all
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(all_filenames.contains(&"main.rs".to_string()));
        assert!(all_filenames.contains(&"ignored.rs".to_string()));
        assert!(all_filenames.contains(&"out.rs".to_string()));
    }

    #[test]
    fn test_nested_gitignore_traversal() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let sub_dir = root.join("packages").join("pkg_a");
        fs::create_dir_all(&sub_dir).unwrap();

        fs::write(root.join("root.rs"), "fn root() {}").unwrap();
        fs::write(sub_dir.join("kept.rs"), "fn kept() {}").unwrap();
        fs::write(sub_dir.join("nested_ignored.rs"), "fn nested() {}").unwrap();
        fs::write(sub_dir.join(".gitignore"), "nested_ignored.rs\n").unwrap();

        let cancel_flag = Arc::new(AtomicBool::new(false));
        let suppression = SuppressionEngine::default_engine();

        let config = ScanConfig {
            directory: root.to_string_lossy().to_string(),
            include_ignored: false,
            ignore_patterns: vec![],
            ..Default::default()
        };
        let discovered = discover_candidate_files(&config, &suppression, &cancel_flag).unwrap();
        let filenames: Vec<String> = discovered
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(filenames.contains(&"root.rs".to_string()));
        assert!(filenames.contains(&"kept.rs".to_string()));
        assert!(!filenames.contains(&"nested_ignored.rs".to_string()));
    }
}
