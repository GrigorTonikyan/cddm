#![forbid(unsafe_code)]

use cddm_core::{PolicySeverity, PolicyViolation};
use comfy_table::{Cell, Color, Table};
use std::fmt::Write as _;

pub fn format_policy_violations_console(violations: &[PolicyViolation]) -> String {
    let mut out = String::from("\n--- Architectural Policy Violations ---\n");
    let mut policy_table = Table::new();
    policy_table.set_header(vec![
        Cell::new("Rule"),
        Cell::new("Type"),
        Cell::new("Severity"),
        Cell::new("Location A"),
        Cell::new("Location B"),
        Cell::new("Message"),
    ]);
    for v in violations {
        let sev_cell = match v.severity {
            PolicySeverity::Error => Cell::new(format!("{:?}", v.severity)).fg(Color::Red),
            PolicySeverity::Warning => Cell::new(format!("{:?}", v.severity)).fg(Color::Yellow),
            PolicySeverity::Info => Cell::new(format!("{:?}", v.severity)).fg(Color::Cyan),
        };
        let loc_a = format!("{}:{}-{}", v.file_a, v.start_line_a, v.end_line_a);
        let loc_b =
            if let (Some(fb), Some(sb), Some(eb)) = (&v.file_b, v.start_line_b, v.end_line_b) {
                format!("{}:{}-{}", fb, sb, eb)
            } else {
                "-".to_string()
            };
        let message_text = match (&v.snippet, &v.remediation) {
            (Some(snip), Some(rem)) => {
                format!("{}\nSnippet: {}\nRemediation: {}", v.message, snip, rem)
            }
            (Some(snip), None) => format!("{}\nSnippet: {}", v.message, snip),
            (None, Some(rem)) => format!("{}\nRemediation: {}", v.message, rem),
            (None, None) => v.message.clone(),
        };
        policy_table.add_row(vec![
            Cell::new(&v.rule_name),
            Cell::new(&v.rule_type),
            sev_cell,
            Cell::new(loc_a),
            Cell::new(loc_b),
            Cell::new(message_text),
        ]);
    }
    let _ = writeln!(out, "{policy_table}");
    out
}

pub fn print_policy_violations_console(violations: &[PolicyViolation]) {
    print!("{}", format_policy_violations_console(violations));
}

pub fn format_policy_violations_markdown(violations: &[PolicyViolation]) -> String {
    let mut out = String::from("\n### Architectural Policy Violations\n\n");
    out.push_str("| Rule | Type | Severity | Location A | Location B | Message |\n");
    out.push_str("| :--- | :--- | :--- | :--- | :--- | :--- |\n");
    for v in violations {
        let loc_a = format!("{}:{}-{}", v.file_a, v.start_line_a, v.end_line_a);
        let loc_b =
            if let (Some(fb), Some(sb), Some(eb)) = (&v.file_b, v.start_line_b, v.end_line_b) {
                format!("{}:{}-{}", fb, sb, eb)
            } else {
                "-".to_string()
            };
        let message_text = match (&v.snippet, &v.remediation) {
            (Some(snip), Some(rem)) => {
                format!("{} (Snippet: `{}`; Remediation: {})", v.message, snip, rem)
            }
            (Some(snip), None) => format!("{} (Snippet: `{}`)", v.message, snip),
            (None, Some(rem)) => format!("{} (Remediation: {})", v.message, rem),
            (None, None) => v.message.clone(),
        };
        let _ = writeln!(
            out,
            "| `{}` | `{}` | `{:?}` | `{}` | `{}` | {} |",
            v.rule_name, v.rule_type, v.severity, loc_a, loc_b, message_text
        );
    }
    out
}

pub fn print_policy_violations_markdown(violations: &[PolicyViolation]) {
    print!("{}", format_policy_violations_markdown(violations));
}
