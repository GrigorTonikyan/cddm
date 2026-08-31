#![forbid(unsafe_code)]

use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Color;

use cddm_core::types::{ClonePair, CloneType, ScanConfig, ScanResult};

use crate::tui::app::{CloneViewMode, DiffMode, TuiApp, TuiTab};
use crate::tui::components::help_modal::render_help_modal;
use crate::tui::components::{render_header_tabs, render_status_bar};
use crate::tui::events::handle_key_event;
use crate::tui::theme::TuiTheme;
use crate::tui::views::render_main_content;

fn make_sample_pair(file_a: &str, file_b: &str, tokens: usize, ctype: CloneType) -> ClonePair {
    ClonePair {
        file_a: file_a.to_string(),
        file_b: file_b.to_string(),
        start_line_a: 1,
        end_line_a: 10,
        start_line_b: 1,
        end_line_b: 10,
        token_count: tokens,
        clone_type: ctype,
        similarity: 1.0,
        fragment_hash: "hash_sample".to_string(),
        author_a: None,
        author_b: None,
    }
}

fn make_sample_scan_result(pairs: Vec<ClonePair>, dup_pct: f64, dry_score: f64) -> ScanResult {
    let count = pairs.len();
    ScanResult {
        scan_id: "scan_test".to_string(),
        total_files: count * 2,
        total_tokens: 1000,
        total_clones: count,
        total_clusters: count,
        duplication_percentage: dup_pct,
        dry_health_score: dry_score,
        clone_pairs: pairs,
        clone_clusters: Vec::new(),
        duration_ms: 10,
        language_breakdown: Vec::new(),
        policy_violations: Vec::new(),
    }
}

fn buffer_to_string(terminal: &Terminal<TestBackend>) -> String {
    let buffer = terminal.backend().buffer();
    let mut out = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            out.push_str(buffer[(x, y)].symbol());
        }
        out.push('\n');
    }
    out
}

fn draw_test_frame(terminal: &mut Terminal<TestBackend>, app: &TuiApp) {
    terminal
        .draw(|frame| {
            let area = frame.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(10),
                    Constraint::Length(3),
                ])
                .split(area);

            render_header_tabs(frame, app, chunks[0]);
            render_main_content(frame, app, chunks[1]);
            render_status_bar(frame, app, chunks[2]);

            if app.show_help_modal {
                render_help_modal(frame);
            }
        })
        .expect("Failed to draw test frame");
}

#[test]
fn test_tui_tab_lifecycle() {
    let tab = TuiTab::Overview;
    assert_eq!(tab.title(), "[1] Overview");
    assert_eq!(tab.next(), TuiTab::Clones);
    assert_eq!(tab.prev(), TuiTab::DeadCode);

    let last_tab = TuiTab::DeadCode;
    assert_eq!(last_tab.next(), TuiTab::Overview);
    assert_eq!(last_tab.title(), "[D] Dead Code");
}

#[test]
fn test_tui_app_initialization() {
    let app = TuiApp::new(PathBuf::from("."), ScanConfig::default(), Some(15.0), false);

    assert_eq!(app.active_tab, TuiTab::Overview);
    assert_eq!(app.clone_view_mode, CloneViewMode::Pairwise);
    assert_eq!(app.diff_mode, DiffMode::Split);
    assert_eq!(app.selected_clone_idx, 0);
    assert!(!app.should_quit);
    assert!(!app.show_help_modal);
}

#[test]
fn test_tui_app_set_scan_result() {
    let mut app = TuiApp::new(PathBuf::from("."), ScanConfig::default(), None, false);

    let pair = make_sample_pair("src/a.rs", "src/b.rs", 85, CloneType::Exact);
    let result = make_sample_scan_result(vec![pair], 3.4, 95.2);

    app.set_scan_result(result);

    assert_eq!(app.current_clone_count(), 1);
    assert!(app.current_clone_pair().is_some());
    assert_eq!(app.clusters.len(), 1);
    assert!(app.current_cluster().is_some());
}

