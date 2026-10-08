import React, { useState } from "react";
import {
  AlertCircle,
  ChevronDown,
  ChevronRight,
  GitBranch,
  GitCompare,
  Layers,
  RefreshCw,
  TrendingDown,
  TrendingUp,
} from "lucide-react";
import { useCDDMStore } from "../store/cddm-store";
import type { CloneStatus, DiffClonePair } from "../types/cddm-types";
import { Win2xWindow } from "./ui/win2x-manager";

export interface DiffScanResultsModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const DiffScanResultsModal: React.FC<DiffScanResultsModalProps> = ({ isOpen, onClose }) => {
  const { diffScanResult, isDiffScanning, diffScanError, startDiffScan } = useCDDMStore();

  const [baseRef, setBaseRef] = useState<string>("main");
  const [targetRef, setTargetRef] = useState<string>("");
  const [statusFilter, setStatusFilter] = useState<CloneStatus | "All">("All");
  const [expandedPairs, setExpandedPairs] = useState<Record<string, boolean>>({});

  if (!isOpen) return null;

  const summary = diffScanResult?.summary ?? null;
  const allClones: DiffClonePair[] = diffScanResult?.diff_clones ?? [];

  const filteredClones =
    statusFilter === "All" ? allClones : allClones.filter((pair) => pair?.status === statusFilter);

