#![forbid(unsafe_code)]

use crate::formatters::print_sarif_report;
use crate::types::{OutputFormat, RulesAction};
use cddm_core::{DEFAULT_RULES_FILE, PolicyEngine, PolicySeverity, ScanConfig, run_scan};
use comfy_table::{Cell, Color, Table};
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
                languages: vec![],
                ignore_patterns: ScanConfig::default().ignore_patterns,
                detect_type2: true,
                scan_self: true,
                enable_git_blame: false,
                cache_dir: None,
                enable_cache: true,
                cddmignore_path: None,
                ignore_tests: false,
                ignore_mocks: false,
                ignore_generated: true,
                rules_path: rules.as_ref().map(|p| p.to_string_lossy().to_string()),
                enforce_policies,
                cross_language: false,
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
                        let mut policy_table = Table::new();
                        policy_table.set_header(vec![
                            Cell::new("Rule"),
                            Cell::new("Type"),
                            Cell::new("Severity"),
                            Cell::new("Location A"),
                            Cell::new("Location B"),
                            Cell::new("Message"),
                        ]);
                        for v in &result.policy_violations {
                            let sev_cell = match v.severity {
                                PolicySeverity::Error => {
                                    Cell::new(format!("{:?}", v.severity)).fg(Color::Red)
                                }
                                PolicySeverity::Warning => {
                                    Cell::new(format!("{:?}", v.severity)).fg(Color::Yellow)
                                }
                                PolicySeverity::Info => {
                                    Cell::new(format!("{:?}", v.severity)).fg(Color::Cyan)
                                }
                            };
                            let loc_a = format!("{}:{}-{}", v.file_a, v.start_line_a, v.end_line_a);
                            let loc_b = if let (Some(fb), Some(sb), Some(eb)) =
                                (&v.file_b, v.start_line_b, v.end_line_b)
                            {
                                format!("{}:{}-{}", fb, sb, eb)
                            } else {
                                "-".to_string()
                            };
                            policy_table.add_row(vec![
                                Cell::new(&v.rule_name),
                                Cell::new(&v.rule_type),
                                sev_cell,
                                Cell::new(loc_a),
                                Cell::new(loc_b),
                                Cell::new(&v.message),
                            ]);
                        }
                        println!("{}", policy_table);
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
                        println!("| Rule | Type | Severity | Location A | Location B | Message |");
                        println!("| :--- | :--- | :--- | :--- | :--- | :--- |");
                        for v in &result.policy_violations {
                            let loc_a = format!("{}:{}-{}", v.file_a, v.start_line_a, v.end_line_a);
                            let loc_b = if let (Some(fb), Some(sb), Some(eb)) =
                                (&v.file_b, v.start_line_b, v.end_line_b)
                            {
                                format!("{}:{}-{}", fb, sb, eb)
                            } else {
                                "-".to_string()
                            };
                            println!(
                                "| `{}` | `{}` | `{:?}` | `{}` | `{}` | {} |",
                                v.rule_name, v.rule_type, v.severity, loc_a, loc_b, v.message
                            );
                        }
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
