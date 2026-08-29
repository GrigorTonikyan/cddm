#![forbid(unsafe_code)]

pub mod constants;
pub mod heal;
pub mod provider;
pub mod types;

pub use constants::*;
pub use heal::{extract_patch_from_response, heal_cluster_refactor};
pub use provider::{
    AiProvider, ClaudeProvider, GeminiProvider, MockAiProvider, OllamaProvider, OpenAiProvider,
    create_ai_provider,
};
pub use types::{
    AiProviderConfig, AiProviderKind, HealIterationLog, HealRefactorRequest, HealRefactorResult,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CloneLocation;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_mock_ai_provider() {
        let mock = MockAiProvider::new(Some("--- a/f.rs\n+++ b/f.rs\n".to_string()));
        let res = mock.complete_prompt("test prompt").await.unwrap();
        assert!(res.contains("--- a/f.rs"));
    }

    #[tokio::test]
    async fn test_create_ai_provider_factory() {
        let config = AiProviderConfig {
            provider: AiProviderKind::Mock,
            ..Default::default()
        };
        let provider = create_ai_provider(&config);
        let res = provider.complete_prompt("hello").await.unwrap();
        assert!(!res.is_empty());
    }

    #[test]
    fn test_extract_patch_from_response() {
        let markdown = "Here is the refactored patch:\n\n```diff\n--- a/main.rs\n+++ \
                        b/main.rs\n@@ -1,1 +1,1 @@\n-test\n+prod\n```\n\nHope this helps!";
        let patch = extract_patch_from_response(markdown);
        assert_eq!(
            patch,
            "--- a/main.rs\n+++ b/main.rs\n@@ -1,1 +1,1 @@\n-test\n+prod"
        );
    }

    #[tokio::test]
    async fn test_heal_cluster_refactor_mock_loop() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("main.rs");
        std::fs::write(&file_path, "fn duplicate_one() { println!(\"dup\"); }\n").unwrap();

        let mock_patch = "--- a/main.rs\n+++ b/main.rs\n@@ -1,1 +1,1 @@\n-fn duplicate_one() { \
                          println!(\"dup\"); }\n+fn duplicate_one() { shared_dup(); }\n";

        let req = HealRefactorRequest {
            cluster_id: Some(1),
            pair_id: None,
            occurrences: vec![CloneLocation {
                file: "main.rs".to_string(),
                start_line: 1,
                end_line: 1,
                author: None,
            }],
            function_name: Some("shared_dup".to_string()),
            target_module: Some("utils.rs".to_string()),
            custom_instructions: None,
            provider_config: AiProviderConfig {
                provider: AiProviderKind::Mock,
                model: Some(mock_patch.to_string()),
                ..Default::default()
            },
            max_iterations: 2,
            apply_branch: None,
            verify: false,
            test_cmd: None,
            workspace_root: Some(dir.path().to_path_buf()),
        };

        let res = heal_cluster_refactor(dir.path(), &req).await.unwrap();
        assert_eq!(res.iterations_run, 1);
        assert!(!res.iterations.is_empty());
    }
}
