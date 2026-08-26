#![forbid(unsafe_code)]

use super::helpers::*;
use crate::tui::app::TuiApp;

/// Render Tab 4: Refactoring Sandbox, AST Rewriter & Autonomous AI Surgeon.
pub fn render_refactor_view(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (top_pane, mid_pane, bot_pane) = split_vertical_3(area, 5, 4);
    render_signature_and_params(frame, app, top_pane);
    render_patch_preview(frame, app, mid_pane);
    render_action_controls(frame, app, bot_pane);
}

fn render_signature_and_params(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let pair = app.current_clone_pair();
    let fn_sig = if pair.is_some() {
        "pub fn deduplicated_helper(param_a: &str, count: usize) -> bool"
    } else {
        "No clone selected (select a clone in Tab 2)"
    };

    let lines = vec![
        styled_kv_line("Target Function:  ", fn_sig, Color::Cyan),
        styled_kv_line(
            "Variance Map:     ",
            "Site 1: text_input (string) | Site 2: data_source (string)",
            Color::Yellow,
        ),
    ];

    let block = create_titled_block(" Refactoring Target Specification ", false);
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn render_patch_preview(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let patch_lines = vec![
        Line::from(vec![Span::styled(
            "--- a/crates/cddm-core/src/detector.rs",
            Style::default().fg(Color::Red),
        )]),
        Line::from(vec![Span::styled(
            "+++ b/crates/cddm-core/src/detector.rs",
            Style::default().fg(Color::Green),
        )]),
        Line::from("@@ -42,7 +42,7 @@"),
        Line::from(vec![Span::styled(
            "-    let result = raw_input.split_whitespace().count() > 0;",
            Style::default().fg(Color::Red),
        )]),
        Line::from(vec![Span::styled(
            "+    let result = deduplicated_helper(raw_input, 0);",
            Style::default().fg(Color::Green),
        )]),
        Line::from(" "),
        Line::from(vec![Span::styled(
            "--- a/crates/cddm-core/src/tokenizer.rs",
            Style::default().fg(Color::Red),
        )]),
        Line::from(vec![Span::styled(
            "+++ b/crates/cddm-core/src/tokenizer.rs",
            Style::default().fg(Color::Green),
        )]),
        Line::from("@@ -88,7 +88,7 @@"),
        Line::from(vec![Span::styled(
            "-    let valid = source_data.split_whitespace().count() > 0;",
            Style::default().fg(Color::Red),
        )]),
        Line::from(vec![Span::styled(
            "+    let valid = deduplicated_helper(source_data, 0);",
            Style::default().fg(Color::Green),
        )]),
    ];

    let block = create_titled_block(" Synthesized Multi-File Git Diff Patch ", false);
    let p = Paragraph::new(patch_lines)
        .block(block)
        .scroll((app.diff_scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });

    frame.render_widget(p, area);
}

fn render_action_controls(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let action_lines = vec![Line::from(vec![
        Span::styled(
            " [r] Apply to Git Branch ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            " [p] Synthesize AI Refactor Prompt ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            " [a] Run AI Code Surgeon (Heal) ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
    ])];

    let block = create_titled_block(" Interactive Deduplication Actions ", true);
    frame.render_widget(Paragraph::new(action_lines).block(block), area);
}
