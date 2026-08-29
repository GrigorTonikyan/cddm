use super::helpers::*;
use crate::tui::app::TuiApp;
use ratatui::widgets::{Row, Table};

/// Render Tab 10: Organization Federation Hub (.cddmhub.toml).
pub fn render_hub_view(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let (repo_pane, detail_pane) = split_horizontal_2(area, 45, 55);
    render_hub_repo_table(frame, repo_pane);
    render_hub_matrix_and_candidates(frame, detail_pane);
}

fn render_hub_repo_table(frame: &mut Frame, area: Rect) {
    let repo_rows = [
        (
            "core-backend",
            "services/core-backend",
            "Rust 2024",
            "98.2",
            "0.8%",
        ),
        (
            "web-frontend",
            "apps/web-frontend",
            "React 19 / TS",
            "96.4",
            "1.4%",
        ),
        (
            "data-pipeline",
            "services/data-pipeline",
            "Python 3.12",
            "97.5",
            "0.5%",
        ),
        (
            "auth-gateway",
            "services/auth-gateway",
            "Go 1.22",
            "99.1",
            "0.2%",
        ),
    ];

    let header = Row::new(vec!["Repository", "Path", "Tech Stack", "DRY", "Cross-Dup"]).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = repo_rows
        .iter()
        .map(|(name, path, tech, dry, dup)| {
            Row::new(vec![
                Span::styled(
                    *name,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(*path, Style::default().fg(Color::DarkGray)),
                Span::styled(*tech, Style::default().fg(Color::White)),
                Span::styled(*dry, Style::default().fg(Color::Green)),
                Span::styled(*dup, Style::default().fg(Color::Magenta)),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(14),
            Constraint::Length(22),
            Constraint::Length(14),
            Constraint::Length(6),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(create_titled_block(
        " Federation Member Repositories (.cddmhub.toml) ",
        true,
    ));

    frame.render_widget(table, area);
}

fn render_hub_matrix_and_candidates(frame: &mut Frame, area: Rect) {
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(6)])
        .split(area);

    let summary_lines = vec![
        Line::from(vec![
            Span::styled(
                "Organization DRY Health: ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                "97.80 / 100.0 (A+)",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Inter-Repo Duplication:  ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                "2.20% across 4 repositories",
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Shared Token Overlap:    ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                "275 duplicated tokens identified",
                Style::default().fg(Color::Magenta),
            ),
        ]),
    ];

    let summary_block = Block::default()
        .title(Span::styled(
            " Organization DRY Health Metric ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let summary_p = Paragraph::new(summary_lines).block(summary_block);
    frame.render_widget(summary_p, right_chunks[0]);

    let candidate_lines = vec![
        Line::from(vec![
            Span::styled(
                "Cluster #1: ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("HTTP Exponential Backoff Strategy"),
        ]),
        Line::from(vec![
            Span::styled("  Members:   ", Style::default().fg(Color::DarkGray)),
            Span::raw("core-backend, data-pipeline (180 tokens)"),
        ]),
        Line::from(vec![
            Span::styled("  Synthesis: ", Style::default().fg(Color::Green)),
            Span::styled("@org/shared-http-utils", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Cluster #2: ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("JSON Schema Validation Helpers"),
        ]),
        Line::from(vec![
            Span::styled("  Members:   ", Style::default().fg(Color::DarkGray)),
            Span::raw("web-frontend, core-backend (95 tokens)"),
        ]),
        Line::from(vec![
            Span::styled("  Synthesis: ", Style::default().fg(Color::Green)),
            Span::styled("@org/shared-validation", Style::default().fg(Color::Cyan)),
        ]),
    ];

    let candidate_block = Block::default()
        .title(Span::styled(
            " Cross-Repo Extraction & Action Candidates ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let candidate_p = Paragraph::new(candidate_lines).block(candidate_block);
    frame.render_widget(candidate_p, right_chunks[1]);
}
