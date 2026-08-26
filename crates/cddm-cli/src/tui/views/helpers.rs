#![forbid(unsafe_code)]

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders};

pub use ratatui::Frame;
pub use ratatui::layout::Rect;
pub use ratatui::style::{Color, Modifier, Style};
pub use ratatui::text::{Line, Span};
pub use ratatui::widgets::{Paragraph, Wrap};

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
        Span::raw(value.to_string()),
    ])
}
