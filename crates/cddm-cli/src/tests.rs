#![forbid(unsafe_code)]

use super::*;
use crate::types::CacheAction;
use cddm_core::{
    CloneCluster, CloneLocation, ClonePair, CloneStatus, CloneType, DiffClonePair, DiffScanResult,
    DiffSummary, LanguageStats, ScanResult, TimelineSnapshot, TimelineTrend,
};
use formatters::*;
use std::path::PathBuf;

fn make_test_result() -> ScanResult {
    ScanResult {
        scan_id: "test-scan-id".to_string(),
        total_files: 3,
        total_tokens: 500,
        total_clones: 1,
        total_clusters: 1,
        duplication_percentage: 20.0,
        dry_health_score: 80.0,
        clone_pairs: vec![ClonePair {
            file_a: "src/a.rs".to_string(),
            start_line_a: 10,
            end_line_a: 20,
            file_b: "src/b.rs".to_string(),
            start_line_b: 30,
            end_line_b: 40,
            token_count: 50,
            similarity: 1.0,
            fragment_hash: "hash1".to_string(),
            clone_type: CloneType::Exact,
            author_a: None,
            author_b: None,
        }],
        clone_clusters: vec![CloneCluster {
            id: 1,
            clone_type: CloneType::Exact,
            token_count: 50,
            similarity: 1.0,
            fragment_hash: "hash1".to_string(),
            occurrences: vec![
                CloneLocation {
                    file: "src/a.rs".to_string(),
                    start_line: 10,
                    end_line: 20,
                    author: None,
                },
                CloneLocation {
                    file: "src/b.rs".to_string(),
                    start_line: 30,
                    end_line: 40,
                    author: None,
                },
            ],
        }],
        duration_ms: 15,
        language_breakdown: vec![LanguageStats {
            language: "Rust".to_string(),
            files: 3,
            tokens: 500,
            clones: 1,
        }],
        policy_violations: Vec::new(),
    }
}

#[test]
fn test_output_format_variants() {
    assert_eq!(OutputFormat::Console, OutputFormat::Console);
    assert_ne!(OutputFormat::Json, OutputFormat::Sarif);
    assert_eq!(OutputFormat::Sarif, OutputFormat::Sarif);
    assert_eq!(OutputFormat::Markdown, OutputFormat::Markdown);
    assert_eq!(OutputFormat::Ndjson, OutputFormat::Ndjson);
}

#[test]
fn test_print_sarif_report_succeeds() {
    let result = make_test_result();
    let res = print_sarif_report(&result);
    assert!(res.is_ok());
}

#[test]
fn test_print_console_and_markdown_reports() {
    let result = make_test_result();
    print_console_report(&result);
    print_markdown_report(&result);
}

#[test]
fn test_print_diff_reports() {
    let diff_result = DiffScanResult {
        scan_id: "test-diff".to_string(),
        summary: DiffSummary {
            base_ref: "main".to_string(),
            target_ref: "HEAD".to_string(),
            base_dry_score: 90.0,
            target_dry_score: 95.0,
            net_dry_delta: 5.0,
            total_changed_files: 2,
            new_clones: 1,
            legacy_clones: 1,
            resolved_clones: 0,
        },
        diff_clones: vec![
            DiffClonePair {
                clone_pair: make_test_result().clone_pairs[0].clone(),
                status: CloneStatus::New,
            },
            DiffClonePair {
                clone_pair: make_test_result().clone_pairs[0].clone(),
                status: CloneStatus::Legacy,
            },
        ],
        duration_ms: 25,
    };

    print_diff_console_report(&diff_result);
    print_diff_markdown_report(&diff_result);
}

#[test]
fn test_print_trend_reports() {
    let trend = TimelineTrend {
        snapshots: vec![
            TimelineSnapshot {
                commit_hash: "1111111111111111111111111111111111111111".to_string(),
                short_hash: "1111111".to_string(),
                author: "Tester".to_string(),
                commit_time: 1700000000,
                formatted_date: "2026-08-20 10:00:00".to_string(),
                message: "initial commit".to_string(),
                tag: Some("v1.0.0".to_string()),
                total_files: 10,
                total_tokens: 1000,
                total_clones: 5,
                total_clusters: 2,
                duplication_percentage: 10.0,
                dry_health_score: 85.0,
            },
            TimelineSnapshot {
                commit_hash: "2222222222222222222222222222222222222222".to_string(),
                short_hash: "2222222".to_string(),
                author: "Tester".to_string(),
                commit_time: 1700100000,
                formatted_date: "2026-08-24 10:00:00".to_string(),
                message: "refactor duplicates".to_string(),
                tag: None,
                total_files: 10,
                total_tokens: 950,
                total_clones: 1,
                total_clusters: 1,
                duplication_percentage: 2.0,
                dry_health_score: 97.0,
            },
        ],
        initial_score: 85.0,
        current_score: 97.0,
        score_delta: 12.0,
        duplication_delta: -8.0,
        churn_hotspots: vec![],
    };

    print_trend_console_report(&trend);
    print_trend_markdown_report(&trend);
}

#[test]
fn test_cli_subcommands_parsing() {
    let cli_lsp = Cli::try_parse_from(["cddm", "lsp", "--min-tokens", "40"]).expect("parse lsp");
    match cli_lsp.command {
        Commands::Lsp { min_tokens, .. } => assert_eq!(min_tokens, 40),
        _ => panic!("expected Lsp command"),
    }

    let cli_heal = Cli::try_parse_from(["cddm", "heal", "--cluster", "2", "--provider", "mock"])
        .expect("parse heal");
    match cli_heal.command {
        Commands::Heal(args) => {
            assert_eq!(args.cluster, Some(2));
            assert_eq!(args.provider, "mock");
        }
        _ => panic!("expected Heal command"),
    }

    let cli_extract = Cli::try_parse_from([
        "cddm",
        "extract",
        "--pair",
        "1",
        "--target",
        "crates/shared_utils",
        "--fn-name",
        "compute_sum",
        "--dry-run",
    ])
    .expect("parse extract");
    match cli_extract.command {
        Commands::Extract(args) => {
            assert_eq!(args.pair, Some(1));
            assert_eq!(args.target, "crates/shared_utils");
            assert_eq!(args.fn_name, Some("compute_sum".to_string()));
            assert!(args.dry_run);
        }
        _ => panic!("expected Extract command"),
    }

    let cli_cache_export =
        Cli::try_parse_from(["cddm", "cache", "export", "--output", "custom.cddmpack"])
            .expect("parse cache export");
    match cli_cache_export.command {
        Commands::Cache { action } => match action {
            CacheAction::Export { output, .. } => {
                assert_eq!(output, Some(PathBuf::from("custom.cddmpack")));
            }
            _ => panic!("expected CacheAction::Export"),
        },
        _ => panic!("expected Cache command"),
    }

    let cli_monorepo =
        Cli::try_parse_from(["cddm", "monorepo", "--min-tokens", "30"]).expect("parse monorepo");
    match cli_monorepo.command {
        Commands::Monorepo { min_tokens, .. } => assert_eq!(min_tokens, 30),
        _ => panic!("expected Monorepo command"),
    }
}