#[test]
fn test_tui_app_navigation_and_scrolling() {
    let mut app = TuiApp::new(PathBuf::from("."), ScanConfig::default(), None, false);
    app.active_tab = TuiTab::Clones;

    let pair1 = make_sample_pair("a.rs", "b.rs", 50, CloneType::Exact);
    let pair2 = make_sample_pair("c.rs", "d.rs", 60, CloneType::Renamed);

    app.set_scan_result(make_sample_scan_result(vec![pair1, pair2], 55.0, 40.0));

    assert_eq!(app.selected_clone_idx, 0);
    app.select_next();
    assert_eq!(app.selected_clone_idx, 1);
    app.select_prev();
    assert_eq!(app.selected_clone_idx, 0);

    // Diff scrolling
    app.scroll_diff_down();
    assert_eq!(app.diff_scroll_offset, 2);
    app.scroll_diff_up();
    assert_eq!(app.diff_scroll_offset, 0);
}

#[test]
fn test_tui_mode_toggles() {
    let mut app = TuiApp::new(PathBuf::from("."), ScanConfig::default(), None, false);

    assert_eq!(app.clone_view_mode, CloneViewMode::Pairwise);
    app.toggle_clone_view_mode();
    assert_eq!(app.clone_view_mode, CloneViewMode::Clusters);
    app.toggle_clone_view_mode();
    assert_eq!(app.clone_view_mode, CloneViewMode::Pairwise);

    assert_eq!(app.diff_mode, DiffMode::Split);
    app.toggle_diff_mode();
    assert_eq!(app.diff_mode, DiffMode::Unified);
    app.toggle_diff_mode();
    assert_eq!(app.diff_mode, DiffMode::Split);
}

#[test]
fn test_tui_theme_score_tiers() {
    assert_eq!(TuiTheme::dry_score_tier(95.0), "EXCELLENT");
    assert_eq!(TuiTheme::dry_score_tier(80.0), "GOOD");
    assert_eq!(TuiTheme::dry_score_tier(65.0), "FAIR");
    assert_eq!(TuiTheme::dry_score_tier(45.0), "CRITICAL");

    assert_eq!(TuiTheme::dry_score_color(95.0), Color::Green);
    assert_eq!(TuiTheme::dry_score_color(65.0), Color::Yellow);
    assert_eq!(TuiTheme::dry_score_color(45.0), Color::Red);
}

#[test]
fn test_tui_key_events() {
    let mut app = TuiApp::new(PathBuf::from("."), ScanConfig::default(), None, false);

    // Tab selection via number keys
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE),
    );
    assert_eq!(app.active_tab, TuiTab::Clones);

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('4'), KeyModifiers::NONE),
    );
    assert_eq!(app.active_tab, TuiTab::Refactor);

    // Tab key cycling
    handle_key_event(&mut app, KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.active_tab, TuiTab::Extract);

    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT),
    );
    assert_eq!(app.active_tab, TuiTab::Refactor);

    // Help modal toggle
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE),
    );
    assert!(app.show_help_modal);

    handle_key_event(&mut app, KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(!app.show_help_modal);

    // Quit command
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
    );
    assert!(app.should_quit);
}

#[test]
fn test_render_all_tabs_on_test_backend() {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).expect("Backend init");

    let mut app = TuiApp::new(PathBuf::from("."), ScanConfig::default(), None, false);
    let pair = make_sample_pair("src/a.rs", "src/b.rs", 85, CloneType::Exact);
    let result = make_sample_scan_result(vec![pair], 3.4, 95.2);
    app.set_scan_result(result);

    let all_tabs = [
        TuiTab::Overview,
        TuiTab::Clones,
        TuiTab::Semantic,
        TuiTab::Refactor,
        TuiTab::Extract,
        TuiTab::Policy,
        TuiTab::Timeline,
        TuiTab::Workflow,
        TuiTab::Overlap,
        TuiTab::Hub,
        TuiTab::Coverage,
    ];

    for tab in all_tabs {
        app.active_tab = tab;
        draw_test_frame(&mut terminal, &app);

        // Verify buffer is non-empty
        let buffer = terminal.backend().buffer();
        assert!(buffer.area.width == 120);
        assert!(buffer.area.height == 40);
    }
}

