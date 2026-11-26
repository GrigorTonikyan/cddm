#![forbid(unsafe_code)]

use crate::formatters::print_sarif_report;
use crate::types::{OutputFormat, RulesAction};
use cddm_core::{
    DEFAULT_RULES_FILE, PolicyEngine, PolicyEvaluationOptions, PolicySeverity, ScanConfig,
    find_workspace_root, run_scan,
};
use std::fs;
use std::path::Path;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

fn load_rules_engine(rules: Option<&Path>, directory: &Path) -> PolicyEngine {
    if let Some(p) = rules {
        return PolicyEngine::from_file(p).unwrap_or_else(|_| PolicyEngine::empty());
    }
    let root_p = directory.join(DEFAULT_RULES_FILE);
    if root_p.exists() {
        return PolicyEngine::from_file(&root_p).unwrap_or_else(|_| PolicyEngine::empty());
    }
    let ws_p = find_workspace_root(directory).join(DEFAULT_RULES_FILE);
    if ws_p.exists() {
        return PolicyEngine::from_file(&ws_p).unwrap_or_else(|_| PolicyEngine::empty());
    }
    PolicyEngine::empty()
}

pub async fn run_rules_command(action: RulesAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        RulesAction::Init {
            output,
            force,
            write,
        } => {
            if output.exists() && !force {
                eprintln!(
                    "[ERROR] '{}' already exists. Use --force to overwrite.",
                    output.display()
                );
                std::process::exit(1);
            }
            let template = PolicyEngine::starter_rules_toml();
            if write {
                fs::write(&output, template)?;
                println!(
                    "[PASS] Generated starter .cddmrules.toml template at '{}'",
                    output.display()
                );
            } else {
                println!("{}", template);
            }
        }
        RulesAction::Check {
            directory,
            rules,
            min_tokens,
            format,
            enforce_policies,
            summary,
            include_ignored,
            ignore_tests,
            no_ignore_tests,
            ignore_mocks,
            no_ignore_mocks,
            ignore_generated,
        } => {
            let engine = load_rules_engine(rules.as_deref(), &directory);
            let force_ignore_tests = if no_ignore_tests {
                Some(false)
            } else {
                ignore_tests
            };
            let force_ignore_mocks = if no_ignore_mocks {
                Some(false)
            } else {
                ignore_mocks
            };
            let force_include_ignored = include_ignored.then_some(true);

            let effective_ignore_tests = force_ignore_tests
                .or(engine.config().ignore_tests)
                .unwrap_or(true);
            let effective_ignore_mocks = force_ignore_mocks
                .or(engine.config().ignore_mocks)
                .unwrap_or(true);
            let effective_include_ignored = force_include_ignored
                .or(engine.config().include_ignored)
                .unwrap_or(false);

            let config = ScanConfig {
                directory: directory.to_string_lossy().to_string(),
                min_tokens,
                rules_path: rules.as_ref().map(|p| p.to_string_lossy().to_string()),
                enforce_policies,
                ignore_tests: effective_ignore_tests,
                ignore_mocks: effective_ignore_mocks,
                ignore_generated,
                include_ignored: effective_include_ignored,
                ..Default::default()
            };

            let (tx, _rx) = mpsc::channel(100);
            let cancel_flag = Arc::new(AtomicBool::new(false));
            let result = run_scan(config, tx, cancel_flag).await?;

            match format {
                OutputFormat::Console => {
                    if summary {
                        println!(
                            "\n=== CDDM Architectural Policy Evaluation Report (Summary Mode) ==="
                        );
                        println!("Scanned Target:     {}", directory.display());
                        println!("Total Violations:   {}", result.policy_violations.len());
                        println!();

                        if result.policy_violations.is_empty() {
                            println!(
                                "[PASS] All architectural boundary and zero-duplication policies \
                                 verified cleanly."
                            );
                        } else {
                            println!("--- Top Violations (Showing Top 5) ---");
                            for (idx, v) in result.policy_violations.iter().take(5).enumerate() {
                                let scope = if let (Some(actual), Some(allowed)) =
                                    (v.line_count, v.max_lines)
                                {
                                    format!("{} ({actual}/{allowed} lines)", v.file_a)
                                } else if let Some(ref fb) = v.file_b {
                                    format!("{} vs {fb}", v.file_a)
                                } else {
                                    format!("{}:{}-{}", v.file_a, v.start_line_a, v.end_line_a)
                                };
                                println!(
                                    "  #{}: [{:?}] {} ({scope})",
                                    idx + 1,
                                    v.severity,
                                    v.rule_name,
                                );
                            }
                        }
                    } else {
                        println!("\n=== CDDM Architectural Policy Evaluation Report ===");
                        println!("Scanned Target:     {}", directory.display());
                        println!("Total Violations:   {}", result.policy_violations.len());
                        println!();

                        if result.policy_violations.is_empty() {
                            println!(
                                "[PASS] All architectural boundary and zero-duplication policies \
                                 verified cleanly."
                            );
                        } else {
                            crate::formatters::print_policy_violations_console(
                                &result.policy_violations,
                            );
                        }
                    }
                }
                OutputFormat::Json => {
                    let eval_opts = PolicyEvaluationOptions {
                        ignore_tests: effective_ignore_tests,
                        ignore_mocks: effective_ignore_mocks,
                        include_ignored: effective_include_ignored,
                        force_ignore_tests,
                        force_ignore_mocks,
                        force_include_ignored,
                    };
                    let eval_res = engine.evaluate_with_options(&result, &eval_opts);
                    if summary {
                        let compact = cddm_core::CompactPolicyResult::from_evaluation(&eval_res, 5);
                        println!("{}", serde_json::to_string_pretty(&compact)?);
                    } else {
                        println!("{}", serde_json::to_string_pretty(&eval_res)?);
                    }
                }
                OutputFormat::Markdown => {
                    println!("# CDDM Architectural Policy Evaluation Report\n");
                    println!("- **Scanned Target**: `{}`", directory.display());
                    println!(
                        "- **Total Violations**: `{}`\n",
                        result.policy_violations.len()
                    );
                    if result.policy_violations.is_empty() {
                        println!(
                            "> [PASS] All architectural boundary and zero-duplication policies \
                             verified cleanly."
                        );
                    } else {
                        crate::formatters::print_policy_violations_markdown(
                            &result.policy_violations,
                        );
                    }
                }
                OutputFormat::Sarif => {
                    print_sarif_report(&result)?;
                }
                OutputFormat::Ndjson => {
                    println!("{}", serde_json::to_string(&result.policy_violations)?);
                }
            }

            if enforce_policies
                && result
                    .policy_violations
                    .iter()
                    .any(|v| v.severity == PolicySeverity::Error)
            {
                eprintln!(
                    "Error: Policy enforcement failed with {} error-level violation(s).",
                    result
                        .policy_violations
                        .iter()
                        .filter(|v| v.severity == PolicySeverity::Error)
                        .count()
                );
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
