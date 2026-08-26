#![forbid(unsafe_code)]

pub mod app;
pub mod components;
pub mod events;
pub mod theme;
pub mod views;

#[cfg(test)]
pub mod tests;

use std::io::{Stdout, stdout};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};

use cddm_core::detector::run_scan;
use cddm_core::types::ScanConfig;

use crate::tui::app::TuiApp;
use crate::tui::components::help_modal::render_help_modal;
use crate::tui::components::{render_header_tabs, render_status_bar};
use crate::tui::events::{TuiEvent, handle_key_event, spawn_event_handler};
use crate::tui::views::render_main_content;

/// Launch the interactive CDDM Terminal UI Studio.
pub async fn run_tui(
    directory: PathBuf,
    min_tokens: usize,
    watch: bool,
    fail_threshold: Option<f64>,
    languages: Vec<String>,
    ignore: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Terminal in Raw Mode & Alternate Screen
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;

    // 2. Install panic hook to restore terminal on unexpected panic
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let mut out = stdout();
        let _ = execute!(out, LeaveAlternateScreen, DisableMouseCapture);
        default_panic(panic_info);
    }));

    // 3. Build Scan Configuration & Run Initial Scan
    let mut config = ScanConfig {
        directory: directory.display().to_string(),
        min_tokens,
        ..Default::default()
    };
    if !languages.is_empty() {
        config.languages = languages;
    }
    if !ignore.is_empty() {
        config.ignore_patterns = ignore;
    }

    let (tx, _rx) = tokio::sync::mpsc::channel(32);
    let cancel = Arc::new(AtomicBool::new(false));
    let initial_res = run_scan(config.clone(), tx, cancel.clone()).await;

    let mut app = TuiApp::new(directory.clone(), config.clone(), fail_threshold, watch);
    if let Ok(res) = initial_res {
        app.set_scan_result(res);
    }

    // 4. Start Event Stream
    let mut event_rx = spawn_event_handler(Duration::from_millis(100));

    // 5. Main Render & Event Loop
    let res = Ok(());
    while !app.should_quit {
        terminal.draw(|frame| {
            let area = frame.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Top Header Tabs
                    Constraint::Min(10),   // Main Content Area
                    Constraint::Length(3), // Bottom Status Bar
                ])
                .split(area);

            render_header_tabs(frame, &app, chunks[0]);
            render_main_content(frame, &app, chunks[1]);
            render_status_bar(frame, &app, chunks[2]);

            if app.show_help_modal {
                render_help_modal(frame);
            }
        })?;

        if let Some(event) = event_rx.recv().await {
            match event {
                TuiEvent::Key(key) => {
                    handle_key_event(&mut app, key);
                }
                TuiEvent::Resize(_, _) => {}
                TuiEvent::Tick => {}
                TuiEvent::Mouse(_) => {}
            }
        }
    }

    // 6. Restore Terminal
    restore_terminal(&mut terminal)?;
    res
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
