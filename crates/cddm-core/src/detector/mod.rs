#![forbid(unsafe_code)]

pub mod discovery;
pub mod indexer;
pub mod runner;
pub mod types;

pub use discovery::{discover_candidate_files, init_policy_engine, init_suppression_engine};
pub use indexer::index_and_match_clone_pairs;
pub use runner::run_scan;
pub use types::{Location, ParsedFile, count_tokens_in_line_span};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CloneType, ScanConfig, ScanResult};
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use tokio::sync::mpsc;

    fn make_test_config(directory: &str, min_tokens: usize) -> ScanConfig {
        ScanConfig {
            directory: directory.to_string(),
            min_tokens,
            languages: vec![],
            ignore_patterns: vec![],
            detect_type2: true,
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
        }
    }

    async fn run_test_scan(config: ScanConfig) -> Result<ScanResult, String> {
        let (tx, _rx) = mpsc::channel(100);
        run_scan(config, tx, Arc::new(AtomicBool::new(false))).await
    }

    fn write_test_file(path: impl AsRef<std::path::Path>, content: &str) {
        let mut file = std::fs::File::create(path).unwrap();
        std::io::Write::write_all(&mut file, content.as_bytes()).unwrap();
    }

    #[tokio::test]
    async fn test_empty_scan() {
        let result = run_test_scan(make_test_config("non_existent_dir", 50))
            .await
            .unwrap();
        assert_eq!(result.total_files, 0);
    }

    #[tokio::test]
    async fn test_scan_with_real_duplicate_files() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let file_a_path = dir.path().join("a.rs");
        let file_b_path = dir.path().join("b.rs");

        let content = r#"
            fn calculate_sum(a: i32, b: i32) -> i32 {
                let mut sum = 0;
                for i in 0..10 {
                    sum += a + b + i;
                    println!("intermediate: {}", sum);
                    if sum > 100 {
                        break;
                    }
                }
                sum
            }
        "#;
        let content_ext = format!(
            "{} {} {} {} {} {}",
            content, content, content, content, content, content
        );

        write_test_file(&file_a_path, &content_ext);
        write_test_file(&file_b_path, &content_ext);

        let result = run_test_scan(make_test_config(&dir.path().to_string_lossy(), 50))
            .await
            .unwrap();
        assert!(result.total_clones > 0);
        assert!(!result.clone_pairs.is_empty());
        assert!(result.total_clusters > 0);
        assert!(!result.clone_clusters.is_empty());
        assert_eq!(result.clone_clusters[0].occurrences.len(), 2);
    }

    #[tokio::test]
    async fn test_scan_cancellation() {
        let (tx, _rx) = mpsc::channel(100);
        let cancel_flag = Arc::new(AtomicBool::new(true)); // Pre-cancelled
        let result = run_scan(make_test_config(".", 50), tx, cancel_flag).await;
        assert_eq!(result.unwrap_err(), "Scan cancelled");
    }

    #[tokio::test]
    async fn test_scan_language_filter() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        write_test_file(dir.path().join("test.rs"), "fn main() {}\n");
        write_test_file(dir.path().join("test.py"), "def main(): pass\n");

        let mut config = make_test_config(&dir.path().to_string_lossy(), 50);
        config.languages = vec!["Rust".to_string()];

        let result = run_test_scan(config).await.unwrap();
        assert_eq!(result.total_files, 1);
        assert_eq!(result.language_breakdown.len(), 1);
        assert_eq!(result.language_breakdown[0].language, "Rust");
    }

    #[tokio::test]
    async fn test_scan_ignore_patterns() {
        use std::fs::{self, File};
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("node_modules")).unwrap();
        File::create(dir.path().join("node_modules").join("test.rs")).unwrap();
        File::create(dir.path().join("main.rs")).unwrap();

        let mut config = make_test_config(&dir.path().to_string_lossy(), 50);
        config.ignore_patterns = vec!["node_modules".to_string()];

        let result = run_test_scan(config).await.unwrap();
        assert_eq!(result.total_files, 1);
    }

    #[tokio::test]
    async fn test_dry_health_score_range() {
        let result = run_test_scan(make_test_config(".", 50)).await;
        if let Ok(res) = result {
            assert!(res.dry_health_score >= 0.0 && res.dry_health_score <= 100.0);
        }
    }

    #[tokio::test]
    async fn test_no_self_overlapping_clones() {
        use std::fs::File;
        use std::io::Write;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("single.rs");
        let content = "fn foo() { println!(\"hello world\"); }\n";
        let mut f = File::create(&file_path).unwrap();
        for _ in 0..20 {
            writeln!(f, "{}", content).unwrap();
        }

        let mut config = make_test_config(&dir.path().to_string_lossy(), 20);
        config.scan_self = true;

        let result = run_test_scan(config).await.unwrap();
        for pair in &result.clone_pairs {
            if pair.file_a == pair.file_b {
                let overlaps =
                    pair.start_line_a <= pair.end_line_b && pair.start_line_b <= pair.end_line_a;
                assert!(
                    !overlaps,
                    "Self clone pair should not overlap with itself: {:?}",
                    pair
                );
            }
        }
    }

    #[tokio::test]
    async fn test_scan_with_disk_caching() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let cache_db = dir.path().join("sub").join("test_cache.redb");

        let code =
            "fn compute_heavy_task() -> u64 { let mut v = 0; for i in 0..100 { v += i * 2; } v }\n";
        let code_long = format!("{} {} {} {} {}", code, code, code, code, code);

        write_test_file(dir.path().join("a.rs"), &code_long);
        write_test_file(dir.path().join("b.rs"), &code_long);

        let mut config = make_test_config(&dir.path().to_string_lossy(), 30);
        config.enable_cache = true;
        config.cache_dir = Some(cache_db.to_string_lossy().to_string());

        // First scan (populates cache)
        let res1 = run_test_scan(config.clone()).await.unwrap();
        assert_eq!(res1.total_files, 2);
        assert!(res1.total_clones > 0);

        // Verify cache file was created
        assert!(cache_db.exists());

        // Second scan (uses cache)
        let res2 = run_test_scan(config).await.unwrap();
        assert_eq!(res2.total_files, 2);
        assert_eq!(res2.total_clones, res1.total_clones);
        assert_eq!(res2.duplication_percentage, res1.duplication_percentage);
    }

    #[tokio::test]
    async fn test_exact_and_renamed_clone_classification() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();

        // Exact clone pair
        let exact_code = r#"
            fn calculate_area(width: f64, height: f64) -> f64 {
                let area = width * height;
                println!("Calculated area: {}", area);
                if area > 1000.0 {
                    println!("Warning: large area");
                }
                area
            }
        "#;
        write_test_file(dir.path().join("exact1.rs"), exact_code);
        write_test_file(dir.path().join("exact2.rs"), exact_code);

        // Renamed clone pair
        let renamed_a = r#"
            fn compute_perimeter(side_a: f64, side_b: f64) -> f64 {
                let perimeter = (side_a + side_b) * 2.0;
                println!("Calculated perimeter: {}", perimeter);
                if perimeter > 500.0 {
                    println!("Warning: large boundary");
                }
                perimeter
            }
        "#;
        let renamed_b = r#"
            fn eval_circumference(dim_x: f64, dim_y: f64) -> f64 {
                let total_boundary = (dim_x + dim_y) * 2.0;
                println!("Calculated boundary: {}", total_boundary);
                if total_boundary > 500.0 {
                    println!("Warning: massive border");
                }
                total_boundary
            }
        "#;
        write_test_file(dir.path().join("renamed1.rs"), renamed_a);
        write_test_file(dir.path().join("renamed2.rs"), renamed_b);

        let config = make_test_config(&dir.path().to_string_lossy(), 20);
        let result = run_test_scan(config).await.unwrap();

        assert!(result.total_clones >= 2);
        let exact_found = result.clone_pairs.iter().any(|p| {
            p.clone_type == CloneType::Exact
                && ((p.file_a.contains("exact1") && p.file_b.contains("exact2"))
                    || (p.file_a.contains("exact2") && p.file_b.contains("exact1")))
        });
        assert!(
            exact_found,
            "Exact clone pair should be classified as Exact"
        );
    }

    #[tokio::test]
    async fn test_polyglot_ast_scan() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();

        // Go duplicate files
        let go_code = r#"
            package main
            import "fmt"
            func CalculateMetric(x int, y int) int {
                res := x * y + 42
                fmt.Printf("Result: %d\n", res)
                return res
            }
        "#;
        write_test_file(dir.path().join("metric1.go"), go_code);
        write_test_file(dir.path().join("metric2.go"), go_code);

        // Java duplicate files
        let java_code = r#"
            public class Processor {
                public int computeBonus(int salary, int tenure) {
                    int bonus = salary * tenure / 100;
                    System.out.println("Bonus: " + bonus);
                    return bonus;
                }
            }
        "#;
        write_test_file(dir.path().join("Proc1.java"), java_code);
        write_test_file(dir.path().join("Proc2.java"), java_code);

        // Zig duplicate files
        let zig_code = r#"
            const std = @import("std");
            pub fn computeSum(a: i32, b: i32, c: i32, d: i32) i32 {
                const total = a + b + c + d + 100;
                std.debug.print("Calculated sum: {d}\n", .{total});
                return total * 2;
            }
        "#;
        write_test_file(dir.path().join("a.zig"), zig_code);
        write_test_file(dir.path().join("b.zig"), zig_code);

        // Scala duplicate files
        let scala_code = r#"
            object Helper {
                def processData(input: String, prefix: String, suffix: String): String = {
                    val formatted = prefix + "_" + input.trim.toUpperCase + "_" + suffix
                    println(s"Processing string payload: $formatted")
                    formatted + "_PROCESSED"
                }
            }
        "#;
        write_test_file(dir.path().join("a.scala"), scala_code);
        write_test_file(dir.path().join("b.scala"), scala_code);

        // Elixir duplicate files
        let elixir_code = r#"
            defmodule Calculator do
                def multiply_and_add(a, b, c, d) do
                    sum = a + b + c + d
                    result = sum * 2 + 42
                    IO.puts("Result calculation computed: #{result}")
                    result
                end
            end
        "#;
        write_test_file(dir.path().join("calc1.ex"), elixir_code);
        write_test_file(dir.path().join("calc2.ex"), elixir_code);

        // SQL duplicate files
        let sql_code = r#"
            SELECT u.id, u.username, u.email, COUNT(p.id) as post_count, SUM(p.views) as total_views
            FROM users u
            INNER JOIN posts p ON u.id = p.user_id
            WHERE u.active = 1 AND u.created_at >= '2026-01-01'
            GROUP BY u.id, u.username, u.email
            HAVING post_count > 5 AND total_views > 1000
            ORDER BY post_count DESC, total_views DESC;
        "#;
        write_test_file(dir.path().join("query1.sql"), sql_code);
        write_test_file(dir.path().join("query2.sql"), sql_code);

        let config = make_test_config(&dir.path().to_string_lossy(), 15);
        let result = run_test_scan(config).await.unwrap();

        assert_eq!(result.total_files, 12);
        assert!(result.total_clones >= 6);
        for lang in &["Go", "Java", "Zig", "Scala", "Elixir", "SQL"] {
            assert!(
                result
                    .language_breakdown
                    .iter()
                    .any(|l| &l.language == lang)
            );
        }
    }

    #[tokio::test]
    async fn test_scan_with_policy_engine() {
        use tempfile::tempdir;
        let dir = tempdir().unwrap();

        std::fs::create_dir_all(dir.path().join("src/domain")).unwrap();
        std::fs::create_dir_all(dir.path().join("src/presentation")).unwrap();
        std::fs::create_dir_all(dir.path().join("src/auth")).unwrap();

        let code = r#"
            pub fn validate_credentials(user: &str, pass: &str) -> bool {
                let valid = user.len() > 3 && pass.len() > 8;
                valid
            }
        "#;
        write_test_file(dir.path().join("src/domain/user.rs"), code);
        write_test_file(dir.path().join("src/presentation/user.rs"), code);
        write_test_file(dir.path().join("src/auth/auth_helper.rs"), code);

        let rules_toml = r#"
[[boundaries]]
name = "domain-isolation"
source = "src/domain/**"
forbidden_targets = ["src/presentation/**"]
severity = "error"

[[zero_duplication]]
name = "auth-protection"
pattern = "src/auth/**"
severity = "error"
"#;
        write_test_file(dir.path().join(".cddmrules.toml"), rules_toml);

        let config = make_test_config(&dir.path().to_string_lossy(), 10);
        let result = run_test_scan(config).await.unwrap();

        assert!(result.total_clones >= 1);
        assert!(!result.policy_violations.is_empty());
        assert!(
            result
                .policy_violations
                .iter()
                .any(|v| v.rule_name == "domain-isolation")
        );
        assert!(
            result
                .policy_violations
                .iter()
                .any(|v| v.rule_name == "auth-protection")
        );
    }
}
