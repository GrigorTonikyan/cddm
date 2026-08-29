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

fn render_target_config(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let pair = app.current_clone_pair();
    let (target_crate, inferred_export) = if let Some(p) = pair {
        (
            format!("crates/cddm_shared_utils (Extracting from {})", p.file_a),
            format!(
                "pub fn deduplicated_{}_helper(src: &str) -> bool",
                p.clone_type.as_str().to_lowercase()
            ),
        )
    } else {
        (
            "crates/cddm_shared_utils (Select a clone in Tab 2)".to_string(),
            "pub fn deduplicated_helper(src: &str) -> bool".to_string(),
        )
    };

    render_two_line_kv_box(
        frame,
        " Target Shared Module Configuration ",
        "Target Crate:     ",
        &target_crate,
        Color::Cyan,
        "Inferred Export:  ",
        &inferred_export,
        Color::Yellow,
        area,
    );
}

fn render_extraction_diffs(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (left_col, right_col) = split_horizontal_2(area, 50, 50);

    let manifest_lines = vec![
        diff_del_line("--- a/Cargo.toml (Workspace Root)"),
        diff_add_line("+++ b/Cargo.toml"),
        Line::from("@@ -3,3 +3,4 @@ members = ["),
        diff_add_line("+    \"crates/cddm_shared_utils\","),
        Line::from(" ]"),
    ];
    render_scrolled_diff_panel(
        frame,
        manifest_lines,
        " Workspace Manifest Diff ",
        app.diff_scroll_offset,
        left_col,
    );

    let caller_lines = if let Some(p) = app.current_clone_pair() {
        vec![
            diff_del_line(format!("--- a/{}", p.file_a)),
            diff_add_line(format!("+++ b/{}", p.file_a)),
            Line::from(format!(
                "@@ -{},{} +{},2 @@",
                p.start_line_a,
                p.end_line_a.saturating_sub(p.start_line_a) + 1,
                p.start_line_a
            )),
            diff_add_line(format!(
                "+use cddm_shared_utils::deduplicated_{}_helper;",
                p.clone_type.as_str().to_lowercase()
            )),
            Line::from(" "),
            diff_del_line(format!(
                "-    /* Replaced duplicate block ({} tokens) */",
                p.token_count
            )),
            diff_add_line(format!(
                "+    let ok = deduplicated_{}_helper(&data);",
                p.clone_type.as_str().to_lowercase()
            )),
        ]
    } else {
        vec![
            Line::from("No clone selected for extraction preview."),
            Line::from("Select a clone pair in Tab 2 to preview extracted module caller diffs."),
        ]
    };
    render_scrolled_diff_panel(
        frame,
        caller_lines,
        " Caller Injected Imports & Callsite Diff ",
        app.diff_scroll_offset,
        right_col,
    );
}

fn render_extraction_actions(frame: &mut Frame, _app: &TuiApp, area: Rect) {
    let badges = [
        (" [x] Apply Shared Crate Extraction ", Color::Green),
        (" [d] Dry-Run Simulation ", Color::Cyan),
        (" [t] Synthesize Unit Tests ", Color::Yellow),
        (" [b] Synthesize Benchmarks ", Color::Magenta),
    ];
    render_action_bar(frame, &badges, " Extraction Operations ", area);
}
