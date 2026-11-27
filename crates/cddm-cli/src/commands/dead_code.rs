#![forbid(unsafe_code)]

use std::error::Error;

use cddm_core::dead_code::{DeadCodeConfig, run_dead_code_detection};
use cddm_core::{DeadCodeGate, PolicyEngine, apply_dead_code_gate};

use crate::formatters::format_dead_code_report;
use crate::types::commands::DeadCodeArgs;

/// Run the `cddm dead-code` CLI command.
pub async fn run_dead_code_command(args: DeadCodeArgs) -> Result<(), Box<dyn Error>> {
    tracing::info!(
        directory = %args.directory.display(),
        format = %args.format,
        min_tokens = args.min_tokens,
        static_only = args.static_only,
        "Executing cddm dead-code command"
    );

    let engine = PolicyEngine::discover(&args.directory, args.rules.as_deref());
    let gate = DeadCodeGate::resolve(
        args.fail_on_findings,
        args.max_items,
        engine.config().dead_code.as_ref(),
    );

    let config = DeadCodeConfig {
        directory: args.directory.to_string_lossy().to_string(),
        min_tokens: args.min_tokens,
        static_only: args.static_only,
        report_path: args.coverage.map(|p| p.to_string_lossy().to_string()),
        report_content: None,
        languages: args.languages,
        ignore: args.ignore,
        include_ignored: Some(args.include_ignored),
    };

    let mut summary = run_dead_code_detection(config).await?;
    if let Some(ref gate_spec) = gate {
        apply_dead_code_gate(&mut summary, gate_spec);
    }

    if args.summary {
        if args.format.eq_ignore_ascii_case("json") {
            let compact = cddm_core::CompactDeadCodeSummary::from_summary(&summary, 5);
            println!("{}", serde_json::to_string_pretty(&compact)?);
        } else {
            let compact = cddm_core::CompactDeadCodeSummary::from_summary(&summary, 5);
            println!("\n=== CDDM Dead Code Analysis Report (Summary Mode) ===\n");
            println!("Total Dead Code Items:   {}", compact.total_dead_items);
            println!("Unreferenced Functions:  {}", compact.dead_functions);
            println!("Unreachable Blocks:      {}", compact.unreachable_blocks);
            println!("Dead Duplicate Clones:   {}", compact.dead_clones);
            println!("Uncovered Test Items:    {}", compact.uncovered_items);
            println!("Total Dead Code Lines:   {}", compact.total_dead_lines);
            println!(
                "Estimated Line Savings:  {:.2}%",
                compact.estimated_savings_pct
            );
            if let Some(ref gate_res) = compact.gate {
                let badge = if gate_res.passed { "[PASS]" } else { "[FAIL]" };
                println!(
                    "Dead Code Gate:          {badge} ({}/{})",
                    gate_res.total_dead_items, gate_res.max_items
                );
            }
            println!();
            if !compact.top_items.is_empty() {
                println!("--- Top Dead Code Candidates (Showing Top 5) ---");
                for (idx, item) in compact.top_items.iter().enumerate() {
                    println!(
                        "  #{}: [{:?}] {} in {}:{} ({} lines saved, {:.0}% confidence)",
                        idx + 1,
                        item.kind,
                        item.symbol_name,
                        item.file_path,
                        item.line_span,
                        item.estimated_lines_saved,
                        item.confidence * 100.0
                    );
                }
            }
        }
    } else {
        let output = format_dead_code_report(&summary, &args.format)?;
        print!("{output}");
    }

    if let Some(ref gate_res) = summary.gate
        && !gate_res.passed
    {
        eprintln!("Error: {}", gate_res.message);
        std::process::exit(1);
    }

    Ok(())
}
