import React, { Suspense, useMemo, useState } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { parsePath } from "../utils/path-utils";
import { CloneClusterCard } from "./CloneClusterCard";
import { ClonePairCard } from "./ClonePairCard";
import { FilterToolbar } from "./scan-results/FilterToolbar";
import { SummaryBanner } from "./scan-results/SummaryBanner";
import { VisualAnalyticsSection } from "./scan-results/VisualAnalyticsSection";
import { Activity, CheckCircle2, ChevronLeft, ChevronRight, Layers } from "lucide-react";

import { lazyModal } from "../utils/lazy-modal";

// Lazy-load heavier modal analyzers for optimal performance
const ExportReportModal = lazyModal(() => import("./ExportReportModal"), "ExportReportModal");
const HealthAuditModal = lazyModal(() => import("./HealthAuditModal"), "HealthAuditModal");
const LanguageAnalyticsModal = lazyModal(
  () => import("./LanguageAnalyticsModal"),
  "LanguageAnalyticsModal",
);
const TreemapExplorerModal = lazyModal(
  () => import("./TreemapExplorerModal"),
  "TreemapExplorerModal",
);

export interface ScanResultsProps {
  className?: string;
}

const LANG_EXTENSIONS: Record<string, string[]> = {
  python: ["py"],
  typescript: ["ts", "tsx"],
  javascript: ["js", "jsx", "mjs", "cjs"],
  rust: ["rs"],
  go: ["go"],
  java: ["java"],
  c: ["c", "h"],
  cpp: ["cpp", "hpp", "cc", "hh", "cxx"],
  csharp: ["cs"],
  json: ["json"],
  css: ["css", "scss", "less"],
  html: ["html", "htm"],
};

function sortByMetric<T extends { similarity: number; token_count: number }>(
  items: T[],
  sortBy: "similarity" | "tokens" | "name",
  nameCompare: (a: T, b: T) => number,
): T[] {
  return items.sort((a, b) => {
    if (sortBy === "similarity") return b.similarity - a.similarity;
    if (sortBy === "tokens") return b.token_count - a.token_count;
    if (sortBy === "name") return nameCompare(a, b);
    return 0;
  });
}

