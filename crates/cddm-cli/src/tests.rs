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
        target_directory: None,
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
    let console = format_console_report(&result);
    let md = format_markdown_report(&result);
    assert!(console.contains("CDDM — Code De-Duplication Meister Report"));
    assert!(md.contains("# CDDM Duplicate Code Scan Report"));
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

    let console = format_diff_console_report(&diff_result);
    let md = format_diff_markdown_report(&diff_result);
    assert!(console.contains("Differential Report"));
    assert!(md.contains("# CDDM Differential Scan Report"));
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
        Commands::Lsp(args) => assert_eq!(args.min_tokens, 40),
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
        Commands::Monorepo(args) => assert_eq!(args.min_tokens, 30),
        _ => panic!("expected Monorepo command"),
    }

    let cli_coverage = Cli::try_parse_from([
        "cddm",
        "coverage",
        "--report",
        "lcov.info",
        "--dead-code-only",
        "--min-hits",
        "10",
    ])
    .expect("parse coverage");
    match cli_coverage.command {
        Commands::Coverage(args) => {
            assert_eq!(args.report, PathBuf::from("lcov.info"));
            assert!(args.dead_code_only);
            assert_eq!(args.min_hits, 10);
        }
        _ => panic!("expected Coverage command"),
    }

    let cli_neural_semantic =
        Cli::try_parse_from(["cddm", "semantic", "--neural", "--neural-threshold", "0.92"])
            .expect("parse semantic neural");
    match cli_neural_semantic.command {
        Commands::Semantic(args) => {
            assert!(args.neural);
            assert!((args.neural_threshold - 0.92).abs() < 1e-4);
        }
        _ => panic!("expected Semantic command"),
    }

    let cli_scan_summary =
        Cli::try_parse_from(["cddm", "scan", "--summary"]).expect("parse scan summary");
    match cli_scan_summary.command {
        Commands::Scan(args) => assert!(args.summary),
        _ => panic!("expected Scan command"),
    }

    let cli_dead_summary =
        Cli::try_parse_from(["cddm", "dead", "--summary"]).expect("parse dead summary");
    match cli_dead_summary.command {
        Commands::DeadCode(args) => assert!(args.summary),
        _ => panic!("expected DeadCode command"),
    }

    let cli_rules_check_summary = Cli::try_parse_from(["cddm", "rules", "check", "--summary"])
        .expect("parse rules check summary");
    match cli_rules_check_summary.command {
        Commands::Rules { action } => match action {
            RulesAction::Check { summary, .. } => assert!(summary),
            _ => panic!("expected RulesAction::Check"),
        },
        _ => panic!("expected Rules command"),
    }

    let cli_rules_check_no_ignore_tests = Cli::try_parse_from([
        "cddm",
        "rules",
        "check",
        "--enforce-policies",
        "--no-ignore-tests",
        "--no-ignore-mocks",
        "--include-ignored",
    ])
    .expect("parse rules check no-ignore-tests");
    match cli_rules_check_no_ignore_tests.command {
        Commands::Rules { action } => match action {
            RulesAction::Check {
                enforce_policies,
                no_ignore_tests,
                no_ignore_mocks,
                include_ignored,
                ..
            } => {
                assert!(enforce_policies);
                assert!(no_ignore_tests);
                assert!(no_ignore_mocks);
                assert!(include_ignored);
            }
            _ => panic!("expected RulesAction::Check"),
        },
        _ => panic!("expected Rules command"),
    }

    let cli_scan_no_ignore =
        Cli::try_parse_from(["cddm", "scan", "--no-ignore"]).expect("parse scan no-ignore");
    match cli_scan_no_ignore.command {
        Commands::Scan(args) => assert!(args.include_ignored),
        _ => panic!("expected Scan command"),
    }

    let cli_scan_include_ignored = Cli::try_parse_from(["cddm", "scan", "--include-ignored"])
        .expect("parse scan include-ignored");
    match cli_scan_include_ignored.command {
        Commands::Scan(args) => assert!(args.include_ignored),
        _ => panic!("expected Scan command"),
    }

    let cli_diff_no_ignore =
        Cli::try_parse_from(["cddm", "diff", "main", "--no-ignore"]).expect("parse diff no-ignore");
    match cli_diff_no_ignore.command {
        Commands::Diff(args) => assert!(args.include_ignored),
        _ => panic!("expected Diff command"),
    }

    let cli_dead_no_ignore =
        Cli::try_parse_from(["cddm", "dead", "--no-ignore"]).expect("parse dead no-ignore");
    match cli_dead_no_ignore.command {
        Commands::DeadCode(args) => assert!(args.include_ignored),
        _ => panic!("expected DeadCode command"),
    }

    let cli_completions =
        Cli::try_parse_from(["cddm", "completions", "bash"]).expect("parse completions bash");
    match cli_completions.command {
        Commands::Completions(args) => assert_eq!(args.shell, clap_complete::Shell::Bash),
        _ => panic!("expected Completions command"),
    }

    let cli_scan_no_ignore_tests = Cli::try_parse_from([
        "cddm",
        "scan",
        "tests",
        "--no-ignore-tests",
        "--no-ignore-mocks",
    ])
    .expect("parse scan no-ignore-tests");
    match cli_scan_no_ignore_tests.command {
        Commands::Scan(args) => {
            assert!(args.no_ignore_tests);
            assert!(args.no_ignore_mocks);
        }
        _ => panic!("expected Scan command"),
    }

    let cli_diff_no_ignore_tests = Cli::try_parse_from([
        "cddm",
        "diff",
        "main",
        "--no-ignore-tests",
        "--no-ignore-mocks",
    ])
    .expect("parse diff no-ignore-tests");
    match cli_diff_no_ignore_tests.command {
        Commands::Diff(args) => {
            assert!(args.no_ignore_tests);
            assert!(args.no_ignore_mocks);
        }
        _ => panic!("expected Diff command"),
    }

    let cli_scan_output = Cli::try_parse_from([
        "cddm",
        "scan",
        ".",
        "--format",
        "sarif",
        "--output",
        "cddm-results.sarif",
    ])
    .expect("parse scan --output");
    match cli_scan_output.command {
        Commands::Scan(args) => {
            assert_eq!(args.format, OutputFormat::Sarif);
            assert_eq!(args.output, Some(PathBuf::from("cddm-results.sarif")));
        }
        _ => panic!("expected Scan command"),
    }

    let cli_diff_output = Cli::try_parse_from([
        "cddm",
        "diff",
        "main",
        "--format",
        "sarif",
        "--output",
        "results.sarif",
    ])
    .expect("parse diff --output");
    match cli_diff_output.command {
        Commands::Diff(args) => {
            assert_eq!(args.format, OutputFormat::Sarif);
            assert_eq!(args.output, Some(PathBuf::from("results.sarif")));
        }
        _ => panic!("expected Diff command"),
    }

    let cli_init_gitea = Cli::try_parse_from([
        "cddm",
        "init",
        "gitea",
        "--fail-threshold",
        "5.0",
        "--write",
    ])
    .expect("parse init gitea");
    match cli_init_gitea.command {
        Commands::Init(args) => {
            assert_eq!(args.platform, PlatformChoice::Gitea);
            assert!((args.fail_threshold - 5.0).abs() < f64::EPSILON);
            assert!(args.write);
        }
        _ => panic!("expected Init command"),
    }
}

