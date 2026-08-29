#![forbid(unsafe_code)]

use super::helpers::*;
use crate::tui::app::TuiApp;

/// Render Tab 4: Refactoring Sandbox, AST Rewriter & Autonomous AI Surgeon.
pub fn render_refactor_view(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (top_section, mid_section, bot_section) = split_vertical_3(area, 5, 4);

    render_signature_and_params(frame, app, top_section);
    render_patch_preview(frame, app, mid_section);
    render_action_controls(frame, app, bot_section);
}

fn render_signature_and_params(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let pair = app.current_clone_pair();
    let (fn_sig, variance_info) = if let Some(p) = pair {
        (
            format!(
                "pub fn deduplicated_{}_helper(data: &str) -> bool",
                p.clone_type.as_str().to_lowercase()
            ),
            format!(
                "Site 1: {} ({}-{}) | Site 2: {} ({}-{})",
                p.file_a, p.start_line_a, p.end_line_a, p.file_b, p.start_line_b, p.end_line_b
            ),
        )
    } else {
        (
            "No clone selected (select a clone in Tab 2)".to_string(),
            "Site 1: N/A | Site 2: N/A".to_string(),
        )
    };

    let lines = vec![
        styled_kv_line("Target Function:  ", &fn_sig, Color::Cyan),
        styled_kv_line("Variance Map:     ", &variance_info, Color::Yellow),
    ];

    let block = create_titled_block(" Refactoring Target Specification ", false);
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn render_patch_preview(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let patch_lines = if let Some(p) = app.current_clone_pair() {
        vec![
            diff_del_line(format!("--- a/{}", p.file_a)),
            diff_add_line(format!("+++ b/{}", p.file_a)),
            Line::from(format!(
                "@@ -{},{} +{},1 @@",
                p.start_line_a,
                p.end_line_a.saturating_sub(p.start_line_a) + 1,
                p.start_line_a
            )),
            diff_del_line(format!(
                "-    /* Duplicate block ({} tokens) */",
                p.token_count
            )),
            diff_add_line(format!(
                "+    let result = deduplicated_{}_helper(data);",
                p.clone_type.as_str().to_lowercase()
            )),
            Line::from(" "),
            diff_del_line(format!("--- a/{}", p.file_b)),
            diff_add_line(format!("+++ b/{}", p.file_b)),
            Line::from(format!(
                "@@ -{},{} +{},1 @@",
                p.start_line_b,
                p.end_line_b.saturating_sub(p.start_line_b) + 1,
                p.start_line_b
            )),
            diff_del_line(format!(
                "-    /* Duplicate block ({} tokens) */",
                p.token_count
            )),
            diff_add_line(format!(
                "+    let valid = deduplicated_{}_helper(source_data);",
                p.clone_type.as_str().to_lowercase()
            )),
        ]
    } else {
        vec![
            Line::from("No clone selected for refactor preview."),
            Line::from(
                "Switch to Tab 2 (Clones) and select a clone pair to preview refactoring patch.",
            ),
        ]
    };

    render_scrolled_diff_panel(
        frame,
        patch_lines,
        " Synthesized Multi-File Git Diff Patch ",
        app.diff_scroll_offset,
        area,
    );
}

fn render_action_controls(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let badges = [
        (" [r] Apply to Git Branch ", Color::Green),
        (" [p] Synthesize AI Refactor Prompt ", Color::Cyan),
        (" [a] Run AI Code Surgeon (Heal) ", Color::Magenta),
    ];
    render_action_bar(frame, &badges, " Interactive Deduplication Actions ", area);
}
