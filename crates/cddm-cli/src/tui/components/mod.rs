#![forbid(unsafe_code)]

pub mod help_modal;

use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};

use crate::tui::app::{TuiApp, TuiTab};
use crate::tui::theme::TuiTheme;

/// Render top header with branding and tab selector.
pub fn render_header_tabs(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let titles: Vec<Line> = TuiTab::ALL
        .iter()
        .map(|tab| {
            if *tab == app.active_tab {
                Line::from(vec![Span::styled(
                    format!(" {} ", tab.title()),
                    TuiTheme::active_tab_style(),
                )])
            } else {
                Line::from(vec![Span::styled(
                    format!(" {} ", tab.title()),
                    TuiTheme::inactive_tab_style(),
                )])
            }
        })
        .collect();

    let title_block = Block::default()
        .title(" CDDM — Code De-Duplication Meister ")
        .title_style(
            Style::default()
                .fg(TuiTheme::BRAND)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(TuiTheme::border_style());

    let tabs = Tabs::new(titles)
        .block(title_block)
        .select(app.active_tab as usize)
        .highlight_style(TuiTheme::active_tab_style())
        .divider(Span::raw("|"));

    frame.render_widget(tabs, area);
}

/// Render bottom status bar with notifications and shortcut hints.
pub fn render_status_bar(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let status_text = app
        .status_message
        .as_ref()
        .map(|(msg, _)| msg.as_str())
        .unwrap_or("Ready. Press ? for shortcuts.");

    let hints = vec![
        Span::styled(" [? Help] ", Style::default().fg(Color::Yellow)),
        Span::styled("[Tab Next] ", Style::default().fg(Color::Cyan)),
        Span::styled("[j/k Nav] ", Style::default().fg(Color::Cyan)),
        Span::styled("[c Clusters] ", Style::default().fg(Color::Cyan)),
        Span::styled("[u Diff] ", Style::default().fg(Color::Cyan)),
        Span::styled("[r Refactor] ", Style::default().fg(Color::Cyan)),
        Span::styled("[e Extract] ", Style::default().fg(Color::Cyan)),
        Span::styled("[q Quit] ", Style::default().fg(Color::Red)),
        Span::raw(" | "),
        Span::styled(
            status_text,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(TuiTheme::border_style());

    let paragraph = Paragraph::new(Line::from(hints))
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}
