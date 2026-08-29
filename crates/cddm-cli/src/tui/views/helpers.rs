#![forbid(unsafe_code)]

pub use ratatui::Frame;
pub use ratatui::layout::{Constraint, Direction, Layout, Rect};
pub use ratatui::style::{Color, Modifier, Style};
pub use ratatui::text::{Line, Span};
pub use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::tui::theme::TuiTheme;

/// Helper to create standard bordered blocks with consistent styling.
pub fn create_titled_block<'a>(title: &'a str, focused: bool) -> Block<'a> {
    let border_style = if focused {
        TuiTheme::focused_border_style()
    } else {
        TuiTheme::border_style()
    };

    Block::default()
        .title(Span::styled(title, TuiTheme::title_style()))
        .borders(Borders::ALL)
        .border_style(border_style)
}

/// Helper to split an area horizontally into two percentage columns.
pub fn split_horizontal_2(area: Rect, left_pct: u16, right_pct: u16) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(left_pct),
            Constraint::Percentage(right_pct),
        ])
        .split(area);
    (chunks[0], chunks[1])
}

/// Helper to split an area vertically into 3 sections (top fixed, middle flexible, bottom fixed).
pub fn split_vertical_3(area: Rect, top_len: u16, bot_len: u16) -> (Rect, Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(top_len),
            Constraint::Min(4),
            Constraint::Length(bot_len),
        ])
        .split(area);
    (chunks[0], chunks[1], chunks[2])
}

/// Helper to split an area vertically into header and body sections.
pub fn split_vertical_header_body(area: Rect, header_height: u16) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(header_height), Constraint::Min(4)])
        .split(area);
    (chunks[0], chunks[1])
}

/// Helper to build a styled label-value Line.
pub fn styled_kv_line(label: &str, value: &str, label_color: Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(label.to_string(), Style::default().fg(label_color)),
        Span::styled(value.to_string(), Style::default().fg(Color::White)),
    ])
}

/// Helper to build a styled action button badge for TUI footers.
pub fn styled_action_badge(label: &'static str, bg: Color) -> Span<'static> {
    Span::styled(
        label,
        Style::default()
            .fg(Color::Black)
            .bg(bg)
            .add_modifier(Modifier::BOLD),
    )
}

/// Helper to build a bold colored span.
pub fn bold_span(text: impl Into<String>, fg: Color) -> Span<'static> {
    Span::styled(
        text.into(),
        Style::default().fg(fg).add_modifier(Modifier::BOLD),
    )
}

/// Helper for a green diff addition line.
pub fn diff_add_line(text: impl Into<String>) -> Line<'static> {
    Line::from(vec![Span::styled(
        text.into(),
        Style::default().fg(Color::Green),
    )])
}

/// Helper for a red diff deletion line.
pub fn diff_del_line(text: impl Into<String>) -> Line<'static> {
    Line::from(vec![Span::styled(
        text.into(),
        Style::default().fg(Color::Red),
    )])
}

/// Helper to render a scrolled diff panel with consistent wrapping and styling.
pub fn render_scrolled_diff_panel(
    frame: &mut Frame,
    lines: Vec<Line<'static>>,
    title: &str,
    scroll_offset: usize,
    area: Rect,
) {
    let block = create_titled_block(title, false);
    let p = Paragraph::new(lines)
        .block(block)
        .scroll((scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });
    frame.render_widget(p, area);
}

/// Helper to render an action button bar with styled badges inside a titled block.
pub fn render_action_bar(
    frame: &mut Frame,
    badges: &[(&'static str, Color)],
    title: &str,
    area: Rect,
) {
    let mut spans = Vec::new();
    for (i, (label, color)) in badges.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(styled_action_badge(label, *color));
    }
    let block = create_titled_block(title, true);
    frame.render_widget(Paragraph::new(vec![Line::from(spans)]).block(block), area);
}

/// Helper to render a two-line key-value config block inside a titled container.
#[allow(clippy::too_many_arguments)]
pub fn render_two_line_kv_box(
    frame: &mut Frame,
    title: &str,
    k1: &str,
    v1: &str,
    c1: Color,
    k2: &str,
    v2: &str,
    c2: Color,
    area: Rect,
) {
    let lines = vec![styled_kv_line(k1, v1, c1), styled_kv_line(k2, v2, c2)];
    let block = create_titled_block(title, false);
    frame.render_widget(Paragraph::new(lines).block(block), area);
}
