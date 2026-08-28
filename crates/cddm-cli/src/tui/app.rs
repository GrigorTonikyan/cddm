#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::time::Instant;

use cddm_core::cluster::cluster_clone_pairs;
use cddm_core::types::{CloneCluster, ClonePair, ScanConfig, ScanResult};

/// Active top-level navigation tab in CDDM TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TuiTab {
    #[default]
    Overview = 0,
    Clones = 1,
    Semantic = 2,
    Refactor = 3,
    Extract = 4,
    Policy = 5,
    Timeline = 6,
    Workflow = 7,
    Overlap = 8,
}

impl TuiTab {
    pub const ALL: [TuiTab; 9] = [
        TuiTab::Overview,
        TuiTab::Clones,
        TuiTab::Semantic,
        TuiTab::Refactor,
        TuiTab::Extract,
        TuiTab::Policy,
        TuiTab::Timeline,
        TuiTab::Workflow,
        TuiTab::Overlap,
    ];

    pub fn title(&self) -> &'static str {
        match self {
            TuiTab::Overview => "[1] Overview",
            TuiTab::Clones => "[2] Clones & Diffs",
            TuiTab::Semantic => "[3] Semantic",
            TuiTab::Refactor => "[4] Refactor",
            TuiTab::Extract => "[5] Extract",
            TuiTab::Policy => "[6] Policies",
            TuiTab::Timeline => "[7] Timeline",
            TuiTab::Workflow => "[8] CI/CD & Hooks",
            TuiTab::Overlap => "[9] Overlap",
        }
    }

    pub fn next(&self) -> Self {
        let idx = (*self as usize + 1) % Self::ALL.len();
        Self::ALL[idx]
    }

    pub fn prev(&self) -> Self {
        let idx = if *self as usize == 0 {
            Self::ALL.len() - 1
        } else {
            *self as usize - 1
        };
        Self::ALL[idx]
    }
}

/// View mode in the Clone Explorer tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CloneViewMode {
    #[default]
    Pairwise,
    Clusters,
}

/// Diff presentation mode in split viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffMode {
    #[default]
    Split,
    Unified,
}

/// Central application state for CDDM Terminal UI Studio.
#[derive(Debug)]
#[allow(dead_code)]
pub struct TuiApp {
    // Configuration
    pub directory: PathBuf,
    pub config: ScanConfig,
    pub fail_threshold: Option<f64>,
    pub watch_mode: bool,

    // Data Cache
    pub scan_result: Option<ScanResult>,
    pub clusters: Vec<CloneCluster>,

    // Navigation & Selection
    pub active_tab: TuiTab,
    pub clone_view_mode: CloneViewMode,
    pub diff_mode: DiffMode,
    pub selected_clone_idx: usize,
    pub selected_cluster_idx: usize,
    pub selected_semantic_idx: usize,
    pub selected_policy_idx: usize,
    pub selected_timeline_idx: usize,

    // Scrolling & Modals
    pub diff_scroll_offset: usize,
    pub list_scroll_offset: usize,
    pub show_help_modal: bool,
    pub show_full_diff: bool,

    // Transient UI State
    pub status_message: Option<(String, Instant)>,
    pub is_scanning: bool,
    pub should_quit: bool,
}

#[allow(dead_code)]
impl TuiApp {
    pub fn new(
        directory: PathBuf,
        config: ScanConfig,
        fail_threshold: Option<f64>,
        watch_mode: bool,
    ) -> Self {
        Self {
            directory,
            config,
            fail_threshold,
            watch_mode,
            scan_result: None,
            clusters: Vec::new(),
            active_tab: TuiTab::Overview,
            clone_view_mode: CloneViewMode::Pairwise,
            diff_mode: DiffMode::Split,
            selected_clone_idx: 0,
            selected_cluster_idx: 0,
            selected_semantic_idx: 0,
            selected_policy_idx: 0,
            selected_timeline_idx: 0,
            diff_scroll_offset: 0,
            list_scroll_offset: 0,
            show_help_modal: false,
            show_full_diff: false,
            status_message: Some((
                "Welcome to CDDM TUI Studio. Press ? for keyboard shortcuts.".into(),
                Instant::now(),
            )),
            is_scanning: false,
            should_quit: false,
        }
    }

