#![forbid(unsafe_code)]

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
        let root_cddmignore = Path::new(&config.directory).join(".cddmignore");
        if root_cddmignore.exists() {
            SuppressionEngine::from_file(
                &root_cddmignore,
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
        let root_rules = Path::new(&config.directory).join(DEFAULT_RULES_FILE);
        if root_rules.exists() {
            PolicyEngine::from_file(&root_rules).unwrap_or_else(|_| PolicyEngine::empty())
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
    let walker = WalkBuilder::new(&config.directory);
    let mut files_to_process = Vec::new();

    for result in walker.build() {
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
