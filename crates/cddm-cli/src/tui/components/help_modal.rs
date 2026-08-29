#![forbid(unsafe_code)]

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::tui::theme::TuiTheme;
use crate::tui::views::helpers::bold_span;

/// Render a centered help dialog detailing all keybindings.
pub fn render_help_modal(frame: &mut Frame) {
    let area = frame.area();
    let popup_area = centered_rect(65, 75, area);

    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" CDDM Studio — Keyboard Shortcuts Reference ")
        .title_alignment(Alignment::Center)
        .title_style(
            Style::default()
                .fg(TuiTheme::BRAND)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TuiTheme::BRAND));

    let shortcuts = vec![
        Line::from(vec![bold_span(" Navigation & Tabs:", Color::Yellow)]),
        Line::from("   1 - 8           Directly switch to tab 1 through 8"),
        Line::from("   Tab / l         Next tab"),
        Line::from("   Shift-Tab / h   Previous tab"),
        Line::from("   j / k (Down/Up) Navigate items / clones in list"),
        Line::from("   J / K (PgDn/Up) Scroll diff view down / up"),
        Line::from(""),
        Line::from(vec![bold_span(" Clone & Diff Actions:", Color::Yellow)]),
        Line::from("   c               Toggle Pairwise vs N-Way Clusters mode"),
        Line::from("   u               Toggle Split vs Unified diff mode"),
        Line::from("   r               Open Refactoring Sandbox for selected clone"),
        Line::from("   e               Open Shared Module Extractor for selected clone"),
        Line::from(""),
        Line::from(vec![bold_span(" Global Controls:", Color::Yellow)]),
        Line::from("   ?               Toggle this help popup dialog"),
        Line::from("   q / Ctrl+C      Exit CDDM Studio"),
        Line::from("   Esc             Close open modal or cancel"),
        Line::from(""),
        Line::from(vec![Span::styled(
            " Press Esc or ? to close this dialog.",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let paragraph = Paragraph::new(shortcuts)
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, popup_area);
}

/// Helper calculating centered rectangle with percentage width and height.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .flex(Flex::Center)
        .split(popup_layout[1])[1]
}
