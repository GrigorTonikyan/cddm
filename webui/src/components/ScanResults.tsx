import React, { useState, useMemo } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { ClonePairCard } from "./ClonePairCard";
import { DuplicationTreemap } from "./DuplicationTreemap";
import { TreemapExplorerModal } from "./TreemapExplorerModal";
import { LanguageAnalyticsModal } from "./LanguageAnalyticsModal";
import { HealthAuditModal } from "./HealthAuditModal";
import { ExportReportModal } from "./ExportReportModal";
import { parsePath, getLanguageStyle } from "../utils/path-utils";
import {
  Activity,
  Award,
  Copy,
  Clock,
  Layers,
  Search,
  Filter,
  ArrowUpDown,
  ChevronLeft,
  ChevronRight,
  Sparkles,
  CheckCircle2,
  PieChart,
  LayoutGrid,
  Maximize2,
  FileDown,
  Sliders,
} from "lucide-react";

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

interface SummaryCardProps {
  title: string;
  value: React.ReactNode;
  subtitle: string;
  icon: React.ReactNode;
}

const SummaryCard: React.FC<SummaryCardProps> = ({ title, value, subtitle, icon }) => (
  <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-4 flex flex-col justify-between shadow-lg">
    <div className="flex items-center justify-between text-slate-400">
      <span className="text-xs font-bold uppercase tracking-wider">{title}</span>
      {icon}
    </div>
    <div className="mt-3">
      <span className="text-3xl font-extrabold font-mono text-slate-100">{value}</span>
      <p className="text-[11px] text-slate-400 mt-1">{subtitle}</p>
    </div>
  </div>
);

