#![forbid(unsafe_code)]

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Gauge, List, ListItem, Paragraph};

use super::helpers::{create_titled_block, split_horizontal_2, split_vertical_header_body};
use crate::tui::app::TuiApp;

/// Render Tab 3: Cross-Language Semantic Matching & Hybrid Embeddings Explorer.
pub fn render_semantic_view(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (list_pane, detail_pane) = split_horizontal_2(area, 40, 60);
    render_semantic_list(frame, app, list_pane);
    render_semantic_details(frame, app, detail_pane);
}

fn render_semantic_list(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let items = vec![
        ListItem::new(Line::from(vec![
            Span::styled(
                " > [Polyglot] ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Rust <-> Python (tokenize_stream vs parse_tokens)"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("   [Polyglot] ", Style::default().fg(Color::Magenta)),
            Span::raw("TypeScript <-> Go (validate_jwt vs CheckAuthToken)"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("   [Polyglot] ", Style::default().fg(Color::Magenta)),
            Span::raw("Java <-> C# (CalculateMetrics vs ComputeStats)"),
        ])),
    ];

    let block = create_titled_block(" Cross-Language Semantic Duplicates ", true);
    frame.render_widget(List::new(items).block(block), area);
}

fn render_semantic_details(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (gauge_area, diff_area) = split_vertical_header_body(area, 7);

    // Hybrid Formula Display
    let block_gauge = create_titled_block(
        " Hybrid Similarity Model: S_hybrid = 0.5 * S_graph + 0.5 * S_token ",
        false,
    );

    let gauge_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .split(block_gauge.inner(gauge_area));

    frame.render_widget(block_gauge, gauge_area);

    let g_hybrid = Gauge::default()
        .gauge_style(Style::default().fg(Color::Magenta))
        .ratio(0.88)
        .label("Combined Hybrid Score: 88.0%");
    let g_graph = Gauge::default()
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(0.92)
        .label("Weisfeiler-Lehman Graph Isomorphism: 92.0%");
    let g_token = Gauge::default()
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(0.84)
        .label("Subword 3-Gram Vector Cosine Similarity: 84.0%");

    frame.render_widget(g_hybrid, gauge_layout[0]);
    frame.render_widget(g_graph, gauge_layout[1]);
    frame.render_widget(g_token, gauge_layout[2]);

    // Polyglot Code Snippet Comparison
    let (left_col, right_col) = split_horizontal_2(diff_area, 50, 50);

    let rust_code = vec![
        Line::from(vec![Span::styled(
            "--- Rust Source (crates/cddm-core/src/...) ---",
            Style::default().fg(Color::Cyan),
        )]),
        Line::from("pub fn tokenize_stream(src: &str) -> Vec<Token> {"),
        Line::from("    let mut tokens = Vec::new();"),
        Line::from("    for item in src.split_whitespace() {"),
        Line::from("        if !item.is_empty() {"),
        Line::from("            tokens.push(Token::from(item));"),
        Line::from("        }"),
        Line::from("    }"),
        Line::from("    tokens"),
        Line::from("}"),
    ];

    let py_code = vec![
        Line::from(vec![Span::styled(
            "+++ Python Source (sdk/python/cddm/...) +++",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from("def parse_tokens(source_code: str) -> list[Token]:"),
        Line::from("    token_list = []"),
        Line::from("    for segment in source_code.split():"),
        Line::from("        if segment:"),
        Line::from("            token_list.append(Token(segment))"),
        Line::from("    return token_list"),
    ];

    let block_left = create_titled_block(" Language A: Rust ", false);
    let block_right = create_titled_block(" Language B: Python ", false);

    let p_left = Paragraph::new(rust_code)
        .block(block_left)
        .scroll((app.diff_scroll_offset as u16, 0));
    let p_right = Paragraph::new(py_code)
        .block(block_right)
        .scroll((app.diff_scroll_offset as u16, 0));

    frame.render_widget(p_left, left_col);
    frame.render_widget(p_right, right_col);
}
