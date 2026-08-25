import React from "react";
import type { LanguageStats } from "../../types/cddm-types";
import { ArrowUpDown, Filter, Search } from "lucide-react";

export interface FilterToolbarProps {
  searchTerm: string;
  onSearchChange: (value: string) => void;
  minSimilarity: number;
  onMinSimilarityChange: (value: number) => void;
  selectedLang: string;
  onSelectedLangChange: (value: string) => void;
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
  sortBy,
  onSortByChange,
  languages,
}) => {
  return (
    <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-4 shadow-lg space-y-3">
      <div className="flex flex-col md:flex-row items-stretch md:items-center justify-between gap-3">
        {/* Search Input */}
        <div className="relative flex-1">
          <Search className="w-4 h-4 text-slate-400 absolute left-3.5 top-1/2 -translate-y-1/2" />
          <input
            type="text"
            value={searchTerm}
            onChange={(e) => onSearchChange(e.target.value)}
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
              onChange={(e) => onMinSimilarityChange(Number(e.target.value))}
              className="w-20 h-1.5 bg-slate-800 rounded-lg appearance-none cursor-pointer accent-indigo-500"
            />
            <span className="text-indigo-300 font-bold min-w-[32px]">{minSimilarity}%</span>
          </div>

          {/* Language Filter Dropdown */}
          <div className="flex items-center gap-2 bg-slate-950 px-3 py-1.5 rounded-lg border border-slate-800 text-xs font-mono">
            <Filter className="w-3.5 h-3.5 text-indigo-400 shrink-0" />
            <select
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
    </div>
  );
};
