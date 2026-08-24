#![forbid(unsafe_code)]

use cddm_core::{
    CloneCluster, CloneStatus, DiffScanResult, ScanConfig, ScanResult, analyze_clone_refactoring,
    analyze_cluster_refactoring, refactor::ClusterRefactorSuggestion, run_diff_scan, run_scan,
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

        /// Target clone cluster 1-based index to synthesize a multi-site refactoring patch
        #[arg(short, long)]
        cluster: Option<usize>,

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

    /// Continuously watch directory for changes and run real-time incremental duplication scans
    Watch {
        /// Directory path to watch (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

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

        /// Debounce interval in milliseconds (default: 250)
        #[arg(long, default_value_t = 250)]
        debounce_ms: u64,

        /// Exit with non-zero status code if duplication percentage exceeds threshold
        #[arg(long)]
        fail_threshold: Option<f64>,
    },

    /// Start the CDDM Language Server Protocol (LSP) daemon over Stdio
    Lsp {
        /// Workspace root directory path (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,
    },

    /// Track historical duplication trends and DRY Health score trajectory across Git history
    Trend {
        /// Target Git repository directory path (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,

        /// Maximum number of historical commits to sample (default: 10)
        #[arg(short = 'n', long, default_value_t = 10)]
        max_samples: usize,

        /// Minimum token count to consider as duplicate clone
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Output report format (console, json, markdown)
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
        format: OutputFormat,
    },

    /// Manage local Git hooks (pre-commit / pre-push) for automated duplication gate enforcement
    Hook {
        /// Action to perform: install, uninstall, or status
        #[command(subcommand)]
        action: HookAction,
    },

    /// Generate turnkey CI/CD workflow configurations (GitHub Actions, GitLab CI, Azure Pipelines)
    Init {
        /// Target CI/CD platform: github, gitlab, or azure
        #[arg(value_enum, default_value_t = PlatformChoice::Github)]
        platform: PlatformChoice,

        /// Duplication percentage threshold to fail on (default: 15.0)
        #[arg(long, default_value_t = 15.0)]
        fail_threshold: f64,

        /// Minimum token count for clone detection (default: 50)
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Output file path (defaults to standard platform config file)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Write directly to disk (default: print to stdout unless --write or output specified)
        #[arg(short = 'w', long, default_value_t = false)]
        write: bool,
    },
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
enum HookAction {
    /// Install a Git hook enforcing code duplication thresholds
    Install {
        /// Hook type to install (pre-commit or pre-push)
        #[arg(short = 't', long, default_value = "pre-commit")]
        hook_type: String,

        /// Duplication percentage threshold to fail on (default: 15.0)
        #[arg(long, default_value_t = 15.0)]
        fail_threshold: f64,

        /// Minimum token count for clone detection (default: 50)
        #[arg(short, long, default_value_t = cddm_core::DEFAULT_MIN_TOKENS)]
        min_tokens: usize,

        /// Repository root directory path (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,
    },
    /// Uninstall an existing CDDM Git hook
    Uninstall {
        /// Hook type to remove (pre-commit or pre-push)
        #[arg(short = 't', long, default_value = "pre-commit")]
        hook_type: String,

        /// Repository root directory path (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,
    },
    /// Check current installation status of Git hooks
    Status {
        /// Repository root directory path (default: current directory)
        #[arg(default_value = cddm_core::DEFAULT_DIRECTORY)]
        directory: PathBuf,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum PlatformChoice {
    Github,
    Gitlab,
    Azure,
}

impl std::fmt::Display for PlatformChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Github => write!(f, "GitHub Actions"),
            Self::Gitlab => write!(f, "GitLab CI"),
            Self::Azure => write!(f, "Azure Pipelines"),
        }
    }
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
            cluster,
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

            if let Some(c_idx) = cluster {
                if result.clone_clusters.is_empty() {
                    println!("No duplicate code clone clusters found to refactor.");
                    return Ok(());
                }

                let target_idx = if c_idx > 0 && c_idx <= result.clone_clusters.len() {
                    c_idx - 1
                } else {
                    eprintln!(
                        "Warning: Specified cluster index {} out of range (total: {}); defaulting \
                         to 1.",
                        c_idx,
                        result.clone_clusters.len()
                    );
                    0
                };

                let selected_cluster = &result.clone_clusters[target_idx];
                let suggestion = analyze_cluster_refactoring(
                    &selected_cluster.id.to_string(),
                    &selected_cluster.occurrences,
                )?;

                print_cluster_refactor_recommendation(selected_cluster, &suggestion);

                if let Some(out_path) = output {
                    fs::write(&out_path, &suggestion.unified_patch)?;
                    println!(
                        "\nMulti-site unified patch written to '{}'.",
                        out_path.display()
                    );
                }
            } else {
                if result.clone_pairs.is_empty() {
                    println!("No duplicate code clone pairs found to refactor.");
                    return Ok(());
                }

                let target_idx = if pair > 0 && pair <= result.clone_pairs.len() {
                    pair - 1
                } else {
                    eprintln!(
                        "Warning: Specified pair index {} out of range (total: {}); defaulting to \
                         1.",
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
        }

        Commands::Serve { port, open } => {
            serve::start_server(port, open).await?;
        }

        Commands::Watch {
            directory,
            min_tokens,
            languages,
            ignore,
            git_blame,
            cache_dir,
            no_cache,
            debounce_ms,
            fail_threshold,
        } => {
            let cache_path = cache_dir.as_ref().map(|p| p.to_string_lossy().to_string());
            let ignore_patterns = if ignore.is_empty() {
                ScanConfig::default().ignore_patterns
            } else {
                ignore
            };

            let config = ScanConfig {
                directory: directory.to_string_lossy().to_string(),
                min_tokens,
                languages,
                ignore_patterns: ignore_patterns.clone(),
                detect_type2: true,
                scan_self: true,
                enable_git_blame: git_blame,
                cache_dir: cache_path,
                enable_cache: !no_cache,
            };

            println!(
                "CDDM Watcher active on '{}' (debounce: {}ms)",
                directory.display(),
                debounce_ms
            );
            println!("Performing initial baseline scan...\n");

            let (tx, _rx) = mpsc::channel(100);
            let cancel_flag = Arc::new(AtomicBool::new(false));
            let mut previous_result = match run_scan(config.clone(), tx, cancel_flag).await {
                Ok(res) => {
                    print_console_report(&res);
                    Some(res)
                }
                Err(err) => {
                    eprintln!("Initial scan failed: {}", err);
                    None
                }
            };

            let watcher = cddm_core::CddmWatcher::watch_directory(&directory)?;
            println!("\nWatching for workspace changes... Press Ctrl+C to exit.\n");

            let mut interval = tokio::time::interval(std::time::Duration::from_millis(debounce_ms));

            loop {
                interval.tick().await;
                let changed = watcher.collect_changed_paths(&ignore_patterns);
                if !changed.is_empty() {
                    let (tx_inc, _rx_inc) = mpsc::channel(100);
                    let cancel = Arc::new(AtomicBool::new(false));
                    let start = std::time::Instant::now();

                    match run_scan(config.clone(), tx_inc, cancel).await {
                        Ok(new_res) => {
                            let duration = start.elapsed().as_millis();
                            let score_delta = if let Some(ref prev) = previous_result {
                                new_res.dry_health_score - prev.dry_health_score
                            } else {
                                0.0
                            };

                            let delta_str = if score_delta > 0.0 {
                                format!("(+{:.1}%)", score_delta)
                            } else if score_delta < 0.0 {
                                format!("({:.1}%)", score_delta)
                            } else {
                                "(+0.0%)".to_string()
                            };

                            println!(
                                "[WATCH] {} file(s) modified | Scanned in {}ms | DRY Health: \
                                 {:.1}% {} | Clones: {} | Clusters: {}",
                                changed.len(),
                                duration,
                                new_res.dry_health_score,
                                delta_str,
                                new_res.total_clones,
                                new_res.total_clusters
                            );

                            if let Some(threshold) = fail_threshold
                                && new_res.duplication_percentage > threshold
                            {
                                eprintln!(
                                    "[WARN] Duplication {:.1}% exceeds failure threshold {:.1}%",
                                    new_res.duplication_percentage, threshold
                                );
                            }

                            previous_result = Some(new_res);
                        }
                        Err(err) => {
                            eprintln!("[WATCH ERROR] Incremental scan failed: {}", err);
                        }
                    }
                }
            }
        }

        Commands::Lsp {
            directory,
            min_tokens,
        } => {
            cddm_lsp::run_server_stdio(directory, min_tokens).await?;
        }

        Commands::Trend {
            directory,
            max_samples,
            min_tokens,
            format,
        } => {
            let cancel_flag = Arc::new(AtomicBool::new(false));
            match cddm_core::collect_git_timeline(&directory, max_samples, min_tokens, cancel_flag)
            {
                Ok(trend) => match format {
                    OutputFormat::Console => print_trend_console_report(&trend),
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&trend)?);
                    }
                    OutputFormat::Markdown => print_trend_markdown_report(&trend),
                    OutputFormat::Sarif => {
                        eprintln!(
                            "[WARN] SARIF format is not applicable for timeline trend. Outputting \
                             JSON."
                        );
                        println!("{}", serde_json::to_string_pretty(&trend)?);
                    }
                },
                Err(err) => {
                    eprintln!("[ERROR] Failed to collect Git timeline trend: {}", err);
                    std::process::exit(1);
                }
            }
        }

        Commands::Hook { action } => match action {
            HookAction::Install {
                hook_type,
                fail_threshold,
                min_tokens,
                directory,
            } => match cddm_core::install_git_hook(
                &directory,
                &hook_type,
                fail_threshold,
                min_tokens,
            ) {
                Ok(msg) => println!("[PASS] {}", msg),
                Err(err) => {
                    eprintln!("[ERROR] Failed to install hook: {}", err);
                    std::process::exit(1);
                }
            },
            HookAction::Uninstall {
                hook_type,
                directory,
            } => match cddm_core::uninstall_git_hook(&directory, &hook_type) {
                Ok(msg) => println!("[PASS] {}", msg),
                Err(err) => {
                    eprintln!("[ERROR] Failed to uninstall hook: {}", err);
                    std::process::exit(1);
                }
            },
            HookAction::Status { directory } => {
                let status = cddm_core::get_hook_status(&directory);
                println!("=== CDDM Git Hook Status ===");
                println!("Hooks Directory: {}", status.hooks_dir);
                println!(
                    "Pre-Commit Hook: {}",
                    if status.pre_commit_installed {
                        "[INSTALLED]"
                    } else {
                        "[NOT INSTALLED]"
                    }
                );
                println!(
                    "Pre-Push Hook:   {}",
                    if status.pre_push_installed {
                        "[INSTALLED]"
                    } else {
                        "[NOT INSTALLED]"
                    }
                );
            }
        },

        Commands::Init {
            platform,
            fail_threshold,
            min_tokens,
            output,
            write,
        } => {
            let content = match platform {
                PlatformChoice::Github => {
                    cddm_core::generate_github_workflow(fail_threshold, min_tokens)
                }
                PlatformChoice::Gitlab => cddm_core::generate_gitlab_ci(fail_threshold, min_tokens),
                PlatformChoice::Azure => {
                    cddm_core::generate_azure_pipelines(fail_threshold, min_tokens)
                }
            };

            let default_out_path = match platform {
                PlatformChoice::Github => PathBuf::from(".github/workflows/cddm.yml"),
                PlatformChoice::Gitlab => PathBuf::from(".gitlab-ci.yml"),
                PlatformChoice::Azure => PathBuf::from("azure-pipelines.yml"),
            };

            let target_path = output.unwrap_or(default_out_path);

            if write {
                if let Some(parent) = target_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&target_path, &content)?;
                println!(
                    "[PASS] Successfully generated {} configuration at '{}'",
                    platform,
                    target_path.display()
                );
            } else {
                println!("{}", content);
            }
        }
    }

    Ok(())
}

