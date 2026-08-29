import React, { useState, useEffect } from "react";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  Activity,
  Flame,
  FileCode2,
  AlertTriangle,
  CheckCircle2,
  RefreshCw,
  UploadCloud,
  Layers,
} from "lucide-react";
import type { CloneCoverageMetric, CoverageCorrelationSummary } from "../types/cddm-types";
import { useCDDMStore } from "../store/cddm-store";

export interface CoverageCorrelationModalProps {
  isOpen: boolean;
  onClose: () => void;
  initialSummary?: CoverageCorrelationSummary | null;
}

export const CoverageCorrelationModal: React.FC<CoverageCorrelationModalProps> = ({
  isOpen,
  onClose,
  initialSummary = null,
}) => {
  const { coverageSummary, isCoverageLoading, correlateCoverage, ingestCoverageReport } =
    useCDDMStore();
  const [filterMode, setFilterMode] = useState<"all" | "dead" | "hot" | "gaps">("all");
  const [rawTracefile, setRawTracefile] = useState<string>("");
  const [ingestSuccess, setIngestSuccess] = useState<boolean>(false);

  const summary = coverageSummary || initialSummary;

  useEffect(() => {
    if (isOpen && !summary) {
      void correlateCoverage();
    }
  }, [isOpen, summary, correlateCoverage]);

  if (!isOpen) return null;

  const handleIngest = async () => {
    if (!rawTracefile.trim()) return;
    try {
      await ingestCoverageReport({ report_content: rawTracefile, format: "auto" });
      setIngestSuccess(true);
      await correlateCoverage({ report_content: rawTracefile });
    } catch {
      setIngestSuccess(false);
    }
  };

  const filteredMetrics: CloneCoverageMetric[] = (summary?.metrics || []).filter((m) => {
    if (filterMode === "dead") return m.is_dead_code;
    if (filterMode === "hot") return m.execution_tier === "HotPath";
    if (filterMode === "gaps") return m.has_test_gap;
    return true;
  });

  return (
    <Win2xWindow
      id="cddm-coverage-modal"
      title="Runtime Execution & Coverage-Aware De-duplication"
      icon={<Activity className="w-4 h-4 text-emerald-400" />}
      isOpen={isOpen}
      onClose={onClose}
      initialWidth={900}
      initialHeight={600}
    >
      <div className="flex flex-col h-full bg-[#1e1e2e] text-slate-200 text-sm">
        {/* Header Stats Bar */}
        <div className="grid grid-cols-5 gap-2 p-3 bg-[#181825] border-b border-slate-700/60 text-xs">
          <div className="flex flex-col items-center justify-center p-2 rounded bg-slate-800/60 border border-slate-700/40">
            <span className="text-slate-400">Total Clones</span>
            <span className="text-lg font-bold text-cyan-400">
              {summary?.total_clone_pairs ?? 0}
            </span>
          </div>
          <div className="flex flex-col items-center justify-center p-2 rounded bg-slate-800/60 border border-slate-700/40">
            <span className="text-slate-400">Covered Rate</span>
            <span className="text-lg font-bold text-emerald-400">
              {summary?.overall_covered_clones_pct?.toFixed(1) ?? "0.0"}%
            </span>
          </div>
          <div className="flex flex-col items-center justify-center p-2 rounded bg-slate-800/60 border border-slate-700/40">
            <span className="text-slate-400">Dead Code Clones</span>
            <span className="text-lg font-bold text-amber-400">
              {summary?.dead_code_clones ?? 0}
            </span>
          </div>
          <div className="flex flex-col items-center justify-center p-2 rounded bg-slate-800/60 border border-slate-700/40">
            <span className="text-slate-400">Test Gaps</span>
            <span className="text-lg font-bold text-rose-400">{summary?.test_gap_clones ?? 0}</span>
          </div>
          <div className="flex flex-col items-center justify-center p-2 rounded bg-slate-800/60 border border-slate-700/40">
            <span className="text-slate-400">Hot Path Clones</span>
            <span className="text-lg font-bold text-red-500">{summary?.hot_path_clones ?? 0}</span>
          </div>
        </div>

        {/* Filter Navigation Bar */}
        <div className="flex items-center justify-between px-4 py-2 bg-[#181825]/80 border-b border-slate-700/40">
          <div className="flex items-center gap-1.5">
            <button
              onClick={() => setFilterMode("all")}
              className={`px-2.5 py-1 text-xs rounded transition-colors ${
                filterMode === "all"
                  ? "bg-cyan-600 text-white font-medium"
                  : "bg-slate-800 text-slate-300 hover:bg-slate-700"
              }`}
            >
              All Clones ({summary?.metrics?.length ?? 0})
            </button>
            <button
              onClick={() => setFilterMode("dead")}
              className={`px-2.5 py-1 text-xs rounded flex items-center gap-1 transition-colors ${
                filterMode === "dead"
                  ? "bg-amber-600 text-white font-medium"
                  : "bg-slate-800 text-slate-300 hover:bg-slate-700"
              }`}
            >
              <FileCode2 className="w-3 h-3 text-amber-400" />
              Dead Code ({summary?.dead_code_clones ?? 0})
            </button>
            <button
              onClick={() => setFilterMode("hot")}
              className={`px-2.5 py-1 text-xs rounded flex items-center gap-1 transition-colors ${
                filterMode === "hot"
                  ? "bg-red-600 text-white font-medium"
                  : "bg-slate-800 text-slate-300 hover:bg-slate-700"
              }`}
            >
              <Flame className="w-3 h-3 text-red-400" />
              Hot Paths ({summary?.hot_path_clones ?? 0})
            </button>
            <button
              onClick={() => setFilterMode("gaps")}
              className={`px-2.5 py-1 text-xs rounded flex items-center gap-1 transition-colors ${
                filterMode === "gaps"
                  ? "bg-rose-600 text-white font-medium"
                  : "bg-slate-800 text-slate-300 hover:bg-slate-700"
              }`}
            >
              <AlertTriangle className="w-3 h-3 text-rose-400" />
              Test Gaps ({summary?.test_gap_clones ?? 0})
            </button>
          </div>

          <button
            onClick={() => void correlateCoverage()}
            disabled={isCoverageLoading}
            className="flex items-center gap-1 px-2.5 py-1 text-xs bg-slate-800 hover:bg-slate-700 text-slate-300 rounded border border-slate-600/40"
          >
            <RefreshCw className={`w-3 h-3 ${isCoverageLoading ? "animate-spin" : ""}`} />
            Rescan
          </button>
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          {/* Ingest Tracefile Accordion */}
          <details className="bg-[#181825] rounded border border-slate-700/60 p-3">
            <summary className="cursor-pointer text-xs font-semibold text-cyan-400 flex items-center gap-1.5">
              <UploadCloud className="w-3.5 h-3.5" />
              Ingest Coverage Report (LCOV, Cobertura XML, Istanbul JSON)
            </summary>
            <div className="mt-3 space-y-2">
              <textarea
                value={rawTracefile}
                onChange={(e) => setRawTracefile(e.target.value)}
                placeholder="Paste lcov.info, coverage.xml, or istanbul.json content here..."
                className="w-full h-24 p-2 text-xs font-mono bg-[#11111b] border border-slate-700 rounded text-slate-300 focus:outline-none focus:border-cyan-500"
              />
              <div className="flex items-center justify-between">
                <button
                  onClick={handleIngest}
                  disabled={isCoverageLoading || !rawTracefile.trim()}
                  className="px-3 py-1 text-xs bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-white rounded font-medium"
                >
                  Parse & Correlate Tracefile
                </button>
                {ingestSuccess && (
                  <span className="flex items-center gap-1 text-xs text-emerald-400 font-medium">
                    <CheckCircle2 className="w-3.5 h-3.5" />
                    Coverage trace correlated successfully!
                  </span>
                )}
              </div>
            </div>
          </details>

          {/* Metrics Table */}
          {filteredMetrics.length === 0 ? (
            <div className="flex flex-col items-center justify-center p-8 bg-[#181825] rounded border border-slate-700/60 text-slate-400 space-y-2">
              <Layers className="w-8 h-8 text-slate-600" />
              <p>No duplicate clone pairs matched the active coverage filter.</p>
            </div>
          ) : (
            <div className="overflow-x-auto rounded border border-slate-700/60 bg-[#181825]">
              <table className="w-full text-left text-xs border-collapse">
                <thead>
                  <tr className="bg-slate-800/80 border-b border-slate-700 text-slate-300 font-semibold">
                    <th className="p-2.5">Pair</th>
                    <th className="p-2.5">Location A</th>
                    <th className="p-2.5">Hits A</th>
                    <th className="p-2.5">Location B</th>
                    <th className="p-2.5">Hits B</th>
                    <th className="p-2.5">Tier</th>
                    <th className="p-2.5">Risk</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-slate-700/40">
                  {filteredMetrics.map((m) => (
                    <tr key={m.clone_pair_id} className="hover:bg-slate-800/40 transition-colors">
                      <td className="p-2.5 font-bold text-cyan-400">#{m.clone_pair_id}</td>
                      <td className="p-2.5 font-mono text-slate-300">
                        {m.file_a}:{m.start_line_a}-{m.end_line_a}
                      </td>
                      <td className="p-2.5 font-mono text-yellow-400">{m.hits_a}</td>
                      <td className="p-2.5 font-mono text-slate-300">
                        {m.file_b}:{m.start_line_b}-{m.end_line_b}
                      </td>
                      <td className="p-2.5 font-mono text-yellow-400">{m.hits_b}</td>
                      <td className="p-2.5">
                        <span
                          className={`px-1.5 py-0.5 rounded text-[10px] font-bold ${
                            m.execution_tier === "HotPath"
                              ? "bg-red-900/60 text-red-400 border border-red-700"
                              : m.execution_tier === "Warm"
                                ? "bg-amber-900/60 text-amber-400 border border-amber-700"
                                : m.execution_tier === "DeadCode"
                                  ? "bg-slate-700/60 text-slate-400 border border-slate-600"
                                  : "bg-cyan-900/60 text-cyan-400 border border-cyan-700"
                          }`}
                        >
                          {m.execution_tier}
                        </span>
                      </td>
                      <td className="p-2.5 font-mono font-semibold text-rose-400">
                        {m.risk_score.toFixed(1)}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </div>
    </Win2xWindow>
  );
};
