#![forbid(unsafe_code)]

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{List, ListItem, Row, Table};

use super::helpers::*;
use crate::tui::app::TuiApp;
use crate::tui::theme::TuiTheme;

/// Render Tab 6: Policy Engine & AST Suppression Directives Inspector.
pub fn render_policy_view(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (left_col, right_col) = split_horizontal_2(area, 50, 50);
    render_policy_rules(frame, app, left_col);
    render_suppression_rules(frame, app, right_col);
}

fn render_policy_rules(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let mut rows = Vec::new();
    if let Some(res) = &app.scan_result {
        if !res.policy_violations.is_empty() {
            for v in &res.policy_violations {
                let color = match v.severity {
                    cddm_core::PolicySeverity::Error => Color::Red,
                    cddm_core::PolicySeverity::Warning => Color::Yellow,
                    cddm_core::PolicySeverity::Info => Color::Cyan,
                };
                rows.push(
                    Row::new(vec![
                        v.rule_name.clone(),
                        format!("{} <-> {}", v.file_a, v.file_b.as_deref().unwrap_or("-")),
                        format!("{:?}: {}", v.severity, v.message),
                    ])
                    .style(Style::default().fg(color)),
                );
            }
        } else {
            rows.push(
                Row::new(vec![
                    "Architectural Boundaries".to_string(),
                    format!("{} files evaluated", res.total_files),
                    "0 Violations (PASS)".to_string(),
                ])
                .style(Style::default().fg(Color::Green)),
            );
            rows.push(
                Row::new(vec![
                    "Zero Duplication Zones".to_string(),
                    "Domain & Core Modules".to_string(),
                    "Compliant (PASS)".to_string(),
                ])
                .style(Style::default().fg(Color::Green)),
            );
        }
    } else {
        rows.push(
            Row::new(vec![
                "Policy Engine".to_string(),
                "No scan executed yet".to_string(),
                "Press 's' to Scan".to_string(),
            ])
            .style(Style::default().fg(Color::DarkGray)),
        );
    }

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(35),
            Constraint::Percentage(40),
            Constraint::Percentage(25),
        ],
    )
    .header(
        Row::new(vec!["Policy Rule", "Scope / Violation", "Status"]).style(
            Style::default()
                .fg(TuiTheme::BRAND)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .block(create_titled_block(
        " Architectural Boundary Rules (.cddmrules.toml) ",
        false,
    ));

    frame.render_widget(table, area);
}

fn render_suppression_rules(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let subchunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let mut ignore_items: Vec<ListItem> =
        if let Ok(cddmignore) = std::fs::read_to_string(".cddmignore") {
            cddmignore
                .lines()
                .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
                .take(8)
                .map(|l| {
                    ListItem::new(Line::from(vec![
                        Span::styled(" [ignore] ", Style::default().fg(Color::Cyan)),
                        Span::raw(l.to_string()),
                    ]))
                })
                .collect()
        } else {
            Vec::new()
        };

    if ignore_items.is_empty() {
        for pat in app.config.ignore_patterns.iter().take(6) {
            ignore_items.push(ListItem::new(Line::from(vec![
                Span::styled(" [glob] ", Style::default().fg(Color::Yellow)),
                Span::raw(pat.clone()),
            ])));
        }
    }

    let list_ignore =
        List::new(ignore_items).block(create_titled_block(" Active .cddmignore Rules ", false));
    frame.render_widget(list_ignore, subchunks[0]);

    let directive_lines = vec![
        Line::from(vec![Span::styled(
            "Supported Inline AST Directives:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  // cddm:ignore              Ignore next AST statement"),
        Line::from("  /* cddm:ignore-start */     Begin ignored code block"),
        Line::from("  /* cddm:ignore-end */       End ignored code block"),
        Line::from("  #[cddm(allow_duplication)]  Rust attribute suppression"),
        Line::from("  # @cddm_ignore              Python / Ruby comment suppression"),
    ];

    let p_dir = Paragraph::new(directive_lines)
        .block(create_titled_block(" Inline Directives & Headers ", false));
    frame.render_widget(p_dir, subchunks[1]);
}
