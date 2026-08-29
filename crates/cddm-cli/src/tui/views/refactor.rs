#![forbid(unsafe_code)]

use super::helpers::*;
use crate::tui::app::TuiApp;

/// Render Tab 4: Refactoring Sandbox, AST Rewriter & Autonomous AI Surgeon.
pub fn render_refactor_view(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let [top_section, mid_section, bot_section] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Min(4),
        Constraint::Length(4),
    ])
    .areas(area);

    render_signature_and_params(frame, app, top_section);
    render_patch_preview(frame, app, mid_section);
    render_action_controls(frame, app, bot_section);
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
        diff_del_line("--- a/crates/cddm-core/src/detector.rs"),
        diff_add_line("+++ b/crates/cddm-core/src/detector.rs"),
        Line::from("@@ -42,7 +42,7 @@"),
        diff_del_line("-    let result = raw_input.split_whitespace().count() > 0;"),
        diff_add_line("+    let result = deduplicated_helper(raw_input, 0);"),
        Line::from(" "),
        diff_del_line("--- a/crates/cddm-core/src/tokenizer.rs"),
        diff_add_line("+++ b/crates/cddm-core/src/tokenizer.rs"),
        Line::from("@@ -88,7 +88,7 @@"),
        diff_del_line("-    let valid = source_data.split_whitespace().count() > 0;"),
        diff_add_line("+    let valid = deduplicated_helper(source_data, 0);"),
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
        styled_action_badge(" [r] Apply to Git Branch ", Color::Green),
        Span::raw("  "),
        styled_action_badge(" [p] Synthesize AI Refactor Prompt ", Color::Cyan),
        Span::raw("  "),
        styled_action_badge(" [a] Run AI Code Surgeon (Heal) ", Color::Magenta),
    ])];

    let block = create_titled_block(" Interactive Deduplication Actions ", true);
    frame.render_widget(Paragraph::new(action_lines).block(block), area);
}
