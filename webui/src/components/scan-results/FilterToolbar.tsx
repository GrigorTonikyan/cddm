import React from "react";
import type { LanguageStats } from "../../types/cddm-types";
import { ArrowUpDown, Filter, Layers, Search, Sparkles, Tag } from "lucide-react";

export interface CloneTypeCounts {
  exact: number;
  renamed: number;
  nearMiss: number;
  semantic: number;
  total: number;
}

export interface FilterToolbarProps {
  searchTerm: string;
  onSearchChange: (value: string) => void;
  minSimilarity: number;
  onMinSimilarityChange: (value: number) => void;
  selectedLang: string;
  onSelectedLangChange: (value: string) => void;
  selectedCloneType: string;
  onSelectedCloneTypeChange: (value: string) => void;
  cloneTypeCounts?: CloneTypeCounts;
  sortBy: "similarity" | "tokens" | "name";
  onSortByChange: (value: "similarity" | "tokens" | "name") => void;
  languages: LanguageStats[];
}

export const FilterToolbar: React.FC<FilterToolbarProps> = ({
  searchTerm,
  onSearchChange,
  minSimilarity,
  onMinSimilarityChange,
  selectedLang,
  onSelectedLangChange,
  selectedCloneType,
  onSelectedCloneTypeChange,
  cloneTypeCounts,
  sortBy,
  onSortByChange,
  languages,
}) => {
  const counts = cloneTypeCounts || { exact: 0, renamed: 0, nearMiss: 0, semantic: 0, total: 0 };

  return (
    <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-4 shadow-lg space-y-3">
      {/* Top Search & Filter Bar */}
      <div className="flex flex-col md:flex-row items-stretch md:items-center justify-between gap-3">
        {/* Search Input */}
        <div className="relative flex-1">
          <Search className="w-4 h-4 text-slate-400 absolute left-3.5 top-1/2 -translate-y-1/2" />
          <input
            id="clones-search-input"
            name="clones_search"
            aria-label="Search clone pairs by file name or path"
            type="text"
            value={searchTerm}
            onChange={(e) => onSearchChange(e.target.value)}
            placeholder="Search by file name or path (e.g. provider.rs)..."
            className="w-full bg-slate-950 border border-slate-800 rounded-lg pl-10 pr-4 py-2 text-xs font-mono text-slate-100 placeholder-slate-400 focus:outline-none focus:border-indigo-500 transition-colors shadow-inner"
          />
        </div>

        {/* Filter Controls Row */}
        <div className="flex flex-wrap items-center gap-3">
          {/* Min Similarity Slider */}
          <div className="flex items-center gap-2 bg-slate-950 px-3 py-1.5 rounded-lg border border-slate-800 text-xs font-mono">
            <label htmlFor="clones-min-similarity" className="text-slate-400 cursor-pointer">
              Min Match:
            </label>
            <input
              id="clones-min-similarity"
              name="min_similarity"
              aria-label="Minimum Similarity Match Percentage"
              type="range"
              min="0"
              max="100"
              step="5"
              value={minSimilarity}
              onChange={(e) => onMinSimilarityChange(Number(e.target.value))}
              className="w-20 h-1.5 bg-slate-800 rounded-lg appearance-none cursor-pointer accent-indigo-500"
            />
            <span className="text-indigo-300 font-bold min-w-8">{minSimilarity}%</span>
          </div>

          {/* Language Filter Dropdown */}
          <div className="flex items-center gap-2 bg-slate-950 px-3 py-1.5 rounded-lg border border-slate-800 text-xs font-mono">
            <Filter className="w-3.5 h-3.5 text-indigo-400 shrink-0" />
            <select
              id="clones-language-filter"
              name="language_filter"
              aria-label="Filter clones by language"
              value={selectedLang}
              onChange={(e) => onSelectedLangChange(e.target.value)}
              className="bg-transparent text-slate-200 focus:outline-none cursor-pointer"
            >
              <option value="ALL" className="bg-slate-900 text-slate-100">
                All Languages
              </option>
              {languages.map((l) => (
                <option key={l.language} value={l.language} className="bg-slate-900 text-slate-100">
                  {l.language} ({l.files})
                </option>
              ))}
            </select>
          </div>

          {/* Sort Selector */}
          <div className="flex items-center gap-2 bg-slate-950 px-3 py-1.5 rounded-lg border border-slate-800 text-xs font-mono">
            <ArrowUpDown className="w-3.5 h-3.5 text-indigo-400 shrink-0" />
            <select
              id="clones-sort-by"
              name="sort_by"
              aria-label="Sort clone pairs by"
              value={sortBy}
              onChange={(e) => onSortByChange(e.target.value as "similarity" | "tokens" | "name")}
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

      {/* Clone Type Quick Filters */}
      <div className="flex flex-wrap items-center gap-2 pt-2 border-t border-slate-800/60 text-xs font-mono">
        <span className="text-slate-400 flex items-center gap-1 shrink-0 mr-1">
          <Tag className="w-3.5 h-3.5 text-indigo-400" />
          <span>Clone Type:</span>
        </span>

        <button
          type="button"
          onClick={() => onSelectedCloneTypeChange("ALL")}
          className={`px-2.5 py-1 rounded-lg border transition-all flex items-center gap-1.5 cursor-pointer ${
            selectedCloneType === "ALL"
              ? "bg-slate-800 text-white font-semibold border-slate-600 shadow-sm"
              : "bg-slate-950/60 text-slate-400 border-slate-800 hover:text-slate-200 hover:bg-slate-900"
          }`}
        >
          <span>All Types</span>
          <span className="text-[10px] px-1.5 py-0.2 bg-slate-800 rounded-full font-bold">
            {counts.total}
          </span>
        </button>

        <button
          type="button"
          onClick={() => onSelectedCloneTypeChange("Exact")}
          className={`px-2.5 py-1 rounded-lg border transition-all flex items-center gap-1.5 cursor-pointer ${
            selectedCloneType === "Exact"
              ? "bg-emerald-950 text-emerald-200 font-semibold border-emerald-600 shadow-sm"
              : "bg-slate-950/60 text-emerald-400/80 border-slate-800 hover:bg-emerald-950/30"
          }`}
        >
          <span className="w-2 h-2 rounded-full bg-emerald-400" />
          <span>Type-1 Exact</span>
          <span className="text-[10px] px-1.5 py-0.2 bg-emerald-950 rounded-full font-bold">
            {counts.exact}
          </span>
        </button>

        <button
          type="button"
          onClick={() => onSelectedCloneTypeChange("Renamed")}
          className={`px-2.5 py-1 rounded-lg border transition-all flex items-center gap-1.5 cursor-pointer ${
            selectedCloneType === "Renamed"
              ? "bg-indigo-950 text-indigo-200 font-semibold border-indigo-600 shadow-sm"
              : "bg-slate-950/60 text-indigo-400/80 border-slate-800 hover:bg-indigo-950/30"
          }`}
        >
          <span className="w-2 h-2 rounded-full bg-indigo-400" />
          <span>Type-2 Renamed</span>
          <span className="text-[10px] px-1.5 py-0.2 bg-indigo-950 rounded-full font-bold">
            {counts.renamed}
          </span>
        </button>

        <button
          type="button"
          onClick={() => onSelectedCloneTypeChange("NearMiss")}
          className={`px-2.5 py-1 rounded-lg border transition-all flex items-center gap-1.5 cursor-pointer ${
            selectedCloneType === "NearMiss"
              ? "bg-amber-950 text-amber-200 font-semibold border-amber-500 shadow-sm shadow-amber-950/50"
              : "bg-slate-950/60 text-amber-400 border-amber-900/40 hover:bg-amber-950/30"
          }`}
        >
          <Layers className="w-3.5 h-3.5 text-amber-400" />
          <span>Type-3 Near-Miss</span>
          <span className="text-[10px] px-1.5 py-0.2 bg-amber-950 border border-amber-800/50 text-amber-300 rounded-full font-bold">
            {counts.nearMiss}
          </span>
        </button>

        <button
          type="button"
          onClick={() => onSelectedCloneTypeChange("Semantic")}
          className={`px-2.5 py-1 rounded-lg border transition-all flex items-center gap-1.5 cursor-pointer ${
            selectedCloneType === "Semantic"
              ? "bg-purple-950 text-purple-200 font-semibold border-purple-600 shadow-sm"
              : "bg-slate-950/60 text-purple-400/80 border-slate-800 hover:bg-purple-950/30"
          }`}
        >
          <Sparkles className="w-3 h-3 text-purple-400" />
          <span>Type-4 Semantic</span>
          <span className="text-[10px] px-1.5 py-0.2 bg-purple-950 rounded-full font-bold">
            {counts.semantic}
          </span>
        </button>
      </div>
    </div>
  );
};
