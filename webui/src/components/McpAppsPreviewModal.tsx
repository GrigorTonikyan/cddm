import { AppWindow, Check, Copy, Download, RefreshCw } from "lucide-react";
import React, { useCallback, useEffect, useState } from "react";
import { API_ROUTES } from "../constants/cddm-constants";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xWindow } from "./ui/win2x-manager";

interface McpAppsPreviewModalProps {
  isOpen: boolean;
  onClose: () => void;
}

interface WidgetData {
  type: string;
  widgetType: string;
  title: string;
  html: string;
  data: Record<string, unknown>;
}

const DEFAULT_DIFF_HTML = `<!DOCTYPE html>
<html><head><style>body{background:#020617;color:#f8fafc;font-family:sans-serif;padding:16px;}.hdr{font-size:14px;color:#a5b4fc;margin-bottom:8px;}table{width:100%;border-collapse:collapse;font-family:monospace;font-size:12px;background:#0f172a;border:1px solid #1e293b;}td{padding:4px 8px;}.del{background:rgba(244,63,94,0.15);color:#fca5a5;}.add{background:rgba(16,185,129,0.15);color:#6ee7b7;}</style></head>
<body><div class="hdr">Interactive Diff Split View (Preview)</div>
<table><tr><td class="del">- fn calculate(a: i32, b: i32) -> i32 { a + b }</td></tr><tr><td class="add">+ fn calculate_sum(a: i32, b: i32) -> i32 { a + b }</td></tr></table>
</body></html>`;

const DEFAULT_TREEMAP_HTML = `<!DOCTYPE html>
<html><head><style>body{background:#020617;color:#f8fafc;font-family:sans-serif;padding:16px;}.grid{display:grid;grid-template-columns:repeat(2,1fr);gap:10px;}.tile{padding:12px;border-radius:6px;background:rgba(99,102,241,0.15);border:1px solid #6366f1;font-size:12px;}</style></head>
<body><div style="font-size:14px;color:#a5b4fc;margin-bottom:10px;">Cluster Treemap (Preview)</div>
<div class="grid"><div class="tile"><strong>Cluster #1</strong><br/>2 sites &bull; 80 tokens</div><div class="tile"><strong>Cluster #2</strong><br/>3 sites &bull; 140 tokens</div></div>
</body></html>`;

