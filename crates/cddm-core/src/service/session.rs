#![forbid(unsafe_code)]

use super::types::{SessionId, SessionState, WorkspaceServiceStatus};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Manages workspace session lifecycle, state, cancellation flags, and latest health snapshots.
#[derive(Debug)]
pub struct SessionManager {
    active_session_id: RwLock<Option<SessionId>>,
    state: RwLock<SessionState>,
    last_scan_timestamp: RwLock<Option<DateTime<Utc>>>,
    last_dry_health_score: RwLock<Option<f64>>,
    cancel_flag: Arc<AtomicBool>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    /// Creates a new SessionManager initialized in Idle state.
    pub fn new() -> Self {
        Self {
            active_session_id: RwLock::new(None),
            state: RwLock::new(SessionState::Idle),
            last_scan_timestamp: RwLock::new(None),
            last_dry_health_score: RwLock::new(None),
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Starts a new session, resetting the cancellation flag and updating state.
    pub async fn start_session(
        &self,
        custom_id: Option<SessionId>,
        state: SessionState,
    ) -> (SessionId, Arc<AtomicBool>) {
        let session_id = custom_id.unwrap_or_else(|| Uuid::new_v4().to_string());

        let mut id_guard = self.active_session_id.write().await;
        *id_guard = Some(session_id.clone());

        let mut state_guard = self.state.write().await;
        *state_guard = state;

        self.cancel_flag.store(false, Ordering::SeqCst);

        (session_id, Arc::clone(&self.cancel_flag))
    }

    /// Marks the active session as finished with a specific state.
    pub async fn complete_session(&self, final_state: SessionState, health: Option<f64>) {
        let mut state_guard = self.state.write().await;
        *state_guard = final_state;

        if let Some(h) = health {
            let mut health_guard = self.last_dry_health_score.write().await;
            *health_guard = Some(h);

            let mut time_guard = self.last_scan_timestamp.write().await;
            *time_guard = Some(Utc::now());
        }

        self.cancel_flag.store(false, Ordering::SeqCst);
    }

    /// Requests cancellation of the currently active session.
    pub async fn cancel_active_session(&self) -> bool {
        let state = *self.state.read().await;
        if state == SessionState::Scanning
            || state == SessionState::Diffing
            || state == SessionState::Refactoring
        {
            self.cancel_flag.store(true, Ordering::SeqCst);
            let mut state_guard = self.state.write().await;
            *state_guard = SessionState::Cancelled;
            true
        } else {
            false
        }
    }

    /// Returns a clone of the cancellation flag for passing to long-running tasks.
    pub fn cancel_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.cancel_flag)
    }

    /// Returns the current workspace status snapshot.
    pub async fn status(&self, subscriber_count: usize) -> WorkspaceServiceStatus {
        WorkspaceServiceStatus {
            active_session_id: self.active_session_id.read().await.clone(),
            state: *self.state.read().await,
            last_scan_timestamp: *self.last_scan_timestamp.read().await,
            last_dry_health_score: *self.last_dry_health_score.read().await,
            total_active_subscribers: subscriber_count,
        }
    }
}
