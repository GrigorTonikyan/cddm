use crate::formatters::format_semantic_report;
use crate::types::OutputFormat;
use cddm_core::{
    DEFAULT_MIN_TOKENS, NeuralEmbeddingConfig, ScanConfig, scan_cross_language_workspace,
    scan_neural_clones_with_options,
};
use comfy_table::{Cell, Color, Table};
use std::path::PathBuf;

/// Executes the dedicated `cddm semantic` CLI command to analyze cross-language clones.
#[allow(clippy::too_many_arguments)]
pub fn run_semantic_command(
    directory: PathBuf,
    threshold: f64,
    min_tokens: usize,
    format: OutputFormat,
    languages: Vec<String>,
    ignore: Vec<String>,
    neural: bool,
    neural_threshold: f32,
    hnsw: bool,
    sq8: bool,
    threads: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    if neural || hnsw || sq8 {
        let neural_config = NeuralEmbeddingConfig {
            similarity_threshold: neural_threshold,
            ..Default::default()
        };

        let use_hnsw = hnsw || sq8;
        let use_sq8 = sq8;
        let result = scan_neural_clones_with_options(&directory, &neural_config, use_hnsw, use_sq8)
            .map_err(|e| format!("Neural code embedding scan failed: {}", e))?;

        match format {
            OutputFormat::Json | OutputFormat::Sarif => {
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
            OutputFormat::Ndjson => {
                for pair in &result.pairs {
                    println!("{}", serde_json::to_string(pair)?);
                }
            }
            OutputFormat::Markdown => {
                println!("# CDDM Neural Code Embedding & Algorithmic Equivalence Report\n");
                if let Some(ref idx) = result.index_type {
                    let mem_str = if let Some(ratio) = result.memory_reduction_ratio {
                        format!(" ({:.1}x memory compression)", ratio)
                    } else {
                        String::new()
                    };
                    println!("- Vector Index: **{}**{}", idx, mem_str);
                }
                println!(
                    "- Embedded Code Blocks: **{}**",
                    result.total_blocks_embedded
                );
                println!(
                    "- Algorithmic Pairs Detected: **{}**",
                    result.total_neural_pairs
                );
                println!(
                    "- High-Confidence Equivalence: **{}**\n",
                    result.high_confidence_count
                );
                println!(
                    "| Location A | Language A | Location B | Language B | Cosine Similarity | \
                     Confidence |"
                );
                println!("| :--- | :--- | :--- | :--- | :--- | :--- |");
                for p in &result.pairs {
                    println!(
                        "| `{}:{}-{}` | `{}` | `{}:{}-{}` | `{}` | `{:.1}%` | `{:?}` |",
                        p.file_a,
                        p.start_line_a,
                        p.end_line_a,
                        p.language_a,
                        p.file_b,
                        p.start_line_b,
                        p.end_line_b,
                        p.language_b,
                        p.similarity * 100.0,
                        p.confidence
                    );
                }
            }
            OutputFormat::Console => {
                println!("\n=== CDDM Neural Algorithmic Equivalence Scan ===");
                if let Some(ref idx) = result.index_type {
                    let mem_str = if let Some(ratio) = result.memory_reduction_ratio {
                        format!(" (Memory Reduction: {:.1}x)", ratio)
                    } else {
                        String::new()
                    };
                    println!("Index Structure: {}{}\n", idx, mem_str);
                }
                println!(
                    "Embedded Blocks: {} | Equivalent Pairs: {} | High Confidence: {}\n",
                    result.total_blocks_embedded,
                    result.total_neural_pairs,
                    result.high_confidence_count
                );

                let mut table = Table::new();
                table.set_header(vec![
                    "Target A",
                    "Lang A",
                    "Target B",
                    "Lang B",
                    "Similarity",
                    "Confidence",
                ]);

                for p in &result.pairs {
                    let loc_a = format!("{}:{}-{}", p.file_a, p.start_line_a, p.end_line_a);
                    let loc_b = format!("{}:{}-{}", p.file_b, p.start_line_b, p.end_line_b);
                    let conf_color = match p.confidence {
                        cddm_core::EquivalenceConfidence::High => Color::Green,
                        cddm_core::EquivalenceConfidence::Medium => Color::Yellow,
                        cddm_core::EquivalenceConfidence::Low => Color::DarkGrey,
                    };

                    table.add_row(vec![
                        Cell::new(loc_a).fg(Color::Cyan),
                        Cell::new(&p.language_a),
                        Cell::new(loc_b).fg(Color::Cyan),
                        Cell::new(&p.language_b),
                        Cell::new(format!("{:.1}%", p.similarity * 100.0)).fg(Color::Magenta),
                        Cell::new(format!("{:?}", p.confidence)).fg(conf_color),
                    ]);
                }

                println!("{table}");
            }
        }
        return Ok(());
    }

    let config = ScanConfig {
        directory: directory.to_string_lossy().to_string(),
        min_tokens: if min_tokens == 0 {
            DEFAULT_MIN_TOKENS
        } else {
            min_tokens
        },
        languages,
        ignore_patterns: if ignore.is_empty() {
            ScanConfig::default().ignore_patterns
        } else {
            ignore
        },
        detect_type2: true,
        detect_type3: true,
        detect_type4: true,
        scan_self: true,
        enable_git_blame: false,
        cache_dir: None,
        enable_cache: true,
        cddmignore_path: None,
        ignore_tests: false,
        ignore_mocks: false,
        ignore_generated: true,
        rules_path: None,
        enforce_policies: false,
        cross_language: true,
        threads,
        in_tree_cache: false,
        include_ignored: false,
    };

    let pairs = scan_cross_language_workspace(&config, threshold)
        .map_err(|e| format!("Semantic cross-language scan failed: {}", e))?;

    let report = format_semantic_report(&pairs, format, threshold);
    println!("{}", report);

    Ok(())
}
