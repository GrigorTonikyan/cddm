#![forbid(unsafe_code)]

use std::path::PathBuf;

use crate::tui::run_tui;

/// Entrypoint for the `cddm tui` CLI subcommand.
pub async fn run_tui_command(
    directory: Option<PathBuf>,
    min_tokens: usize,
    watch: bool,
    fail_threshold: Option<f64>,
    languages: Vec<String>,
    ignore: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = directory.unwrap_or_else(|| PathBuf::from("."));

    run_tui(
        target_dir,
        min_tokens,
        watch,
        fail_threshold,
        languages,
        ignore,
    )
    .await
}
