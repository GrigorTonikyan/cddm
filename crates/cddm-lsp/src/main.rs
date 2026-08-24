#![forbid(unsafe_code)]

use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Direct logging exclusively to stderr so stdout remains pure JSON-RPC 2.0 transport
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let min_tokens = cddm_core::DEFAULT_MIN_TOKENS;

    tracing::info!("Starting CDDM Language Server Protocol daemon over Stdio...");
    cddm_lsp::run_server_stdio(root, min_tokens).await?;
    Ok(())
}
