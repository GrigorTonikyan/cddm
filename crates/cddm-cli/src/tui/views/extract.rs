#![forbid(unsafe_code)]

use super::helpers::*;
use crate::tui::app::TuiApp;

/// Render Tab 5: Shared Module & Crate Extractor (`cddm extract`).
pub fn render_extract_view(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (top_pane, mid_pane, bot_pane) = split_vertical_3(area, 5, 4);
    render_target_config(frame, app, top_pane);
    render_extraction_diffs(frame, app, mid_pane);
    render_extraction_actions(frame, app, bot_pane);
}

fn render_target_config(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let lines = vec![
        styled_kv_line(
            "Target Crate:     ",
            "crates/cddm-shared-helpers (Cargo Workspace Member)",
            Color::Cyan,
        ),
        styled_kv_line(
            "Inferred Export:  ",
            "pub fn deduplicated_tokens_validator(src: &str) -> bool",
            Color::Yellow,
        ),
    ];

    let block = create_titled_block(" Target Shared Module Configuration ", false);
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn render_extraction_diffs(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (left_col, right_col) = split_horizontal_2(area, 50, 50);

    let manifest_lines = vec![
        Line::from(vec![Span::styled(
            "--- a/Cargo.toml (Workspace Root)",
            Style::default().fg(Color::Red),
        )]),
        Line::from(vec![Span::styled(
            "+++ b/Cargo.toml",
            Style::default().fg(Color::Green),
        )]),
        Line::from("@@ -3,3 +3,4 @@ members = ["),
        Line::from(vec![Span::styled(
            "+    \"crates/cddm-shared-helpers\",",
            Style::default().fg(Color::Green),
        )]),
        Line::from(" ]"),
    ];

    let block_manifest = create_titled_block(" Workspace Manifest Diff ", false);
    let p_manifest = Paragraph::new(manifest_lines)
        .block(block_manifest)
        .scroll((app.diff_scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });
    frame.render_widget(p_manifest, left_col);

    let caller_lines = vec![
        Line::from(vec![Span::styled(
            "--- a/crates/cddm-cli/src/main.rs",
            Style::default().fg(Color::Red),
        )]),
        Line::from(vec![Span::styled(
            "+++ b/crates/cddm-cli/src/main.rs",
            Style::default().fg(Color::Green),
        )]),
        Line::from("@@ -1,4 +1,5 @@"),
        Line::from(vec![Span::styled(
            "+use cddm_shared_helpers::deduplicated_tokens_validator;",
            Style::default().fg(Color::Green),
        )]),
        Line::from(" "),
        Line::from(vec![Span::styled(
            "-    let ok = local_raw_validator(&data);",
            Style::default().fg(Color::Red),
        )]),
        Line::from(vec![Span::styled(
            "+    let ok = deduplicated_tokens_validator(&data);",
            Style::default().fg(Color::Green),
        )]),
    ];

    let block_caller = create_titled_block(" Caller Injected Imports & Callsite Diff ", false);
    let p_caller = Paragraph::new(caller_lines)
        .block(block_caller)
        .scroll((app.diff_scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });
    frame.render_widget(p_caller, right_col);
}

fn render_extraction_actions(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let action_lines = vec![Line::from(vec![
        Span::styled(
            " [x] Apply Shared Crate Extraction ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            " [d] Dry-Run Simulation ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            " [t] Synthesize Unit Tests ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            " [b] Synthesize Benchmarks ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
    ])];

    let block = create_titled_block(" Extraction Operations ", true);
    frame.render_widget(Paragraph::new(action_lines).block(block), area);
}
