#![forbid(unsafe_code)]

use crate::formatters::print_sarif_report;
use crate::types::{OutputFormat, RulesAction};
use cddm_core::{DEFAULT_RULES_FILE, PolicyEngine, PolicySeverity, ScanConfig, run_scan};
use std::fs;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

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
        } => {
            let config = ScanConfig {
                directory: directory.to_string_lossy().to_string(),
                min_tokens,
                rules_path: rules.as_ref().map(|p| p.to_string_lossy().to_string()),
                enforce_policies,
                ..Default::default()
            };

            let (tx, _rx) = mpsc::channel(100);
            let cancel_flag = Arc::new(AtomicBool::new(false));
            let result = run_scan(config, tx, cancel_flag).await?;

            match format {
                OutputFormat::Console => {
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
                OutputFormat::Json => {
                    let engine = if let Some(ref p) = rules {
                        PolicyEngine::from_file(p).unwrap_or_else(|_| PolicyEngine::empty())
                    } else {
                        let root_p = directory.join(DEFAULT_RULES_FILE);
                        if root_p.exists() {
                            PolicyEngine::from_file(&root_p)
                                .unwrap_or_else(|_| PolicyEngine::empty())
                        } else {
                            PolicyEngine::empty()
                        }
                    };
                    let eval_res = engine.evaluate(&result);
                    println!("{}", serde_json::to_string_pretty(&eval_res)?);
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
