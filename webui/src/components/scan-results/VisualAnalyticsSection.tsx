import React, { useMemo, useState } from "react";
import type { ScanResult } from "../../types/cddm-types";
import { getLanguageStyle } from "../../utils/path-utils";
import { DuplicationTreemap } from "../DuplicationTreemap";
import { LayoutGrid, Maximize2, PieChart, Sparkles } from "lucide-react";

export interface VisualAnalyticsSectionProps {
  results: ScanResult;
  searchTerm: string;
  onSelectFilterPath: (path: string) => void;
  onOpenTreemapModal: () => void;
  onOpenLanguageModal: () => void;
}

export const VisualAnalyticsSection: React.FC<VisualAnalyticsSectionProps> = ({
  results,
  searchTerm,
  onSelectFilterPath,
  onOpenTreemapModal,
  onOpenLanguageModal,
}) => {
  const [analyticsView, setAnalyticsView] = useState<"treemap" | "languages">("treemap");

  const totalTokensAllLangs = useMemo(() => {
    return results.language_breakdown.reduce((sum, item) => sum + item.tokens, 0);
  }, [results.language_breakdown]);

  return (
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
                onOpenTreemapModal();
              } else {
                onOpenLanguageModal();
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
          onSelectFilterPath={onSelectFilterPath}
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
                const pct = totalTokensAllLangs > 0 ? (item.tokens / totalTokensAllLangs) * 100 : 0;
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
                const pct = totalTokensAllLangs > 0 ? (item.tokens / totalTokensAllLangs) * 100 : 0;
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
  );
};
