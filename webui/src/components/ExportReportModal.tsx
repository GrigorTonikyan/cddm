import React, { useState, useMemo } from "react";
import { ScanResult } from "../types/cddm-types";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  FileDown,
  Copy,
  Check,
  Download,
  FileCode,
  FileText,
  ShieldCheck,
  Terminal,
} from "lucide-react";

export interface ExportReportModalProps {
  isOpen: boolean;
  onClose: () => void;
  results: ScanResult;
}

export const ExportReportModal: React.FC<ExportReportModalProps> = ({
  isOpen,
  onClose,
  results,
}) => {
  const [activeTab, setActiveTab] = useState<"sarif" | "json" | "markdown" | "ci">("sarif");
  const [copied, setCopied] = useState<string | null>(null);

  // Generate SARIF v2.1.0 compliant report JSON client-side
  const sarifData = useMemo(() => {
    return {
      $schema:
        "https://docs.oasis-open.org/sarif/sarif/v2.1.0/cos02/schemas/sarif-schema-2.1.0.json",
      version: "2.1.0",
      runs: [
        {
          tool: {
            driver: {
              name: "CDDM",
              version: "0.1.0",
              informationUri: "https://github.com/GrigorTonikyan/cddm",
              rules: [
                {
                  id: "CDDM001",
                  name: "CodeDuplication",
                  shortDescription: { text: "Duplicate code fragment detected" },
                  defaultConfiguration: { level: "warning" },
                },
              ],
            },
          },
          results: results.clone_pairs.map((pair, idx) => ({
            ruleId: "CDDM001",
            ruleIndex: 0,
            level: "warning",
            message: {
              text: `Code clone detected (${(pair.similarity * 100).toFixed(0)}% match, ${pair.token_count} tokens) between ${pair.file_a}:${pair.start_line_a}-${pair.end_line_a} and ${pair.file_b}:${pair.start_line_b}-${pair.end_line_b}`,
            },
            locations: [
              {
                physicalLocation: {
                  artifactLocation: { uri: pair.file_a },
                  region: {
                    startLine: pair.start_line_a,
                    endLine: pair.end_line_a,
                  },
                },
              },
              {
                physicalLocation: {
                  artifactLocation: { uri: pair.file_b },
                  region: {
                    startLine: pair.start_line_b,
                    endLine: pair.end_line_b,
                  },
                },
              },
            ],
            properties: {
              index: idx + 1,
              clone_type: pair.clone_type,
              fragment_hash: pair.fragment_hash,
              similarity: pair.similarity,
              token_count: pair.token_count,
            },
          })),
        },
      ],
    };
  }, [results]);

  const sarifString = useMemo(() => JSON.stringify(sarifData, null, 2), [sarifData]);
  const jsonString = useMemo(() => JSON.stringify(results, null, 2), [results]);

  const markdownString = useMemo(() => {
    return `# CDDM Code De-Duplication & Architecture Report

## Summary
- **DRY Health Score**: ${results.dry_health_score.toFixed(1)} / 100
- **Duplication Rate**: ${results.duplication_percentage.toFixed(2)}%
- **Files Scanned**: ${results.total_files.toLocaleString()}
- **Total Indexed Tokens**: ${results.total_tokens.toLocaleString()}
- **Identified Clone Pairs**: ${results.total_clones.toLocaleString()}
- **Scan Duration**: ${results.duration_ms} ms

## Top Clone Pairs
| # | File A | File B | Tokens | Similarity | Type |
|---|--------|--------|--------|------------|------|
${results.clone_pairs
  .slice(0, 10)
  .map(
    (p, i) =>
      `| ${i + 1} | \`${p.file_a}:${p.start_line_a}-${p.end_line_a}\` | \`${p.file_b}:${p.start_line_b}-${p.end_line_b}\` | ${p.token_count} | ${(p.similarity * 100).toFixed(0)}% | ${p.clone_type || "Exact"} |`,
  )
  .join("\n")}
`;
  }, [results]);

  const ciYamlString = useMemo(() => {
    return `name: CDDM Duplication Quality Gate

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  cddm-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust Toolchain
        uses: dtolnay/rust-toolchain@stable
      - name: Run CDDM Duplication Scan & SARIF Export
        run: |
          cargo run -p cddm-cli -- scan . --min-tokens 50 --fail-threshold 15.0 --sarif cddm-results.sarif
      - name: Upload SARIF to GitHub Code Scanning
        uses: github/codeql-action/upload-sarif@v3
        if: always()
        with:
          sarif_file: cddm-results.sarif
`;
  }, []);

  if (!isOpen) return null;

  const handleCopy = (text: string, key: string) => {
    void navigator.clipboard.writeText(text);
    setCopied(key);
    setTimeout(() => setCopied(null), 2000);
  };

  const handleDownload = (content: string, filename: string, mime: string) => {
    const blob = new Blob([content], { type: mime });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const footerContent = (
    <>
      <div className="flex items-center gap-2 text-xs font-mono text-slate-400">
        <ShieldCheck className="w-3.5 h-3.5 text-indigo-400" />
        <span>OASIS SARIF v2.1.0 & ISO Standard Compliant</span>
      </div>
      <button
        type="button"
        onClick={onClose}
        className="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold text-xs transition-colors"
      >
        Close
      </button>
    </>
  );

  return (
    <Win2xWindow
      id="cddm-export-reports-window"
      windowType="export-reports"
      isOpen={isOpen}
      onClose={onClose}
      title="Report Center & SARIF Exporter"
      subtitle="OASIS SARIF v2.1.0, JSON, Markdown, and CI/CD Quality Gate Artifacts"
      badge="SARIF v2.1.0"
      icon={<FileDown className="w-4 h-4 text-indigo-400" />}
      footer={footerContent}
      initialWidth={920}
      initialHeight={680}
    >
      <div className="space-y-4">
        {/* Navigation Tabs */}
        <div className="flex items-center gap-1 bg-slate-950 p-1 rounded-xl border border-slate-800 text-xs font-mono">
          <button
            type="button"
            onClick={() => setActiveTab("sarif")}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-1.5 transition-all ${
              activeTab === "sarif"
                ? "bg-indigo-600 text-white font-semibold shadow-sm"
                : "text-slate-400 hover:text-slate-200"
            }`}
          >
            <ShieldCheck className="w-3.5 h-3.5" />
            OASIS SARIF v2.1.0
          </button>
          <button
            type="button"
            onClick={() => setActiveTab("json")}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-1.5 transition-all ${
              activeTab === "json"
                ? "bg-indigo-600 text-white font-semibold shadow-sm"
                : "text-slate-400 hover:text-slate-200"
            }`}
          >
            <FileCode className="w-3.5 h-3.5" />
            Scan JSON
          </button>
          <button
            type="button"
            onClick={() => setActiveTab("markdown")}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-1.5 transition-all ${
              activeTab === "markdown"
                ? "bg-indigo-600 text-white font-semibold shadow-sm"
                : "text-slate-400 hover:text-slate-200"
            }`}
          >
            <FileText className="w-3.5 h-3.5" />
            Markdown Summary
          </button>
          <button
            type="button"
            onClick={() => setActiveTab("ci")}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-1.5 transition-all ${
              activeTab === "ci"
                ? "bg-indigo-600 text-white font-semibold shadow-sm"
                : "text-slate-400 hover:text-slate-200"
            }`}
          >
            <Terminal className="w-3.5 h-3.5" />
            CI / GitHub Actions
          </button>
        </div>

        {/* Tab Content: SARIF */}
        {activeTab === "sarif" && (
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-xs font-mono text-slate-400">
                SARIF v2.1.0 document with {results.clone_pairs.length} diagnostics
              </span>
              <div className="flex items-center gap-2">
                <button
                  type="button"
                  onClick={() => handleCopy(sarifString, "sarif")}
                  className="px-2.5 py-1 rounded bg-slate-900 border border-slate-800 hover:bg-slate-800 text-slate-300 text-xs font-mono flex items-center gap-1.5"
                >
                  {copied === "sarif" ? (
                    <>
                      <Check className="w-3 h-3 text-emerald-400" />
                      <span>Copied</span>
                    </>
                  ) : (
                    <>
                      <Copy className="w-3 h-3" />
                      <span>Copy SARIF</span>
                    </>
                  )}
                </button>
                <button
                  type="button"
                  onClick={() =>
                    handleDownload(sarifString, "cddm-results.sarif", "application/json")
                  }
                  className="px-2.5 py-1 rounded bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs font-mono flex items-center gap-1.5 shadow-sm"
                >
                  <Download className="w-3 h-3" />
                  <span>Download .sarif</span>
                </button>
              </div>
            </div>
            <pre className="max-h-96 overflow-auto p-3.5 font-mono text-xs leading-relaxed bg-slate-950 rounded-xl border border-slate-800 text-slate-300 select-text">
              {sarifString}
            </pre>
          </div>
        )}

        {/* Tab Content: Scan JSON */}
        {activeTab === "json" && (
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-xs font-mono text-slate-400">
                Complete raw scan data payload
              </span>
              <div className="flex items-center gap-2">
                <button
                  type="button"
                  onClick={() => handleCopy(jsonString, "json")}
                  className="px-2.5 py-1 rounded bg-slate-900 border border-slate-800 hover:bg-slate-800 text-slate-300 text-xs font-mono flex items-center gap-1.5"
                >
                  {copied === "json" ? (
                    <>
                      <Check className="w-3 h-3 text-emerald-400" />
                      <span>Copied</span>
                    </>
                  ) : (
                    <>
                      <Copy className="w-3 h-3" />
                      <span>Copy JSON</span>
                    </>
                  )}
                </button>
                <button
                  type="button"
                  onClick={() =>
                    handleDownload(jsonString, "cddm-results.json", "application/json")
                  }
                  className="px-2.5 py-1 rounded bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs font-mono flex items-center gap-1.5 shadow-sm"
                >
                  <Download className="w-3 h-3" />
                  <span>Download .json</span>
                </button>
              </div>
            </div>
            <pre className="max-h-96 overflow-auto p-3.5 font-mono text-xs leading-relaxed bg-slate-950 rounded-xl border border-slate-800 text-slate-300 select-text">
              {jsonString}
            </pre>
          </div>
        )}

        {/* Tab Content: Markdown Summary */}
        {activeTab === "markdown" && (
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-xs font-mono text-slate-400">
                Formatted markdown summary for PRs and issues
              </span>
              <button
                type="button"
                onClick={() => handleCopy(markdownString, "markdown")}
                className="px-2.5 py-1 rounded bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs font-mono flex items-center gap-1.5 shadow-sm"
              >
                {copied === "markdown" ? (
                  <>
                    <Check className="w-3 h-3 text-emerald-400" />
                    <span>Copied</span>
                  </>
                ) : (
                  <>
                    <Copy className="w-3 h-3" />
                    <span>Copy Markdown</span>
                  </>
                )}
              </button>
            </div>
            <pre className="max-h-96 overflow-auto p-3.5 font-mono text-xs leading-relaxed bg-slate-950 rounded-xl border border-slate-800 text-slate-300 select-text">
              {markdownString}
            </pre>
          </div>
        )}

        {/* Tab Content: CI Configuration */}
        {activeTab === "ci" && (
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-xs font-mono text-slate-400">
                GitHub Actions workflow configuration (.github/workflows/cddm.yml)
              </span>
              <button
                type="button"
                onClick={() => handleCopy(ciYamlString, "ci")}
                className="px-2.5 py-1 rounded bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs font-mono flex items-center gap-1.5 shadow-sm"
              >
                {copied === "ci" ? (
                  <>
                    <Check className="w-3 h-3 text-emerald-400" />
                    <span>Copied</span>
                  </>
                ) : (
                  <>
                    <Copy className="w-3 h-3" />
                    <span>Copy YAML</span>
                  </>
                )}
              </button>
            </div>
            <pre className="max-h-96 overflow-auto p-3.5 font-mono text-xs leading-relaxed bg-slate-950 rounded-xl border border-slate-800 text-slate-300 select-text">
              {ciYamlString}
            </pre>
          </div>
        )}
      </div>
    </Win2xWindow>
  );
};
