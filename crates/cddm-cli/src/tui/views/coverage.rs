#![forbid(unsafe_code)]

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Gauge, List, ListItem, Paragraph};

use super::helpers::{bold_span, create_titled_block};
use crate::tui::app::TuiApp;

/// Render Tab 11: Dynamic Runtime Execution & Coverage-Aware De-duplication.
pub fn render_coverage_view(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let vertical_sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Min(8),
        ])
        .split(area);

    // Top: Coverage Gauge Bar
    let gauge = Gauge::default()
        .block(create_titled_block(
            " Duplicate Execution Coverage Rate ",
            true,
        ))
        .gauge_style(
            Style::default()
                .fg(Color::Green)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .percent(88)
        .label("88.4% Instrumented (1,240 / 1,402 duplicate lines)");
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

    render_telemetry_stat(
        frame,
        kpi_columns[0],
        "Dead Code Clones",
        "2 Pairs (0 hits)",
        Color::Yellow,
    );
    render_telemetry_stat(
        frame,
        kpi_columns[1],
        "Hot Path Clones",
        "2 Pairs (>5k hits)",
        Color::Red,
    );
    render_telemetry_stat(
        frame,
        kpi_columns[2],
        "Test Gap Clones",
        "1 Asymmetric Pair",
        Color::Magenta,
    );
    render_telemetry_stat(
        frame,
        kpi_columns[3],
        "Total Trace Hits",
        "23,550 executions",
        Color::Cyan,
    );

    // Bottom: Prioritized Action Items List
    let bottom_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(vertical_sections[2]);

    let clone_entries = vec![
        ListItem::new(vec![
            Line::from(vec![
                Span::styled(
                    "[HOT PATH] ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Pair #1: src/auth/jwt.rs:45 <-> src/auth/v2.rs:50",
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(
                "  Executions: 14,200/hr | Risk: 98.5/100 | Action: Critical high-traffic \
                 deduplication target",
            ),
        ]),
        ListItem::new(vec![
            Line::from(vec![
                Span::styled(
                    "[HOT PATH] ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Pair #2: src/utils/crypto.rs:12 <-> src/helpers/hash.rs:30",
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(
                "  Executions: 8,900/hr  | Risk: 92.0/100 | Action: High-throughput math routine",
            ),
        ]),
        ListItem::new(vec![
            Line::from(vec![
                Span::styled(
                    "[TEST GAP] ",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Pair #3: src/api/handler.rs:88 <-> src/api/v2/handler.rs:90",
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(
                "  Executions: 450 hits (A: 450 hits 100% covered, B: 0 hits unmonitored) | \
                 Action: Unify handler",
            ),
        ]),
        ListItem::new(vec![
            Line::from(vec![
                Span::styled(
                    "[DEAD CODE] ",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Pair #4: src/legacy/parser.rs:102 <-> src/compat/v1.rs:40",
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
            Line::from(
                "  Executions: 0 hits (30d production trace) | Action: Safe for immediate \
                 dead-code pruning",
            ),
        ]),
    ];

    let action_list = List::new(clone_entries).block(create_titled_block(
        " Correlated Duplicate Execution Priorities (lcov.info) ",
        false,
    ));
    frame.render_widget(action_list, bottom_split[0]);

    let recommendation_text = vec![
        Line::from(vec![bold_span("Coverage Guidance & Rules:", Color::Cyan)]),
        Line::from(""),
        Line::from("1. Hot Path Priority:"),
        Line::from("   Refactor clones with >1,000 hits first to"),
        Line::from("   maximize CPU cache efficiency and speed."),
        Line::from(""),
        Line::from("2. Dead Code Pruning:"),
        Line::from("   Clones with 0 hits over instrumented tests"),
        Line::from("   can be deleted safely to reduce tokens."),
        Line::from(""),
        Line::from("3. Test Gap Elimination:"),
        Line::from("   Merge asymmetric clones into one covered"),
        Line::from("   implementation to avoid silent regressions."),
    ];

    let rec_p = Paragraph::new(recommendation_text)
        .block(create_titled_block(" Execution Guidelines ", false));
    frame.render_widget(rec_p, bottom_split[1]);
}

fn render_telemetry_stat(frame: &mut Frame, area: Rect, label: &str, value: &str, accent: Color) {
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
