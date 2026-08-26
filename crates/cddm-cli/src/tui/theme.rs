#![forbid(unsafe_code)]

use ratatui::style::{Color, Modifier, Style};

/// Centralized color palette and styling definitions for CDDM Terminal UI.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct TuiTheme;

#[allow(dead_code)]
impl TuiTheme {
    // Brand & Primary
    pub const BRAND: Color = Color::Cyan;
    pub const ACCENT: Color = Color::LightCyan;
    pub const BG_DARK: Color = Color::Reset;

    // Semantic States
    pub const SUCCESS: Color = Color::Green;
    pub const WARNING: Color = Color::Yellow;
    pub const DANGER: Color = Color::Red;
    pub const INFO: Color = Color::Blue;
    pub const MUTED: Color = Color::DarkGray;
    pub const HIGHLIGHT: Color = Color::Magenta;
    pub const TEXT_MAIN: Color = Color::White;

    /// Style for primary window headers and active tabs
    pub fn active_tab_style() -> Style {
        Style::default()
            .fg(Color::Black)
            .bg(Self::BRAND)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for inactive tabs
    pub fn inactive_tab_style() -> Style {
        Style::default().fg(Color::Gray)
    }

    /// Style for section titles
    pub fn title_style() -> Style {
        Style::default()
            .fg(Self::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for highlighted / selected list items
    pub fn selected_item_style() -> Style {
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightBlue)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for borders
    pub fn border_style() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    /// Style for active / focused borders
    pub fn focused_border_style() -> Style {
        Style::default().fg(Self::BRAND)
    }

    /// Color coding for DRY health scores
    pub fn dry_score_color(score: f64) -> Color {
        if score >= 90.0 {
            Self::SUCCESS
        } else if score >= 75.0 {
            Color::LightGreen
        } else if score >= 60.0 {
            Self::WARNING
        } else {
            Self::DANGER
        }
    }

    /// Badge text label for DRY health tiers
    pub fn dry_score_tier(score: f64) -> &'static str {
        if score >= 90.0 {
            "EXCELLENT"
        } else if score >= 75.0 {
            "GOOD"
        } else if score >= 60.0 {
            "FAIR"
        } else {
            "CRITICAL"
        }
    }
}
