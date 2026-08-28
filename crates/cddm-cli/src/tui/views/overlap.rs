#![forbid(unsafe_code)]

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Gauge, List, ListItem, Paragraph};

use super::helpers::{create_titled_block, split_horizontal_2, split_vertical_header_body};
use crate::tui::app::TuiApp;
use cddm_core::get_canonical_algorithms;

/// Render Tab 9: Ecosystem Library Reimplementation & Overlap Detector.
pub fn render_overlap_view(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let (list_pane, detail_pane) = split_horizontal_2(area, 40, 60);
    render_overlap_list(frame, list_pane);
    render_overlap_details(frame, detail_pane);
}

fn render_overlap_list(frame: &mut Frame, area: Rect) {
    let algos = get_canonical_algorithms();
    let mut items = Vec::new();

    for (i, algo) in algos.iter().enumerate() {
        let prefix = if i == 0 { " > " } else { "   " };
        let style = if i == 0 {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan)
        };

        items.push(ListItem::new(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(
                format!("[{}] ", algo.category),
                Style::default().fg(Color::DarkGray),
            ),
            Span::raw(&algo.name),
        ])));
    }

    let block = create_titled_block(" Ecosystem Library Overlap Rules ", true);
    frame.render_widget(List::new(items).block(block), area);
}

fn render_overlap_details(frame: &mut Frame, area: Rect) {
    let (top_pane, bottom_pane) = split_vertical_header_body(area, 6);

    let block_top =
        create_titled_block(" Detected Algorithm: Array Chunking (Collections) ", false);
    frame.render_widget(block_top, top_pane);

    let gauge_area = Rect::new(
        top_pane.x + 2,
        top_pane.y + 2,
        top_pane.width.saturating_sub(4),
        2,
    );

    let g = Gauge::default()
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(0.95)
        .label("Detection Confidence: 95.0% (Matched keywords: chunk, chunks, batch)");
    frame.render_widget(g, gauge_area);

    let info_lines = vec![
        Line::from(vec![
            Span::styled("Description: ", Style::default().fg(Color::Yellow)),
            Span::raw("Splitting an array or slice into fixed-size contiguous chunks."),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Recommended Replacement: ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "itertools (Rust) / lodash-es (TS)",
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Install Command:         ",
                Style::default().fg(Color::Magenta),
            ),
            Span::styled("cargo add itertools", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "--- Replacement Code Snippet ---",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from("use itertools::Itertools;"),
        Line::from("let chunks = items.iter().chunks(chunk_size);"),
    ];

    let block_bottom = create_titled_block(" Recommendation & Migration ", false);
    let p = Paragraph::new(info_lines).block(block_bottom);
    frame.render_widget(p, bottom_pane);
}
