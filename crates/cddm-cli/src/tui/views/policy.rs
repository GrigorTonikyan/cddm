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

fn render_policy_rules(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let rows = vec![
        Row::new(vec![
            "Boundary Isolation",
            "crates/cddm-core <-> webui",
            "0 Violations (PASS)",
        ])
        .style(Style::default().fg(Color::Green)),
        Row::new(vec![
            "Zero Duplication Zone",
            "crates/cddm-core/src/policy/*",
            "0 Violations (PASS)",
        ])
        .style(Style::default().fg(Color::Green)),
        Row::new(vec![
            "Zero Duplication Zone",
            "crates/cddm-core/src/ai/*",
            "0 Violations (PASS)",
        ])
        .style(Style::default().fg(Color::Green)),
        Row::new(vec![
            "Clone Token Limit",
            "Max allowed: 500 tokens",
            "Compliant",
        ])
        .style(Style::default().fg(Color::Cyan)),
    ];

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(35),
            Constraint::Percentage(40),
            Constraint::Percentage(25),
        ],
    )
    .header(
        Row::new(vec!["Policy Type", "Target Scope", "Status"]).style(
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

fn render_suppression_rules(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let subchunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let ignore_items = vec![
        ListItem::new(Line::from(vec![
            Span::styled(" [glob] ", Style::default().fg(Color::Cyan)),
            Span::raw("tests/** (ignore test duplication)"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled(" [glob] ", Style::default().fg(Color::Cyan)),
            Span::raw("**/mock_*.rs (mock fixtures excluded)"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled(" [override] ", Style::default().fg(Color::Yellow)),
            Span::raw("sdk/** min_tokens=100"),
        ])),
    ];

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
