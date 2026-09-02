#![forbid(unsafe_code)]

use std::error::Error;

use cddm_core::dead_code::{DeadClonePruneConfig, prune_dead_clone_clusters};

use crate::formatters::format_prune_report;
use crate::types::commands::PruneArgs;

/// Run the `cddm prune` CLI command.
pub async fn run_prune_command(args: PruneArgs) -> Result<(), Box<dyn Error>> {
    tracing::info!(
        directory = %args.directory.display(),
        dry_run = args.dry_run,
        safe_only = args.safe_only,
        threshold = args.threshold,
        min_tokens = args.min_tokens,
        format = %args.format,
        "Executing cddm prune command"
    );

    let config = DeadClonePruneConfig {
        directory: args.directory.to_string_lossy().to_string(),
        min_tokens: args.min_tokens,
        dry_run: args.dry_run,
        safe_only: args.safe_only,
        confidence_threshold: args.threshold,
        item_ids: None,
        languages: args.languages,
        ignore: args.ignore,
    };

    let result = prune_dead_clone_clusters(config).await?;
    let output = format_prune_report(&result, &args.format)?;

    print!("{output}");

    Ok(())
}
