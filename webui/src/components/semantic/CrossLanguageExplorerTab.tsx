import React, { useState } from "react";
import { useCDDMStore } from "../../store/cddm-store";
import { Sparkles, Play, Search, Eye } from "lucide-react";
import type { CrossLanguageClonePair } from "../../types/cddm-types";

export interface CrossLanguageExplorerTabProps {
  onInspectPair: (pair: CrossLanguageClonePair) => void;
}

export const CrossLanguageExplorerTab: React.FC<CrossLanguageExplorerTabProps> = ({
  onInspectPair,
}) => {
  const { crossLanguageClones, isCrossLanguageLoading, scanCrossLanguageClones } = useCDDMStore();
  const [threshold, setThreshold] = useState<number>(0.7);
  const [searchQuery, setSearchQuery] = useState<string>("");

  const handleScan = () => {
    void scanCrossLanguageClones(threshold);
  };

  const filteredPairs = crossLanguageClones.filter((pair) => {
    const q = searchQuery.toLowerCase();
    return (
      pair.function_a.toLowerCase().includes(q) ||
      pair.function_b.toLowerCase().includes(q) ||
      pair.file_a.toLowerCase().includes(q) ||
      pair.file_b.toLowerCase().includes(q) ||
      pair.language_a.toLowerCase().includes(q) ||
      pair.language_b.toLowerCase().includes(q)
    );
  });

  return (
    <div className="space-y-4">
      {/* Controls Bar */}
      <div className="p-4 bg-slate-900/80 border border-slate-800 rounded-xl flex flex-wrap items-center justify-between gap-4">
        <div className="flex items-center gap-4 flex-1 min-w-[280px]">
          <div className="flex-1 space-y-1">
            <div className="flex justify-between text-xs font-mono text-slate-300">
              <span>Similarity Cutoff:</span>
              <span className="text-purple-300 font-bold">{(threshold * 100).toFixed(0)}%</span>
            </div>
            <input
              type="range"
              min="0.50"
              max="0.95"
              step="0.05"
              value={threshold}
              onChange={(e) => setThreshold(Number(e.target.value))}
              className="w-full h-1.5 bg-slate-950 rounded-lg appearance-none cursor-pointer accent-purple-500 border border-slate-800"
            />
          </div>

          <button
            type="button"
            onClick={handleScan}
            disabled={isCrossLanguageLoading}
            className="px-4 py-2 bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 text-white font-bold rounded-lg text-xs flex items-center gap-2 transition-all shadow-md shadow-purple-950/40 cursor-pointer disabled:opacity-50"
          >
            <Play className="w-3.5 h-3.5 fill-current" />
            <span>{isCrossLanguageLoading ? "Analyzing..." : "Discover Polyglot Clones"}</span>
          </button>
        </div>

        {crossLanguageClones.length > 0 && (
          <div className="relative min-w-[220px]">
            <Search className="w-3.5 h-3.5 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search polyglot clones..."
              className="w-full bg-slate-950 border border-slate-800 rounded-lg pl-8 pr-3 py-1.5 text-xs text-slate-200 font-mono focus:outline-none focus:border-purple-500"
            />
          </div>
        )}
      </div>

      {/* Results List */}
      {isCrossLanguageLoading ? (
        <div className="py-20 text-center text-slate-400 text-xs font-mono animate-pulse bg-slate-900/40 border border-slate-800 rounded-xl">
          Extracting CFG/PDG graph isomorphism models and computing subword TF-IDF embeddings across
          polyglot files...
        </div>
      ) : crossLanguageClones.length === 0 ? (
        <div className="py-16 text-center text-slate-400 text-xs font-mono bg-slate-900/40 border border-slate-800 rounded-xl space-y-2">
          <Sparkles className="w-6 h-6 text-purple-400 mx-auto opacity-80" />
          <p className="font-semibold text-slate-300">
            No cross-language semantic duplicates discovered yet.
          </p>
          <p className="text-[11px] text-slate-400">
            Click &quot;Discover Polyglot Clones&quot; to scan all multi-language source files in
            the workspace.
          </p>
        </div>
      ) : (
        <div className="border border-slate-800 rounded-xl overflow-hidden shadow-lg bg-slate-900/70">
          <div className="overflow-x-auto">
            <table className="w-full text-left text-xs font-mono">
              <thead className="bg-slate-950/80 text-slate-400 border-b border-slate-800 uppercase tracking-wider text-[10px]">
                <tr>
                  <th className="py-2.5 px-3">Function A</th>
                  <th className="py-2.5 px-3">Function B</th>
                  <th className="py-2.5 px-3 text-center">Graph Sim</th>
                  <th className="py-2.5 px-3 text-center">Token Sim</th>
                  <th className="py-2.5 px-3 text-center">Hybrid Score</th>
                  <th className="py-2.5 px-3 text-right">Actions</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-800/60">
                {filteredPairs.map((pair, idx) => (
                  <tr
                    key={`cross-lang-pair-${idx}-${pair.file_a}-${pair.file_b}`}
                    className="hover:bg-slate-800/40 transition-colors group"
                  >
                    <td className="py-2.5 px-3">
                      <div className="flex items-center gap-1.5">
                        <span className="px-1.5 py-0.5 rounded bg-indigo-950 text-indigo-300 border border-indigo-800/50 text-[10px]">
                          {pair.language_a}
                        </span>
                        <span className="font-bold text-slate-200">{pair.function_a}</span>
                      </div>
                      <div className="text-[10px] text-slate-400 truncate max-w-[200px] mt-0.5">
                        {pair.file_a}:{pair.lines_a[0]}-{pair.lines_a[1]}
                      </div>
                    </td>

                    <td className="py-2.5 px-3">
                      <div className="flex items-center gap-1.5">
                        <span className="px-1.5 py-0.5 rounded bg-purple-950 text-purple-300 border border-purple-800/50 text-[10px]">
                          {pair.language_b}
                        </span>
                        <span className="font-bold text-slate-200">{pair.function_b}</span>
                      </div>
                      <div className="text-[10px] text-slate-400 truncate max-w-[200px] mt-0.5">
                        {pair.file_b}:{pair.lines_b[0]}-{pair.lines_b[1]}
                      </div>
                    </td>

                    <td className="py-2.5 px-3 text-center text-slate-300">
                      {(pair.graph_similarity * 100).toFixed(1)}%
                    </td>

                    <td className="py-2.5 px-3 text-center text-slate-300">
                      {(pair.token_similarity * 100).toFixed(1)}%
                    </td>

                    <td className="py-2.5 px-3 text-center">
                      <span className="px-2 py-0.5 rounded-full font-bold bg-emerald-950 text-emerald-300 border border-emerald-800/60">
                        {(pair.hybrid_score * 100).toFixed(1)}%
                      </span>
                    </td>

                    <td className="py-2.5 px-3 text-right">
                      <button
                        type="button"
                        onClick={() => onInspectPair(pair)}
                        className="px-2.5 py-1 bg-slate-800 hover:bg-indigo-600 text-slate-200 hover:text-white rounded-lg text-[11px] font-semibold transition-all inline-flex items-center gap-1 cursor-pointer"
                        title="Load into CFG Visualizer"
                      >
                        <Eye className="w-3 h-3" />
                        <span>Inspect</span>
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
};
