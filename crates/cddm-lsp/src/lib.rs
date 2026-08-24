#![forbid(unsafe_code)]

pub mod code_actions;
pub mod diagnostics;
pub mod hover;
pub mod server;
pub mod state;
pub mod utils;

pub use server::CddmLspServer;
pub use state::ServerState;

use std::path::PathBuf;
use tower_lsp::{LspService, Server};

/// Launches the CDDM Language Server Protocol daemon on standard input/output streams.
pub async fn run_server_stdio(root: PathBuf, min_tokens: usize) -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| CddmLspServer::new(client, root, min_tokens));

    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_lsp_service_creation() {
        let temp = tempdir().expect("temp dir");
        let (service, _) =
            LspService::new(|client| CddmLspServer::new(client, temp.path().to_path_buf(), 50));

        assert_eq!(
            service.inner().state.get_workspace_root().await,
            temp.path()
        );
    }
}
