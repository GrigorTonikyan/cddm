#![forbid(unsafe_code)]

use std::time::Duration;

use crossterm::event::{
    self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers, MouseEvent,
};
use tokio::sync::mpsc;

use crate::tui::app::{TuiApp, TuiTab};

/// Asynchronous terminal and background events in CDDM TUI.
#[derive(Debug)]
#[allow(dead_code)]
pub enum TuiEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Tick,
}

/// Start an asynchronous event listener stream.
pub fn spawn_event_handler(tick_rate: Duration) -> mpsc::UnboundedReceiver<TuiEvent> {
    let (tx, rx) = mpsc::unbounded_channel();

    std::thread::spawn(move || {
        loop {
            if event::poll(tick_rate).unwrap_or(false) {
                if let Ok(crossterm_event) = event::read() {
                    let send_err = match crossterm_event {
                        CrosstermEvent::Key(key) => tx.send(TuiEvent::Key(key)).is_err(),
                        CrosstermEvent::Mouse(mouse) => tx.send(TuiEvent::Mouse(mouse)).is_err(),
                        CrosstermEvent::Resize(w, h) => tx.send(TuiEvent::Resize(w, h)).is_err(),
                        _ => false,
                    };
                    if send_err {
                        break;
                    }
                }
            } else if tx.send(TuiEvent::Tick).is_err() {
                break;
            }
        }
    });

    rx
}

/// Process a keyboard event and mutate the TuiApp state.
pub fn handle_key_event(app: &mut TuiApp, key: KeyEvent) {
    if app.show_help_modal {
        match key.code {
            KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Enter => {
                app.show_help_modal = false;
            }
            _ => {}
        }
        return;
    }

    match (key.code, key.modifiers) {
        // Global Quits
        (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
            app.should_quit = true;
        }

        // Help Modal
        (KeyCode::Char('?'), _) => {
            app.show_help_modal = !app.show_help_modal;
        }

        // Direct Tab Shortcuts (1 - 8)
        (KeyCode::Char('1'), _) => app.active_tab = TuiTab::Overview,
        (KeyCode::Char('2'), _) => app.active_tab = TuiTab::Clones,
        (KeyCode::Char('3'), _) => app.active_tab = TuiTab::Semantic,
        (KeyCode::Char('4'), _) => app.active_tab = TuiTab::Refactor,
        (KeyCode::Char('5'), _) => app.active_tab = TuiTab::Extract,
        (KeyCode::Char('6'), _) => app.active_tab = TuiTab::Policy,
        (KeyCode::Char('7'), _) => app.active_tab = TuiTab::Timeline,
        (KeyCode::Char('8'), _) => app.active_tab = TuiTab::Workflow,

        // Tab Switching
        (KeyCode::Tab, KeyModifiers::NONE) | (KeyCode::Char('l'), KeyModifiers::NONE) => {
            app.active_tab = app.active_tab.next();
        }
        (KeyCode::BackTab, _) | (KeyCode::Char('h'), KeyModifiers::NONE) => {
            app.active_tab = app.active_tab.prev();
        }

        // Navigation (Up / Down)
        (KeyCode::Down, _) | (KeyCode::Char('j'), KeyModifiers::NONE) => {
            app.select_next();
        }
        (KeyCode::Up, _) | (KeyCode::Char('k'), KeyModifiers::NONE) => {
            app.select_prev();
        }

        // Diff Scrolling (Shift + J / K or PageDown / PageUp)
        (KeyCode::PageDown, _) | (KeyCode::Char('J'), _) => {
            app.scroll_diff_down();
        }
        (KeyCode::PageUp, _) | (KeyCode::Char('K'), _) => {
            app.scroll_diff_up();
        }

        // Toggle Modes
        (KeyCode::Char('c'), KeyModifiers::NONE) => {
            app.toggle_clone_view_mode();
        }
        (KeyCode::Char('u'), KeyModifiers::NONE) => {
            app.toggle_diff_mode();
        }

        // Action Shortcuts
        (KeyCode::Char('r'), KeyModifiers::NONE) => {
            app.active_tab = TuiTab::Refactor;
            app.set_status("Switched to Refactor Sandbox");
        }
        (KeyCode::Char('e'), KeyModifiers::NONE) => {
            app.active_tab = TuiTab::Extract;
            app.set_status("Switched to Shared Module Extractor");
        }

        _ => {}
    }
}
