#![forbid(unsafe_code)]

use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Gauge, Paragraph, Row, Table};

use super::helpers::{create_titled_block, split_horizontal_2, split_vertical_3};
use crate::tui::app::TuiApp;
use crate::tui::theme::TuiTheme;

/// Render Tab 1: Codebase Overview and DRY Health Metrics.
pub fn render_overview_view(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (gauge_area, metrics_area, config_area) = split_vertical_3(area, 6, 5);
    render_health_gauge(frame, app, gauge_area);
    render_metrics_and_languages(frame, app, metrics_area);
    render_config_summary(frame, app, config_area);
}

fn render_health_gauge(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (score, dup_pct) = match &app.scan_result {
        Some(res) => (res.dry_health_score, res.duplication_percentage),
        None => (100.0, 0.0),
    };

    let score_color = TuiTheme::dry_score_color(score);
    let tier_label = TuiTheme::dry_score_tier(score);

    let title = format!(
        " DRY Health Score: {:.1} / 100.0  [{}]  (Duplication: {:.1}%) ",
        score, tier_label, dup_pct
    );

    let block = create_titled_block(&title, false);
    let gauge = Gauge::default()
        .block(block)
        .gauge_style(
            Style::default()
                .fg(score_color)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .ratio((score / 100.0).clamp(0.0, 1.0))
        .label(format!("{:.1}% DRY Health", score));

    frame.render_widget(gauge, area);
}

fn render_metrics_and_languages(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (left_box, right_box) = split_horizontal_2(area, 50, 50);

    let (files, tokens, clones, clusters) = match &app.scan_result {
        Some(r) => (
            r.total_files,
            r.total_tokens,
            r.total_clones,
            r.total_clusters,
        ),
        None => (0, 0, 0, 0),
    };

    let metric_data = [
        ("Scanned Code Files", format!("{} files", files)),
        ("Total Processed Tokens", format!("{} tokens", tokens)),
        ("Identified Clone Pairs", format!("{} pairs", clones)),
        ("Connected Clone Clusters", format!("{} clusters", clusters)),
    ];
    let rows: Vec<Row> = metric_data
        .into_iter()
        .map(|(k, v)| Row::new(vec![k.to_string(), v]))
        .collect();

    render_overview_table(frame, " Codebase Modularity Metrics ", rows, left_box);

    let (exact, renamed, near_miss, semantic) = match &app.scan_result {
        Some(r) => {
            let mut e = 0;
            let mut rn = 0;
            let mut nm = 0;
            let mut sm = 0;
            for p in &r.clone_pairs {
                match p.clone_type {
                    cddm_core::types::CloneType::Exact => e += 1,
                    cddm_core::types::CloneType::Renamed => rn += 1,
                    cddm_core::types::CloneType::NearMiss => nm += 1,
                    cddm_core::types::CloneType::Semantic => sm += 1,
                }
            }
            (e, rn, nm, sm)
        }
        None => (0, 0, 0, 0),
    };

    let type_data = [
        ("Type-1 Exact Clones", format!("{} occurrences", exact)),
        (
            "Type-2 Renamed Identifiers",
            format!("{} occurrences", renamed),
        ),
        (
            "Type-3 Near-Miss Statements",
            format!("{} occurrences", near_miss),
        ),
        (
            "Type-4 Semantic / Polyglot",
            format!("{} occurrences", semantic),
        ),
    ];
    let type_rows: Vec<Row> = type_data
        .into_iter()
        .map(|(k, v)| Row::new(vec![k.to_string(), v]))
        .collect();

    render_overview_table(
        frame,
        " Duplication Classification Breakdown ",
        type_rows,
        right_box,
    );
}

fn render_overview_table(frame: &mut Frame, title: &str, rows: Vec<Row>, area: Rect) {
    let table = Table::new(
        rows,
        [Constraint::Percentage(60), Constraint::Percentage(40)],
    )
    .block(create_titled_block(title, false))
    .style(Style::default().fg(Color::White));

    frame.render_widget(table, area);
}

fn render_config_summary(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let lines = vec![
        Line::from(vec![
            Span::styled("Target Directory: ", Style::default().fg(Color::Cyan)),
            Span::raw(app.directory.display().to_string()),
            Span::raw(" | "),
            Span::styled("Min Tokens: ", Style::default().fg(Color::Cyan)),
            Span::raw(app.config.min_tokens.to_string()),
            Span::raw(" | "),
            Span::styled("Watch Mode: ", Style::default().fg(Color::Cyan)),
            Span::raw(if app.watch_mode { "Active" } else { "Disabled" }),
        ]),
        Line::from(vec![Span::styled(
            "Tip: Switch to Tab [2] to inspect split diffs or Tab [4] to auto-refactor.",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let block = create_titled_block(" Active Configuration & Workspace Environment ", false);
    frame.render_widget(Paragraph::new(lines).block(block), area);
}