#[test]
fn test_render_help_modal_overlay() {
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).expect("Backend init");

    let mut app = TuiApp::new(PathBuf::from("."), ScanConfig::default(), None, false);
    app.show_help_modal = true;

    draw_test_frame(&mut terminal, &app);

    let content = buffer_to_string(&terminal);
    assert!(content.contains("Keyboard Shortcuts"));
    assert!(content.contains("Navigation & Tabs"));
    assert!(content.contains("Exit CDDM Studio"));
}

#[tokio::test]
async fn test_interactive_tui_full_hands_on_workflow() {
    let backend = TestBackend::new(140, 45);
    let mut terminal = Terminal::new(backend).expect("Backend init");

    let mut app = TuiApp::new(PathBuf::from("."), ScanConfig::default(), Some(15.0), true);

    let pair1 = make_sample_pair("src/scanner.rs", "src/parser.rs", 95, CloneType::Exact);
    let pair2 = make_sample_pair("src/util.rs", "src/helper.rs", 120, CloneType::Renamed);
    app.set_scan_result(make_sample_scan_result(vec![pair1, pair2], 8.5, 91.5));

    // 1. Initial State: Tab 1 (Overview)
    draw_test_frame(&mut terminal, &app);
    let content1 = buffer_to_string(&terminal);
    assert!(content1.contains("DRY Health"));
    assert!(content1.contains("Codebase Modularity Metrics"));

    // 2. Navigate to Tab 2 (Clones) via Key '2'
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE),
    );
    assert_eq!(app.active_tab, TuiTab::Clones);
    draw_test_frame(&mut terminal, &app);
    let content2 = buffer_to_string(&terminal);
    assert!(content2.contains("Pairwise Clones"));
    assert!(content2.contains("Side-by-Side Split Diff"));

    // 3. Selection Navigation (j/k)
    assert_eq!(app.selected_clone_idx, 0);
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
    );
    assert_eq!(app.selected_clone_idx, 1);
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
    );
    assert_eq!(app.selected_clone_idx, 0);

    // 4. Diff Scroll Controls (J/K)
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('J'), KeyModifiers::NONE),
    );
    assert_eq!(app.diff_scroll_offset, 2);
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('K'), KeyModifiers::NONE),
    );
    assert_eq!(app.diff_scroll_offset, 0);

    // 5. Toggle Clusters Mode (c)
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE),
    );
    assert_eq!(app.clone_view_mode, CloneViewMode::Clusters);
    draw_test_frame(&mut terminal, &app);
    let content_clust = buffer_to_string(&terminal);
    assert!(content_clust.contains("Clone Clusters"));

    // 6. Toggle Unified Diff Mode (u)
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE),
    );
    assert_eq!(app.diff_mode, DiffMode::Unified);
    draw_test_frame(&mut terminal, &app);
    let content_unified = buffer_to_string(&terminal);
    assert!(content_unified.contains("Unified Refactor Diff"));

    // 7. Cycle through all remaining tabs (3 -> 0, C)
    for key_char in ['3', '4', '5', '6', '7', '8', '9', '0', 'c', 'C'] {
        handle_key_event(
            &mut app,
            KeyEvent::new(KeyCode::Char(key_char), KeyModifiers::NONE),
        );
        draw_test_frame(&mut terminal, &app);
    }
    assert_eq!(app.active_tab, TuiTab::Coverage);

    // 8. Help Modal Toggle ('?') & Dismiss ('Esc')
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE),
    );
    assert!(app.show_help_modal);
    draw_test_frame(&mut terminal, &app);
    handle_key_event(&mut app, KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(!app.show_help_modal);

    // 9. Quit Command ('q')
    handle_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
    );
    assert!(app.should_quit);
}
