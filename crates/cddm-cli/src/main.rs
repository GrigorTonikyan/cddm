#![forbid(unsafe_code)]

use cddm_core::{
    CloneStatus, DiffScanResult, ScanConfig, ScanResult, analyze_clone_refactoring, run_diff_scan,
    run_scan,
};
use clap::{Parser, Subcommand, ValueEnum};
use comfy_table::{Cell, Color, Table};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(
    name = "cddm",
    author = "Grigor Tonikyan",
    version,
    about = "CDDM — Code De-Duplication Meister: High-Performance Polyglot Code Clone & \
             Modularity Analyzer",
    long_about = "CDDM analyzes codebases for duplicate code fragments, evaluates DRY health \
                  scores, and generates actionable structural reports."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

mod serve;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan target directory for code duplication & DRY health score
    Scan {
        /// Directory path to scan (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Output report format (console, json, markdown, sarif)
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
        format: OutputFormat,

        /// Exit with non-zero status code if duplication percentage exceeds threshold (0-100)
        #[arg(long)]
        fail_threshold: Option<f64>,

        /// Specific language(s) to scan (e.g. Rust, TypeScript, Python)
        #[arg(short, long)]
        languages: Vec<String>,

        /// Glob patterns to ignore (e.g. node_modules, target)
        #[arg(short, long)]
        ignore: Vec<String>,

        /// Enable in-process git blame author & line age annotation
        #[arg(long, default_value_t = false)]
        git_blame: bool,

        /// Custom path for persistent redb cache database (default: .cddm/cache.db)
        #[arg(long)]
        cache_dir: Option<PathBuf>,

        /// Bypass persistent disk cache and force full re-scan
        #[arg(long, default_value_t = false)]
        no_cache: bool,

        /// Clear existing persistent cache database before scanning
        #[arg(long, default_value_t = false)]
        clear_cache: bool,
    },

    /// Differential duplication scan comparing current changes against a Git base revision
    Diff {
        /// Base Git revision to compare against (e.g. main, origin/main, HEAD~1)
        base_ref: String,

        /// Target Git revision (default: working directory / HEAD)
        target_ref: Option<String>,

        /// Directory path of the Git repository to scan (default: current directory)
        #[arg(short, long, default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Output report format (console, json, markdown)
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
        format: OutputFormat,

        /// Exit with non-zero status code if new clones are introduced or exceed threshold
        #[arg(long)]
        fail_threshold: Option<f64>,

        /// Specific language(s) to scan
        #[arg(short, long)]
        languages: Vec<String>,

        /// Glob patterns to ignore
        #[arg(short, long)]
        ignore: Vec<String>,

        /// Enable in-process git blame annotation
        #[arg(long, default_value_t = false)]
        git_blame: bool,

        /// Custom path for persistent redb cache database
        #[arg(long)]
        cache_dir: Option<PathBuf>,

        /// Bypass persistent disk cache
        #[arg(long, default_value_t = false)]
        no_cache: bool,
    },

    /// Generate automated refactoring patch recommendations for duplicate code
    Refactor {
        /// Target clone pair 1-based index from scan report (default: 1)
        #[arg(short, long, default_value_t = 1)]
        pair: usize,

        /// Directory path to scan (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Write generated unified patch to specified output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Specific language(s) to scan
        #[arg(short, long)]
        languages: Vec<String>,

        /// Glob patterns to ignore
        #[arg(short, long)]
        ignore: Vec<String>,
    },

    /// Launch interactive WebUI HTTP server with embedded React app
    Serve {
        /// Port to bind WebUI HTTP server to (default: 3000)
        #[arg(short, long, default_value_t = serve::DEFAULT_PORT)]
        port: u16,

        /// Automatically open browser tab
        #[arg(short, long, default_value_t = false)]
        open: bool,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum OutputFormat {
    Console,
    Json,
    Markdown,
    Sarif,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            directory,
            min_tokens,
            format,
            fail_threshold,
            languages,
            ignore,
            git_blame,
            cache_dir,
            no_cache,
            clear_cache,
        } => {
            let cache_path = cache_dir.as_ref().map(|p| p.to_string_lossy().to_string());

            if clear_cache {
                let path_to_clear = cache_dir
                    .clone()
                    .unwrap_or_else(|| directory.join(cddm_core::DEFAULT_CACHE_FILE));
                if path_to_clear.exists() {
                    let _ = fs::remove_file(&path_to_clear);
                    eprintln!("Cleared cache database at '{}'", path_to_clear.display());
                }
            }

            let config = ScanConfig {
                directory: directory.to_string_lossy().to_string(),
                min_tokens,
                languages,
                ignore_patterns: if ignore.is_empty() {
                    ScanConfig::default().ignore_patterns
                } else {
                    ignore
                },
                detect_type2: true,
                scan_self: true,
                enable_git_blame: git_blame,
                cache_dir: cache_path,
                enable_cache: !no_cache,
            };

            let (tx, mut rx) = mpsc::channel::<cddm_core::ScanProgress>(100);
            let cancel_flag = Arc::new(AtomicBool::new(false));

            if format == OutputFormat::Console {
                tokio::spawn(async move {
                    while let Some(progress) = rx.recv().await {
                        eprintln!(
                            "[{}] {}% - {}",
                            progress.phase,
                            (progress.progress * 100.0) as u32,
                            progress.message
                        );
                    }
                });
            }

            let result = run_scan(config, tx, cancel_flag).await?;

            match format {
                OutputFormat::Console => print_console_report(&result),
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                OutputFormat::Markdown => print_markdown_report(&result),
                OutputFormat::Sarif => print_sarif_report(&result)?,
            }

            if let Some(threshold) = fail_threshold
                && result.duplication_percentage > threshold
            {
                eprintln!(
                    "Error: Duplication percentage {:.2}% exceeds failure threshold {:.2}%",
                    result.duplication_percentage, threshold
                );
                std::process::exit(1);
            }
        }

        Commands::Diff {
            base_ref,
            target_ref,
            directory,
            min_tokens,
            format,
            fail_threshold,
            languages,
            ignore,
            git_blame,
            cache_dir,
            no_cache,
        } => {
            let cache_path = cache_dir.as_ref().map(|p| p.to_string_lossy().to_string());

            let config = ScanConfig {
                directory: directory.to_string_lossy().to_string(),
                min_tokens,
                languages,
                ignore_patterns: if ignore.is_empty() {
                    ScanConfig::default().ignore_patterns
                } else {
                    ignore
                },
                detect_type2: true,
                scan_self: true,
                enable_git_blame: git_blame,
                cache_dir: cache_path,
                enable_cache: !no_cache,
            };

            let (tx, mut rx) = mpsc::channel::<cddm_core::ScanProgress>(100);
            let cancel_flag = Arc::new(AtomicBool::new(false));

            if format == OutputFormat::Console {
                tokio::spawn(async move {
                    while let Some(progress) = rx.recv().await {
                        eprintln!(
                            "[{}] {}% - {}",
                            progress.phase,
                            (progress.progress * 100.0) as u32,
                            progress.message
                        );
                    }
                });
            }

            let diff_result =
                run_diff_scan(&base_ref, target_ref.as_deref(), config, tx, cancel_flag).await?;

            match format {
                OutputFormat::Console => print_diff_console_report(&diff_result),
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&diff_result)?);
                }
                OutputFormat::Markdown => print_diff_markdown_report(&diff_result),
                OutputFormat::Sarif => {
                    eprintln!("Warning: SARIF format for diff scanning falls back to JSON");
                    println!("{}", serde_json::to_string_pretty(&diff_result)?);
                }
            }

            if let Some(threshold) = fail_threshold {
                if (diff_result.summary.new_clones as f64) > threshold {
                    eprintln!(
                        "Error: Introduced {} new clones, exceeding failure threshold of {:.0}",
                        diff_result.summary.new_clones, threshold
                    );
                    std::process::exit(1);
                }
            } else if diff_result.summary.new_clones > 0 {
                eprintln!(
                    "Notice: {} new clone pairs introduced in this changeset.",
                    diff_result.summary.new_clones
                );
            }
        }

        Commands::Refactor {
            pair,
            directory,
            min_tokens,
            output,
            languages,
            ignore,
        } => {
            let config = ScanConfig {
                directory: directory.to_string_lossy().to_string(),
                min_tokens,
                languages,
                ignore_patterns: if ignore.is_empty() {
                    ScanConfig::default().ignore_patterns
                } else {
                    ignore
                },
                detect_type2: true,
                scan_self: true,
                enable_git_blame: false,
                cache_dir: None,
                enable_cache: true,
            };

            let (tx, _rx) = mpsc::channel(100);
            let cancel_flag = Arc::new(AtomicBool::new(false));

            let result = run_scan(config, tx, cancel_flag).await?;

            if result.clone_pairs.is_empty() {
                println!("No duplicate code clone pairs found to refactor.");
                return Ok(());
            }

            let target_idx = if pair > 0 && pair <= result.clone_pairs.len() {
                pair - 1
            } else {
                eprintln!(
                    "Warning: Specified pair index {} out of range (total: {}); defaulting to 1.",
                    pair,
                    result.clone_pairs.len()
                );
                0
            };

            let selected = &result.clone_pairs[target_idx];
            let suggestion = analyze_clone_refactoring(
                &selected.file_a,
                (selected.start_line_a, selected.end_line_a),
                &selected.file_b,
                (selected.start_line_b, selected.end_line_b),
            )?;

            print_refactor_recommendation(selected, &suggestion);

            if let Some(out_path) = output {
                fs::write(&out_path, &suggestion.unified_patch)?;
                println!("\nUnified patch written to '{}'.", out_path.display());
            }
        }

        Commands::Serve { port, open } => {
            serve::start_server(port, open).await?;
        }
    }

    Ok(())
}