  const handleRunDiff = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!baseRef.trim()) return;
    await startDiffScan(baseRef.trim(), targetRef.trim() || undefined);
  };

  const toggleExpand = (pairId: string) => {
    setExpandedPairs((prev) => ({ ...prev, [pairId]: !prev[pairId] }));
  };

  const isPositiveDelta = (summary?.net_dry_delta ?? 0) >= 0;

  const footerContent = (
    <div className="flex items-center justify-between w-full">
      <div className="flex items-center gap-4 text-xs font-mono text-slate-400">
        <span className="flex items-center gap-1.5">
          <span className="w-2 h-2 rounded-full bg-rose-400" /> New Clones
        </span>
        <span className="flex items-center gap-1.5">
          <span className="w-2 h-2 rounded-full bg-slate-400" /> Legacy
        </span>
        <span className="flex items-center gap-1.5">
          <span className="w-2 h-2 rounded-full bg-emerald-400" /> Resolved
        </span>
      </div>
      <button
        type="button"
        onClick={onClose}
        className="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-medium transition-colors"
      >
        Close
      </button>
    </div>
  );

  return (
    <Win2xWindow
      id="cddm-diff-scan-modal"
      windowType="diff-scan"
      isOpen={isOpen}
      onClose={onClose}
      title="Differential Codebase Scan & Branch Comparison"
      subtitle="Analyze newly introduced, pre-existing legacy, and resolved clone pairs relative to baseline Git ref"
      badge={summary ? `${allClones.length} Clones` : "Idle"}
      footer={footerContent}
      initialWidth={920}
      initialHeight={680}
    >
      <div className="space-y-6">
        {/* Branch / Ref Selection Toolbar */}
        <form
          onSubmit={handleRunDiff}
          className="bg-slate-900/60 p-4 rounded-xl border border-slate-800/80 flex flex-wrap items-end gap-3"
        >
          <div className="flex-1 min-w-[200px]">
            <label
              htmlFor="diff-base-ref"
              className="block text-[11px] font-bold uppercase tracking-wider text-slate-400 mb-1.5"
            >
              Baseline Git Ref
            </label>
            <div className="relative">
              <GitBranch className="w-4 h-4 text-indigo-400 absolute left-3 top-1/2 -translate-y-1/2 pointer-events-none" />
              <input
                id="diff-base-ref"
                type="text"
                value={baseRef}
                onChange={(e) => setBaseRef(e.target.value)}
                placeholder="e.g. main, origin/main, HEAD~1"
                required
                className="w-full bg-slate-950 border border-slate-800 rounded-lg pl-9 pr-3 py-1.5 text-xs text-slate-200 font-mono focus:outline-none focus:border-indigo-500"
              />
            </div>
          </div>

          <div className="flex-1 min-w-[200px]">
            <label
              htmlFor="diff-target-ref"
              className="block text-[11px] font-bold uppercase tracking-wider text-slate-400 mb-1.5"
            >
              Target Git Ref (Optional)
            </label>
            <div className="relative">
              <GitBranch className="w-4 h-4 text-purple-400 absolute left-3 top-1/2 -translate-y-1/2 pointer-events-none" />
              <input
                id="diff-target-ref"
                type="text"
                value={targetRef}
                onChange={(e) => setTargetRef(e.target.value)}
                placeholder="Leave empty for working directory"
                className="w-full bg-slate-950 border border-slate-800 rounded-lg pl-9 pr-3 py-1.5 text-xs text-slate-200 font-mono focus:outline-none focus:border-indigo-500"
              />
            </div>
          </div>

          <button
            type="submit"
            disabled={isDiffScanning || !baseRef.trim()}
            className="px-4 py-2 rounded-lg bg-linear-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 disabled:opacity-50 text-white text-xs font-semibold flex items-center gap-2 transition-all shadow-md cursor-pointer"
          >
            <RefreshCw className={`w-3.5 h-3.5 ${isDiffScanning ? "animate-spin" : ""}`} />
            <span>{isDiffScanning ? "Comparing..." : "Run Diff Scan"}</span>
          </button>
        </form>

        {/* Error Notification */}
        {diffScanError && (
          <div className="p-3 bg-rose-950/40 border border-rose-800/60 rounded-xl text-rose-300 text-xs flex items-center gap-2">
            <AlertCircle className="w-4 h-4 shrink-0 text-rose-400" />
            <span>{diffScanError}</span>
          </div>
        )}

        {/* Uninitialized Empty State */}
        {!summary && !isDiffScanning && !diffScanError && (
          <div className="text-center py-16 px-4 bg-slate-900/30 border border-slate-800/40 rounded-xl">
            <GitCompare className="w-12 h-12 text-slate-600 mx-auto mb-3" />
            <h3 className="text-sm font-semibold text-slate-300">
              No Differential Comparison Computed
            </h3>
            <p className="text-xs text-slate-400 mt-1 max-w-md mx-auto">
              Select a baseline Git reference above and click &quot;Run Diff Scan&quot; to inspect
              newly introduced clone drift, legacy clones, and DRY score deltas.
            </p>
          </div>
        )}

        {/* Summary Metric Cards */}
        {summary && (
          <div className="space-y-4">
            <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3">
              <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-3">
                <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                  Net DRY Delta
                </span>
                <p
                  className={`text-base font-mono font-bold mt-1 flex items-center gap-1 ${
                    isPositiveDelta ? "text-emerald-400" : "text-rose-400"
                  }`}
                >
                  {isPositiveDelta ? (
                    <TrendingUp className="w-4 h-4" />
                  ) : (
                    <TrendingDown className="w-4 h-4" />
                  )}
                  {summary.net_dry_delta >= 0 ? "+" : ""}
                  {summary.net_dry_delta.toFixed(2)}%
                </p>
              </div>

              <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-3">
                <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                  New Clones
                </span>
                <p className="text-base font-mono font-bold text-rose-400 mt-1">
                  +{summary.new_clones}
                </p>
              </div>

              <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-3">
                <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                  Legacy Clones
                </span>
                <p className="text-base font-mono font-bold text-slate-300 mt-1">
                  {summary.legacy_clones}
                </p>
              </div>

              <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-3">
                <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                  Base DRY Score
                </span>
                <p className="text-base font-mono font-bold text-slate-200 mt-1">
                  {summary.base_dry_score.toFixed(1)}%
                </p>
              </div>

              <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-3">
                <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                  Target DRY Score
                </span>
                <p className="text-base font-mono font-bold text-indigo-300 mt-1">
                  {summary.target_dry_score.toFixed(1)}%
                </p>
              </div>

              <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-3">
                <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                  Changed Files
                </span>
                <p className="text-base font-mono font-bold text-slate-200 mt-1">
                  {summary.total_changed_files}
                </p>
              </div>
            </div>

            {/* Filter Tabs & Clone Pair List */}
            <div className="border border-slate-800/80 rounded-xl overflow-hidden bg-slate-900/40">
              <div className="px-4 py-2.5 bg-slate-900/80 border-b border-slate-800/80 flex flex-wrap items-center justify-between gap-2">
                <span className="text-xs font-bold text-slate-300 flex items-center gap-1.5">
                  <Layers className="w-3.5 h-3.5 text-indigo-400" />
                  <span>Differential Clone Pairs</span>
                </span>
                <div className="flex items-center gap-1">
                  {(["All", "New", "Legacy", "Resolved"] as const).map((filter) => (
                    <button
                      key={filter}
                      type="button"
                      onClick={() => setStatusFilter(filter)}
                      className={`px-2.5 py-1 rounded-md text-xs font-medium transition-colors ${
                        statusFilter === filter
                          ? "bg-indigo-600 text-white"
                          : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
                      }`}
                    >
                      {filter}
                    </button>
                  ))}
                </div>
              </div>

              <div className="divide-y divide-slate-800/60 max-h-96 overflow-y-auto">
                {filteredClones.length === 0 ? (
                  <div className="p-8 text-center text-xs text-slate-500">
                    No clone pairs matching &quot;{statusFilter}&quot; status filter.
                  </div>
                ) : (
                  filteredClones.map((diffPair, idx) => {
                    const pair = diffPair?.clone_pair;
                    const pairId = `${pair?.file_a || idx}:${pair?.start_line_a}-${pair?.file_b || idx}:${pair?.start_line_b}`;
                    const isExpanded = !!expandedPairs[pairId];
                    const status = diffPair?.status ?? "Legacy";

                    return (
                      <div key={pairId} className="p-3.5 hover:bg-slate-800/30 transition-colors">
                        <div className="flex flex-wrap items-center justify-between gap-2">
                          <div className="flex items-center gap-2">
                            <button
                              type="button"
                              onClick={() => toggleExpand(pairId)}
                              aria-label={isExpanded ? "Collapse diff" : "Expand diff"}
                              className="text-slate-400 hover:text-slate-200"
                            >
                              {isExpanded ? (
                                <ChevronDown className="w-4 h-4" />
                              ) : (
                                <ChevronRight className="w-4 h-4" />
                              )}
                            </button>
                            <span
                              className={`px-2 py-0.5 rounded text-[10px] font-mono font-bold uppercase tracking-wider ${
                                status === "New"
                                  ? "bg-rose-950 text-rose-300 border border-rose-800/50"
                                  : status === "Resolved"
                                    ? "bg-emerald-950 text-emerald-300 border border-emerald-800/50"
                                    : "bg-slate-800 text-slate-300 border border-slate-700/50"
                              }`}
                            >
                              {status === "New"
                                ? "+ New"
                                : status === "Resolved"
                                  ? "Resolved"
                                  : "Legacy"}
                            </span>
                            <span className="text-xs font-mono text-slate-200">
                              {pair?.file_a || "File A"}
                              <span className="text-slate-500 mx-1">vs</span>
                              {pair?.file_b || "File B"}
                            </span>
                          </div>

                          <div className="flex items-center gap-3 text-xs font-mono text-slate-400">
                            <span>{pair?.token_count ?? 0} tokens</span>
                            <span className="text-indigo-400">
                              {((pair?.similarity ?? 1) * 100).toFixed(0)}% match
                            </span>
                          </div>
                        </div>

                        {/* Collapsible Details */}
                        {isExpanded && pair && (
                          <div className="mt-3 pl-6 grid grid-cols-1 md:grid-cols-2 gap-3 text-xs font-mono">
                            <div className="p-2.5 bg-slate-950 rounded-lg border border-slate-800/80">
                              <span className="text-[10px] text-slate-400 uppercase block mb-1">
                                File A: Lines {pair.start_line_a} - {pair.end_line_a}
                              </span>
                              <div className="text-slate-300 text-[11px] truncate">
                                Hash: {pair.fragment_hash}
                              </div>
                            </div>
                            <div className="p-2.5 bg-slate-950 rounded-lg border border-slate-800/80">
                              <span className="text-[10px] text-slate-400 uppercase block mb-1">
                                File B: Lines {pair.start_line_b} - {pair.end_line_b}
                              </span>
                              <div className="text-slate-300 text-[11px] truncate">
                                Type: {pair.clone_type}
                              </div>
                            </div>
                          </div>
                        )}
                      </div>
                    );
                  })
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    </Win2xWindow>
  );
};