export const ScanResults: React.FC<ScanResultsProps> = ({ className = "" }) => {
  const {
    results,
    isTreemapModalOpen,
    isLanguageModalOpen,
    isHealthAuditOpen,
    isExportReportOpen,
    setIsTreemapModalOpen,
    setIsLanguageModalOpen,
    setIsHealthAuditOpen,
    setIsExportReportOpen,
    setIsScanConfigOpen,
  } = useCDDMStore();

  const [searchTerm, setSearchTerm] = useState("");
  const [selectedLang, setSelectedLang] = useState<string>("ALL");
  const [minSimilarity, setMinSimilarity] = useState<number>(0);
  const [sortBy, setSortBy] = useState<"similarity" | "tokens" | "name">("similarity");
  const [currentPage, setCurrentPage] = useState(1);
  const [analyticsView, setAnalyticsView] = useState<"treemap" | "languages">("treemap");

  const itemsPerPage = 25;

  // Language Breakdown total tokens calculation
  const totalTokensAllLangs = useMemo(() => {
    if (!results) return 0;
    return results.language_breakdown.reduce((sum, item) => sum + item.tokens, 0);
  }, [results?.language_breakdown]);

  // Filter & Sort Clone Pairs
  const filteredPairs = useMemo(() => {
    if (!results) return [];
    return results.clone_pairs
      .filter((pair) => {
        const matchesSim = pair.similarity * 100 >= minSimilarity;
        if (!matchesSim) return false;

        const term = searchTerm.toLowerCase().trim();
        if (term) {
          const fileA = pair.file_a.toLowerCase();
          const fileB = pair.file_b.toLowerCase();
          if (!fileA.includes(term) && !fileB.includes(term)) return false;
        }

        if (selectedLang !== "ALL") {
          const extA = pair.file_a.split(".").pop()?.toLowerCase() || "";
          const extB = pair.file_b.split(".").pop()?.toLowerCase() || "";
          const allowedExts = LANG_EXTENSIONS[selectedLang.toLowerCase()] || [];
          if (!allowedExts.includes(extA) && !allowedExts.includes(extB)) {
            return false;
          }
        }

        return true;
      })
      .sort((a, b) => {
        if (sortBy === "similarity") return b.similarity - a.similarity;
        if (sortBy === "tokens") return b.token_count - a.token_count;
        if (sortBy === "name")
          return parsePath(a.file_a).filename.localeCompare(parsePath(b.file_a).filename);
        return 0;
      });
  }, [results?.clone_pairs, searchTerm, minSimilarity, selectedLang, sortBy]);

  // Pagination Slice
  const totalPages = Math.ceil(filteredPairs.length / itemsPerPage) || 1;
  const paginatedPairs = useMemo(() => {
    const start = (currentPage - 1) * itemsPerPage;
    return filteredPairs.slice(start, start + itemsPerPage);
  }, [filteredPairs, currentPage]);

  if (!results) return null;

  const scoreColor =
    results.dry_health_score >= 80
      ? "text-emerald-400 border-emerald-500/40 bg-emerald-950/20 shadow-emerald-950/30 hover:border-emerald-400/80"
      : results.dry_health_score >= 60
        ? "text-amber-400 border-amber-500/40 bg-amber-950/20 shadow-amber-950/30 hover:border-amber-400/80"
        : "text-rose-400 border-rose-500/40 bg-rose-950/20 shadow-rose-950/30 hover:border-rose-400/80";

  return (
    <div className={`space-y-6 ${className}`}>
      {/* Action Bar Header with Quick Launch Buttons */}
      <div className="flex flex-wrap items-center justify-between gap-3 bg-slate-900/60 p-3 rounded-xl border border-slate-800 text-xs font-mono">
        <div className="flex items-center gap-2 text-slate-300 font-bold">
          <Activity className="w-4 h-4 text-indigo-400" />
          <span>Codebase Analysis Overview</span>
        </div>

        <div className="flex flex-wrap items-center gap-2">
          <button
            type="button"
            onClick={() => setIsScanConfigOpen(true)}
            className="px-3 py-1.5 rounded-lg bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-300 flex items-center gap-1.5 transition-colors shadow-sm"
          >
            <Sliders className="w-3.5 h-3.5 text-indigo-400" />
            <span>Scan Settings</span>
          </button>
          <button
            type="button"
            onClick={() => setIsHealthAuditOpen(true)}
            className="px-3 py-1.5 rounded-lg bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-300 flex items-center gap-1.5 transition-colors shadow-sm"
          >
            <Award className="w-3.5 h-3.5 text-emerald-400" />
            <span>Health Audit</span>
          </button>
          <button
            type="button"
            onClick={() => setIsExportReportOpen(true)}
            className="px-3 py-1.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-semibold flex items-center gap-1.5 transition-colors shadow-sm"
          >
            <FileDown className="w-3.5 h-3.5" />
            <span>Export & Reports</span>
          </button>
        </div>
      </div>

      {/* Top Metrics Cards Banner */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-5 gap-4">
        {/* DRY Health Score Card (Clickable to open HealthAuditModal) */}
        <div
          onClick={() => setIsHealthAuditOpen(true)}
          className={`border rounded-xl p-4 flex flex-col justify-between shadow-lg relative overflow-hidden cursor-pointer transition-all ${scoreColor}`}
          title="Click to open full DRY Health Score Audit Window"
        >
          <div className="flex items-center justify-between">
            <span className="text-xs font-bold uppercase tracking-wider opacity-90 flex items-center gap-1.5">
              <span>DRY Health Score</span>
              <Maximize2 className="w-3 h-3 opacity-60" />
            </span>
            <Award className="w-5 h-5" />
          </div>
          <div className="mt-3">
            <div className="flex items-baseline gap-1">
              <span className="text-3xl font-extrabold font-mono tracking-tight">
                {results.dry_health_score.toFixed(1)}
              </span>
              <span className="text-sm opacity-60">/ 100</span>
            </div>
            <div className="w-full bg-slate-900/60 rounded-full h-1.5 mt-2 overflow-hidden border border-slate-700/30">
              <div
                className={`h-full transition-all duration-500 ${
                  results.dry_health_score >= 80
                    ? "bg-emerald-400"
                    : results.dry_health_score >= 60
                      ? "bg-amber-400"
                      : "bg-rose-400"
                }`}
                style={{ width: `${Math.min(100, Math.max(0, results.dry_health_score))}%` }}
              />
            </div>
          </div>
        </div>

        {/* Duplication Rate */}
        <SummaryCard
          title="Duplication Rate"
          value={`${results.duplication_percentage.toFixed(2)}%`}
          subtitle="Total code redundancy"
          icon={<Copy className="w-5 h-5 text-indigo-400" />}
        />

        {/* Files Scanned */}
        <SummaryCard
          title="Files Scanned"
          value={results.total_files.toLocaleString()}
          subtitle={`${results.total_tokens.toLocaleString()} tokens indexed`}
          icon={<Layers className="w-5 h-5 text-indigo-400" />}
        />

        {/* Clone Pairs */}
        <SummaryCard
          title="Clone Pairs"
          value={results.total_clones.toLocaleString()}
          subtitle="Identified duplicate fragments"
          icon={<Activity className="w-5 h-5 text-indigo-400" />}
        />

        {/* Scan Duration */}
        <SummaryCard
          title="Engine Speed"
          value={
            <>
              {results.duration_ms}
              <span className="text-xs text-slate-400 font-mono"> ms</span>
            </>
          }
          subtitle="Winnowing M61 execution"
          icon={<Clock className="w-5 h-5 text-indigo-400" />}
        />
      </div>

      {/* Visual Analytics Toolbar & Views */}
      <div className="space-y-3">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div className="flex items-center gap-2">
            <span className="text-xs font-bold text-slate-400 uppercase tracking-wider">
              Visual Analytics
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="flex items-center gap-1 bg-slate-900 p-1 rounded-lg border border-slate-800 text-xs font-mono">
              <button
                type="button"
                onClick={() => setAnalyticsView("treemap")}
                className={`px-3 py-1 rounded-md flex items-center gap-1.5 transition-all ${
                  analyticsView === "treemap"
                    ? "bg-indigo-600 text-white font-semibold shadow-sm"
                    : "text-slate-400 hover:text-slate-200"
                }`}
              >
                <LayoutGrid className="w-3.5 h-3.5" />
                Duplication Treemap
              </button>
              <button
                type="button"
                onClick={() => setAnalyticsView("languages")}
                className={`px-3 py-1 rounded-md flex items-center gap-1.5 transition-all ${
                  analyticsView === "languages"
                    ? "bg-indigo-600 text-white font-semibold shadow-sm"
                    : "text-slate-400 hover:text-slate-200"
                }`}
              >
                <PieChart className="w-3.5 h-3.5" />
                Language Breakdown
              </button>
            </div>

            {/* Expand Active View to Window */}
            <button
              type="button"
              onClick={() => {
                if (analyticsView === "treemap") {
                  setIsTreemapModalOpen(true);
                } else {
                  setIsLanguageModalOpen(true);
                }
              }}
              className="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 hover:bg-slate-800 text-slate-300 text-xs font-mono flex items-center gap-1.5 transition-colors"
              title="Open current analytics view into a dedicated desktop modal window"
            >
              <Maximize2 className="w-3.5 h-3.5 text-indigo-400" />
              <span>Open in Window</span>
            </button>
          </div>
        </div>

        {analyticsView === "treemap" ? (
          <DuplicationTreemap
            clonePairs={results.clone_pairs}
            totalTokens={results.total_tokens}
            selectedFilterPath={searchTerm}
            onSelectFilterPath={(path) => {
              setSearchTerm(path);
              setCurrentPage(1);
            }}
          />
        ) : (
          results.language_breakdown.length > 0 && (
            <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-5 shadow-lg space-y-4">
              <div className="flex items-center justify-between">
                <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider flex items-center gap-2">
                  <Sparkles className="w-4 h-4 text-indigo-400" />
                  Language Breakdown
                </h3>
                <span className="text-xs font-mono text-slate-400">
                  {results.language_breakdown.length} Languages Detected
                </span>
              </div>

              {/* Segmented Distribution Bar */}
              <div className="w-full h-3 bg-slate-950 rounded-full overflow-hidden flex border border-slate-800 shadow-inner">
                {results.language_breakdown.map((item) => {
                  const style = getLanguageStyle(item.language);
                  const pct =
                    totalTokensAllLangs > 0 ? (item.tokens / totalTokensAllLangs) * 100 : 0;
                  return (
                    <div
                      key={item.language}
                      className={`h-full ${style.bar} transition-all duration-300`}
                      style={{ width: `${pct}%` }}
                      title={`${item.language}: ${item.files} files (${pct.toFixed(1)}% tokens)`}
                    />
                  );
                })}
              </div>

              {/* Language Legend Grid */}
              <div className="flex flex-wrap items-center gap-3 pt-1">
                {results.language_breakdown.map((item) => {
                  const style = getLanguageStyle(item.language);
                  const pct =
                    totalTokensAllLangs > 0 ? (item.tokens / totalTokensAllLangs) * 100 : 0;
                  return (
                    <div
                      key={item.language}
                      className={`flex items-center gap-2 px-3 py-1 rounded-lg border text-xs font-mono transition-all ${style.bg} ${style.text} ${style.border}`}
                    >
                      <span className={`w-2 h-2 rounded-full ${style.bar}`} />
                      <span className="font-semibold">{item.language}</span>
                      <span className="opacity-40">|</span>
                      <span>{item.files} files</span>
                      <span className="opacity-40">|</span>
                      <span>{pct.toFixed(1)}%</span>
                    </div>
                  );
                })}
              </div>
            </div>
          )
        )}
      </div>

      {/* Filter and Search Toolbar */}
      <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-4 shadow-lg space-y-3">
        <div className="flex flex-col md:flex-row items-stretch md:items-center justify-between gap-3">
          {/* Search Input */}
          <div className="relative flex-1">
            <Search className="w-4 h-4 text-slate-400 absolute left-3.5 top-1/2 -translate-y-1/2" />
            <input
              type="text"
              value={searchTerm}
              onChange={(e) => {
                setSearchTerm(e.target.value);
                setCurrentPage(1);
              }}
              placeholder="Search by file name or path (e.g. gradio_demo.py)..."
              className="w-full bg-slate-950 border border-slate-800 rounded-lg pl-10 pr-4 py-2 text-xs font-mono text-slate-100 placeholder-slate-400 focus:outline-none focus:border-indigo-500 transition-colors shadow-inner"
            />
          </div>

          {/* Filter Controls Row */}
          <div className="flex flex-wrap items-center gap-3">
            {/* Min Similarity Slider */}
            <div className="flex items-center gap-2 bg-slate-950 px-3 py-1.5 rounded-lg border border-slate-800 text-xs font-mono">
              <span className="text-slate-400">Min Match:</span>
              <input
                type="range"
                min="0"
                max="100"
                step="5"
                value={minSimilarity}
                onChange={(e) => {
                  setMinSimilarity(Number(e.target.value));
                  setCurrentPage(1);
                }}
                className="w-20 h-1.5 bg-slate-800 rounded-lg appearance-none cursor-pointer accent-indigo-500"
              />
              <span className="text-indigo-300 font-bold min-w-[32px]">{minSimilarity}%</span>
            </div>

            {/* Language Filter Dropdown */}
            <div className="flex items-center gap-2 bg-slate-950 px-3 py-1.5 rounded-lg border border-slate-800 text-xs font-mono">
              <Filter className="w-3.5 h-3.5 text-indigo-400 shrink-0" />
              <select
                value={selectedLang}
                onChange={(e) => {
                  setSelectedLang(e.target.value);
                  setCurrentPage(1);
                }}
                className="bg-transparent text-slate-200 focus:outline-none cursor-pointer"
              >
                <option value="ALL" className="bg-slate-900 text-slate-100">
                  All Languages
                </option>
                {results.language_breakdown.map((l) => (
                  <option
                    key={l.language}
                    value={l.language}
                    className="bg-slate-900 text-slate-100"
                  >
                    {l.language} ({l.files})
                  </option>
                ))}
              </select>
            </div>

            {/* Sort Selector */}
            <div className="flex items-center gap-2 bg-slate-950 px-3 py-1.5 rounded-lg border border-slate-800 text-xs font-mono">
              <ArrowUpDown className="w-3.5 h-3.5 text-indigo-400 shrink-0" />
              <select
                value={sortBy}
                onChange={(e) => setSortBy(e.target.value as any)}
                className="bg-transparent text-slate-200 focus:outline-none cursor-pointer"
              >
                <option value="similarity" className="bg-slate-900 text-slate-100">
                  Highest Similarity
                </option>
                <option value="tokens" className="bg-slate-900 text-slate-100">
                  Most Tokens
                </option>
                <option value="name" className="bg-slate-900 text-slate-100">
                  File Name
                </option>
              </select>
            </div>
          </div>
        </div>
      </div>

      {/* Clone Pair List Section */}
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h3 className="text-lg font-bold text-slate-100 flex items-center gap-2">
            <span>Detected Clone Pairs</span>
            <span className="text-xs bg-indigo-950 text-indigo-300 border border-indigo-800/50 px-2.5 py-0.5 rounded-full font-mono">
              {filteredPairs.length.toLocaleString()} matching pairs
            </span>
          </h3>

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
        {filteredPairs.length === 0 ? (
          <div className="bg-slate-900/60 border border-slate-800/80 rounded-xl p-12 text-center text-slate-400 space-y-3">
            <CheckCircle2 className="w-12 h-12 text-emerald-400 mx-auto opacity-80" />
            <h4 className="text-base font-semibold text-slate-200">No Clone Pairs Found</h4>
            <p className="text-xs text-slate-400 max-w-md mx-auto">
              No duplicate code fragments match your current search query or active filter settings.
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
        )}

        {/* Bottom Pagination Controls */}
        {totalPages > 1 && (
          <div className="flex items-center justify-between bg-slate-900/60 border border-slate-800/80 rounded-xl p-4 font-mono text-xs text-slate-400">
            <div>
              Showing {(currentPage - 1) * itemsPerPage + 1}–
              {Math.min(currentPage * itemsPerPage, filteredPairs.length)} of{" "}
              {filteredPairs.length.toLocaleString()} clones
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
      <TreemapExplorerModal
        isOpen={isTreemapModalOpen}
        onClose={() => setIsTreemapModalOpen(false)}
        clonePairs={results.clone_pairs}
        totalTokens={results.total_tokens}
        selectedFilterPath={searchTerm}
        onSelectFilterPath={(p) => {
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
    </div>
  );
};
