import {
  AlertTriangle,
  CheckCircle2,
  FileCode,
  Loader2,
  RefreshCw,
  Scissors,
  Search,
  Trash2,
} from "lucide-react";
import React, { useEffect, useMemo, useState } from "react";
import { useCDDMStore } from "../store/cddm-store";
import type { DeadCodeItem, DeadCodeKind } from "../types/dead-code-types";
import { Win2xWindow } from "./ui/win2x-manager";

export interface DeadCodeExplorerModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const DeadCodeExplorerModal: React.FC<DeadCodeExplorerModalProps> = ({
  isOpen,
  onClose,
}) => {
  const {
    deadCodeSummary,
    isDeadCodeLoading,
    deadCodeError,
    scanDeadCode,
    isDeadCodePruning,
    lastPruneResult,
    deadCodePruneError,
    pruneDeadCode,
  } = useCDDMStore();

  const [activeFilter, setActiveFilter] = useState<"all" | DeadCodeKind>("all");
  const [selectedPackage, setSelectedPackage] = useState<string>("all");
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set());
  const [dryRun, setDryRun] = useState(true);
  const [safeOnly, setSafeOnly] = useState(true);

  useEffect(() => {
    if (isOpen && !deadCodeSummary && !isDeadCodeLoading) {
      void scanDeadCode({ static_only: false });
    }
  }, [isOpen, deadCodeSummary, isDeadCodeLoading, scanDeadCode]);

  const packagesList = useMemo(() => {
    if (!deadCodeSummary?.items) return [];
    const pkgs = new Set<string>();
    for (const item of deadCodeSummary.items) {
      if (item.package_name) pkgs.add(item.package_name);
    }
    return Array.from(pkgs).sort();
  }, [deadCodeSummary]);

  const filteredItems = useMemo(() => {
    if (!deadCodeSummary?.items) return [];
    return deadCodeSummary.items.filter((item: DeadCodeItem) => {
      const matchesKind = activeFilter === "all" || item.kind === activeFilter;
      const matchesPkg =
        selectedPackage === "all" ||
        item.package_name === selectedPackage ||
        (!item.package_name && selectedPackage === "root");
      const q = searchQuery.toLowerCase().trim();
      const matchesSearch =
        !q ||
        item.file_path.toLowerCase().includes(q) ||
        item.symbol_name.toLowerCase().includes(q) ||
        item.reason.toLowerCase().includes(q) ||
        (item.package_name && item.package_name.toLowerCase().includes(q));
      return matchesKind && matchesPkg && matchesSearch;
    });
  }, [deadCodeSummary, activeFilter, selectedPackage, searchQuery]);

  const handleToggleSelectAll = () => {
    if (selectedIds.size === filteredItems.length && filteredItems.length > 0) {
      setSelectedIds(new Set());
    } else {
      setSelectedIds(new Set(filteredItems.map((i) => i.id)));
    }
  };

  const handleToggleItem = (id: number) => {
    setSelectedIds((prev) => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const handleExecutePrune = async () => {
    const itemIds = selectedIds.size > 0 ? Array.from(selectedIds) : undefined;
    const res = await pruneDeadCode({
      dry_run: dryRun,
      safe_only: safeOnly,
      item_ids: itemIds,
    });
    if (res && !dryRun && res.pruned_items > 0) {
      setSelectedIds(new Set());
      void scanDeadCode({ static_only: false });
    }
  };

  if (!isOpen) return null;

  return (
    <Win2xWindow
      id="cddm-dead-code-modal"
      title="Polyglot Dead Code Explorer & Safe Pruner"
      icon={<Trash2 className="w-4 h-4 text-rose-400" />}
      isOpen={isOpen}
      onClose={onClose}
      initialWidth={950}
      initialHeight={680}
    >
      <div className="flex flex-col h-full bg-[#1e1e2e] text-slate-200 text-sm overflow-hidden">
        {/* KPI Telemetry Banner */}
        <div className="grid grid-cols-2 sm:grid-cols-5 gap-2 p-3 bg-[#181825] border-b border-slate-700/60 text-xs">
          <div className="flex flex-col items-center justify-center p-2 rounded bg-slate-800/60 border border-slate-700/40">
            <span className="text-slate-400">Dead Items</span>
            <span className="text-lg font-bold text-white font-mono mt-0.5">
              {deadCodeSummary?.total_dead_items ?? 0}
            </span>
          </div>
          <div className="flex flex-col items-center justify-center p-2 rounded bg-slate-800/60 border border-slate-700/40">
            <span className="text-amber-400">Unreferenced</span>
            <span className="text-lg font-bold text-amber-300 font-mono mt-0.5">
              {deadCodeSummary?.dead_functions ?? 0}
            </span>
          </div>
          <div className="flex flex-col items-center justify-center p-2 rounded bg-slate-800/60 border border-slate-700/40">
            <span className="text-rose-400">Unreachable</span>
            <span className="text-lg font-bold text-rose-300 font-mono mt-0.5">
              {deadCodeSummary?.unreachable_blocks ?? 0}
            </span>
          </div>
          <div className="flex flex-col items-center justify-center p-2 rounded bg-slate-800/60 border border-slate-700/40">
            <span className="text-purple-400">Dead Clones</span>
            <span className="text-lg font-bold text-purple-300 font-mono mt-0.5">
              {deadCodeSummary?.dead_clones ?? 0}
            </span>
          </div>
          <div className="flex flex-col items-center justify-center p-2 rounded bg-slate-800/60 border border-slate-700/40">
            <span className="text-emerald-400">Dead Lines</span>
            <span className="text-lg font-bold text-emerald-300 font-mono mt-0.5">
              {deadCodeSummary?.total_dead_lines ?? 0}{" "}
              <span className="text-xs text-slate-400 font-normal">
                ({deadCodeSummary?.estimated_savings_pct.toFixed(1) ?? "0.0"}%)
              </span>
            </span>
          </div>
        </div>

        {/* Toolbar & Filter Tabs */}
        <div className="p-3 border-b border-slate-700/40 flex flex-wrap items-center justify-between gap-3 bg-[#181825]/80">
          <div className="flex flex-wrap items-center gap-1.5 text-xs font-mono">
            <button
              type="button"
              onClick={() => setActiveFilter("all")}
              className={`px-2.5 py-1 rounded border transition-colors ${
                activeFilter === "all"
                  ? "bg-indigo-600 text-white border-indigo-500 font-semibold"
                  : "bg-slate-850 text-slate-400 border-slate-700 hover:text-white"
              }`}
            >
              All ({deadCodeSummary?.total_dead_items ?? 0})
            </button>
            <button
              type="button"
              onClick={() => setActiveFilter("unreferenced_function")}
              className={`px-2.5 py-1 rounded border transition-colors ${
                activeFilter === "unreferenced_function"
                  ? "bg-amber-600 text-white border-amber-500 font-semibold"
                  : "bg-slate-850 text-slate-400 border-slate-700 hover:text-white"
              }`}
            >
              Functions ({deadCodeSummary?.dead_functions ?? 0})
            </button>
            <button
              type="button"
              onClick={() => setActiveFilter("unreachable_block")}
              className={`px-2.5 py-1 rounded border transition-colors ${
                activeFilter === "unreachable_block"
                  ? "bg-rose-600 text-white border-rose-500 font-semibold"
                  : "bg-slate-850 text-slate-400 border-slate-700 hover:text-white"
              }`}
            >
              Unreachable ({deadCodeSummary?.unreachable_blocks ?? 0})
            </button>
            <button
              type="button"
              onClick={() => setActiveFilter("dead_clone")}
              className={`px-2.5 py-1 rounded border transition-colors ${
                activeFilter === "dead_clone"
                  ? "bg-purple-600 text-white border-purple-500 font-semibold"
                  : "bg-slate-850 text-slate-400 border-slate-700 hover:text-white"
              }`}
            >
              Dead Clones ({deadCodeSummary?.dead_clones ?? 0})
            </button>
          </div>

          <div className="flex items-center gap-2">
            {packagesList.length > 0 && (
              <select
                id="dead-code-package-filter"
                aria-label="Filter dead code by workspace package"
                value={selectedPackage}
                onChange={(e) => setSelectedPackage(e.target.value)}
                className="px-2 py-1 bg-slate-900 border border-slate-700 rounded text-xs text-slate-300 focus:outline-hidden focus:border-indigo-500 font-mono"
              >
                <option value="all">All Packages ({packagesList.length})</option>
                {packagesList.map((pkg) => (
                  <option key={pkg} value={pkg}>
                    {pkg}
                  </option>
                ))}
              </select>
            )}

            <div className="relative w-40 sm:w-48">
              <Search className="w-3.5 h-3.5 text-slate-400 absolute left-2.5 top-1/2 -translate-y-1/2" />
              <input
                id="dead-code-search-query"
                name="dead_code_search"
                aria-label="Search dead code items by file or symbol"
                type="text"
                placeholder="Search file, symbol..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full pl-8 pr-2.5 py-1 bg-slate-900 border border-slate-700 rounded text-xs text-slate-200 placeholder:text-slate-500 focus:outline-hidden focus:border-indigo-500 font-mono"
              />
            </div>
            <button
              type="button"
              onClick={() => void scanDeadCode({ static_only: false })}
              disabled={isDeadCodeLoading}
              className="px-2.5 py-1 rounded bg-slate-800 hover:bg-slate-700 disabled:opacity-50 text-slate-300 text-xs font-mono flex items-center gap-1.5 transition-colors border border-slate-700"
            >
              <RefreshCw className={`w-3.5 h-3.5 ${isDeadCodeLoading ? "animate-spin" : ""}`} />
              <span>Rescan</span>
            </button>
          </div>
        </div>

        {/* Pruning Synthesizer Control Bar */}
        <div className="px-3 py-2 bg-slate-900/90 border-b border-slate-700/60 flex flex-wrap items-center justify-between gap-3 text-xs font-mono">
          <div className="flex items-center gap-4">
            <label className="flex items-center gap-1.5 cursor-pointer text-slate-300 hover:text-white select-none">
              <input
                type="checkbox"
                checked={dryRun}
                onChange={(e) => setDryRun(e.target.checked)}
                className="rounded border-slate-700 bg-slate-800 text-indigo-600 focus:ring-0"
              />
              <span>Dry Run Preview</span>
            </label>
            <label className="flex items-center gap-1.5 cursor-pointer text-slate-300 hover:text-white select-none">
              <input
                type="checkbox"
                checked={safeOnly}
                onChange={(e) => setSafeOnly(e.target.checked)}
                className="rounded border-slate-700 bg-slate-800 text-indigo-600 focus:ring-0"
              />
              <span>Strict Safe-Only (≥90%)</span>
            </label>
            {filteredItems.length > 0 && (
              <button
                type="button"
                onClick={handleToggleSelectAll}
                className="text-indigo-400 hover:text-indigo-300 text-[11px]"
              >
                {selectedIds.size === filteredItems.length ? "Deselect All" : "Select All"}
              </button>
            )}
          </div>

          <button
            type="button"
            onClick={() => void handleExecutePrune()}
            disabled={isDeadCodePruning || filteredItems.length === 0}
            className="px-3 py-1.5 rounded bg-rose-600 hover:bg-rose-500 disabled:opacity-50 text-white font-semibold flex items-center gap-1.5 shadow-sm transition-colors"
          >
            {isDeadCodePruning ? (
              <Loader2 className="w-3.5 h-3.5 animate-spin" />
            ) : (
              <Scissors className="w-3.5 h-3.5" />
            )}
            <span>
              {dryRun ? "Preview Pruning" : "Prune Dead Code"}
              {selectedIds.size > 0 ? ` (${selectedIds.size})` : " (All)"}
            </span>
          </button>
        </div>

        {/* Pruning Result Notification Banner */}
        {lastPruneResult && (
          <div
            className={`p-2.5 mx-3 mt-2 rounded border text-xs font-mono flex items-center justify-between ${
              lastPruneResult.dry_run
                ? "bg-amber-950/40 border-amber-800 text-amber-300"
                : "bg-emerald-950/40 border-emerald-800 text-emerald-300"
            }`}
          >
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 shrink-0" />
              <span>
                {lastPruneResult.dry_run ? "[DRY RUN] " : "[APPLIED] "}
                Pruned {lastPruneResult.pruned_items} items ({lastPruneResult.total_lines_removed}{" "}
                LOC saved) across {lastPruneResult.files_affected.length} files.
              </span>
            </div>
            {lastPruneResult.skipped_items > 0 && (
              <span className="text-slate-400">
                ({lastPruneResult.skipped_items} skipped unsafe)
              </span>
            )}
          </div>
        )}

        {/* Content Body */}
        <div className="flex-1 overflow-y-auto p-3 space-y-2 min-h-0">
          {isDeadCodeLoading ? (
            <div className="h-48 flex flex-col items-center justify-center gap-3 text-slate-400">
              <Loader2 className="w-8 h-8 animate-spin text-rose-500" />
              <p className="text-xs font-mono">Running AST and symbol dead code analysis...</p>
            </div>
          ) : deadCodeError || deadCodePruneError ? (
            <div className="p-3 rounded-lg bg-rose-950/50 border border-rose-800 text-rose-300 text-xs font-mono flex items-center gap-2">
              <AlertTriangle className="w-4 h-4 text-rose-400 shrink-0" />
              <span>{deadCodeError || deadCodePruneError}</span>
            </div>
          ) : filteredItems.length === 0 ? (
            <div className="h-48 flex flex-col items-center justify-center gap-2 text-slate-400">
              <CheckCircle2 className="w-8 h-8 text-emerald-400" />
              <p className="text-sm font-semibold text-white">Zero Dead Code Detected</p>
              <p className="text-xs text-slate-500">
                {searchQuery || activeFilter !== "all"
                  ? "No dead code items match current filter criteria."
                  : "All functions, symbols, and blocks in the analyzed scope are reachable and active."}
              </p>
            </div>
          ) : (
            filteredItems.map((item) => (
              <div
                key={item.id}
                className={`p-3 bg-slate-900/80 border rounded-lg flex flex-col sm:flex-row sm:items-center justify-between gap-2.5 transition-colors ${
                  selectedIds.has(item.id)
                    ? "border-indigo-500/80 bg-indigo-950/20"
                    : "border-slate-800 hover:border-slate-700"
                }`}
              >
                <div className="flex items-start gap-2.5 min-w-0">
                  <input
                    type="checkbox"
                    aria-label={`Select dead code item ${item.symbol_name}`}
                    checked={selectedIds.has(item.id)}
                    onChange={() => handleToggleItem(item.id)}
                    className="mt-1 rounded border-slate-700 bg-slate-800 text-indigo-600 focus:ring-0 cursor-pointer"
                  />
                  <div className="p-1.5 bg-slate-800 border border-slate-700 rounded text-slate-400 shrink-0 mt-0.5">
                    <FileCode className="w-3.5 h-3.5 text-indigo-400" />
                  </div>
                  <div className="min-w-0">
                    <div className="flex flex-wrap items-center gap-2">
                      <span className="font-mono font-bold text-xs text-white truncate">
                        {item.symbol_name}
                      </span>
                      <span
                        className={`text-[9px] px-1.5 py-0.5 rounded font-mono uppercase font-semibold border ${
                          item.kind === "unreferenced_function"
                            ? "bg-amber-950 text-amber-300 border-amber-800/60"
                            : item.kind === "unreachable_block"
                              ? "bg-rose-950 text-rose-300 border-rose-800/60"
                              : item.kind === "dead_clone"
                                ? "bg-purple-950 text-purple-300 border-purple-800/60"
                                : "bg-cyan-950 text-cyan-300 border-cyan-800/60"
                        }`}
                      >
                        {item.kind.replace("_", " ")}
                      </span>
                      <span className="text-[9px] font-mono text-emerald-400 bg-emerald-950/60 border border-emerald-800/50 px-1.5 py-0.5 rounded">
                        +{item.estimated_lines_saved} LOC saved
                      </span>
                      {item.package_name && (
                        <span className="text-[9px] font-mono text-indigo-300 bg-indigo-950/70 border border-indigo-800/60 px-1.5 py-0.5 rounded">
                          {item.package_name}
                        </span>
                      )}
                      {item.is_exported && (
                        <span className="text-[9px] font-mono text-amber-300 bg-amber-950/60 border border-amber-800/50 px-1.5 py-0.5 rounded">
                          Exported
                        </span>
                      )}
                      {item.cross_package_callers && item.cross_package_callers.length > 0 && (
                        <span className="text-[9px] font-mono text-cyan-300 bg-cyan-950/60 border border-cyan-800/50 px-1.5 py-0.5 rounded">
                          {item.cross_package_callers.length} callers
                        </span>
                      )}
                    </div>
                    <div className="text-[11px] font-mono text-slate-400 truncate mt-0.5">
                      {item.file_path}:{item.line_start}-{item.line_end}
                    </div>
                    <div className="text-[11px] text-slate-400 mt-0.5">{item.reason}</div>
                  </div>
                </div>

                <div className="shrink-0 flex items-center gap-2 self-end sm:self-center">
                  <span className="text-[11px] font-mono text-slate-400">
                    Confidence: {(item.confidence * 100).toFixed(0)}%
                  </span>
                </div>
              </div>
            ))
          )}
        </div>

        {/* Footer */}
        <div className="px-4 py-2 border-t border-slate-700/60 bg-[#181825] flex items-center justify-between text-xs text-slate-400 font-mono">
          <span>
            Showing {filteredItems.length} of {deadCodeSummary?.total_dead_items ?? 0} items
            {selectedIds.size > 0 && ` (${selectedIds.size} selected)`}
          </span>
          <span>CDDM PDG & Closed-Loop Reachability Engine</span>
        </div>
      </div>
    </Win2xWindow>
  );
};

export default DeadCodeExplorerModal;
