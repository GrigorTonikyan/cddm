#![forbid(unsafe_code)]

use crate::formatters::{print_trend_console_report, print_trend_markdown_report};
use crate::types::OutputFormat;
use cddm_core::collect_git_timeline;
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};

pub fn run_trend_command(
    directory: PathBuf,
    max_samples: usize,
    min_tokens: usize,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let cancel_flag = Arc::new(AtomicBool::new(false));
    match collect_git_timeline(&directory, max_samples, min_tokens, cancel_flag) {
        Ok(trend) => match format {
            OutputFormat::Console => print_trend_console_report(&trend),
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&trend)?);
            }
            OutputFormat::Markdown => print_trend_markdown_report(&trend),
            OutputFormat::Sarif => {
                eprintln!(
                    "[WARN] SARIF format is not applicable for timeline trend. Outputting JSON."
                );
                println!("{}", serde_json::to_string_pretty(&trend)?);
            }
        },
        Err(err) => {
            eprintln!("[ERROR] Failed to collect Git timeline trend: {}", err);
            std::process::exit(1);
        }
    }
    Ok(())
}
