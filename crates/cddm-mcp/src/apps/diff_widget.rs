#![forbid(unsafe_code)]

use super::types::{AppWidget, widget_types};
use serde_json::json;

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Generates an interactive diff split-view HTML widget for clone pairs or refactoring diffs.
#[allow(clippy::too_many_arguments)]
pub fn generate_diff_split_view_widget(
    title: &str,
    file_a: &str,
    lines_a: (usize, usize),
    file_b: &str,
    lines_b: (usize, usize),
    original_code: &str,
    refactored_code: &str,
    diff_patch: Option<&str>,
) -> AppWidget {
    let escaped_title = escape_html(title);
    let escaped_file_a = escape_html(file_a);
    let escaped_file_b = escape_html(file_b);

    let orig_lines: Vec<&str> = original_code.lines().collect();
    let refac_lines: Vec<&str> = refactored_code.lines().collect();
    let max_lines = orig_lines.len().max(refac_lines.len());

    let mut rows_html = String::new();
    for i in 0..max_lines {
        let left_num = if i < orig_lines.len() {
            format!("{}", lines_a.0 + i)
        } else {
            String::new()
        };
        let left_content = if i < orig_lines.len() {
            escape_html(orig_lines[i])
        } else {
            String::new()
        };
        let right_num = if i < refac_lines.len() {
            format!("{}", lines_b.0 + i)
        } else {
            String::new()
        };
        let right_content = if i < refac_lines.len() {
            escape_html(refac_lines[i])
        } else {
            String::new()
        };

        let is_diff =
            i >= orig_lines.len() || i >= refac_lines.len() || orig_lines[i] != refac_lines[i];

        let left_class = if is_diff && i < orig_lines.len() {
            "line-del"
        } else {
            ""
        };
        let right_class = if is_diff && i < refac_lines.len() {
            "line-add"
        } else {
            ""
        };

        rows_html.push_str(&format!(
            r#"<tr class="{row_class}">
  <td class="lineno">{left_num}</td>
  <td class="code {left_class}"><pre>{left_content}</pre></td>
  <td class="lineno">{right_num}</td>
  <td class="code {right_class}"><pre>{right_content}</pre></td>
</tr>"#,
            row_class = if is_diff { "diff-row" } else { "" },
            left_num = left_num,
            left_class = left_class,
            left_content = left_content,
            right_num = right_num,
            right_class = right_class,
            right_content = right_content,
        ));
    }

    let escaped_raw_refac = escape_html(refactored_code);
    let patch_str = diff_patch.unwrap_or("");
    let escaped_raw_patch = escape_html(patch_str);

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width, initial-scale=1.0"/>
<title>{title}</title>
<style>
  :root {{
    --bg-main: #020617;
    --bg-surface: #0f172a;
    --border: #1e293b;
    --text-primary: #f8fafc;
    --text-muted: #94a3b8;
    --indigo: #6366f1;
    --del-bg: rgba(244, 63, 94, 0.15);
    --del-border: #f43f5e;
    --add-bg: rgba(16, 185, 129, 0.15);
    --add-border: #10b981;
  }}
  body {{
    margin: 0;
    padding: 16px;
    font-family: ui-sans-serif, system-ui, -apple-system, sans-serif;
    background: var(--bg-main);
    color: var(--text-primary);
  }}
  .cddm-app-header {{
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border);
    padding-bottom: 12px;
    margin-bottom: 16px;
    gap: 12px;
  }}
  .cddm-app-title {{
    font-size: 16px;
    font-weight: 700;
    color: #fff;
    display: flex;
    align-items: center;
    gap: 8px;
  }}
  .cddm-app-badge {{
    font-size: 11px;
    padding: 2px 8px;
    border-radius: 9999px;
    background: rgba(99, 102, 241, 0.2);
    color: #a5b4fc;
    border: 1px solid rgba(99, 102, 241, 0.3);
    font-family: monospace;
  }}
  .cddm-app-actions {{
    display: flex;
    gap: 8px;
  }}
  button.action-btn {{
    background: var(--bg-surface);
    color: var(--text-primary);
    border: 1px solid var(--border);
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }}
  button.action-btn:hover {{
    background: #1e293b;
    border-color: #334155;
  }}
  button.action-btn-primary {{
    background: var(--indigo);
    border-color: var(--indigo);
    color: #fff;
  }}
  button.action-btn-primary:hover {{
    background: #4f46e5;
  }}
  .diff-table-container {{
    overflow-x: auto;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: var(--bg-surface);
  }}
  table.diff-table {{
    width: 100%;
    border-collapse: collapse;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 12px;
  }}
  table.diff-table th {{
    background: #090d16;
    padding: 8px 12px;
    font-weight: 600;
    text-align: left;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border);
  }}
  table.diff-table td {{
    padding: 2px 8px;
    vertical-align: top;
    white-space: pre;
  }}
  td.lineno {{
    width: 40px;
    text-align: right;
    color: #475569;
    user-select: none;
    border-right: 1px solid var(--border);
    background: #020617;
  }}
  td.code {{
    font-family: inherit;
  }}
  td.code pre {{
    margin: 0;
    font-family: inherit;
  }}
  td.line-del {{
    background: var(--del-bg);
    color: #fca5a5;
  }}
  td.line-add {{
    background: var(--add-bg);
    color: #6ee7b7;
  }}
  tr.diff-row {{
    border-left: 2px solid var(--indigo);
  }}
  .cddm-app-status {{
    margin-top: 12px;
    font-size: 12px;
    color: var(--text-muted);
    font-family: monospace;
    display: flex;
    justify-content: space-between;
  }}
