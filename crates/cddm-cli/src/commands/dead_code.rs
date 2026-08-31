#![forbid(unsafe_code)]

use std::error::Error;

use cddm_core::dead_code::{DeadCodeConfig, run_dead_code_detection};

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

    let config = DeadCodeConfig {
        directory: args.directory.to_string_lossy().to_string(),
        min_tokens: args.min_tokens,
        static_only: args.static_only,
        report_path: args.coverage.map(|p| p.to_string_lossy().to_string()),
        report_content: None,
        languages: args.languages,
        ignore: args.ignore,
    };

    let summary = run_dead_code_detection(config).await?;
    let output = format_dead_code_report(&summary, &args.format)?;

    print!("{output}");

    Ok(())
}
