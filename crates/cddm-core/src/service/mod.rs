#![forbid(unsafe_code)]

pub mod event_bus;
pub mod orchestrator;
pub mod session;
pub mod types;

pub use event_bus::EventBus;
pub use orchestrator::WorkspaceService;
pub use session::SessionManager;
pub use types::{
    OrchestratedScanRequest, OrchestratedScanResponse, SessionId, SessionState, WorkspaceEvent,
    WorkspaceServiceStatus,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ScanConfig;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn make_test_config(directory: &str) -> ScanConfig {
        ScanConfig {
            directory: directory.to_string(),
            min_tokens: 20,
            languages: vec![],
            ignore_patterns: vec![],
            detect_type2: true,
            detect_type3: true,
            scan_self: false,
            enable_git_blame: false,
            cache_dir: None,
            enable_cache: false,
            cddmignore_path: None,
            ignore_tests: false,
            ignore_mocks: false,
            ignore_generated: true,
            rules_path: None,
            enforce_policies: false,
            cross_language: false,
            threads: None,
        }
    }

    #[tokio::test]
    async fn test_service_initial_status() {
        let service = WorkspaceService::new();
        let status = service.status().await;
        assert_eq!(status.state, SessionState::Idle);
        assert!(status.active_session_id.is_none());
        assert_eq!(status.total_active_subscribers, 0);
    }

    #[tokio::test]
    async fn test_service_event_subscription() {
        let service = WorkspaceService::new();
        let mut subscriber = service.subscribe();
        let status = service.status().await;
        assert_eq!(status.total_active_subscribers, 1);

        let event = WorkspaceEvent::FileWatchDelta {
            event_type: "modify".to_string(),
            path: "src/main.rs".to_string(),
            timestamp: chrono::Utc::now(),
        };

        let published = service.event_bus().publish(event.clone()).unwrap();
        assert_eq!(published, 1);

        let received = subscriber.recv().await.unwrap();
        match received {
            WorkspaceEvent::FileWatchDelta { path, .. } => {
                assert_eq!(path, "src/main.rs");
            }
            _ => panic!("Unexpected event received"),
        }
    }

    #[tokio::test]
    async fn test_service_scan_orchestration() {
        let dir = tempdir().unwrap();
        let file_a = dir.path().join("a.rs");
        let file_b = dir.path().join("b.rs");

        let code = r#"
            pub fn compute_sum_values(items: &[i32]) -> i32 {
                let mut sum = 0;
                for item in items {
                    sum += item;
                }
                sum
            }
        "#;

        let mut fa = File::create(&file_a).unwrap();
        fa.write_all(code.as_bytes()).unwrap();

        let mut fb = File::create(&file_b).unwrap();
        fb.write_all(code.as_bytes()).unwrap();

        let service = WorkspaceService::new();
        let mut rx = service.subscribe();

        let req = OrchestratedScanRequest {
            config: make_test_config(dir.path().to_str().unwrap()),
            session_id: Some("test-session-123".to_string()),
        };

        let response = service.execute_scan(req).await.unwrap();
        assert_eq!(response.session_id, "test-session-123");
        assert_eq!(response.scan_result.clone_pairs.len(), 1);
        assert_eq!(response.clone_clusters.len(), 1);

        // Check that events were emitted over the bus
        let mut received_progress = false;
        let mut received_completed = false;

        while let Ok(event) = rx.try_recv() {
            match event {
                WorkspaceEvent::ScanProgress { .. } => received_progress = true,
                WorkspaceEvent::ScanCompleted { session_id, .. } => {
                    assert_eq!(session_id, "test-session-123");
                    received_completed = true;
                }
                _ => {}
            }
        }

        assert!(received_progress);
        assert!(received_completed);
    }

    #[tokio::test]
    async fn test_service_query_delegation() {
        let service = WorkspaceService::new();
        let code = "fn add(a: i32, b: i32) -> i32 { a + b }";

        let tokens = service.query_tokens("src/math.rs", code).await;
        assert!(tokens.is_some());
        let tok_data = tokens.unwrap();
        assert_eq!(tok_data.language, "Rust");
        assert!(!tok_data.tokens.is_empty());

        let fingerprints = service.query_fingerprints("src/math.rs", code, 10).await;
        assert!(!fingerprints.is_empty());

        let ast = service.query_ast_summary("src/math.rs", code, "rs").await;
        assert!(ast.is_some());
        assert_eq!(ast.unwrap().extension, "rs");

        let interner = crate::cpg::SymbolInterner::new();
        let cpg = service
            .query_cpg("src/math.rs", code, "Rust", &interner)
            .await;
        assert!(cpg.is_some());

        let stats = service.query_cache_stats().await;
        assert!(stats.entries >= 2);

        service.clear_query_cache().await;
        let stats_after = service.query_cache_stats().await;
        assert_eq!(stats_after.entries, 0);
    }
}
