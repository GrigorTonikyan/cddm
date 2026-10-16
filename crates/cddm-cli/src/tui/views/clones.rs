#![forbid(unsafe_code)]

use std::path::Path;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

use super::helpers::{
    bold_span, create_titled_block, diff_add_line, diff_del_line, split_horizontal_2,
    split_vertical_header_body,
};
use crate::tui::app::{CloneViewMode, DiffMode, TuiApp};
use crate::tui::theme::TuiTheme;

/// Render Tab 2: Clone Explorer and Side-by-Side Split Diff Visualizer.
pub fn render_clones_view(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (left_pane, right_pane) = split_horizontal_2(area, 40, 60);
    render_clone_list(frame, app, left_pane);
    render_diff_viewer(frame, app, right_pane);
}

fn render_clone_list(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let title = match app.clone_view_mode {
        CloneViewMode::Pairwise => " Pairwise Clones (Press 'c' for Clusters) ",
        CloneViewMode::Clusters => " N-Way Clone Clusters (Press 'c' for Pairwise) ",
    };

    let items: Vec<ListItem> = match app.clone_view_mode {
        CloneViewMode::Pairwise => {
            if let Some(res) = &app.scan_result {
                res.clone_pairs
                    .iter()
                    .enumerate()
                    .map(|(idx, pair)| {
                        let is_selected = idx == app.selected_clone_idx;
                        let type_badge = format!("[{:?}]", pair.clone_type);
                        let file_a_name = Path::new(&pair.file_a)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("file_a");
                        let file_b_name = Path::new(&pair.file_b)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("file_b");

                        let line_text = format!(
                            "{:<10} {:>3} tok | {}:{} <-> {}:{}",
                            type_badge,
                            pair.token_count,
                            file_a_name,
                            pair.start_line_a,
                            file_b_name,
                            pair.start_line_b
                        );

                        if is_selected {
                            ListItem::new(Line::from(vec![Span::styled(
                                format!(" > {}", line_text),
                                TuiTheme::selected_item_style(),
                            )]))
                        } else {
                            ListItem::new(Line::from(vec![Span::raw(format!("   {}", line_text))]))
                        }
                    })
                    .collect()
            } else {
                vec![ListItem::new("   No scan results available.")]
            }
        }
        CloneViewMode::Clusters => {
            if app.clusters.is_empty() {
                vec![ListItem::new("   No clusters formed.")]
            } else {
                app.clusters
                    .iter()
                    .enumerate()
                    .map(|(idx, cluster)| {
                        let is_selected = idx == app.selected_cluster_idx;
                        let line_text = format!(
                            "Cluster #{:<2} ({} sites, {:>3} tokens, {:.0}% sim)",
                            cluster.id,
                            cluster.occurrences.len(),
                            cluster.token_count,
                            cluster.similarity * 100.0
                        );

                        if is_selected {
                            ListItem::new(Line::from(vec![Span::styled(
                                format!(" > {}", line_text),
                                TuiTheme::selected_item_style(),
                            )]))
                        } else {
                            ListItem::new(Line::from(vec![Span::raw(format!("   {}", line_text))]))
                        }
                    })
                    .collect()
            }
        }
    };

    let block = create_titled_block(title, true);
    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn render_diff_viewer(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let diff_title = match app.diff_mode {
        DiffMode::Split => " Side-by-Side Split Diff (Press 'u' for Unified) ",
        DiffMode::Unified => " Unified Refactor Diff (Press 'u' for Split) ",
    };

    let block = create_titled_block(diff_title, false);
    let current_pair = app.current_clone_pair();

    if let Some(pair) = current_pair {
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let (header_area, diff_area) = split_vertical_header_body(inner, 4);

        let header_lines = vec![
            Line::from(vec![
                Span::styled("Fragment A: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!(
                    "{} (lines {}-{})",
                    pair.file_a, pair.start_line_a, pair.end_line_a
                )),
            ]),
            Line::from(vec![
                Span::styled("Fragment B: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!(
                    "{} (lines {}-{})",
                    pair.file_b, pair.start_line_b, pair.end_line_b
                )),
                Span::raw(" | "),
                Span::styled(
                    format!("{:.1}% similarity", pair.similarity * 100.0),
                    Style::default().fg(Color::Green),
                ),
            ]),
        ];

        let header_p = Paragraph::new(header_lines).block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(TuiTheme::border_style()),
        );
        frame.render_widget(header_p, header_area);

        match app.diff_mode {
            DiffMode::Split => {
                let (left_col, right_col) = split_horizontal_2(diff_area, 50, 50);
                let snippet_a =
                    read_source_snippet(&pair.file_a, pair.start_line_a, pair.end_line_a);
                let snippet_b =
                    read_source_snippet(&pair.file_b, pair.start_line_b, pair.end_line_b);

                let mut left_content = vec![Line::from(vec![Span::styled(
                    format!(
                        "--- Fragment A: {} ({}-{}) ---",
                        pair.file_a, pair.start_line_a, pair.end_line_a
                    ),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )])];
                for line in snippet_a {
                    left_content.push(Line::from(line));
                }

                let mut right_content = vec![Line::from(vec![Span::styled(
                    format!(
                        "+++ Fragment B: {} ({}-{}) +++",
                        pair.file_b, pair.start_line_b, pair.end_line_b
                    ),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )])];
                for line in snippet_b {
                    right_content.push(Line::from(line));
                }

                let left_p = Paragraph::new(left_content)
                    .block(Block::default().borders(Borders::RIGHT))
                    .scroll((app.diff_scroll_offset as u16, 0))
                    .wrap(Wrap { trim: false });

                let right_p = Paragraph::new(right_content)
                    .scroll((app.diff_scroll_offset as u16, 0))
                    .wrap(Wrap { trim: false });

                frame.render_widget(left_p, left_col);
                frame.render_widget(right_p, right_col);
            }
            DiffMode::Unified => {
                let snippet_a =
                    read_source_snippet(&pair.file_a, pair.start_line_a, pair.end_line_a);
                let snippet_b =
                    read_source_snippet(&pair.file_b, pair.start_line_b, pair.end_line_b);

                let mut unified_lines = vec![Line::from(vec![bold_span(
                    format!(
                        "@@ Deduplication Refactoring: {} <-> {} @@",
                        pair.file_a, pair.file_b
                    ),
                    Color::Magenta,
                )])];
                for line in snippet_a {
                    unified_lines.push(diff_del_line(format!("- {}", line)));
                }
                for line in snippet_b {
                    unified_lines.push(diff_add_line(format!("+ {}", line)));
                }

                let unified_p =
                    Paragraph::new(unified_lines).scroll((app.diff_scroll_offset as u16, 0));
                frame.render_widget(unified_p, diff_area);
            }
        }
    } else {
        let empty_p = Paragraph::new("Select a clone pair from the left list to view split diff.")
            .block(block);
        frame.render_widget(empty_p, area);
    }
}

fn read_source_snippet(path: &str, start_line: usize, end_line: usize) -> Vec<String> {
    if let Ok(content) = std::fs::read_to_string(path) {
        let lines: Vec<&str> = content.lines().collect();
        let start = start_line.saturating_sub(1);
        let end = end_line.min(lines.len());
        if start < lines.len() && start <= end {
            return lines[start..end]
                .iter()
                .enumerate()
                .map(|(i, l)| format!("{} | {}", start + i + 1, l))
                .collect();
        }
    }
    vec![
        format!("// Fragment site (lines {}-{})", start_line, end_line),
        format!("// File: {}", path),
    ]
}
