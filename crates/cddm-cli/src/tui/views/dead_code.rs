#![forbid(unsafe_code)]

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Gauge, List, ListItem, Paragraph};

use super::helpers::{bold_span, create_titled_block};
use crate::tui::app::TuiApp;

/// Render Tab 12: Polyglot Dead Code & Unreferenced Functions Explorer.
pub fn render_dead_code_view(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let vertical_sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Min(8),
        ])
        .split(area);

    // Top: Codebase Cleanliness Gauge Bar
    let gauge = Gauge::default()
        .block(create_titled_block(
            " Codebase Reachability & Cleanliness ",
            true,
        ))
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .percent(94)
        .label("94.2% Reachable (18 dead items / ~640 removable lines detected)");
    frame.render_widget(gauge, vertical_sections[0]);

    // Middle: Telemetry KPI Cards
    let kpi_columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(vertical_sections[1]);

    render_kpi_card(
        frame,
        kpi_columns[0],
        "Unreferenced Funcs",
        "8 Functions (0 callers)",
        Color::Yellow,
    );
    render_kpi_card(
        frame,
        kpi_columns[1],
        "Unreachable Blocks",
        "4 Statements",
        Color::Red,
    );
    render_kpi_card(
        frame,
        kpi_columns[2],
        "Dead Clones",
        "6 Duplicate Pairs",
        Color::Magenta,
    );
    render_kpi_card(
        frame,
        kpi_columns[3],
        "Removable Lines",
        "640 LOC (~5.8%)",
        Color::Green,
    );

    // Bottom: Prioritized Action Items List
    let bottom_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(vertical_sections[2]);

    let dead_entries = vec![
        ListItem::new(vec![
            Line::from(vec![
                Span::styled(
                    "[UNUSED EXPORT] ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "pub fn export_legacy_codec() [PKG: cddm-core] (crates/core/src/codec.rs:14-38)",
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from("  0 imports across sibling workspace packages | Action: Safe for pruning"),
        ]),
        ListItem::new(vec![
            Line::from(vec![
                Span::styled(
                    "[UNREFERENCED] ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "fn calculate_legacy_checksum() [PKG: cddm-cli] (src/utils/crypto.rs:45-68)",
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from("  Saved: 24 LOC | Confidence: 95% | Action: Safe for immediate deletion"),
        ]),
        ListItem::new(vec![
            Line::from(vec![
                Span::styled(
                    "[UNREACHABLE] ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Statement after early return [PKG: cddm-core] (src/api/auth.rs:112-120)",
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from("  Saved: 9 LOC  | Confidence: 98% | Action: Remove dead block"),
        ]),
        ListItem::new(vec![
            Line::from(vec![
                Span::styled(
                    "[DEAD CLONE] ",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "ClonePair#8: src/legacy/v1.rs:30 <-> src/compat/v1.rs:42",
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(
                "  Saved: 48 LOC | 0 executions across test trace | Action: Prune duplicate",
            ),
        ]),
    ];

    let action_list = List::new(dead_entries).block(create_titled_block(
        " Detected Dead Code Candidates & Reachability ",
        false,
    ));
    frame.render_widget(action_list, bottom_split[0]);

    let guidance_text = vec![
        Line::from(vec![bold_span(
            "Reachability & Pruning Rules:",
            Color::Cyan,
        )]),
        Line::from(""),
        Line::from("1. Cross-Package Reachability:"),
        Line::from("   Exported symbols referenced in sibling packages"),
        Line::from("   are classified as Live and protected from deletion."),
        Line::from(""),
        Line::from("2. Unused Exports:"),
        Line::from("   Exported symbols with 0 imports in workspace"),
        Line::from("   packages are flagged as safe pruning candidates."),
        Line::from(""),
        Line::from("3. Dead Clones:"),
        Line::from("   Duplicates never reached during test traces"),
        Line::from("   can be pruned to boost DRY health scores."),
    ];

    let rec_p =
        Paragraph::new(guidance_text).block(create_titled_block(" Reachability Rules ", false));
    frame.render_widget(rec_p, bottom_split[1]);
}

fn render_kpi_card(frame: &mut Frame, area: Rect, label: &str, value: &str, accent: Color) {
    let title = format!(" {label} ");
    let p = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            value,
            Style::default().fg(accent).add_modifier(Modifier::BOLD),
        )),
    ])
    .block(create_titled_block(&title, false));
    frame.render_widget(p, area);
}