fn scan_metrics_summary(result: &ScanResult) -> [(&'static str, String); 7] {
    [
        ("Scan ID", result.scan_id.clone()),
        ("Total Files", result.total_files.to_string()),
        ("Total Tokens", result.total_tokens.to_string()),
        ("Total Clone Pairs", result.total_clones.to_string()),
        (
            "Duplication Rate",
            format!("{:.2}%", result.duplication_percentage),
        ),
        (
            "DRY Health Score",
            format!("{:.1} / 100.0", result.dry_health_score),
        ),
        ("Scan Duration", format!("{} ms", result.duration_ms)),
    ]
}

fn print_console_report(result: &ScanResult) {
    println!("\n=== CDDM — Code De-Duplication Meister Report ===");
    for (k, v) in scan_metrics_summary(result) {
        println!("{:<18} {}", format!("{}:", k), v);
    }
    println!();

    if !result.clone_pairs.is_empty() {
        let mut table = Table::new();
        table.set_header(vec![
            Cell::new("File A"),
            Cell::new("Lines A"),
            Cell::new("File B"),
            Cell::new("Lines B"),
            Cell::new("Tokens"),
            Cell::new("Similarity"),
        ]);

        for pair in result.clone_pairs.iter().take(25) {
            table.add_row(vec![
                Cell::new(&pair.file_a),
                Cell::new(format!("{}-{}", pair.start_line_a, pair.end_line_a)),
                Cell::new(&pair.file_b),
                Cell::new(format!("{}-{}", pair.start_line_b, pair.end_line_b)),
                Cell::new(pair.token_count),
                Cell::new(format!("{:.1}%", pair.similarity * 100.0)).fg(Color::Yellow),
            ]);
        }

        println!("{}", table);
        if result.clone_pairs.len() > 25 {
            println!(
                "... and {} more clone pairs.",
                result.clone_pairs.len() - 25
            );
        }
    } else {
        println!("Zero code duplication detected!");
    }
}

fn print_markdown_report(result: &ScanResult) {
    println!("# CDDM Duplicate Code Scan Report\n");
    for (k, v) in scan_metrics_summary(result) {
        println!("- **{}**: `{}`", k, v);
    }
    println!();

    if !result.clone_pairs.is_empty() {
        println!("| File A | Lines A | File B | Lines B | Tokens | Similarity |");
        println!("| :--- | :--- | :--- | :--- | :--- | :--- |");
        for pair in &result.clone_pairs {
            println!(
                "| `{}` | {}-{} | `{}` | {}-{} | {} | {:.1}% |",
                pair.file_a,
                pair.start_line_a,
                pair.end_line_a,
                pair.file_b,
                pair.start_line_b,
                pair.end_line_b,
                pair.token_count,
                pair.similarity * 100.0
            );
        }
    } else {
        println!("Zero code duplication detected!");
    }
}

fn print_sarif_report(result: &ScanResult) -> Result<(), Box<dyn std::error::Error>> {
    let sarif_json = cddm_core::generate_sarif_json(result);
    println!("{}", serde_json::to_string_pretty(&sarif_json)?);
    Ok(())
}

fn print_diff_console_report(diff_result: &DiffScanResult) {
    let sum = &diff_result.summary;
    println!("\n=== CDDM — Code De-Duplication Meister Differential Report ===");
    println!("{:<22} {}", "Base Reference:", sum.base_ref);
    println!("{:<22} {}", "Target Reference:", sum.target_ref);
    println!(
        "{:<22} {:.1} / 100.0",
        "Baseline DRY Score:", sum.base_dry_score
    );
    println!(
        "{:<22} {:.1} / 100.0",
        "Target DRY Score:", sum.target_dry_score
    );
    let delta_str = if sum.net_dry_delta >= 0.0 {
        format!("+{:.2}% (Improved)", sum.net_dry_delta)
    } else {
        format!("{:.2}% (Regressed)", sum.net_dry_delta)
    };
    println!("{:<22} {}", "Net DRY Delta:", delta_str);
    println!("{:<22} {}", "Changed Files:", sum.total_changed_files);
    println!("{:<22} {}", "New Clones:", sum.new_clones);
    println!("{:<22} {}", "Legacy Clones:", sum.legacy_clones);
    println!("{:<22} {}", "Resolved Clones:", sum.resolved_clones);
    println!("{:<22} {} ms", "Duration:", diff_result.duration_ms);
    println!();

    if !diff_result.diff_clones.is_empty() {
        let mut table = Table::new();
        table.set_header(vec![
            Cell::new("Status"),
            Cell::new("File A"),
            Cell::new("Lines A"),
            Cell::new("File B"),
            Cell::new("Lines B"),
            Cell::new("Tokens"),
            Cell::new("Similarity"),
        ]);

        for item in diff_result.diff_clones.iter().take(25) {
            let pair = &item.clone_pair;
            let status_cell = match item.status {
                CloneStatus::New => Cell::new("NEW").fg(Color::Red),
                CloneStatus::Legacy => Cell::new("LEGACY").fg(Color::Blue),
                CloneStatus::Resolved => Cell::new("RESOLVED").fg(Color::Green),
            };

            table.add_row(vec![
                status_cell,
                Cell::new(&pair.file_a),
                Cell::new(format!("{}-{}", pair.start_line_a, pair.end_line_a)),
                Cell::new(&pair.file_b),
                Cell::new(format!("{}-{}", pair.start_line_b, pair.end_line_b)),
                Cell::new(pair.token_count),
                Cell::new(format!("{:.1}%", pair.similarity * 100.0)).fg(Color::Yellow),
            ]);
        }

        println!("{}", table);
    } else {
        println!("No clone pairs present in compared changeset.");
    }
}

fn print_diff_markdown_report(diff_result: &DiffScanResult) {
    let sum = &diff_result.summary;
    println!("# CDDM Differential Scan Report\n");
    println!("- **Base Reference**: `{}`", sum.base_ref);
    println!("- **Target Reference**: `{}`", sum.target_ref);
    println!("- **Baseline DRY Score**: `{:.1}`", sum.base_dry_score);
    println!("- **Target DRY Score**: `{:.1}`", sum.target_dry_score);
    println!("- **Net DRY Delta**: `{:.2}%`", sum.net_dry_delta);
    println!("- **New Clones**: `{}`", sum.new_clones);
    println!("- **Legacy Clones**: `{}`", sum.legacy_clones);
    println!();

    if !diff_result.diff_clones.is_empty() {
        println!("| Status | File A | Lines A | File B | Lines B | Tokens |");
        println!("| :--- | :--- | :--- | :--- | :--- | :--- |");
        for item in &diff_result.diff_clones {
            let pair = &item.clone_pair;
            println!(
                "| `{}` | `{}` | {}-{} | `{}` | {}-{} | {} |",
                item.status,
                pair.file_a,
                pair.start_line_a,
                pair.end_line_a,
                pair.file_b,
                pair.start_line_b,
                pair.end_line_b,
                pair.token_count
            );
        }
    }
}

fn print_refactor_recommendation(
    selected: &cddm_core::ClonePair,
    suggestion: &cddm_core::RefactorSuggestion,
) {
    println!("\n=== CDDM Automated Refactoring Advisor ===");
    println!(
        "{:<24} {}:{}-{}",
        "Target Fragment A:", selected.file_a, selected.start_line_a, selected.end_line_a
    );
    println!(
        "{:<24} {}:{}-{}",
        "Target Fragment B:", selected.file_b, selected.start_line_b, selected.end_line_b
    );
    println!("{:<24} {}", "Refactoring Strategy:", suggestion.strategy);
    println!(
        "{:<24} {}",
        "Suggested Helper:", suggestion.suggested_function_name
    );
    println!("{:<24} {}", "Target Module:", suggestion.target_module_hint);
    println!(
        "{:<24} {}",
        "Estimated Lines Saved:", suggestion.lines_saved
    );
    println!("\n--- Generated Unified Patch Preview ---\n");
    println!("{}", suggestion.unified_patch);
}

#[cfg(test)]
mod tests {
    use super::*;
    use cddm_core::{
        ClonePair, CloneStatus, CloneType, DiffClonePair, DiffScanResult, DiffSummary,
        LanguageStats, ScanResult,
    };

    fn make_test_result() -> ScanResult {
        ScanResult {
            scan_id: "test-scan-cli".to_string(),
            total_files: 3,
            total_tokens: 500,
            total_clones: 1,
            duplication_percentage: 10.0,
            dry_health_score: 90.0,
            clone_pairs: vec![ClonePair {
                file_a: "src/a.rs".to_string(),
                start_line_a: 1,
                end_line_a: 10,
                file_b: "src/b.rs".to_string(),
                start_line_b: 1,
                end_line_b: 10,
                token_count: 50,
                similarity: 1.0,
                fragment_hash: "hash_cli".to_string(),
                clone_type: CloneType::Exact,
                author_a: None,
                author_b: None,
            }],
            duration_ms: 15,
            language_breakdown: vec![LanguageStats {
                language: "Rust".to_string(),
                files: 3,
                tokens: 500,
                clones: 1,
            }],
        }
    }

    #[test]
    fn test_output_format_variants() {
        assert_eq!(OutputFormat::Console, OutputFormat::Console);
        assert_ne!(OutputFormat::Json, OutputFormat::Sarif);
        assert_eq!(OutputFormat::Sarif, OutputFormat::Sarif);
        assert_eq!(OutputFormat::Markdown, OutputFormat::Markdown);
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
}
