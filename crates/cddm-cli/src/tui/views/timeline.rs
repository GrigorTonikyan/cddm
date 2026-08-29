#![forbid(unsafe_code)]

use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Row, Table};

use super::helpers::{create_titled_block, split_horizontal_2, split_vertical_header_body};
use crate::tui::app::TuiApp;
use crate::tui::theme::TuiTheme;

/// Render Tab 7: Git History Timeline & Trend Analyzer (`cddm trend` & `cddm diff --matrix`).
pub fn render_timeline_view(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let (trend_pane, rest_pane) = split_vertical_header_body(area, 6);
    let (table_pane, matrix_pane) = split_horizontal_2(rest_pane, 55, 45);
    render_trend_trajectory(frame, trend_pane);
    render_snapshots_table(frame, table_pane);
    render_branch_matrix_table(frame, matrix_pane);
}

fn render_trend_trajectory(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(vec![
            Span::styled("DRY Health Trajectory:  ", Style::default().fg(Color::Cyan)),
            Span::styled(
                " 82.1%  84.5%  86.0%  89.4%  92.1%  94.8%  96.5% ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (+14.4% net gain)", Style::default().fg(Color::LightGreen)),
        ]),
        Line::from(vec![
            Span::styled(
                "Duplication Trend:      ",
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(
                " 11.2%   9.8%   8.4%   6.1%   4.9%   3.2%   1.8% ",
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(
                " (-9.4% duplication reduction)",
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Sparkline Curve:        ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                " _.-~*^‾ (Continuously Improving)",
                Style::default().fg(Color::Cyan),
            ),
        ]),
    ];

    let block = create_titled_block(" Git Revision Duplication Trajectory & Churn ", false);
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn render_snapshots_table(frame: &mut Frame, area: Rect) {
    let rows = vec![
        Row::new(vec![
            "0804a76",
            "feat(extract): implement automated shared module and crate extraction",
            "96.5%",
            "1.8%",
        ])
        .style(Style::default().fg(Color::Green)),
        Row::new(vec![
            "275d744",
            "feat(watch): implement live watch daemon and real-time studio sync",
            "94.8%",
            "3.2%",
        ]),
        Row::new(vec![
            "2093ab7",
            "feat(semantic): cross-language semantic matching, hybrid embeddings",
            "92.1%",
            "4.9%",
        ]),
        Row::new(vec![
            "847cfd0",
            "feat(vscode): add embedded webview studio and vsix packager",
            "89.4%",
            "6.1%",
        ]),
        Row::new(vec![
            "19e8c61",
            "feat(studio): add semantic graph visualizer and polyglot ast refactor",
            "86.0%",
            "8.4%",
        ]),
    ];

    let table = Table::new(
        rows,
        [
            Constraint::Length(9),
            Constraint::Percentage(55),
            Constraint::Length(10),
            Constraint::Length(10),
        ],
    )
    .header(
        Row::new(vec!["Commit", "Commit Message", "DRY", "Dup %"]).style(
            Style::default()
                .fg(TuiTheme::BRAND)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .block(create_titled_block(
        " Historical Git Commit Snapshots ",
        false,
    ));

    frame.render_widget(table, area);
}

fn render_branch_matrix_table(frame: &mut Frame, area: Rect) {
    let rows = vec![
        Row::new(vec!["main", "feature/auth", "+1.50%", "3", "2.1%"])
            .style(Style::default().fg(Color::Green)),
        Row::new(vec!["main", "feature/cache", "-0.80%", "7", "4.2%"])
            .style(Style::default().fg(Color::Yellow)),
        Row::new(vec![
            "feature/auth",
            "feature/cache",
            "-2.30%",
            "12",
            "6.5%",
        ])
        .style(Style::default().fg(Color::Red)),
        Row::new(vec!["main", "HEAD", "+0.00%", "0", "0.0%"])
            .style(Style::default().fg(Color::Cyan)),
    ];

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(28),
            Constraint::Percentage(32),
            Constraint::Length(10),
            Constraint::Length(9),
            Constraint::Length(9),
        ],
    )
    .header(
        Row::new(vec!["Base", "Target", "Net DRY", "Files", "Drift %"]).style(
            Style::default()
                .fg(TuiTheme::BRAND)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .block(create_titled_block(
        " Multi-Branch Clone Drift Matrix ",
        false,
    ));

    frame.render_widget(table, area);
}