fn scan_metrics_summary(result: &ScanResult) -> [(&'static str, String); 8] {
    [
        ("Scan ID", result.scan_id.clone()),
        ("Total Files", result.total_files.to_string()),
        ("Total Tokens", result.total_tokens.to_string()),
        ("Total Clone Pairs", result.total_clones.to_string()),
        ("Total Clone Clusters", result.total_clusters.to_string()),
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
        println!("{:<22} {}", format!("{}:", k), v);
    }
    println!();

    if !result.clone_clusters.is_empty() {
        println!("--- Clone Clusters (N-way Equivalence Classes) ---");
        let mut cluster_table = Table::new();
        cluster_table.set_header(vec![
            Cell::new("Cluster"),
            Cell::new("Type"),
            Cell::new("Occurrences"),
            Cell::new("Tokens"),
            Cell::new("Similarity"),
            Cell::new("Locations"),
        ]);

        for cluster in result.clone_clusters.iter().take(20) {
            let locs_str = cluster
                .occurrences
                .iter()
                .map(|loc| format!("{}:{}-{}", loc.file, loc.start_line, loc.end_line))
                .collect::<Vec<_>>()
                .join(", ");
            let locs_truncated = if locs_str.len() > 55 {
                format!("{}...", &locs_str[..52])
            } else {
                locs_str
            };

            cluster_table.add_row(vec![
                Cell::new(format!("#{}", cluster.id)),
                Cell::new(format!("{:?}", cluster.clone_type)),
                Cell::new(cluster.occurrences.len()),
                Cell::new(cluster.token_count),
                Cell::new(format!("{:.1}%", cluster.similarity * 100.0)).fg(Color::Yellow),
                Cell::new(locs_truncated),
            ]);
        }
        println!("{}", cluster_table);
        if result.clone_clusters.len() > 20 {
            println!(
                "... and {} more clone clusters.",
                result.clone_clusters.len() - 20
            );
        }
        println!();
    }

    if !result.clone_pairs.is_empty() {
        println!("--- Pairwise Clones ---");
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

    if !result.clone_clusters.is_empty() {
        println!("### N-way Clone Clusters\n");
        println!("| Cluster | Type | Occurrences | Tokens | Similarity | Locations |");
        println!("| :--- | :--- | :--- | :--- | :--- | :--- |");
        for cluster in &result.clone_clusters {
            let locs_str = cluster
                .occurrences
                .iter()
                .map(|loc| format!("`{}`:{}-{}", loc.file, loc.start_line, loc.end_line))
                .collect::<Vec<_>>()
                .join("<br>");
            println!(
                "| `#{}` | `{:?}` | {} | {} | {:.1}% | {} |",
                cluster.id,
                cluster.clone_type,
                cluster.occurrences.len(),
                cluster.token_count,
                cluster.similarity * 100.0,
                locs_str
            );
        }
        println!();
    }

    if !result.clone_pairs.is_empty() {
        println!("### Pairwise Clones\n");
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

fn print_cluster_refactor_recommendation(
    cluster: &CloneCluster,
    suggestion: &ClusterRefactorSuggestion,
) {
    println!("\n=== CDDM — Multi-Site Cluster Refactoring Recommendation ===");
    println!("{:<24} Cluster #{}", "Cluster Target:", cluster.id);
    println!("{:<24} {:?}", "Clone Classification:", cluster.clone_type);
    println!(
        "{:<24} {} locations",
        "Total Occurrences:",
        cluster.occurrences.len()
    );
    println!("{:<24} {}", "Refactoring Strategy:", suggestion.strategy);
    println!(
        "{:<24} {}",
        "Suggested Helper:", suggestion.suggested_function_name
    );
    println!("{:<24} {}", "Target Module:", suggestion.target_module_hint);
    println!(
        "{:<24} {}",
        "Total Lines Saved:", suggestion.total_lines_saved
    );
    println!("\n--- Occurrence Sites ---");
    for (i, site) in suggestion.sites.iter().enumerate() {
        println!(
            "  Site {}: {}:{}-{}",
            i + 1,
            site.file,
            site.start_line,
            site.end_line
        );
    }
    println!("\n--- Generated Multi-File Unified Patch Preview ---\n");
    println!("{}", suggestion.unified_patch);
}

fn print_trend_console_report(trend: &cddm_core::TimelineTrend) {
    println!("\n=== CDDM Historical Duplication Trend & Timeline Evolution ===\n");

    let mut table = Table::new();
    table.set_header(vec![
        "Commit",
        "Date",
        "Author",
        "Message",
        "Files",
        "Tokens",
        "Clones",
        "Clusters",
        "Dup %",
        "DRY Score",
    ]);

    for s in &trend.snapshots {
        let tag_suffix = s
            .tag
            .as_ref()
            .map(|t| format!(" ({t})"))
            .unwrap_or_default();
        let commit_cell = format!("{}{tag_suffix}", s.short_hash);
        let score_color = if s.dry_health_score >= 90.0 {
            Color::Green
        } else if s.dry_health_score >= 80.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        table.add_row(vec![
            Cell::new(commit_cell),
            Cell::new(&s.formatted_date),
            Cell::new(&s.author),
            Cell::new(if s.message.len() > 30 {
                format!("{}...", &s.message[..27])
            } else {
                s.message.clone()
            }),
            Cell::new(s.total_files),
            Cell::new(s.total_tokens),
            Cell::new(s.total_clones),
            Cell::new(s.total_clusters),
            Cell::new(format!("{:.1}%", s.duplication_percentage)),
            Cell::new(format!("{:.1}", s.dry_health_score)).fg(score_color),
        ]);
    }
    println!("{table}\n");

    let delta_sign = if trend.score_delta >= 0.0 { "+" } else { "" };
    println!(
        "Summary: Initial Score: {:.1} -> Current Score: {:.1} ({}{:.1} DRY delta) | Duplication \
         Change: {:+.2}%",
        trend.initial_score,
        trend.current_score,
        delta_sign,
        trend.score_delta,
        trend.duplication_delta
    );
}

fn print_trend_markdown_report(trend: &cddm_core::TimelineTrend) {
    println!("# CDDM Historical Duplication Trend\n");
    println!(
        "| Commit | Date | Author | Message | Files | Tokens | Clones | Clusters | Duplication | \
         DRY Score |"
    );
    println!("| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |");
    for s in &trend.snapshots {
        let tag_str = s
            .tag
            .as_ref()
            .map(|t| format!(" ({t})"))
            .unwrap_or_default();
        let msg_clean = s.message.replace('|', "\\|");
        println!(
            "| `{}`{} | {} | {} | {} | {} | {} | {} | {} | {:.1}% | **{:.1}** |",
            s.short_hash,
            tag_str,
            s.formatted_date,
            s.author,
            msg_clean,
            s.total_files,
            s.total_tokens,
            s.total_clones,
            s.total_clusters,
            s.duplication_percentage,
            s.dry_health_score
        );
    }
    println!(
        "\n**Historical DRY Score Delta**: {:+.1} (Initial: {:.1} -> Current: {:.1})\n",
        trend.score_delta, trend.initial_score, trend.current_score
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use cddm_core::{
        CloneCluster, CloneLocation, ClonePair, CloneStatus, CloneType, DiffClonePair,
        DiffScanResult, DiffSummary, LanguageStats, ScanResult, TimelineSnapshot, TimelineTrend,
    };

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
        let cli_lsp =
            Cli::try_parse_from(["cddm", "lsp", "--min-tokens", "40"]).expect("parse lsp");
        match cli_lsp.command {
            Commands::Lsp { min_tokens, .. } => assert_eq!(min_tokens, 40),
            _ => panic!("expected Lsp command"),
        }

        let cli_trend =
            Cli::try_parse_from(["cddm", "trend", "--max-samples", "15"]).expect("parse trend");
        match cli_trend.command {
            Commands::Trend { max_samples, .. } => assert_eq!(max_samples, 15),
            _ => panic!("expected Trend command"),
        }

        let cli_hook = Cli::try_parse_from([
            "cddm",
            "hook",
            "install",
            "--hook-type",
            "pre-push",
            "--fail-threshold",
            "12.5",
        ])
        .expect("parse hook");
        match cli_hook.command {
            Commands::Hook { action } => match action {
                HookAction::Install {
                    hook_type,
                    fail_threshold,
                    ..
                } => {
                    assert_eq!(hook_type, "pre-push");
                    assert_eq!(fail_threshold, 12.5);
                }
                _ => panic!("expected HookAction::Install"),
            },
            _ => panic!("expected Hook command"),
        }

        let cli_init = Cli::try_parse_from(["cddm", "init", "github", "--fail-threshold", "10.0"])
            .expect("parse init");
        match cli_init.command {
            Commands::Init {
                platform,
                fail_threshold,
                ..
            } => {
                assert_eq!(platform, PlatformChoice::Github);
                assert_eq!(fail_threshold, 10.0);
            }
            _ => panic!("expected Init command"),
        }
    }
}
