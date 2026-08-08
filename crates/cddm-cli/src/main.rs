use cddm_core::{ScanConfig, ScanResult, run_scan};
use clap::{Parser, Subcommand, ValueEnum};
use comfy_table::{Cell, Color, Table};
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(
    name = "cddm",
    author = "Grigor Tonikyan",
    version = "0.1.0",
    about = "CDDM — Code De-Duplication Meister: High-Performance Polyglot Code Clone & Modularity Analyzer",
    long_about = "CDDM analyzes codebases for duplicate code fragments, evaluates DRY health scores, and generates actionable structural reports."
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
        #[arg(default_value = ".")]
        directory: PathBuf,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = 50)]
        min_tokens: usize,

        /// Output report format (console, json, markdown)
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
    },

    /// Launch interactive WebUI HTTP server with embedded React app
    Serve {
        /// Port to bind WebUI HTTP server to (default: 3000)
        #[arg(short, long, default_value_t = 3000)]
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
                enable_git_blame: git_blame,
            };

            let (tx, mut rx) = mpsc::channel::<cddm_core::ScanProgress>(100);
            let cancel_flag = Arc::new(AtomicBool::new(false));

            // Spawn progress printer for console mode
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
        Commands::Serve { port, open } => {
            serve::start_server(port, open).await?;
        }
    }

    Ok(())
}

fn print_console_report(result: &ScanResult) {
    println!("\n=== CDDM — Code De-Duplication Meister Report ===");
    println!("Scan ID:           {}", result.scan_id);
    println!("Total Files:       {}", result.total_files);
    println!("Total Tokens:      {}", result.total_tokens);
    println!("Total Clone Pairs: {}", result.total_clones);
    println!("Duplication:       {:.2}%", result.duplication_percentage);
    println!("DRY Health Score:  {:.1} / 100.0", result.dry_health_score);
    println!("Duration:          {} ms\n", result.duration_ms);

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
        println!("✔ Zero code duplication detected!");
    }
}

fn print_markdown_report(result: &ScanResult) {
    println!("# CDDM Duplicate Code Scan Report\n");
    println!("- **Scan ID**: `{}`", result.scan_id);
    println!("- **Total Files**: {}", result.total_files);
    println!("- **Total Tokens**: {}", result.total_tokens);
    println!("- **Total Clone Pairs**: {}", result.total_clones);
    println!(
        "- **Duplication Rate**: {:.2}%",
        result.duplication_percentage
    );
    println!(
        "- **DRY Health Score**: {:.1} / 100",
        result.dry_health_score
    );
    println!("- **Scan Duration**: {} ms\n", result.duration_ms);

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
        println!("✔ Zero code duplication detected!");
    }
}
