#![forbid(unsafe_code)]

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{List, ListItem, Row, Table};

use super::helpers::*;
use crate::tui::app::TuiApp;
use crate::tui::theme::TuiTheme;
use cddm_core::{DEFAULT_RULES_FILE, PolicyConfig, PolicyEngine, find_workspace_root};

fn load_active_policy_config(app: &TuiApp) -> PolicyConfig {
    let dir_rules = app.directory.join(DEFAULT_RULES_FILE);
    if dir_rules.exists()
        && let Ok(engine) = PolicyEngine::from_file(&dir_rules)
    {
        return engine.config().clone();
    }
    let ws_rules = find_workspace_root(&app.directory).join(DEFAULT_RULES_FILE);
    if ws_rules.exists()
        && let Ok(engine) = PolicyEngine::from_file(&ws_rules)
    {
        return engine.config().clone();
    }
    PolicyConfig::default()
}

/// Render Tab 6: Policy Engine & AST Suppression Directives Inspector.
pub fn render_policy_view(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let (left_col, right_col) = split_horizontal_2(area, 50, 50);
    let policy_cfg = load_active_policy_config(app);
    render_policy_rules(frame, app, &policy_cfg, left_col);
    render_suppression_rules(frame, app, &policy_cfg, right_col);
}

fn render_policy_rules(frame: &mut Frame, app: &TuiApp, policy_cfg: &PolicyConfig, area: Rect) {
    let mut rows = Vec::new();
    let max_line_limits: Vec<usize> = policy_cfg
        .line_budgets
        .iter()
        .map(|lb| lb.max_file_lines)
        .chain(
            policy_cfg
                .token_budgets
                .iter()
                .filter_map(|tb| tb.max_file_lines),
        )
        .collect();
    let line_limit_label = if max_line_limits.is_empty() {
        "max_file_lines Limits".to_string()
    } else {
        let min_limit = max_line_limits.iter().min().copied().unwrap_or(500);
        format!("<= {min_limit} lines ({} rule(s))", max_line_limits.len())
    };
    let eff_ignore_tests = policy_cfg.ignore_tests.unwrap_or(app.config.ignore_tests);
    let eff_ignore_mocks = policy_cfg.ignore_mocks.unwrap_or(app.config.ignore_mocks);
    let test_scope_status = if !eff_ignore_tests && !eff_ignore_mocks {
        "Tests & Mocks ENFORCED".to_string()
    } else if !eff_ignore_tests {
        "Tests ENFORCED / Mocks Ignored".to_string()
    } else {
        "Tests & Mocks Ignored".to_string()
    };

    if let Some(res) = &app.scan_result {
        if !res.policy_violations.is_empty() {
            for v in &res.policy_violations {
                let color = match v.severity {
                    cddm_core::PolicySeverity::Error => Color::Red,
                    cddm_core::PolicySeverity::Warning => Color::Yellow,
                    cddm_core::PolicySeverity::Info => Color::Cyan,
                };
                let scope_cell = if let (Some(actual), Some(allowed)) = (v.line_count, v.max_lines)
                {
                    format!("{actual}/{allowed} lines ({})", v.file_a)
                } else if let Some(ref fb) = v.file_b {
                    format!("{} <-> {fb}", v.file_a)
                } else {
                    format!("{}:{}-{}", v.file_a, v.start_line_a, v.end_line_a)
                };
                rows.push(
                    Row::new(vec![
                        v.rule_name.clone(),
                        scope_cell,
                        format!("{:?}: {}", v.severity, v.message),
                    ])
                    .style(Style::default().fg(color)),
                );
            }
        } else {
            rows.push(
                Row::new(vec![
                    "Architectural Boundaries".to_string(),
                    format!("{} files evaluated", res.total_files),
                    "0 Violations (PASS)".to_string(),
                ])
                .style(Style::default().fg(Color::Green)),
            );
            rows.push(
                Row::new(vec![
                    "Zero Duplication Zones".to_string(),
                    "Domain & Core Modules".to_string(),
                    "Compliant (PASS)".to_string(),
                ])
                .style(Style::default().fg(Color::Green)),
            );
            rows.push(
                Row::new(vec![
                    "Prohibited Patterns".to_string(),
                    "Regex & AST Sensors".to_string(),
                    "Compliant (PASS)".to_string(),
                ])
                .style(Style::default().fg(Color::Green)),
            );
            rows.push(
                Row::new(vec![
                    "Zero-Emoji Sensor".to_string(),
                    "Codebase Glyph Scanner".to_string(),
                    "Compliant (PASS)".to_string(),
                ])
                .style(Style::default().fg(Color::Green)),
            );
            rows.push(
                Row::new(vec![
                    "Token Budgets".to_string(),
                    "File & Total Limits".to_string(),
                    "Within Limits (PASS)".to_string(),
                ])
                .style(Style::default().fg(Color::Green)),
            );
            rows.push(
                Row::new(vec![
                    "Line Budgets (max_file_lines)".to_string(),
                    line_limit_label,
                    format!("PASS ({test_scope_status})"),
                ])
                .style(Style::default().fg(Color::Green)),
            );
            rows.push(
                Row::new(vec![
                    "Forbidden Files".to_string(),
                    "Conflicting Tooling & Lockfiles".to_string(),
                    "Compliant (PASS)".to_string(),
                ])
                .style(Style::default().fg(Color::Green)),
            );
            rows.push(
                Row::new(vec![
                    "SSoT Synchronization".to_string(),
                    "Config & Code Version Sync".to_string(),
                    "In Sync (PASS)".to_string(),
                ])
                .style(Style::default().fg(Color::Green)),
            );
            rows.push(
                Row::new(vec![
                    "Route Documentation".to_string(),
                    "Living Route Completeness".to_string(),
                    "Documented (PASS)".to_string(),
                ])
                .style(Style::default().fg(Color::Green)),
            );
        }
    } else {
        rows.push(
            Row::new(vec![
                "Policy Engine".to_string(),
                format!("{line_limit_label} | {test_scope_status}"),
                "Press 's' to Scan".to_string(),
            ])
            .style(Style::default().fg(Color::DarkGray)),
        );
    }

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(45),
            Constraint::Percentage(30),
        ],
    )
    .header(
        Row::new(vec!["Policy Rule", "Scope / Violation", "Status"]).style(
            Style::default()
                .fg(TuiTheme::BRAND)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .block(create_titled_block(
        " Architectural Boundary Rules (.cddmrules.toml) ",
        false,
    ));

    frame.render_widget(table, area);
}

fn render_suppression_rules(
    frame: &mut Frame,
    app: &TuiApp,
    policy_cfg: &PolicyConfig,
    area: Rect,
) {
    let subchunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let eff_ignore_tests = policy_cfg.ignore_tests.unwrap_or(app.config.ignore_tests);
    let eff_ignore_mocks = policy_cfg.ignore_mocks.unwrap_or(app.config.ignore_mocks);
    let tests_label = if eff_ignore_tests {
        "Tests: IGNORED"
    } else {
        "Tests: ENFORCED"
    };
    let mocks_label = if eff_ignore_mocks {
        "Mocks: IGNORED"
    } else {
        "Mocks: ENFORCED"
    };

    let mut ignore_items: Vec<ListItem> = vec![ListItem::new(Line::from(vec![
        Span::styled(
            " [policy] ",
            Style::default().fg(if !eff_ignore_tests {
                Color::Green
            } else {
                Color::Yellow
            }),
        ),
        Span::raw(format!("{tests_label} | {mocks_label}")),
    ]))];

    if let Ok(cddmignore) = std::fs::read_to_string(".cddmignore") {
        ignore_items.extend(
            cddmignore
                .lines()
                .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
                .take(7)
                .map(|l| {
                    ListItem::new(Line::from(vec![
                        Span::styled(" [ignore] ", Style::default().fg(Color::Cyan)),
                        Span::raw(l.to_string()),
                    ]))
                }),
        );
    } else {
        for pat in app.config.ignore_patterns.iter().take(6) {
            ignore_items.push(ListItem::new(Line::from(vec![
                Span::styled(" [glob] ", Style::default().fg(Color::Yellow)),
                Span::raw(pat.clone()),
            ])));
        }
    }

    let list_ignore =
        List::new(ignore_items).block(create_titled_block(" Active .cddmignore Rules ", false));
    frame.render_widget(list_ignore, subchunks[0]);

    let directive_lines = vec![
        Line::from(vec![Span::styled(
            "Supported Inline AST Directives:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  // cddm:ignore              Ignore next AST statement"),
        Line::from("  /* cddm:ignore-start */     Begin ignored code block"),
        Line::from("  /* cddm:ignore-end */       End ignored code block"),
        Line::from("  #[cddm(allow_duplication)]  Rust attribute suppression"),
        Line::from("  # @cddm_ignore              Python / Ruby comment suppression"),
    ];

    let p_dir = Paragraph::new(directive_lines)
        .block(create_titled_block(" Inline Directives & Headers ", false));
    frame.render_widget(p_dir, subchunks[1]);
}