</style>
</head>
<body class="diff-split-view-app" data-widget-type="diff-split-view">
  <div class="cddm-app-header">
    <div class="cddm-app-title">
      <span>{title}</span>
      <span class="cddm-app-badge">MCP App 2026-07-28</span>
    </div>
    <div class="cddm-app-actions">
      <button class="action-btn" id="btnCopyRefac" onclick="copyRefactored()">Copy Refactored</button>
      <button class="action-btn" id="btnCopyPatch" onclick="copyPatch()">Copy Patch</button>
      <button class="action-btn action-btn-primary" id="btnAccept" onclick="acceptChanges()">Accept Changes</button>
    </div>
  </div>
  <div class="diff-table-container">
    <table class="diff-table">
      <thead>
        <tr>
          <th colspan="2">Original ({file_a}:{start_a}-{end_a})</th>
          <th colspan="2">Refactored ({file_b}:{start_b}-{end_b})</th>
        </tr>
      </thead>
      <tbody>
        {rows}
      </tbody>
    </table>
  </div>
  <div class="cddm-app-status">
    <span id="statusMsg">Ready. {total_lines} lines compared.</span>
    <span>CDDM Generative UI Widget</span>
  </div>

  <textarea id="rawRefac" style="display:none;">{raw_refac}</textarea>
  <textarea id="rawPatch" style="display:none;">{raw_patch}</textarea>

  <script>
    function copyRefactored() {{
      const text = document.getElementById('rawRefac').value;
      navigator.clipboard.writeText(text).then(() => {{
        document.getElementById('statusMsg').innerText = 'Refactored code copied to clipboard!';
      }});
    }}
    function copyPatch() {{
      const text = document.getElementById('rawPatch').value;
      navigator.clipboard.writeText(text).then(() => {{
        document.getElementById('statusMsg').innerText = 'Patch copied to clipboard!';
      }});
    }}
    function acceptChanges() {{
      const payload = {{
        action: 'accept-refactor',
        fileA: '{file_a}',
        fileB: '{file_b}',
        patch: document.getElementById('rawPatch').value
      }};
      window.parent.postMessage({{ type: 'cddm:mcp-app-action', payload }}, '*');
      document.getElementById('statusMsg').innerText = 'Patch accept notification emitted to host.';
    }}
  </script>
</body>
</html>"#,
        title = escaped_title,
        file_a = escaped_file_a,
        start_a = lines_a.0,
        end_a = lines_a.1,
        file_b = escaped_file_b,
        start_b = lines_b.0,
        end_b = lines_b.1,
        rows = rows_html,
        total_lines = max_lines,
        raw_refac = escaped_raw_refac,
        raw_patch = escaped_raw_patch,
    );

    let data = json!({
        "fileA": file_a,
        "linesA": [lines_a.0, lines_a.1],
        "fileB": file_b,
        "linesB": [lines_b.0, lines_b.1],
        "originalLineCount": orig_lines.len(),
        "refactoredLineCount": refac_lines.len(),
        "diffPatch": patch_str,
    });

    AppWidget::new(widget_types::DIFF_SPLIT_VIEW, title, html, data)
}
