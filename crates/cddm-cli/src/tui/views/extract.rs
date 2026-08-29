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
        diff_del_line("--- a/Cargo.toml (Workspace Root)"),
        diff_add_line("+++ b/Cargo.toml"),
        Line::from("@@ -3,3 +3,4 @@ members = ["),
        diff_add_line("+    \"crates/cddm-shared-helpers\","),
        Line::from(" ]"),
    ];

    let block_manifest = create_titled_block(" Workspace Manifest Diff ", false);
    let p_manifest = Paragraph::new(manifest_lines)
        .block(block_manifest)
        .scroll((app.diff_scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });
    frame.render_widget(p_manifest, left_col);

    let caller_lines = vec![
        diff_del_line("--- a/crates/cddm-cli/src/main.rs"),
        diff_add_line("+++ b/crates/cddm-cli/src/main.rs"),
        Line::from("@@ -1,4 +1,5 @@"),
        diff_add_line("+use cddm_shared_helpers::deduplicated_tokens_validator;"),
        Line::from(" "),
        diff_del_line("-    let ok = local_raw_validator(&data);"),
        diff_add_line("+    let ok = deduplicated_tokens_validator(&data);"),
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
        styled_action_badge(" [x] Apply Shared Crate Extraction ", Color::Green),
        Span::raw("  "),
        styled_action_badge(" [d] Dry-Run Simulation ", Color::Cyan),
        Span::raw("  "),
        styled_action_badge(" [t] Synthesize Unit Tests ", Color::Yellow),
        Span::raw("  "),
        styled_action_badge(" [b] Synthesize Benchmarks ", Color::Magenta),
    ])];

    let block = create_titled_block(" Extraction Operations ", true);
    frame.render_widget(Paragraph::new(action_lines).block(block), area);
}
