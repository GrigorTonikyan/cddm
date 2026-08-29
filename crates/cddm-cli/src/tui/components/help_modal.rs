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

    let mut shortcuts = Vec::new();
    append_shortcut_section(
        &mut shortcuts,
        " Navigation & Tabs:",
        &[
            ("1 - 9, 0, C", "Directly switch tabs (1 to 11)"),
            ("Tab / l", "Next tab"),
            ("Shift-Tab / h", "Previous tab"),
            ("j / k (Down/Up)", "Navigate items / clones in list"),
            ("J / K (PgDn/Up)", "Scroll diff view down / up"),
        ],
    );
    append_shortcut_section(
        &mut shortcuts,
        " Clone & Diff Actions:",
        &[
            ("c", "Toggle Pairwise vs N-Way Clusters mode"),
            ("u", "Toggle Split vs Unified diff mode"),
            ("r", "Open Refactoring Sandbox for selected clone"),
            ("e", "Open Shared Module Extractor for selected clone"),
        ],
    );
    append_shortcut_section(
        &mut shortcuts,
        " Global Controls:",
        &[
            ("?", "Toggle this help popup dialog"),
            ("q / Ctrl+C", "Exit CDDM Studio"),
            ("Esc", "Close open modal or cancel"),
        ],
    );
    shortcuts.push(Line::from(vec![Span::styled(
        " Press Esc or ? to close this dialog.",
        Style::default().fg(Color::DarkGray),
    )]));

    let paragraph = Paragraph::new(shortcuts)
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, popup_area);
}

fn append_shortcut_section(
    lines: &mut Vec<Line<'static>>,
    header: &'static str,
    bindings: &[(&'static str, &'static str)],
) {
    lines.push(Line::from(vec![bold_span(header, Color::Yellow)]));
    for (key, desc) in bindings {
        lines.push(Line::from(format!("   {:<16}{}", key, desc)));
    }
    lines.push(Line::from(""));
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
