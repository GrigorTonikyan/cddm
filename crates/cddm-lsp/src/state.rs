#![forbid(unsafe_code)]

use cddm_core::{
    CloneCluster, ClonePair, ScanConfig, ScanResult, cluster::cluster_clone_pairs, run_scan,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::sync::{RwLock, mpsc};
use tower_lsp::lsp_types::Url;

/// Inner mutable state for the CDDM Language Server.
#[derive(Debug)]
pub struct ServerStateInner {
    pub workspace_root: PathBuf,
    pub min_tokens: usize,
    pub languages: Vec<String>,
    pub ignore_patterns: Vec<String>,
    pub open_documents: HashMap<Url, String>,
    pub last_scan_result: Option<ScanResult>,
    pub last_clusters: Vec<CloneCluster>,
}

impl ServerStateInner {
    #[must_use]
    pub fn new(root: PathBuf, min_tokens: usize) -> Self {
        Self {
            workspace_root: root,
            min_tokens,
            languages: Vec::new(),
            ignore_patterns: Vec::new(),
            open_documents: HashMap::new(),
            last_scan_result: None,
            last_clusters: Vec::new(),
        }
    }

    #[must_use]
    pub fn build_scan_config(&self) -> ScanConfig {
        ScanConfig {
            directory: self.workspace_root.to_string_lossy().to_string(),
            min_tokens: self.min_tokens,
            languages: self.languages.clone(),
            ignore_patterns: self.ignore_patterns.clone(),
            detect_type2: true,
            scan_self: true,
            enable_git_blame: false,
            cache_dir: None,
            enable_cache: true,
            cddmignore_path: None,
            ignore_tests: false,
            ignore_mocks: false,
            ignore_generated: true,
            rules_path: None,
            enforce_policies: false,
            cross_language: false,
        }
    }
}

/// Thread-safe handle to the CDDM Language Server state.
#[derive(Clone, Debug)]
pub struct ServerState {
    inner: Arc<RwLock<ServerStateInner>>,
    is_scanning: Arc<AtomicBool>,
}

impl ServerState {
    #[must_use]
    pub fn new(root: PathBuf, min_tokens: usize) -> Self {
        Self {
            inner: Arc::new(RwLock::new(ServerStateInner::new(root, min_tokens))),
            is_scanning: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn set_workspace_root(&self, root: PathBuf) {
        let mut state = self.inner.write().await;
        state.workspace_root = root;
    }

    pub async fn get_workspace_root(&self) -> PathBuf {
        let state = self.inner.read().await;
        state.workspace_root.clone()
    }

    pub async fn set_min_tokens(&self, min_tokens: usize) {
        let mut state = self.inner.write().await;
        state.min_tokens = min_tokens;
    }

    pub async fn insert_document(&self, url: Url, text: String) {
        let mut state = self.inner.write().await;
        state.open_documents.insert(url, text);
    }

    pub async fn update_document(&self, url: &Url, text: String) {
        let mut state = self.inner.write().await;
        state.open_documents.insert(url.clone(), text);
    }

    pub async fn remove_document(&self, url: &Url) {
        let mut state = self.inner.write().await;
        state.open_documents.remove(url);
    }

    pub async fn get_document_text(&self, url: &Url) -> Option<String> {
        let state = self.inner.read().await;
        state.open_documents.get(url).cloned()
    }

    pub async fn get_last_scan_result(&self) -> Option<ScanResult> {
        let state = self.inner.read().await;
        state.last_scan_result.clone()
    }

    pub async fn get_last_clusters(&self) -> Vec<CloneCluster> {
        let state = self.inner.read().await;
        state.last_clusters.clone()
    }

    pub async fn get_clone_pairs_for_file(&self, file_path_or_url: &str) -> Vec<ClonePair> {
        let state = self.inner.read().await;
        let Some(scan) = &state.last_scan_result else {
            return Vec::new();
        };

        let norm_target = crate::utils::normalize_path_for_compare(file_path_or_url);

        scan.clone_pairs
            .iter()
            .filter(|clone| {
                let norm_a = crate::utils::normalize_path_for_compare(&clone.file_a);
                let norm_b = crate::utils::normalize_path_for_compare(&clone.file_b);
                norm_a == norm_target
                    || norm_b == norm_target
                    || norm_a.ends_with(&norm_target)
                    || norm_b.ends_with(&norm_target)
                    || norm_target.ends_with(&norm_a)
                    || norm_target.ends_with(&norm_b)
            })
            .cloned()
            .collect()
    }

    /// Performs a full scan of the workspace and updates cached results and clusters.
    pub async fn run_workspace_scan(&self) -> Result<ScanResult, String> {
        if self.is_scanning.swap(true, Ordering::SeqCst) {
            let state = self.inner.read().await;
            return state
                .last_scan_result
                .clone()
                .ok_or_else(|| "Scan already running".to_string());
        }

        let config = {
            let state = self.inner.read().await;
            state.build_scan_config()
        };

        let (progress_tx, mut _progress_rx) = mpsc::channel(100);
        let cancel_flag = Arc::new(AtomicBool::new(false));

        let scan_res = run_scan(config, progress_tx, cancel_flag).await?;
        let clusters = cluster_clone_pairs(&scan_res.clone_pairs);

        {
            let mut state = self.inner.write().await;
            state.last_scan_result = Some(scan_res.clone());
            state.last_clusters = clusters;
        }

        self.is_scanning.store(false, Ordering::SeqCst);
        Ok(scan_res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_server_state_lifecycle() {
        let temp = tempdir().expect("temp dir");
        let state = ServerState::new(temp.path().to_path_buf(), 50);

        assert_eq!(state.get_workspace_root().await, temp.path());

        let url = Url::parse("file:///test/file.rs").expect("valid url");
        state
            .insert_document(url.clone(), "fn main() {}".to_string())
            .await;

        let doc = state.get_document_text(&url).await;
        assert_eq!(doc.as_deref(), Some("fn main() {}"));

        state.remove_document(&url).await;
        assert_eq!(state.get_document_text(&url).await, None);
    }
}
