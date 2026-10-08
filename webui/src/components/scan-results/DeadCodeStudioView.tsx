import {
  AlertTriangle,
  CheckCircle2,
  ExternalLink,
  FileCode2,
  Loader2,
  RefreshCw,
  Scissors,
  Search,
  Trash2,
} from "lucide-react";
import React, { useMemo, useState } from "react";
import { useCDDMStore } from "../../store/cddm-store";
import type { DeadCodeItem, DeadCodeKind } from "../../types/dead-code-types";
import { getEditorDisplayName, getIdeDeeplink } from "../../utils/ide-links";
import { parsePath } from "../../utils/path-utils";

export interface DeadCodeStudioViewProps {
  className?: string;
}

export const DeadCodeStudioView: React.FC<DeadCodeStudioViewProps> = ({ className = "" }) => {
  const {
    deadCodeSummary,
    isDeadCodeLoading,
    deadCodeError,
    scanDeadCode,
    isDeadCodePruning,
    lastPruneResult,
    deadCodePruneError,
    pruneDeadCode,
    preferredEditor,
  } = useCDDMStore();

  const [activeFilter, setActiveFilter] = useState<"all" | DeadCodeKind>("all");
  const [selectedPackage, setSelectedPackage] = useState<string>("all");
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set());
  const [dryRun, setDryRun] = useState(true);
  const [safeOnly, setSafeOnly] = useState(true);

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

  const cleanlinessPct = Math.max(
    0,
    Math.min(100, 100 - (deadCodeSummary?.estimated_savings_pct ?? 0)),
  ).toFixed(1);

  return (
    <div className={`space-y-4 ${className}`}>
      {/* Codebase Reachability & Cleanliness Gauge Bar */}
      <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-4 shadow-lg">
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-2 mb-2 font-mono text-xs">
          <div className="flex items-center gap-2">
            <span className="w-2.5 h-2.5 rounded-full bg-cyan-400 animate-pulse" />
            <span className="font-semibold text-slate-200">
              Codebase Reachability & Cleanliness
            </span>
          </div>
          <div className="text-slate-400">
            <span className="font-bold text-cyan-300">{cleanlinessPct}% Reachable</span> (
            {deadCodeSummary?.total_dead_items ?? 0} dead items / ~
            {deadCodeSummary?.total_dead_lines ?? 0} removable lines)
          </div>
        </div>
        <div className="w-full bg-slate-950 rounded-full h-2.5 border border-slate-800/80 overflow-hidden">
          <div
            className="bg-linear-to-r from-indigo-500 via-cyan-400 to-emerald-400 h-2.5 rounded-full transition-all duration-500"
            style={{ width: `${cleanlinessPct}%` }}
          />
        </div>
      </div>

      {/* 4 Telemetry KPI Cards */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-slate-900/80 border border-amber-900/40 rounded-xl p-4 flex flex-col justify-between shadow-lg">
          <span className="text-xs font-mono text-amber-400/90 font-medium">
            Unreferenced Funcs
          </span>
          <div className="mt-2 flex items-baseline justify-between">
            <span className="text-2xl font-bold font-mono text-amber-300">
              {deadCodeSummary?.dead_functions ?? 0}
            </span>
            <span className="text-[11px] font-mono text-slate-500">0 references</span>
          </div>
        </div>

        <div className="bg-slate-900/80 border border-rose-900/40 rounded-xl p-4 flex flex-col justify-between shadow-lg">
          <span className="text-xs font-mono text-rose-400/90 font-medium">Unreachable Blocks</span>
          <div className="mt-2 flex items-baseline justify-between">
            <span className="text-2xl font-bold font-mono text-rose-300">
              {deadCodeSummary?.unreachable_blocks ?? 0}
            </span>
            <span className="text-[11px] font-mono text-slate-500">dead branches</span>
          </div>
        </div>

        <div className="bg-slate-900/80 border border-purple-900/40 rounded-xl p-4 flex flex-col justify-between shadow-lg">
          <span className="text-xs font-mono text-purple-400/90 font-medium">Dead Clones</span>
          <div className="mt-2 flex items-baseline justify-between">
            <span className="text-2xl font-bold font-mono text-purple-300">
              {deadCodeSummary?.dead_clones ?? 0}
            </span>
            <span className="text-[11px] font-mono text-slate-500">duplicate dead code</span>
          </div>
        </div>

        <div className="bg-slate-900/80 border border-emerald-900/40 rounded-xl p-4 flex flex-col justify-between shadow-lg">
          <span className="text-xs font-mono text-emerald-400/90 font-medium">Removable Lines</span>
          <div className="mt-2 flex items-baseline justify-between">
            <span className="text-2xl font-bold font-mono text-emerald-300">
              {deadCodeSummary?.total_dead_lines ?? 0}
            </span>
            <span className="text-[11px] font-mono text-emerald-400/80">
              ~{deadCodeSummary?.estimated_savings_pct.toFixed(1) ?? "0.0"}%
            </span>
          </div>
        </div>
      </div>

      {/* Filter & Search Toolbar */}
      <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-4 shadow-lg space-y-3">
        <div className="flex flex-col md:flex-row items-stretch md:items-center justify-between gap-3">
          {/* Kind Quick Filter Tabs */}
          <div className="flex flex-wrap items-center gap-1.5 text-xs font-mono">
            <button
              type="button"
              onClick={() => setActiveFilter("all")}
              className={`px-3 py-1.5 rounded-lg border transition-all cursor-pointer ${
                activeFilter === "all"
                  ? "bg-indigo-600 text-white border-indigo-500 font-semibold shadow-sm"
                  : "bg-slate-950 text-slate-400 border-slate-800 hover:text-white"
              }`}
            >
              All ({deadCodeSummary?.total_dead_items ?? 0})
            </button>
            <button
              type="button"
              onClick={() => setActiveFilter("unreferenced_function")}
              className={`px-3 py-1.5 rounded-lg border transition-all cursor-pointer ${
                activeFilter === "unreferenced_function"
                  ? "bg-amber-600 text-white border-amber-500 font-semibold shadow-sm"
                  : "bg-slate-950 text-slate-400 border-slate-800 hover:text-white"
              }`}
            >
              Functions ({deadCodeSummary?.dead_functions ?? 0})
            </button>
            <button
              type="button"
              onClick={() => setActiveFilter("unreachable_block")}
              className={`px-3 py-1.5 rounded-lg border transition-all cursor-pointer ${
                activeFilter === "unreachable_block"
                  ? "bg-rose-600 text-white border-rose-500 font-semibold shadow-sm"
                  : "bg-slate-950 text-slate-400 border-slate-800 hover:text-white"
              }`}
            >
              Unreachable ({deadCodeSummary?.unreachable_blocks ?? 0})
            </button>
            <button
              type="button"
              onClick={() => setActiveFilter("dead_clone")}
              className={`px-3 py-1.5 rounded-lg border transition-all cursor-pointer ${
                activeFilter === "dead_clone"
                  ? "bg-purple-600 text-white border-purple-500 font-semibold shadow-sm"
                  : "bg-slate-950 text-slate-400 border-slate-800 hover:text-white"
              }`}
            >
              Dead Clones ({deadCodeSummary?.dead_clones ?? 0})
            </button>
          </div>

          <div className="flex flex-wrap items-center gap-2">
            {packagesList.length > 0 && (
              <select
                id="dead-code-studio-pkg"
                aria-label="Filter dead code by workspace package"
                value={selectedPackage}
                onChange={(e) => setSelectedPackage(e.target.value)}
                className="px-2.5 py-1.5 bg-slate-950 border border-slate-800 rounded-lg text-xs text-slate-300 focus:outline-hidden focus:border-indigo-500 font-mono"
              >
                <option value="all">All Packages ({packagesList.length})</option>
                {packagesList.map((pkg) => (
                  <option key={pkg} value={pkg}>
                    {pkg}
                  </option>
                ))}
              </select>
            )}

            <div className="relative w-48 sm:w-56">
              <Search className="w-3.5 h-3.5 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
              <input
                id="dead-code-studio-search"
                name="dead_code_studio_search"
                aria-label="Search dead code items by file or symbol"
                type="text"
                placeholder="Search file, symbol..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full pl-9 pr-3 py-1.5 bg-slate-950 border border-slate-800 rounded-lg text-xs text-slate-200 placeholder:text-slate-500 focus:outline-hidden focus:border-indigo-500 font-mono"
              />
            </div>

            <button
              type="button"
              onClick={() => void scanDeadCode({ static_only: false })}
              disabled={isDeadCodeLoading}
              className="px-3 py-1.5 rounded-lg bg-slate-950 hover:bg-slate-800 disabled:opacity-50 text-slate-300 text-xs font-mono flex items-center gap-1.5 transition-colors border border-slate-800 cursor-pointer"
            >
              <RefreshCw className={`w-3.5 h-3.5 ${isDeadCodeLoading ? "animate-spin" : ""}`} />
              <span>Rescan</span>
            </button>
          </div>
        </div>

        {/* Pruning Synthesizer Control Bar */}
        <div className="pt-3 border-t border-slate-800/60 flex flex-wrap items-center justify-between gap-3 text-xs font-mono">
          <div className="flex items-center gap-4">
            <label className="flex items-center gap-2 cursor-pointer select-none text-slate-300 hover:text-white">
              <input
                type="checkbox"
                aria-label={`Select All (${filteredItems.length})`}
                checked={selectedIds.size === filteredItems.length && filteredItems.length > 0}
                onChange={handleToggleSelectAll}
                className="rounded border-slate-700 bg-slate-950 text-indigo-600 focus:ring-indigo-500"
              />
              <span>Select All</span>
              <span className="text-slate-400"> ({filteredItems.length})</span>
            </label>
            <span className="text-slate-500">|</span>
            <span className="text-indigo-400 font-semibold">{selectedIds.size} selected</span>
          </div>

          <div className="flex flex-wrap items-center gap-3">
            <label className="flex items-center gap-1.5 cursor-pointer select-none text-slate-300 hover:text-white">
              <input
                type="checkbox"
                checked={dryRun}
                onChange={(e) => setDryRun(e.target.checked)}
                className="rounded border-slate-700 bg-slate-950 text-indigo-600 focus:ring-indigo-500"
              />
              <span>Dry Run Preview</span>
            </label>

            <label className="flex items-center gap-1.5 cursor-pointer select-none text-slate-300 hover:text-white">
              <input
                type="checkbox"
                checked={safeOnly}
                onChange={(e) => setSafeOnly(e.target.checked)}
                className="rounded border-slate-700 bg-slate-950 text-indigo-600 focus:ring-indigo-500"
              />
              <span>Strict Safe-Only (≥90%)</span>
            </label>

            <button
              type="button"
              onClick={() => void handleExecutePrune()}
              disabled={
                isDeadCodePruning || (!deadCodeSummary?.items?.length && selectedIds.size === 0)
              }
              className={`px-3 py-1.5 rounded-lg text-xs font-mono font-semibold flex items-center gap-1.5 transition-all cursor-pointer ${
                dryRun
                  ? "bg-indigo-600 hover:bg-indigo-500 text-white"
                  : "bg-rose-600 hover:bg-rose-500 text-white shadow-rose-900/30"
              } disabled:opacity-40`}
            >
              {isDeadCodePruning ? (
                <Loader2 className="w-3.5 h-3.5 animate-spin" />
              ) : dryRun ? (
                <Scissors className="w-3.5 h-3.5" />
              ) : (
                <Trash2 className="w-3.5 h-3.5" />
              )}
              <span>
                {isDeadCodePruning
                  ? "Pruning..."
                  : dryRun
                    ? "Preview Pruning"
                    : "Execute Safe Pruning"}
              </span>
            </button>
          </div>
        </div>
      </div>

      {/* Last Prune Status Banner */}
      {lastPruneResult && (
        <div className="p-3 rounded-xl bg-slate-900 border border-emerald-500/40 text-xs font-mono text-emerald-300 flex items-center justify-between gap-3 shadow-lg">
          <div className="flex items-center gap-2">
            <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
            <span>
              {lastPruneResult.dry_run ? "[DRY RUN] " : ""}
              Pruned {lastPruneResult.pruned_items} items ({lastPruneResult.total_lines_removed} LOC
              saved) across {lastPruneResult.files_affected.length} files
            </span>
          </div>
          {lastPruneResult.files_affected.length > 0 && (
            <span
              className="text-slate-400 truncate max-w-xs"
              title={lastPruneResult.files_affected.join(", ")}
            >
              Files: {lastPruneResult.files_affected.join(", ")}
            </span>
          )}
        </div>
      )}

      {/* Error Banners */}
      {(deadCodeError || deadCodePruneError) && (
        <div className="p-3 rounded-xl bg-rose-950/80 border border-rose-800 text-xs font-mono text-rose-300 flex items-center gap-2">
          <AlertTriangle className="w-4 h-4 text-rose-400 shrink-0" />
          <span>{deadCodeError || deadCodePruneError}</span>
        </div>
      )}

      {/* Items List */}
      {isDeadCodeLoading ? (
        <div className="p-12 text-center text-slate-400 space-y-3 bg-slate-900/60 border border-slate-800/80 rounded-xl">
          <Loader2 className="w-8 h-8 animate-spin mx-auto text-indigo-400" />
          <p className="text-sm font-mono">Running polyglot reachability & dead code analysis...</p>
        </div>
      ) : !deadCodeSummary || deadCodeSummary.items.length === 0 ? (
        <div className="p-12 text-center text-slate-400 space-y-3 bg-slate-900/60 border border-slate-800/80 rounded-xl">
          <CheckCircle2 className="w-10 h-10 text-emerald-400 mx-auto opacity-80" />
          <h4 className="text-base font-semibold text-slate-200">No Dead Code Detected</h4>
          <p className="text-xs text-slate-400 max-w-md mx-auto">
            Zero unreferenced functions or unreachable blocks found, or no dead code scan has been
            executed yet.
          </p>
          <button
            type="button"
            onClick={() => void scanDeadCode({ static_only: false })}
            className="px-4 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-mono text-xs font-semibold inline-flex items-center gap-2 shadow-lg transition-colors cursor-pointer"
          >
            <RefreshCw className="w-3.5 h-3.5" />
            <span>Run Dead Code Scan</span>
          </button>
        </div>
      ) : filteredItems.length === 0 ? (
        <div className="p-12 text-center text-slate-400 space-y-3 bg-slate-900/60 border border-slate-800/80 rounded-xl">
          <CheckCircle2 className="w-8 h-8 text-slate-500 mx-auto" />
          <h4 className="text-sm font-semibold text-slate-300">No Matching Items</h4>
          <p className="text-xs text-slate-500">
            No dead code candidates match your current filter and search query.
          </p>
        </div>
      ) : (
        <div className="space-y-2.5">
          {filteredItems.map((item) => {
            const isSelected = selectedIds.has(item.id);
            const parsed = parsePath(item.file_path);
            const ideLink = getIdeDeeplink(parsed.fullNormalized, item.line_start, preferredEditor);

            const kindBadge =
              item.kind === "unreferenced_function"
                ? "bg-amber-950/80 text-amber-300 border-amber-800/50"
                : item.kind === "unreachable_block"
                  ? "bg-rose-950/80 text-rose-300 border-rose-800/50"
                  : "bg-purple-950/80 text-purple-300 border-purple-800/50";

            return (
              <div
                key={item.id}
                onClick={() => handleToggleItem(item.id)}
                className={`p-3.5 rounded-xl border transition-all cursor-pointer flex flex-col md:flex-row md:items-center justify-between gap-3 ${
                  isSelected
                    ? "bg-slate-900 border-indigo-500/80 shadow-md shadow-indigo-950/20"
                    : "bg-slate-900/70 border-slate-800/80 hover:border-slate-700 hover:bg-slate-900/90"
                }`}
              >
                <div className="flex items-start md:items-center gap-3 min-w-0 flex-1">
                  <input
                    type="checkbox"
                    checked={isSelected}
                    onChange={() => handleToggleItem(item.id)}
                    onClick={(e) => e.stopPropagation()}
                    className="mt-1 md:mt-0 rounded border-slate-700 bg-slate-950 text-indigo-600 focus:ring-indigo-500 cursor-pointer"
                  />
                  <div className="min-w-0 flex-1 space-y-1">
                    <div className="flex flex-wrap items-center gap-2">
                      <span
                        className={`text-[10px] font-mono px-2 py-0.5 rounded border uppercase font-bold ${kindBadge}`}
                      >
                        {item.kind.replace("_", " ")}
                      </span>
                      <code className="text-xs font-mono font-bold text-slate-100 truncate">
                        {item.symbol_name}
                      </code>
                      {item.package_name && (
                        <span className="text-[10px] font-mono px-1.5 py-0.2 bg-slate-800 text-slate-400 rounded">
                          {item.package_name}
                        </span>
                      )}
                      {item.is_exported && (
                        <span className="text-[10px] font-mono px-1.5 py-0.2 bg-blue-950/70 text-blue-300 border border-blue-800/40 rounded">
                          Exported
                        </span>
                      )}
                      {item.cross_package_callers && item.cross_package_callers.length > 0 && (
                        <span className="text-[10px] font-mono px-1.5 py-0.2 bg-slate-800 text-slate-400 rounded">
                          {item.cross_package_callers.length} callers
                        </span>
                      )}
                      <span className="text-[10px] font-mono px-1.5 py-0.2 bg-emerald-950/70 text-emerald-300 border border-emerald-800/40 rounded">
                        +{item.estimated_lines_saved} LOC
                      </span>
                    </div>

                    <div className="flex items-center gap-2 text-xs font-mono text-slate-400 min-w-0">
                      <FileCode2 className="w-3.5 h-3.5 text-indigo-400 shrink-0" />
                      <span className="truncate" title={parsed.fullNormalized}>
                        {item.file_path}:{item.line_start}-{item.line_end}
                      </span>
                      <a
                        href={ideLink}
                        onClick={(e) => e.stopPropagation()}
                        title={`Open in ${getEditorDisplayName(preferredEditor)} at line ${item.line_start}`}
                        className="p-1 text-slate-500 hover:text-indigo-300 hover:bg-slate-800 rounded transition-colors"
                      >
                        <ExternalLink className="w-3 h-3" />
                      </a>
                    </div>

                    <p className="text-xs text-slate-400 font-mono">{item.reason}</p>
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
};
