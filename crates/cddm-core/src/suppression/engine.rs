#![forbid(unsafe_code)]

use super::directives::{is_generated_header, is_mock_path, is_test_path, parse_inline_directives};
use super::parser::{format_rules_to_raw, parse_cddmignore_content};
use crate::types::{CloneType, SuppressionConfig, SuppressionDirective};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashMap;
use std::path::Path;

/// Engine managing `.cddmignore` rule parsing, inline code directives, and AST-aware suppression.
#[derive(Clone, Debug)]
pub struct SuppressionEngine {
    /// Active suppression configuration
    config: SuppressionConfig,
    /// Compiled gitignore matcher for ignored paths
    gitignore: Gitignore,
    /// Threshold overrides by pattern: matcher -> min_tokens
    threshold_overrides: Vec<(Gitignore, usize)>,
    /// Clone type exclusions by pattern: matcher -> Vec<CloneType>
    type_filters: Vec<(Gitignore, Vec<CloneType>)>,
    /// Cache of parsed inline directives per file: path -> Vec<SuppressionDirective>
    inline_directives_cache: HashMap<String, Vec<SuppressionDirective>>,
}

impl SuppressionEngine {
    /// Constructs a new SuppressionEngine with the given configuration.
    pub fn new(mut config: SuppressionConfig) -> Result<Self, String> {
        let mut builder = GitignoreBuilder::new("");
        let mut threshold_overrides = Vec::new();
        let mut type_filters = Vec::new();

        for rule in &config.rules {
            if rule.rule_type == "ignore" {
                let _ = builder.add_line(None, &rule.pattern);
            } else if rule.rule_type == "threshold" {
                if let Some(tokens) = rule.min_tokens {
                    let mut b = GitignoreBuilder::new("");
                    if b.add_line(None, &rule.pattern).is_ok()
                        && let Ok(gi) = b.build()
                    {
                        threshold_overrides.push((gi, tokens));
                    }
                }
            } else if rule.rule_type == "type_filter" && !rule.ignored_clone_types.is_empty() {
                let mut b = GitignoreBuilder::new("");
                if b.add_line(None, &rule.pattern).is_ok()
                    && let Ok(gi) = b.build()
                {
                    type_filters.push((gi, rule.ignored_clone_types.clone()));
                }
            }
        }

        let gitignore = builder
            .build()
            .map_err(|e| format!("Failed to compile .cddmignore pattern set: {e}"))?;

        // Normalize raw content
        if config.raw_cddmignore.is_none() {
            config.raw_cddmignore = Some(format_rules_to_raw(&config.rules));
        }

        Ok(Self {
            config,
            gitignore,
            threshold_overrides,
            type_filters,
            inline_directives_cache: HashMap::new(),
        })
    }

    /// Creates a default suppression engine with standard defaults.
    pub fn default_engine() -> Self {
        Self::with_options(false, false, true)
    }

    /// Creates a suppression engine with specified category flags.
    pub fn with_options(ignore_tests: bool, ignore_mocks: bool, ignore_generated: bool) -> Self {
        Self::new(SuppressionConfig {
            rules: Vec::new(),
            ignore_tests,
            ignore_mocks,
            ignore_generated,
            raw_cddmignore: None,
        })
        .unwrap_or_else(|_| Self {
            config: SuppressionConfig {
                rules: Vec::new(),
                ignore_tests,
                ignore_mocks,
                ignore_generated,
                raw_cddmignore: None,
            },
            gitignore: GitignoreBuilder::new("").build().unwrap(),
            threshold_overrides: Vec::new(),
            type_filters: Vec::new(),
            inline_directives_cache: HashMap::new(),
        })
    }

