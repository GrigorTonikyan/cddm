#![forbid(unsafe_code)]

pub mod clones;
pub mod coverage;
pub mod extract;
pub mod helpers;
pub mod hub;
pub mod overlap;
pub mod overview;
pub mod policy;
pub mod refactor;
pub mod semantic;
pub mod timeline;
pub mod workflow;

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::{TuiApp, TuiTab};

/// Dispatch rendering to the currently active tab view.
pub fn render_main_content(frame: &mut Frame, app: &TuiApp, area: Rect) {
    match app.active_tab {
        TuiTab::Overview => overview::render_overview_view(frame, app, area),
        TuiTab::Clones => clones::render_clones_view(frame, app, area),
        TuiTab::Semantic => semantic::render_semantic_view(frame, app, area),
        TuiTab::Refactor => refactor::render_refactor_view(frame, app, area),
        TuiTab::Extract => extract::render_extract_view(frame, app, area),
        TuiTab::Policy => policy::render_policy_view(frame, app, area),
        TuiTab::Timeline => timeline::render_timeline_view(frame, app, area),
        TuiTab::Workflow => workflow::render_workflow_view(frame, app, area),
        TuiTab::Overlap => overlap::render_overlap_view(frame, app, area),
        TuiTab::Hub => hub::render_hub_view(frame, app, area),
        TuiTab::Coverage => coverage::render_coverage_view(frame, app, area),
    }
}
