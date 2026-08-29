#![forbid(unsafe_code)]

use super::event_bus::EventBus;
use super::session::SessionManager;
use super::types::{
    OrchestratedScanRequest, OrchestratedScanResponse, SessionState, WorkspaceEvent,
    WorkspaceServiceStatus,
};
use crate::cluster::cluster_clone_pairs;
use crate::detector::run_scan;
use crate::types::ScanProgress;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{broadcast, mpsc};

/// Unified Workspace Service orchestrating core CDDM engine operations across all interaction pillars.
#[derive(Debug, Clone)]
pub struct WorkspaceService {
    event_bus: EventBus,
    session_manager: Arc<SessionManager>,
}

impl Default for WorkspaceService {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceService {
    /// Creates a new WorkspaceService instance with default event bus and session manager.
    pub fn new() -> Self {
        Self {
            event_bus: EventBus::default(),
            session_manager: Arc::new(SessionManager::new()),
        }
    }

    /// Access the underlying event bus directly.
    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    /// Subscribes to real-time reactive workspace events.
    pub fn subscribe(&self) -> broadcast::Receiver<WorkspaceEvent> {
        self.event_bus.subscribe()
    }

    /// Retrieves current workspace service snapshot status.
    pub async fn status(&self) -> WorkspaceServiceStatus {
        let subscriber_count = self.event_bus.subscriber_count();
        self.session_manager.status(subscriber_count).await
    }

    /// Requests cancellation of any running scan or refactoring operation.
    pub async fn cancel_active_operation(&self) -> bool {
        let cancelled = self.session_manager.cancel_active_session().await;
        if cancelled {
            let status = self.status().await;
            if let Some(session_id) = status.active_session_id {
                let _ = self
                    .event_bus
                    .publish(WorkspaceEvent::OperationCancelled { session_id });
            }
        }
        cancelled
    }

    /// Executes an orchestrated workspace code scan with automatic event streaming and clustering.
    pub async fn execute_scan(
        &self,
        request: OrchestratedScanRequest,
    ) -> Result<OrchestratedScanResponse, String> {
        let start_time = Instant::now();
        let (session_id, cancel_flag) = self
            .session_manager
            .start_session(request.session_id, SessionState::Scanning)
            .await;

        let (progress_tx, mut progress_rx) = mpsc::channel::<ScanProgress>(128);

        // Forward progress updates from channel to reactive event bus
        let bus = self.event_bus.clone();
        let s_id = session_id.clone();
        let forwarder_handle = tokio::spawn(async move {
            while let Some(progress) = progress_rx.recv().await {
                let event = WorkspaceEvent::ScanProgress {
                    session_id: s_id.clone(),
                    phase: progress.phase,
                    progress: progress.progress,
                    message: progress.message,
                };
                let _ = bus.publish(event);
            }
        });

        let scan_result_res = run_scan(request.config, progress_tx, cancel_flag).await;
        let _ = forwarder_handle.await;

        let elapsed_ms = start_time.elapsed().as_millis() as u64;

        match scan_result_res {
            Ok(scan_result) => {
                let clone_clusters = if scan_result.clone_clusters.is_empty() {
                    cluster_clone_pairs(&scan_result.clone_pairs)
                } else {
                    scan_result.clone_clusters.clone()
                };

                let dry_health = scan_result.dry_health_score;

                self.session_manager
                    .complete_session(SessionState::Completed, Some(dry_health))
                    .await;

                let _ = self.event_bus.publish(WorkspaceEvent::ScanCompleted {
                    session_id: session_id.clone(),
                    dry_health_score: dry_health,
                    duplication_percentage: scan_result.duplication_percentage,
                    total_clone_pairs: scan_result.clone_pairs.len(),
                    total_clone_clusters: clone_clusters.len(),
                    duration_ms: elapsed_ms,
                });

                Ok(OrchestratedScanResponse {
                    session_id,
                    scan_result,
                    clone_clusters,
                    dry_health_score: dry_health,
                    elapsed_ms,
                })
            }
            Err(err) => {
                self.session_manager
                    .complete_session(SessionState::Failed, None)
                    .await;

                let _ = self.event_bus.publish(WorkspaceEvent::OperationFailed {
                    session_id: session_id.clone(),
                    error_message: err.clone(),
                });

                Err(err)
            }
        }
    }
}