    /// Update with newly completed scan results.
    pub fn set_scan_result(&mut self, result: ScanResult) {
        let clusters = cluster_clone_pairs(&result.clone_pairs);
        self.clusters = clusters;
        self.scan_result = Some(result);
        self.selected_clone_idx = 0;
        self.selected_cluster_idx = 0;
        self.diff_scroll_offset = 0;
        self.set_status("Scan completed successfully.");
    }

    /// Set a transient status message.
    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some((msg.into(), Instant::now()));
    }

    /// Currently selected clone pair (if any).
    pub fn current_clone_pair(&self) -> Option<&ClonePair> {
        let res = self.scan_result.as_ref()?;
        res.clone_pairs.get(self.selected_clone_idx)
    }

    /// Currently selected clone cluster (if any).
    pub fn current_cluster(&self) -> Option<&CloneCluster> {
        self.clusters.get(self.selected_cluster_idx)
    }

    /// Total count of clone items in current view mode.
    pub fn current_clone_count(&self) -> usize {
        match self.clone_view_mode {
            CloneViewMode::Pairwise => self
                .scan_result
                .as_ref()
                .map(|r| r.clone_pairs.len())
                .unwrap_or(0),
            CloneViewMode::Clusters => self.clusters.len(),
        }
    }

    /// Navigate down in current list.
    pub fn select_next(&mut self) {
        match self.active_tab {
            TuiTab::Clones => match self.clone_view_mode {
                CloneViewMode::Pairwise => {
                    let total = self.current_clone_count();
                    if total > 0 && self.selected_clone_idx + 1 < total {
                        self.selected_clone_idx += 1;
                        self.diff_scroll_offset = 0;
                    }
                }
                CloneViewMode::Clusters => {
                    let total = self.clusters.len();
                    if total > 0 && self.selected_cluster_idx + 1 < total {
                        self.selected_cluster_idx += 1;
                        self.diff_scroll_offset = 0;
                    }
                }
            },
            TuiTab::Semantic => {
                self.selected_semantic_idx = self.selected_semantic_idx.saturating_add(1);
            }
            TuiTab::Policy => {
                self.selected_policy_idx = self.selected_policy_idx.saturating_add(1);
            }
            TuiTab::Timeline => {
                self.selected_timeline_idx = self.selected_timeline_idx.saturating_add(1);
            }
            _ => {
                self.diff_scroll_offset = self.diff_scroll_offset.saturating_add(1);
            }
        }
    }

    /// Navigate up in current list.
    pub fn select_prev(&mut self) {
        match self.active_tab {
            TuiTab::Clones => match self.clone_view_mode {
                CloneViewMode::Pairwise => {
                    if self.selected_clone_idx > 0 {
                        self.selected_clone_idx -= 1;
                        self.diff_scroll_offset = 0;
                    }
                }
                CloneViewMode::Clusters => {
                    if self.selected_cluster_idx > 0 {
                        self.selected_cluster_idx -= 1;
                        self.diff_scroll_offset = 0;
                    }
                }
            },
            TuiTab::Semantic => {
                self.selected_semantic_idx = self.selected_semantic_idx.saturating_sub(1);
            }
            TuiTab::Policy => {
                self.selected_policy_idx = self.selected_policy_idx.saturating_sub(1);
            }
            TuiTab::Timeline => {
                self.selected_timeline_idx = self.selected_timeline_idx.saturating_sub(1);
            }
            _ => {
                self.diff_scroll_offset = self.diff_scroll_offset.saturating_sub(1);
            }
        }
    }

    /// Scroll diff view down.
    pub fn scroll_diff_down(&mut self) {
        self.diff_scroll_offset = self.diff_scroll_offset.saturating_add(2);
    }

    /// Scroll diff view up.
    pub fn scroll_diff_up(&mut self) {
        self.diff_scroll_offset = self.diff_scroll_offset.saturating_sub(2);
    }

    /// Toggle pairwise vs clusters view in clone tab.
    pub fn toggle_clone_view_mode(&mut self) {
        self.clone_view_mode = match self.clone_view_mode {
            CloneViewMode::Pairwise => CloneViewMode::Clusters,
            CloneViewMode::Clusters => CloneViewMode::Pairwise,
        };
        self.set_status(format!("View mode: {:?}", self.clone_view_mode));
    }

    /// Toggle split vs unified diff display.
    pub fn toggle_diff_mode(&mut self) {
        self.diff_mode = match self.diff_mode {
            DiffMode::Split => DiffMode::Unified,
            DiffMode::Unified => DiffMode::Split,
        };
        self.set_status(format!("Diff mode: {:?}", self.diff_mode));
    }
}
