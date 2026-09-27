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

#[derive(Debug, Clone)]
pub struct TreemapClusterItem {
    pub id: usize,
    pub name: String,
    pub clone_type: String,
    pub occurrence_count: usize,
    pub token_count: usize,
    pub files: Vec<String>,
}

/// Generates an interactive cluster-treemap HTML widget for duplication visualization.
pub fn generate_cluster_treemap_widget(
    title: &str,
    clusters: &[TreemapClusterItem],
    dry_health_score: f64,
) -> AppWidget {
    let escaped_title = escape_html(title);
    let total_clusters = clusters.len();
    let total_occurrences: usize = clusters.iter().map(|c| c.occurrence_count).sum();
    let total_tokens: usize = clusters.iter().map(|c| c.token_count).sum();

    let mut tiles_html = String::new();
    for c in clusters {
        let esc_name = escape_html(&c.name);
        let esc_type = escape_html(&c.clone_type);
        let files_preview = escape_html(&c.files.join(", "));

        let color_class = match c.clone_type.to_lowercase().as_str() {
            "exact" | "type1" | "type 1" => "tile-exact",
            "renamed" | "type2" | "type 2" => "tile-renamed",
            "gapped" | "type3" | "type 3" => "tile-gapped",
            _ => "tile-semantic",
        };

        // Relative size weight based on token count (min 1, max 4)
        let weight = ((c.token_count as f64 / 100.0).ceil() as usize).clamp(1, 4);

        tiles_html.push_str(&format!(
            r#"<div class="treemap-tile {color_class} weight-{weight}" data-id="{id}" data-type="{clone_type}" data-files="{files_preview}" onclick="selectCluster({id}, '{esc_name}')">
  <div class="tile-header">
    <span class="tile-badge">#{id}</span>
    <span class="tile-type">{clone_type}</span>
  </div>
  <div class="tile-title">{esc_name}</div>
  <div class="tile-stats">{occs} sites &bull; {tokens} tokens</div>
</div>"#,
            color_class = color_class,
            weight = weight,
            id = c.id,
            clone_type = esc_type,
            files_preview = files_preview,
            esc_name = esc_name,
            occs = c.occurrence_count,
            tokens = c.token_count,
        ));
    }

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
    --exact: #10b981;
    --renamed: #6366f1;
    --gapped: #f59e0b;
    --semantic: #a855f7;
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
  .cddm-metrics {{
    display: flex;
    gap: 16px;
    font-size: 12px;
    color: var(--text-muted);
  }}
  .metric-val {{
    color: #fff;
    font-weight: 700;
  }}
  .filter-bar {{
    display: flex;
    gap: 8px;
    margin-bottom: 14px;
    align-items: center;
  }}
  .filter-input {{
    background: var(--bg-surface);
    border: 1px solid var(--border);
    color: #fff;
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 12px;
    flex: 1;
  }}
  .treemap-grid {{
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 10px;
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 12px;
    background: #090d16;
    max-height: 480px;
    overflow-y: auto;
  }}
  .treemap-tile {{
    border-radius: 6px;
    padding: 12px;
    cursor: pointer;
    transition: transform 0.15s, border-color 0.15s;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    min-height: 90px;
  }}
  .treemap-tile:hover {{
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
  }}
  .tile-exact {{
    background: rgba(16, 185, 129, 0.15);
    border: 1px solid rgba(16, 185, 129, 0.4);
  }}
  .tile-renamed {{
    background: rgba(99, 102, 241, 0.15);
    border: 1px solid rgba(99, 102, 241, 0.4);
  }}
  .tile-gapped {{
    background: rgba(245, 158, 11, 0.15);
    border: 1px solid rgba(245, 158, 11, 0.4);
  }}
  .tile-semantic {{
    background: rgba(168, 85, 247, 0.15);
    border: 1px solid rgba(168, 85, 247, 0.4);
  }}
  .tile-header {{
    display: flex;
    justify-content: space-between;
    font-size: 11px;
    font-family: monospace;
  }}
  .tile-badge {{
    font-weight: 700;
    color: #fff;
  }}
  .tile-type {{
    text-transform: uppercase;
    font-size: 10px;
    letter-spacing: 0.05em;
  }}
  .tile-title {{
    font-weight: 600;
    font-size: 13px;
    margin: 6px 0;
    color: #f1f5f9;
    word-break: break-all;
  }}
  .tile-stats {{
    font-size: 11px;
    color: var(--text-muted);
    font-family: monospace;
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
<body class="cluster-treemap-app" data-widget-type="cluster-treemap">
  <div class="cddm-app-header">
    <div class="cddm-app-title">
      <span>{title}</span>
      <span class="cddm-app-badge">MCP App 2026-07-28</span>
    </div>
    <div class="cddm-metrics">
      <span>Clusters: <span class="metric-val">{total_clusters}</span></span>
      <span>Sites: <span class="metric-val">{total_occs}</span></span>
      <span>Tokens: <span class="metric-val">{total_tokens}</span></span>
      <span>DRY Score: <span class="metric-val">{dry_score:.1}%</span></span>
    </div>
  </div>
  <div class="filter-bar">
    <input type="text" id="searchInput" class="filter-input" placeholder="Filter clusters by file path or type..." oninput="filterTiles()"/>
  </div>
  <div class="treemap-grid" id="tilesContainer">
    {tiles}
  </div>
  <div class="cddm-app-status">
    <span id="selectedMsg">Click a cluster tile to inspect or trigger refactoring.</span>
    <span>CDDM Generative Treemap Widget</span>
  </div>
  <script>
    function filterTiles() {{
      const query = document.getElementById('searchInput').value.toLowerCase();
      const tiles = document.querySelectorAll('.treemap-tile');
      tiles.forEach(tile => {{
        const files = (tile.getAttribute('data-files') || '').toLowerCase();
        const type = (tile.getAttribute('data-type') || '').toLowerCase();
        const text = tile.innerText.toLowerCase();
        if (files.includes(query) || type.includes(query) || text.includes(query)) {{
          tile.style.display = 'flex';
        }} else {{
          tile.style.display = 'none';
        }}
      }});
    }}
    function selectCluster(id, name) {{
      document.getElementById('selectedMsg').innerText = 'Selected Cluster #' + id + ': ' + name;
      window.parent.postMessage({{
        type: 'cddm:mcp-app-action',
        payload: {{ action: 'select-cluster', clusterId: id, clusterName: name }}
      }}, '*');
    }}
  </script>
</body>
</html>"#,
        title = escaped_title,
        total_clusters = total_clusters,
        total_occs = total_occurrences,
        total_tokens = total_tokens,
        dry_score = dry_health_score,
        tiles = tiles_html,
    );

    let data = json!({
        "clusterCount": total_clusters,
        "occurrenceCount": total_occurrences,
        "tokenCount": total_tokens,
        "dryHealthScore": dry_health_score,
        "clusters": clusters.iter().map(|c| json!({
            "id": c.id,
            "name": c.name,
            "cloneType": c.clone_type,
            "occurrences": c.occurrence_count,
            "tokens": c.token_count,
            "files": c.files,
        })).collect::<Vec<_>>()
    });

    AppWidget::new(widget_types::CLUSTER_TREEMAP, title, html, data)
}
