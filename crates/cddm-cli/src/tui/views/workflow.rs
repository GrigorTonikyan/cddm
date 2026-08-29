#![forbid(unsafe_code)]

use super::helpers::*;
use crate::tui::app::TuiApp;

/// Render Tab 8: CI/CD Workflows & Git Quality Gate Hook Manager (`cddm hook`, `cddm init`).
pub fn render_workflow_view(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let (top_pane, mid_pane, bot_pane) = split_vertical_3(area, 5, 4);
    render_hook_status(frame, top_pane);
    render_ci_preview(frame, mid_pane);
    render_workflow_actions(frame, bot_pane);
}

fn render_hook_status(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(vec![
            Span::styled("Pre-Commit Git Hook:  ", Style::default().fg(Color::Cyan)),
            Span::styled(
                " INSTALLED & ACTIVE (.git/hooks/pre-commit) ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" (Enforcing --fail-threshold 15.0)"),
        ]),
        Line::from(vec![
            Span::styled("Pre-Push Git Hook:    ", Style::default().fg(Color::Cyan)),
            Span::styled(" Not Installed ", Style::default().fg(Color::Yellow)),
        ]),
    ];

    let block = create_titled_block(" Local Repository Git Quality Gate Hooks ", false);
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

const CI_WORKFLOW_YAML: &str = r#"# .github/workflows/cddm.yml — Turnkey Quality Gate
name: CDDM Code Duplication Analysis
on: [push, pull_request]
jobs:
  cddm-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: GrigorTonikyan/cddm-action@v2
        with:
          fail-threshold: 15.0
          format: sarif
          output: cddm-results.sarif
      - uses: github/codeql-action/upload-sarif@v3
        with:
          sarif_file: cddm-results.sarif"#;

fn render_ci_preview(frame: &mut Frame, area: Rect) {
    let block = create_titled_block(
        " Generated CI/CD Workflow Specification (.github/workflows/cddm.yml) ",
        false,
    );
    let p = Paragraph::new(CI_WORKFLOW_YAML)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(p, area);
}

fn render_workflow_actions(frame: &mut Frame, area: Rect) {
    let action_lines = vec![Line::from(vec![
        Span::styled(
            " [i] Install Pre-Commit Hook ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            " [u] Uninstall Hook ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            " [w] Write .github/workflows/cddm.yml ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ])];

    let block = create_titled_block(" Turnkey Actions ", true);
    frame.render_widget(Paragraph::new(action_lines).block(block), area);
}
