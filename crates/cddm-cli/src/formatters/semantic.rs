use crate::types::OutputFormat;
use cddm_core::semantic_graph::CrossLanguageClonePair;

/// Formats cross-language semantic clone scan results into console, markdown, or JSON string.
pub fn format_semantic_report(
    pairs: &[CrossLanguageClonePair],
    format: OutputFormat,
    threshold: f64,
) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(pairs).unwrap_or_default(),
        OutputFormat::Ndjson => serde_json::to_string(pairs).unwrap_or_default(),
        OutputFormat::Markdown => format_semantic_markdown(pairs, threshold),
        OutputFormat::Console | OutputFormat::Sarif => format_semantic_console(pairs, threshold),
    }
}

fn format_semantic_console(pairs: &[CrossLanguageClonePair], threshold: f64) -> String {
    let mut out = String::new();
    out.push_str("\n\x1b[36m====================================================================================================\x1b[0m\n");
    out.push_str("   CDDM Cross-Language Semantic Clone Analysis (Type-4 Isomorphism & Hybrid Embeddings)   \n");
    out.push_str("\x1b[36m====================================================================================================\x1b[0m\n\n");

    out.push_str(&format!(
        "\x1b[33mAnalyzed cross-language pairs with hybrid similarity threshold >= {:.1}%\x1b[0m\n",
        threshold * 100.0
    ));
    out.push_str(&format!(
        "\x1b[32mDiscovered {} cross-language semantic clone pair(s)\x1b[0m\n\n",
        pairs.len()
    ));

    if pairs.is_empty() {
        out.push_str(
            "\x1b[32m[PASS] No cross-language duplicate logic exceeding threshold \
             detected.\x1b[0m\n\n",
        );
        return out;
    }

    out.push_str("+----------------------+-----------------------+----------------------+-----------------------+-----------+-----------+------------+\n");
    out.push_str("| Source A (Lang/Fn)   | Location A            | Source B (Lang/Fn)   | Location B            | Graph Sim | Token Sim | Hybrid Sim |\n");
    out.push_str("+----------------------+-----------------------+----------------------+-----------------------+-----------+-----------+------------+\n");

    for p in pairs {
        let src_a = format!("{}: {}", p.language_a, p.function_a);
        let loc_a = format!("{}:{}-{}", p.file_a, p.lines_a.0, p.lines_a.1);
        let src_b = format!("{}: {}", p.language_b, p.function_b);
        let loc_b = format!("{}:{}-{}", p.file_b, p.lines_b.0, p.lines_b.1);

        let trunc_src_a = if src_a.len() > 20 {
            format!("{}...", &src_a[..17])
        } else {
            format!("{:20}", src_a)
        };
        let trunc_loc_a = if loc_a.len() > 21 {
            format!("{}...", &loc_a[..18])
        } else {
            format!("{:21}", loc_a)
        };
        let trunc_src_b = if src_b.len() > 20 {
            format!("{}...", &src_b[..17])
        } else {
            format!("{:20}", src_b)
        };
        let trunc_loc_b = if loc_b.len() > 21 {
            format!("{}...", &loc_b[..18])
        } else {
            format!("{:21}", loc_b)
        };

        out.push_str(&format!(
            "| {} | {} | {} | {} | {:>8.1}% | {:>8.1}% | \x1b[1;32m{:>9.1}%\x1b[0m |\n",
            trunc_src_a,
            trunc_loc_a,
            trunc_src_b,
            trunc_loc_b,
            p.graph_similarity * 100.0,
            p.token_similarity * 100.0,
            p.hybrid_score * 100.0
        ));
    }

    out.push_str("+----------------------+-----------------------+----------------------+-----------------------+-----------+-----------+------------+\n\n");
    out
}

fn format_semantic_markdown(pairs: &[CrossLanguageClonePair], threshold: f64) -> String {
    let mut out = String::new();
    out.push_str("# CDDM Cross-Language Semantic Clone Report\n\n");
    out.push_str(&format!(
        "> **Similarity Threshold**: $\\ge {:.1}\\%$ | **Detected Cross-Language Clones**: {}\n\n",
        threshold * 100.0,
        pairs.len()
    ));

    if pairs.is_empty() {
        out.push_str("No cross-language duplicate logic exceeding threshold detected.\n");
        return out;
    }

    out.push_str(
        "| Language A | Function A | File A | Language B | Function B | File B | Graph Sim | \
         Token Sim | Hybrid Score |\n",
    );
    out.push_str("| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");

    for p in pairs {
        out.push_str(&format!(
            "| `{}` | `{}` | `{}:{}-{}` | `{}` | `{}` | `{}:{}-{}` | `{:.1}%` | `{:.1}%` | \
             **`{:.1}%`** |\n",
            p.language_a,
            p.function_a,
            p.file_a,
            p.lines_a.0,
            p.lines_a.1,
            p.language_b,
            p.function_b,
            p.file_b,
            p.lines_b.0,
            p.lines_b.1,
            p.graph_similarity * 100.0,
            p.token_similarity * 100.0,
            p.hybrid_score * 100.0
        ));
    }

    out.push('\n');
    out
}