    /// Loads suppression configuration from a `.cddmignore` file on disk.
    pub fn from_file(
        file_path: &Path,
        ignore_tests: bool,
        ignore_mocks: bool,
        ignore_generated: bool,
    ) -> Result<Self, String> {
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            format!(
                "Failed to read .cddmignore at '{}': {e}",
                file_path.display()
            )
        })?;
        Self::from_str(&content, ignore_tests, ignore_mocks, ignore_generated)
    }

    /// Parses suppression configuration from a string.
    pub fn from_str(
        content: &str,
        ignore_tests: bool,
        ignore_mocks: bool,
        ignore_generated: bool,
    ) -> Result<Self, String> {
        let rules = parse_cddmignore_content(content);
        Self::new(SuppressionConfig {
            rules,
            ignore_tests,
            ignore_mocks,
            ignore_generated,
            raw_cddmignore: Some(content.to_string()),
        })
    }

    /// Checks if a file path is ignored by path rules, test/mock filters, or generated headers.
    pub fn is_path_ignored(&self, path: &Path, content: Option<&str>) -> bool {
        // 1. Check gitignore match
        if self.gitignore.matched(path, false).is_ignore() {
            return true;
        }

        let norm_str = path.to_string_lossy().replace('\\', "/");
        let clean_str = norm_str.trim_start_matches("./");
        if self
            .gitignore
            .matched(Path::new(clean_str), false)
            .is_ignore()
            || self
                .gitignore
                .matched(Path::new(&norm_str), false)
                .is_ignore()
        {
            return true;
        }

        if let Ok(cur) = std::env::current_dir() {
            let cur_norm = cur.to_string_lossy().replace('\\', "/");
            let cur_norm_lower = cur_norm.to_lowercase();
            let norm_lower = norm_str.to_lowercase();
            if norm_lower.starts_with(&cur_norm_lower) {
                let rel = norm_str[cur_norm.len()..].trim_start_matches('/');
                if self.gitignore.matched(Path::new(rel), false).is_ignore() {
                    return true;
                }
            } else if let Ok(rel) = path.strip_prefix(&cur) {
                let rel_str = rel.to_string_lossy().replace('\\', "/");
                let clean_rel = rel_str.trim_start_matches("./");
                if self
                    .gitignore
                    .matched(Path::new(clean_rel), false)
                    .is_ignore()
                    || self
                        .gitignore
                        .matched(Path::new(&rel_str), false)
                        .is_ignore()
                {
                    return true;
                }
            }
        }

        // 2. Check test files filter
        if self.config.ignore_tests && is_test_path(path) {
            return true;
        }

        // 3. Check mock files filter
        if self.config.ignore_mocks && is_mock_path(path) {
            return true;
        }

        // 4. Check auto-generated header
        if self.config.ignore_generated
            && let Some(text) = content
            && is_generated_header(text)
        {
            return true;
        }

        false
    }

    /// Determines if a file is a test file based on conventions.
    pub fn is_test_path(path: &Path) -> bool {
        is_test_path(path)
    }

    /// Determines if a file is a mock or test fixture.
    pub fn is_mock_path(path: &Path) -> bool {
        is_mock_path(path)
    }

    /// Inspects the top lines of a file to detect common automated code generation headers.
    pub fn is_generated_header(content: &str) -> bool {
        is_generated_header(content)
    }

    /// Parses raw `.cddmignore` text into a list of `SuppressionRule` records.
    pub fn parse_cddmignore_content(content: &str) -> Vec<crate::types::SuppressionRule> {
        parse_cddmignore_content(content)
    }

    /// Generates a turnkey standard `.cddmignore` template file content.
    pub fn generate_default_cddmignore() -> String {
        super::parser::generate_default_cddmignore()
    }

    /// Parses inline suppression directives from source code.
    pub fn parse_inline_directives(file_path: &str, content: &str) -> Vec<SuppressionDirective> {
        parse_inline_directives(file_path, content)
    }

    /// Caches inline directives for a file.
    pub fn register_file_directives(&mut self, file_path: &str, content: &str) {
        let directives = Self::parse_inline_directives(file_path, content);
        if !directives.is_empty() {
            self.inline_directives_cache
                .insert(file_path.to_string(), directives);
        }
    }

    /// Checks if a specific line span in a file is suppressed.
    pub fn is_span_suppressed(&self, file_path: &str, start_line: usize, end_line: usize) -> bool {
        if let Some(directives) = self.inline_directives_cache.get(file_path) {
            for d in directives {
                if start_line <= d.end_line && end_line >= d.start_line {
                    return true;
                }
            }
        }
        false
    }

    fn matches_gitignore_pattern(gi: &ignore::gitignore::Gitignore, path: &Path) -> bool {
        let norm_str = path.to_string_lossy().replace('\\', "/");
        let clean_str = norm_str.trim_start_matches("./");
        gi.matched(path, false).is_ignore()
            || gi.matched(Path::new(&norm_str), false).is_ignore()
            || gi.matched(Path::new(clean_str), false).is_ignore()
    }

    /// Gets the effective minimum token threshold for a file path, taking overrides into account.
    pub fn get_effective_min_tokens(&self, path: &Path, default_min: usize) -> usize {
        for (gi, min) in &self.threshold_overrides {
            if Self::matches_gitignore_pattern(gi, path) {
                return *min;
            }
        }
        default_min
    }

    /// Checks if a clone type is excluded for a given file path.
    pub fn is_clone_type_ignored(&self, path: &Path, clone_type: &CloneType) -> bool {
        for (gi, types) in &self.type_filters {
            if Self::matches_gitignore_pattern(gi, path) && types.contains(clone_type) {
                return true;
            }
        }
        false
    }

    /// Returns a reference to the active configuration.
    pub fn config(&self) -> &SuppressionConfig {
        &self.config
    }
}
