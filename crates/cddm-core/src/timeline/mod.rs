#![forbid(unsafe_code)]

pub mod collector;
pub mod eval;

pub use collector::collect_git_timeline;
pub use eval::{
    count_tokens_in_line_span, evaluate_in_memory_duplication, extract_files_from_tree,
    is_ignored_dir,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use tempfile::tempdir;

    #[test]
    fn test_evaluate_in_memory_duplication_empty() {
        let (files, tokens, clones, clusters, dup, score) = evaluate_in_memory_duplication(&[], 50);
        assert_eq!(files, 0);
        assert_eq!(tokens, 0);
        assert_eq!(clones, 0);
        assert_eq!(clusters, 0);
        assert_eq!(dup, 0.0);
        assert_eq!(score, 100.0);
    }

    #[test]
    fn test_evaluate_in_memory_duplication_duplicate_files() {
        let code = r#"
            fn calculate_sum(a: i32, b: i32) -> i32 {
                let temp1 = a * 2;
                let temp2 = b * 3;
                let result = temp1 + temp2;
                println!("Result is {}", result);
                result + 100
            }
        "#;

        let files = vec![
            ("src/module_a.rs".to_string(), code.to_string()),
            ("src/module_b.rs".to_string(), code.to_string()),
        ];

        let (file_count, token_count, clone_count, cluster_count, dup_pct, dry_score) =
            evaluate_in_memory_duplication(&files, 20);

        assert_eq!(file_count, 2);
        assert!(token_count > 0);
        assert!(clone_count >= 1);
        assert!(cluster_count >= 1);
        assert!(dup_pct > 0.0);
        assert!(dry_score < 100.0);
    }

    #[test]
    fn test_collect_git_timeline_real_workspace() {
        let repo_root = Path::new(".");
        let cancel_flag = Arc::new(AtomicBool::new(false));

        let trend_result = collect_git_timeline(repo_root, 5, 50, cancel_flag);
        assert!(trend_result.is_ok(), "Expected git timeline to succeed");

        let trend = trend_result.expect("trend result");
        assert!(!trend.snapshots.is_empty());
        assert!(trend.snapshots.len() <= 6);
        assert!(trend.current_score >= 0.0 && trend.current_score <= 100.0);
    }

    #[test]
    fn test_collect_git_timeline_non_git_dir() {
        let temp = tempdir().expect("tempdir");
        let cancel_flag = Arc::new(AtomicBool::new(false));

        let trend_result = collect_git_timeline(temp.path(), 5, 50, cancel_flag);
        assert!(trend_result.is_err());
    }
}