export const ScanResults: React.FC<ScanResultsProps> = ({ className = "" }) => {
  const {
    results,
    viewMode,
    setViewMode,
    isTreemapModalOpen,
    isLanguageModalOpen,
    isHealthAuditOpen,
    isExportReportOpen,
    setIsTreemapModalOpen,
    setIsLanguageModalOpen,
    setIsHealthAuditOpen,
    setIsExportReportOpen,
  } = useCDDMStore();

  const [searchTerm, setSearchTerm] = useState("");
  const [selectedLang, setSelectedLang] = useState<string>("ALL");
  const [selectedCloneType, setSelectedCloneType] = useState<string>("ALL");
  const [minSimilarity, setMinSimilarity] = useState<number>(0);
  const [sortBy, setSortBy] = useState<"similarity" | "tokens" | "name">("similarity");
  const [currentPage, setCurrentPage] = useState(1);

  const itemsPerPage = 25;

  // Compute Clone Type distribution counts
  const cloneTypeCounts = useMemo(() => {
    if (!results || !Array.isArray(results.clone_pairs)) {
      return { exact: 0, renamed: 0, nearMiss: 0, semantic: 0, total: 0 };
    }
    let exact = 0;
    let renamed = 0;
    let nearMiss = 0;
    let semantic = 0;
    for (const pair of results.clone_pairs) {
      if (pair.clone_type === "Exact") exact++;
      else if (pair.clone_type === "Renamed") renamed++;
      else if (pair.clone_type === "NearMiss") nearMiss++;
      else if (pair.clone_type === "Semantic") semantic++;
    }
    return { exact, renamed, nearMiss, semantic, total: results.clone_pairs.length };
  }, [results]);

  function matchesScanFilters(
    files: string[],
    cloneType: string,
    similarity: number,
    minSimilarity: number,
    selectedCloneType: string,
    searchTerm: string,
    selectedLang: string,
  ): boolean {
    if (similarity * 100 < minSimilarity) return false;
    if (selectedCloneType !== "ALL" && cloneType !== selectedCloneType) return false;

    const term = searchTerm.toLowerCase().trim();
    if (term) {
      const hasMatch = files.some((f) => f.toLowerCase().includes(term));
      if (!hasMatch) return false;
    }

    if (selectedLang !== "ALL") {
      const allowedExts = LANG_EXTENSIONS[selectedLang.toLowerCase()] || [];
      const hasLangMatch = files.some((f) => {
        const ext = f.split(".").pop()?.toLowerCase() || "";
        return allowedExts.includes(ext);
      });
      if (!hasLangMatch) return false;
    }

    return true;
  }

  // Filter & Sort Clone Pairs
  const filteredPairs = useMemo(() => {
    if (!results || !Array.isArray(results.clone_pairs)) return [];
    const filtered = results.clone_pairs.filter((pair) =>
      matchesScanFilters(
        [pair.file_a, pair.file_b],
        pair.clone_type,
        pair.similarity,
        minSimilarity,
        selectedCloneType,
        searchTerm,
        selectedLang,
      ),
    );

    return sortByMetric(filtered, sortBy, (a, b) =>
      parsePath(a.file_a).filename.localeCompare(parsePath(b.file_a).filename),
    );
  }, [results?.clone_pairs, searchTerm, minSimilarity, selectedLang, selectedCloneType, sortBy]);

  // Filter & Sort Clone Clusters
  const filteredClusters = useMemo(() => {
    if (!results || !Array.isArray(results.clone_clusters)) return [];
    const filtered = results.clone_clusters.filter((cluster) =>
      matchesScanFilters(
        cluster.occurrences.map((loc) => loc.file),
        cluster.clone_type,
        cluster.similarity,
        minSimilarity,
        selectedCloneType,
        searchTerm,
        selectedLang,
      ),
    );

    return sortByMetric(filtered, sortBy, (a, b) => a.id - b.id);
  }, [results?.clone_clusters, searchTerm, minSimilarity, selectedLang, selectedCloneType, sortBy]);

  // Pagination Slice based on active viewMode
  const activeItemsCount = viewMode === "pairs" ? filteredPairs.length : filteredClusters.length;
  const totalPages = Math.ceil(activeItemsCount / itemsPerPage) || 1;

  const paginatedPairs = useMemo(() => {
    const start = (currentPage - 1) * itemsPerPage;
    return filteredPairs.slice(start, start + itemsPerPage);
  }, [filteredPairs, currentPage]);

  const paginatedClusters = useMemo(() => {
    const start = (currentPage - 1) * itemsPerPage;
    return filteredClusters.slice(start, start + itemsPerPage);
  }, [filteredClusters, currentPage]);

  if (!results) return null;

  return (
    <div className={`space-y-6 ${className}`}>
      {/* Results Header Bar */}
      <div className="flex flex-wrap items-center justify-between gap-3 pb-2 border-b border-slate-800/80">
        <div className="flex items-center gap-2 text-xs font-bold text-slate-400 uppercase tracking-wider">
          <Activity className="w-4 h-4 text-indigo-400" />
          <span>Codebase Analysis Overview</span>
        </div>

        <div className="flex items-center gap-2 text-xs font-mono text-slate-400">
          <span className="bg-slate-900 border border-slate-800/80 px-2.5 py-1 rounded-md">
            Scan ID:{" "}
            <span className="text-slate-200">
              {results.scan_id ? results.scan_id.slice(0, 8) : "latest"}
            </span>
          </span>
        </div>
      </div>

      {/* Top Metrics Cards Banner */}
      <SummaryBanner results={results} onOpenHealthAudit={() => setIsHealthAuditOpen(true)} />

      {/* Visual Analytics Toolbar & Views */}
      <VisualAnalyticsSection
        results={results}
        searchTerm={searchTerm}
        onSelectFilterPath={(path) => {
          setSearchTerm(path);
          setCurrentPage(1);
        }}
        onOpenTreemapModal={() => setIsTreemapModalOpen(true)}
        onOpenLanguageModal={() => setIsLanguageModalOpen(true)}
      />

      {/* Filter and Search Toolbar */}
      <FilterToolbar
        searchTerm={searchTerm}
        onSearchChange={(term) => {
          setSearchTerm(term);
          setCurrentPage(1);
        }}
        minSimilarity={minSimilarity}
        onMinSimilarityChange={(sim) => {
          setMinSimilarity(sim);
          setCurrentPage(1);
        }}
        selectedLang={selectedLang}
        onSelectedLangChange={(lang) => {
          setSelectedLang(lang);
          setCurrentPage(1);
        }}
        selectedCloneType={selectedCloneType}
        onSelectedCloneTypeChange={(type) => {
          setSelectedCloneType(type);
          setCurrentPage(1);
        }}
        cloneTypeCounts={cloneTypeCounts}
        sortBy={sortBy}
        onSortByChange={setSortBy}
        languages={results.language_breakdown || []}
      />

      {/* Duplications List Section */}
      <div className="space-y-4">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div className="flex items-center gap-3">
            <h3 className="text-lg font-bold text-slate-100 flex items-center gap-2">
              <span>
                {viewMode === "pairs" ? "Detected Clone Pairs" : "Detected Clone Clusters"}
              </span>
            </h3>

            {/* View Mode Toggle: Pairs vs Clusters */}
            <div className="flex items-center gap-1 bg-slate-900 p-1 rounded-lg border border-slate-800 text-xs font-mono">
              <button
                type="button"
                onClick={() => {
                  setViewMode("pairs");
                  setCurrentPage(1);
                }}
                className={`px-3 py-1 rounded-md flex items-center gap-1.5 transition-all ${
                  viewMode === "pairs"
                    ? "bg-indigo-600 text-white font-semibold shadow-sm"
                    : "text-slate-400 hover:text-slate-200"
                }`}
              >
                <Activity className="w-3.5 h-3.5" />
                <span>Pairwise ({filteredPairs.length})</span>
              </button>
              <button
                type="button"
                onClick={() => {
                  setViewMode("clusters");
                  setCurrentPage(1);
                }}
                className={`px-3 py-1 rounded-md flex items-center gap-1.5 transition-all ${
                  viewMode === "clusters"
                    ? "bg-purple-600 text-white font-semibold shadow-sm"
                    : "text-slate-400 hover:text-slate-200"
                }`}
              >
                <Layers className="w-3.5 h-3.5" />
                <span>N-Way Clusters ({filteredClusters.length})</span>
              </button>
            </div>
          </div>

          {/* Top Pagination Controls */}
          {totalPages > 1 && (
            <div className="flex items-center gap-2 text-xs font-mono text-slate-400">
              <span>
                Page {currentPage} of {totalPages}
              </span>
              <button
                type="button"
                disabled={currentPage === 1}
                onClick={() => setCurrentPage((p) => Math.max(1, p - 1))}
                className="p-1 rounded bg-slate-900 border border-slate-800 disabled:opacity-40 hover:bg-slate-800 text-slate-200"
              >
                <ChevronLeft className="w-4 h-4" />
              </button>
              <button
                type="button"
                disabled={currentPage === totalPages}
                onClick={() => setCurrentPage((p) => Math.min(totalPages, p + 1))}
                className="p-1 rounded bg-slate-900 border border-slate-800 disabled:opacity-40 hover:bg-slate-800 text-slate-200"
              >
                <ChevronRight className="w-4 h-4" />
              </button>
            </div>
          )}
        </div>

        {/* Cards Rendering */}
        {viewMode === "pairs" ? (
          filteredPairs.length === 0 ? (
            <div className="bg-slate-900/60 border border-slate-800/80 rounded-xl p-12 text-center text-slate-400 space-y-3">
              <CheckCircle2 className="w-12 h-12 text-emerald-400 mx-auto opacity-80" />
              <h4 className="text-base font-semibold text-slate-200">No Clone Pairs Found</h4>
              <p className="text-xs text-slate-400 max-w-md mx-auto">
                No duplicate code fragments match your current search query or active filter
                settings.
              </p>
            </div>
          ) : (
            <div className="space-y-3">
              {paginatedPairs.map((pair, idx) => {
                const globalIndex = (currentPage - 1) * itemsPerPage + idx + 1;
                return (
                  <ClonePairCard
                    key={`${pair.file_a}-${pair.file_b}-${idx}`}
                    pair={pair}
                    index={globalIndex}
                  />
                );
              })}
            </div>
          )
        ) : filteredClusters.length === 0 ? (
          <div className="bg-slate-900/60 border border-slate-800/80 rounded-xl p-12 text-center text-slate-400 space-y-3">
            <CheckCircle2 className="w-12 h-12 text-emerald-400 mx-auto opacity-80" />
            <h4 className="text-base font-semibold text-slate-200">No Clone Clusters Found</h4>
            <p className="text-xs text-slate-400 max-w-md mx-auto">
              No N-way clone clusters match your current search query or active filter settings.
            </p>
          </div>
        ) : (
          <div className="space-y-3">
            {paginatedClusters.map((cluster, idx) => {
              const globalIndex = (currentPage - 1) * itemsPerPage + idx + 1;
              return (
                <CloneClusterCard
                  key={`cluster-${cluster.id}-${idx}`}
                  cluster={cluster}
                  index={globalIndex}
                />
              );
            })}
          </div>
        )}

        {/* Bottom Pagination Controls */}
        {totalPages > 1 && (
          <div className="flex items-center justify-between bg-slate-900/60 border border-slate-800/80 rounded-xl p-4 font-mono text-xs text-slate-400">
            <div>
              Showing {(currentPage - 1) * itemsPerPage + 1}–
              {Math.min(currentPage * itemsPerPage, activeItemsCount)} of{" "}
              {activeItemsCount.toLocaleString()} {viewMode === "pairs" ? "clones" : "clusters"}
            </div>
            <div className="flex items-center gap-2">
              <button
                type="button"
                disabled={currentPage === 1}
                onClick={() => setCurrentPage((p) => Math.max(1, p - 1))}
                className="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 hover:bg-slate-800 text-slate-200 disabled:opacity-40"
              >
                Previous
              </button>
              <span>
                {currentPage} / {totalPages}
              </span>
              <button
                type="button"
                disabled={currentPage === totalPages}
                onClick={() => setCurrentPage((p) => Math.min(totalPages, p + 1))}
                className="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 hover:bg-slate-800 text-slate-200 disabled:opacity-40"
              >
                Next
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Win2x Modals */}
      <Suspense fallback={null}>
        <TreemapExplorerModal
          isOpen={isTreemapModalOpen}
          onClose={() => setIsTreemapModalOpen(false)}
          clonePairs={results.clone_pairs}
          totalTokens={results.total_tokens}
          selectedFilterPath={searchTerm}
          onSelectFilterPath={(p: string) => {
            setSearchTerm(p);
            setCurrentPage(1);
          }}
        />

        <LanguageAnalyticsModal
          isOpen={isLanguageModalOpen}
          onClose={() => setIsLanguageModalOpen(false)}
          languages={results.language_breakdown}
          totalTokens={results.total_tokens}
          totalFiles={results.total_files}
        />

        <HealthAuditModal
          isOpen={isHealthAuditOpen}
          onClose={() => setIsHealthAuditOpen(false)}
          results={results}
        />

        <ExportReportModal
          isOpen={isExportReportOpen}
          onClose={() => setIsExportReportOpen(false)}
          results={results}
        />
      </Suspense>
    </div>
  );
};
