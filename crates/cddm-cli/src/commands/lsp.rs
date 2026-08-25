#![forbid(unsafe_code)]

use std::path::PathBuf;

pub async fn run_lsp_command(
    directory: PathBuf,
    min_tokens: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    cddm_lsp::run_server_stdio(directory, min_tokens).await?;
    Ok(())
}
