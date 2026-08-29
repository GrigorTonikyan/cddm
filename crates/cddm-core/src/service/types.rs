#![forbid(unsafe_code)]

use crate::types::{CloneCluster, ScanConfig, ScanPhase, ScanResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unique identifier for an execution or scan session.
pub type SessionId = String;

/// High-level lifecycle state of a workspace service session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    Idle,
    Scanning,
    Diffing,
    Refactoring,
    Watching,
    Completed,
    Failed,
    Cancelled,
}

/// Reactive domain event emitted across the workspace service event bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WorkspaceEvent {
    /// Progress notification during multi-phase scan.
    ScanProgress {
        session_id: SessionId,
        phase: ScanPhase,
        progress: f64,
        message: String,
    },
    /// Emitted when a full scan finishes successfully.
    ScanCompleted {
        session_id: SessionId,
        dry_health_score: f64,
        duplication_percentage: f64,
        total_clone_pairs: usize,
        total_clone_clusters: usize,
        duration_ms: u64,
    },
    /// Emitted when a scan or refactor operation fails.
    OperationFailed {
        session_id: SessionId,
        error_message: String,
    },
    /// Emitted when an operation is cancelled by client request.
    OperationCancelled { session_id: SessionId },
    /// Emitted when a live file watch event occurs.
    FileWatchDelta {
        event_type: String,
        path: String,
        timestamp: DateTime<Utc>,
    },
    /// Emitted when an automated refactoring patch is synthesized.
    RefactorSynthesized {
        cluster_id: usize,
        occurrences: usize,
        invariants_count: usize,
    },
}

/// Snapshot summary of current workspace health and active session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceServiceStatus {
    pub active_session_id: Option<SessionId>,
    pub state: SessionState,
    pub last_scan_timestamp: Option<DateTime<Utc>>,
    pub last_dry_health_score: Option<f64>,
    pub total_active_subscribers: usize,
}

/// High-level options for executing an orchestrated workspace scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratedScanRequest {
    pub config: ScanConfig,
    pub session_id: Option<SessionId>,
}

/// Result returned from an orchestrated workspace scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratedScanResponse {
    pub session_id: SessionId,
    pub scan_result: ScanResult,
    pub clone_clusters: Vec<CloneCluster>,
    pub dry_health_score: f64,
    pub elapsed_ms: u64,
}