#[test]
fn test_emit_report_output_writes_file_and_creates_parents() {
    let temp = tempfile::tempdir().expect("tempdir");
    let nested_path = temp.path().join("reports").join("scan.sarif");
    emit_report_output(
        "{\"version\":\"2.1.0\"}\n",
        Some(&nested_path),
        "Scan report",
    )
    .expect("emit report to nested path");
    let written = std::fs::read_to_string(&nested_path).expect("read written report");
    assert_eq!(written, "{\"version\":\"2.1.0\"}\n");
}

#[test]
fn test_cli_parses_quality_gate_and_dead_code_flags() {
    let scan_cli = Cli::try_parse_from(["cddm", "scan", ".", "--min-dry-score", "90.5"])
        .expect("parse scan --min-dry-score");
    match scan_cli.command {
        Commands::Scan(args) => {
            assert_eq!(args.min_dry_score, Some(90.5));
        }
        _ => panic!("expected Scan command"),
    }

    let dc_cli = Cli::try_parse_from([
        "cddm",
        "dead-code",
        ".",
        "--fail-on-findings",
        "--max-items",
        "3",
        "--rules",
        "custom.toml",
    ])
    .expect("parse dead-code flags");
    match dc_cli.command {
        Commands::DeadCode(args) => {
            assert!(args.fail_on_findings);
            assert_eq!(args.max_items, Some(3));
            assert_eq!(args.rules, Some(PathBuf::from("custom.toml")));
        }
        _ => panic!("expected DeadCode command"),
    }
}