export const McpAppsPreviewModal: React.FC<McpAppsPreviewModalProps> = ({ isOpen, onClose }) => {
  const [activeTab, setActiveTab] = useState<"diff" | "treemap" | "raw">("diff");
  const [loading, setLoading] = useState(false);
  const [copied, setCopied] = useState(false);
  const [diffWidget, setDiffWidget] = useState<WidgetData | null>(null);
  const [treemapWidget, setTreemapWidget] = useState<WidgetData | null>(null);

  const results = useCDDMStore((state) => state.results);

  const fetchWidgets = useCallback(async () => {
    setLoading(true);
    try {
      // 1. Fetch Diff Split-View Widget
      const diffPayload = {
        widget_type: "diff-split-view",
        title: "Clone Pair Refactoring Diff",
        file_a: results?.clone_pairs?.[0]?.file_a || "crates/cddm-core/src/scan.rs",
        start_line_a: results?.clone_pairs?.[0]?.start_line_a || 40,
        end_line_a: results?.clone_pairs?.[0]?.end_line_a || 55,
        file_b: results?.clone_pairs?.[0]?.file_b || "crates/cddm-core/src/refactor.rs",
        start_line_b: results?.clone_pairs?.[0]?.start_line_b || 80,
        end_line_b: results?.clone_pairs?.[0]?.end_line_b || 95,
        original_code:
          "fn process_data(items: &[String]) -> usize {\n    items.iter().filter(|x| !x.is_empty()).count()\n}",
        refactored_code:
          "pub fn count_non_empty_items(items: &[String]) -> usize {\n    items.iter().filter(|x| !x.is_empty()).count()\n}",
        diff_patch:
          "--- a/crates/cddm-core/src/scan.rs\n+++ b/crates/cddm-core/src/refactor.rs\n@@ -1,3 +1,3 @@\n-fn process_data(items: &[String]) -> usize {\n+pub fn count_non_empty_items(items: &[String]) -> usize {\n     items.iter().filter(|x| !x.is_empty()).count()\n }",
      };

      const diffRes = await fetch(API_ROUTES.MCP_APPS_RENDER, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(diffPayload),
      });
      if (diffRes.ok) {
        const data = await diffRes.json();
        if (data.widget) setDiffWidget(data.widget);
      }

      // 2. Fetch Cluster Treemap Widget
      const treemapPayload = {
        widget_type: "cluster-treemap",
        title: "Codebase Duplication Treemap",
        clusters: results?.clone_clusters?.slice(0, 8).map((c) => ({
          id: c.id,
          name: `Cluster #${c.id}`,
          clone_type: String(c.clone_type),
          occurrence_count: c.occurrences?.length || 2,
          token_count: c.token_count || 60,
          files: c.occurrences?.map((o) => o.file) || [],
        })) || [
          {
            id: 1,
            name: "Cluster #1",
            clone_type: "exact",
            occurrence_count: 2,
            token_count: 90,
            files: ["src/a.rs", "src/b.rs"],
          },
          {
            id: 2,
            name: "Cluster #2",
            clone_type: "renamed",
            occurrence_count: 3,
            token_count: 140,
            files: ["src/c.rs", "src/d.rs"],
          },
        ],
        dry_health_score: results?.dry_health_score ?? 94.2,
      };

      const treemapRes = await fetch(API_ROUTES.MCP_APPS_RENDER, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(treemapPayload),
      });
      if (treemapRes.ok) {
        const data = await treemapRes.json();
        if (data.widget) setTreemapWidget(data.widget);
      }
    } catch {
      // Offline fallback: keep pre-rendered default HTML
    } finally {
      setLoading(false);
    }
  }, [results]);

  useEffect(() => {
    if (isOpen) {
      void fetchWidgets();
    }
  }, [isOpen, fetchWidgets]);

  if (!isOpen) return null;

  const currentHtml =
    activeTab === "diff"
      ? diffWidget?.html || DEFAULT_DIFF_HTML
      : treemapWidget?.html || DEFAULT_TREEMAP_HTML;

  const handleCopy = () => {
    void navigator.clipboard.writeText(currentHtml);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleDownload = () => {
    const blob = new Blob([currentHtml], { type: "text/html" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `cddm-mcp-${activeTab}-widget.html`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <Win2xWindow
      id="cddm-mcp-apps-modal"
      title="MCP Apps Generative UI Studio"
      icon={<AppWindow className="w-4 h-4 text-cyan-400" />}
      isOpen={isOpen}
      onClose={onClose}
      initialWidth={1000}
      initialHeight={720}
    >
      <div className="flex flex-col h-full bg-slate-900 text-slate-200 text-sm overflow-hidden">
        {/* Header Toolbar */}
        <div className="px-4 py-2.5 bg-slate-950/50 border-b border-slate-800/80 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="text-xs bg-indigo-950 text-indigo-300 font-mono px-2 py-0.5 rounded-full border border-indigo-800/50">
              MCP 2026-07-28
            </span>
            <span className="text-xs text-slate-400 hidden sm:inline">
              Interactive generative UI widgets rendered directly inside preview hosts
            </span>
          </div>

          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={() => void fetchWidgets()}
              disabled={loading}
              className="p-1.5 rounded-lg bg-slate-800/80 hover:bg-slate-800 text-slate-300 hover:text-white transition-colors"
              title="Refresh widget renders"
            >
              <RefreshCw className={`w-3.5 h-3.5 ${loading ? "animate-spin" : ""}`} />
            </button>
            <button
              type="button"
              onClick={handleCopy}
              className="p-1.5 rounded-lg bg-slate-800/80 hover:bg-slate-800 text-slate-300 hover:text-white transition-colors"
              title="Copy HTML to clipboard"
            >
              {copied ? (
                <Check className="w-3.5 h-3.5 text-emerald-400" />
              ) : (
                <Copy className="w-3.5 h-3.5" />
              )}
            </button>
            <button
              type="button"
              onClick={handleDownload}
              className="p-1.5 rounded-lg bg-slate-800/80 hover:bg-slate-800 text-slate-300 hover:text-white transition-colors"
              title="Download standalone HTML widget"
            >
              <Download className="w-3.5 h-3.5" />
            </button>
          </div>
        </div>

        {/* Tab Controls */}
        <div className="px-4 py-2 bg-slate-950/30 border-b border-slate-800/60 flex items-center justify-between text-xs">
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={() => setActiveTab("diff")}
              className={`px-3 py-1 rounded-lg font-medium transition-colors ${
                activeTab === "diff"
                  ? "bg-indigo-600 text-white shadow-sm"
                  : "bg-slate-950 text-slate-400 hover:text-slate-200 border border-slate-800"
              }`}
            >
              Diff Split-View Widget
            </button>
            <button
              type="button"
              onClick={() => setActiveTab("treemap")}
              className={`px-3 py-1 rounded-lg font-medium transition-colors ${
                activeTab === "treemap"
                  ? "bg-indigo-600 text-white shadow-sm"
                  : "bg-slate-950 text-slate-400 hover:text-slate-200 border border-slate-800"
              }`}
            >
              Cluster Treemap Widget
            </button>
            <button
              type="button"
              onClick={() => setActiveTab("raw")}
              className={`px-3 py-1 rounded-lg font-medium transition-colors ${
                activeTab === "raw"
                  ? "bg-indigo-600 text-white shadow-sm"
                  : "bg-slate-950 text-slate-400 hover:text-slate-200 border border-slate-800"
              }`}
            >
              Raw HTML & Schema
            </button>
          </div>

          <div className="font-mono text-slate-500 text-[11px] hidden sm:block">
            MIME: text/html &bull; Sandboxed Host
          </div>
        </div>

        {/* Main Body Preview */}
        <div className="flex-1 p-3 bg-slate-950/40 overflow-hidden flex flex-col">
          {activeTab === "raw" ? (
            <div className="flex-1 flex flex-col gap-3 font-mono text-xs overflow-auto">
              <div className="flex-1 p-3 bg-slate-950 border border-slate-800 rounded-xl overflow-auto text-slate-300">
                <pre>{currentHtml}</pre>
              </div>
            </div>
          ) : (
            <div className="flex-1 border border-slate-800/80 rounded-xl overflow-hidden shadow-inner bg-slate-950">
              <iframe
                title="MCP App Generative UI Widget"
                srcDoc={currentHtml}
                sandbox="allow-scripts allow-modals"
                className="w-full h-full border-0"
              />
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="px-4 py-2 border-t border-slate-800/80 bg-slate-950/60 flex items-center justify-between text-xs text-slate-400 font-mono">
          <span>Active widget: {activeTab === "diff" ? "diff-split-view" : "cluster-treemap"}</span>
          <span>CDDM MCP 2026-07-28 Apps Framework</span>
        </div>
      </div>
    </Win2xWindow>
  );
};

export default McpAppsPreviewModal;
