#![forbid(unsafe_code)]

use crate::code_actions::generate_code_actions;
use crate::diagnostics::{clone_pair_to_diagnostics, generate_workspace_diagnostics};
use crate::hover::generate_hover;
use crate::state::ServerState;
use crate::utils::{line_range_to_lsp_range, normalize_path_for_compare, path_to_url, url_to_path};
use std::path::PathBuf;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CodeActionOptions, CodeActionParams, CodeActionProviderCapability, CodeActionResponse,
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, ExecuteCommandOptions, ExecuteCommandParams, GotoDefinitionParams,
    GotoDefinitionResponse, Hover, HoverParams, HoverProviderCapability, InitializeParams,
    InitializeResult, InitializedParams, Location, MessageType, OneOf, ReferenceParams,
    ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use tower_lsp::{Client, LanguageServer};

/// CDDM Language Server Protocol implementation.
#[derive(Debug)]
pub struct CddmLspServer {
    pub client: Client,
    pub state: ServerState,
}

impl CddmLspServer {
    #[must_use]
    pub fn new(client: Client, workspace_root: PathBuf, min_tokens: usize) -> Self {
        Self {
            client,
            state: ServerState::new(workspace_root, min_tokens),
        }
    }

    /// Re-scans the workspace and broadcasts diagnostics to the LSP client.
    pub async fn rescan_and_publish_diagnostics(&self) {
        let root = self.state.get_workspace_root().await;
        self.client
            .log_message(
                MessageType::LOG,
                format!("CDDM: Scanning workspace `{}`...", root.display()),
            )
            .await;

        match self.state.run_workspace_scan().await {
            Ok(scan_result) => {
                let diagnostics_map = generate_workspace_diagnostics(&scan_result, &root);

                // Publish diagnostics for each file with findings
                for (url, diags) in diagnostics_map {
                    self.client.publish_diagnostics(url, diags, None).await;
                }

                self.client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "CDDM: Scan complete. Found {} clone pairs (DRY Health Score: {:.1}%)",
                            scan_result.clone_pairs.len(),
                            scan_result.dry_health_score
                        ),
                    )
                    .await;
            }
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("CDDM: Scan failed: {e}"))
                    .await;
            }
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for CddmLspServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        if let Some(root_uri) = params.root_uri
            && let Some(root_path) = url_to_path(&root_uri)
        {
            self.state.set_workspace_root(root_path).await;
        }

        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
                code_action_kinds: None,
                work_done_progress_options: Default::default(),
                resolve_provider: Some(false),
            })),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            definition_provider: Some(OneOf::Left(true)),
            references_provider: Some(OneOf::Left(true)),
            execute_command_provider: Some(ExecuteCommandOptions {
                commands: vec![
                    "cddm.rescanWorkspace".to_string(),
                    "cddm.openLocation".to_string(),
                ],
                work_done_progress_options: Default::default(),
            }),
            ..ServerCapabilities::default()
        };

        let server_info = ServerInfo {
            name: "cddm-lsp".to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        };

        Ok(InitializeResult {
            capabilities,
            server_info: Some(server_info),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "CDDM Language Server v{} initialized",
                    env!("CARGO_PKG_VERSION")
                ),
            )
            .await;

        self.rescan_and_publish_diagnostics().await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let url = params.text_document.uri;
        let text = params.text_document.text;
        self.state.insert_document(url.clone(), text).await;

        // Publish diagnostics for opened file if available
        let root = self.state.get_workspace_root().await;
        if let Some(scan) = self.state.get_last_scan_result().await {
            let mut diags = Vec::new();
            for clone in &scan.clone_pairs {
                diags.extend(clone_pair_to_diagnostics(clone, &url, &root));
            }
            self.client.publish_diagnostics(url, diags, None).await;
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let url = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().last() {
            self.state.update_document(&url, change.text).await;
        }
    }

    async fn did_save(&self, _params: DidSaveTextDocumentParams) {
        self.rescan_and_publish_diagnostics().await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.state.remove_document(&params.text_document.uri).await;
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let url = params.text_document.uri;
        let range = params.range;
        let root = self.state.get_workspace_root().await;
        let clones = self.state.get_clone_pairs_for_file(url.as_str()).await;

        let actions = generate_code_actions(&url, &range, &clones, &root);
        Ok(Some(actions))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let url = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let root = self.state.get_workspace_root().await;
        let clones = self.state.get_clone_pairs_for_file(url.as_str()).await;

        let hover = generate_hover(&url, &pos, &clones, &root);
        Ok(hover)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let url = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let root = self.state.get_workspace_root().await;
        let clones = self.state.get_clone_pairs_for_file(url.as_str()).await;

        let target_norm = if let Some(path) = url_to_path(&url) {
            normalize_path_for_compare(&path.to_string_lossy())
        } else {
            normalize_path_for_compare(url.as_str())
        };

        for clone in clones {
            let norm_a = normalize_path_for_compare(&clone.file_a);
            let norm_b = normalize_path_for_compare(&clone.file_b);

            let is_a = norm_a == target_norm
                || target_norm.ends_with(&norm_a)
                || norm_a.ends_with(&target_norm);
            let is_b = norm_b == target_norm
                || target_norm.ends_with(&norm_b)
                || norm_b.ends_with(&target_norm);

            if !is_a && !is_b {
                continue;
            }

            let (my_start, my_end, other_file, other_start, other_end) = if is_a {
                (
                    clone.start_line_a,
                    clone.end_line_a,
                    &clone.file_b,
                    clone.start_line_b,
                    clone.end_line_b,
                )
            } else {
                (
                    clone.start_line_b,
                    clone.end_line_b,
                    &clone.file_a,
                    clone.start_line_a,
                    clone.end_line_a,
                )
            };

            let start_0 = if my_start > 0 { my_start - 1 } else { 0 };
            let end_0 = if my_end > 0 { my_end - 1 } else { 0 };

            if (pos.line as usize) >= start_0 && (pos.line as usize) <= end_0 {
                let counterpart_path = root.join(other_file);
                if let Some(counterpart_url) = path_to_url(&counterpart_path) {
                    let range = line_range_to_lsp_range(other_start, other_end);
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                        uri: counterpart_url,
                        range,
                    })));
                }
            }
        }

        Ok(None)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let url = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        let root = self.state.get_workspace_root().await;
        let clones = self.state.get_clone_pairs_for_file(url.as_str()).await;

        let mut locations = Vec::new();
        let target_norm = if let Some(path) = url_to_path(&url) {
            normalize_path_for_compare(&path.to_string_lossy())
        } else {
            normalize_path_for_compare(url.as_str())
        };

        for clone in clones {
            let norm_a = normalize_path_for_compare(&clone.file_a);
            let norm_b = normalize_path_for_compare(&clone.file_b);

            let is_a = norm_a == target_norm
                || target_norm.ends_with(&norm_a)
                || norm_a.ends_with(&target_norm);
            let is_b = norm_b == target_norm
                || target_norm.ends_with(&norm_b)
                || norm_b.ends_with(&target_norm);

            if !is_a && !is_b {
                continue;
            }

            let (my_start, my_end, other_file, other_start, other_end) = if is_a {
                (
                    clone.start_line_a,
                    clone.end_line_a,
                    &clone.file_b,
                    clone.start_line_b,
                    clone.end_line_b,
                )
            } else {
                (
                    clone.start_line_b,
                    clone.end_line_b,
                    &clone.file_a,
                    clone.start_line_a,
                    clone.end_line_a,
                )
            };

            let start_0 = if my_start > 0 { my_start - 1 } else { 0 };
            let end_0 = if my_end > 0 { my_end - 1 } else { 0 };

            if (pos.line as usize) >= start_0 && (pos.line as usize) <= end_0 {
                let counterpart_path = root.join(other_file);
                if let Some(counterpart_url) = path_to_url(&counterpart_path) {
                    let range = line_range_to_lsp_range(other_start, other_end);
                    locations.push(Location {
                        uri: counterpart_url,
                        range,
                    });
                }
            }
        }

        if locations.is_empty() {
            Ok(None)
        } else {
            Ok(Some(locations))
        }
    }

    async fn execute_command(
        &self,
        params: ExecuteCommandParams,
    ) -> Result<Option<serde_json::Value>> {
        if params.command == "cddm.rescanWorkspace" {
            self.rescan_and_publish_diagnostics().await;
            return Ok(Some(serde_json::json!({ "status": "ok" })));
        }

        Ok(None)
    }
}
