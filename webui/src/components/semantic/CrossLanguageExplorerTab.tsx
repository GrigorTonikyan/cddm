import React, { useState } from "react";
import { useCDDMStore } from "../../store/cddm-store";
import { Sparkles, Play, Search, Cpu, Network } from "lucide-react";
import type { CrossLanguageClonePair, NeuralClonePair } from "../../types/cddm-types";
import { SemanticPairsTable } from "./SemanticPairsTable";

export interface CrossLanguageExplorerTabProps {
  onInspectPair: (pair: CrossLanguageClonePair) => void;
}

export const CrossLanguageExplorerTab: React.FC<CrossLanguageExplorerTabProps> = ({
  onInspectPair,
}) => {
  const {
    crossLanguageClones,
    isCrossLanguageLoading,
    scanCrossLanguageClones,
    neuralResult,
    isNeuralLoading,
    scanNeuralClones,
  } = useCDDMStore();

  const [activeSubMode, setActiveSubMode] = useState<"hybrid" | "neural">("hybrid");
  const [threshold, setThreshold] = useState<number>(0.7);
  const [neuralThreshold, setNeuralThreshold] = useState<number>(0.85);
  const [searchQuery, setSearchQuery] = useState<string>("");

  const handleScan = () => {
    if (activeSubMode === "hybrid") {
      void scanCrossLanguageClones(threshold);
    } else {
      void scanNeuralClones({ threshold: neuralThreshold });
    }
  };

  const filteredHybridPairs = crossLanguageClones.filter((pair) => {
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

  const filteredNeuralPairs = (neuralResult?.pairs || []).filter((pair: NeuralClonePair) => {
    const q = searchQuery.toLowerCase();
    return (
      pair.file_a.toLowerCase().includes(q) ||
      pair.file_b.toLowerCase().includes(q) ||
      pair.language_a.toLowerCase().includes(q) ||
      pair.language_b.toLowerCase().includes(q)
    );
  });

  return (
    <div className="space-y-4">
      {/* Mode Switcher */}
      <div className="flex items-center gap-2 border-b border-slate-800 pb-2">
        <button
          type="button"
          onClick={() => setActiveSubMode("hybrid")}
          className={`px-3 py-1.5 rounded-lg text-xs font-mono font-semibold flex items-center gap-1.5 transition-all cursor-pointer ${
            activeSubMode === "hybrid"
              ? "bg-purple-950/80 text-purple-200 border border-purple-800/80 shadow-sm"
              : "text-slate-400 hover:text-slate-200 hover:bg-slate-900"
          }`}
        >
          <Network className="w-3.5 h-3.5" />
          <span>Graph Hybrid (CFG/PDG)</span>
        </button>
        <button
          type="button"
          onClick={() => setActiveSubMode("neural")}
          className={`px-3 py-1.5 rounded-lg text-xs font-mono font-semibold flex items-center gap-1.5 transition-all cursor-pointer ${
            activeSubMode === "neural"
              ? "bg-indigo-950/80 text-indigo-200 border border-indigo-800/80 shadow-sm"
              : "text-slate-400 hover:text-slate-200 hover:bg-slate-900"
          }`}
        >
          <Cpu className="w-3.5 h-3.5" />
          <span>Local Neural Embeddings</span>
        </button>
      </div>

      {/* Controls Bar */}
      <div className="p-4 bg-slate-900/80 border border-slate-800 rounded-xl flex flex-wrap items-center justify-between gap-4">
        <div className="flex items-center gap-4 flex-1 min-w-[280px]">
          <div className="flex-1 space-y-1">
            <div className="flex justify-between text-xs font-mono text-slate-300">
              <label htmlFor="cross-lang-cutoff-slider" className="cursor-pointer">
                {activeSubMode === "hybrid" ? "Hybrid Cutoff:" : "Neural Cosine Cutoff:"}
              </label>
              <span className="text-purple-300 font-bold">
                {((activeSubMode === "hybrid" ? threshold : neuralThreshold) * 100).toFixed(0)}%
              </span>
            </div>
            <input
              id="cross-lang-cutoff-slider"
              name="cutoff_threshold"
              aria-label="Similarity Cutoff Threshold Percentage"
              type="range"
              min={activeSubMode === "hybrid" ? "0.50" : "0.70"}
              max="0.98"
              step="0.02"
              value={activeSubMode === "hybrid" ? threshold : neuralThreshold}
              onChange={(e) =>
                activeSubMode === "hybrid"
                  ? setThreshold(Number(e.target.value))
                  : setNeuralThreshold(Number(e.target.value))
              }
              className="w-full h-1.5 bg-slate-950 rounded-lg appearance-none cursor-pointer accent-purple-500 border border-slate-800"
            />
          </div>

          <button
            type="button"
            onClick={handleScan}
            disabled={isCrossLanguageLoading || isNeuralLoading}
            className="px-4 py-2 bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 text-white font-bold rounded-lg text-xs flex items-center gap-2 transition-all shadow-md shadow-purple-950/40 cursor-pointer disabled:opacity-50"
          >
            <Play className="w-3.5 h-3.5 fill-current" />
            <span>
              {isCrossLanguageLoading || isNeuralLoading
                ? "Analyzing..."
                : activeSubMode === "hybrid"
                  ? "Discover Polyglot Clones"
                  : "Run Neural Embedding Scan"}
            </span>
          </button>
        </div>

        <div className="relative min-w-[220px]">
          <Search className="w-3.5 h-3.5 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
          <input
            id="cross-lang-search-input"
            name="search_query"
            aria-label="Search cross-language clones"
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search clones..."
            className="w-full bg-slate-950 border border-slate-800 rounded-lg pl-8 pr-3 py-1.5 text-xs text-slate-200 font-mono focus:outline-none focus:border-purple-500"
          />
        </div>
      </div>

      {/* Results View */}
      {activeSubMode === "hybrid" ? (
        isCrossLanguageLoading ? (
          <div className="py-20 text-center text-slate-400 text-xs font-mono animate-pulse bg-slate-900/40 border border-slate-800 rounded-xl">
            Extracting CFG/PDG graph isomorphism models and computing subword TF-IDF embeddings...
          </div>
        ) : filteredHybridPairs.length === 0 ? (
          <div className="py-16 text-center text-slate-400 text-xs font-mono bg-slate-900/40 border border-slate-800 rounded-xl space-y-2">
            <Sparkles className="w-6 h-6 text-purple-400 mx-auto opacity-80" />
            <p className="font-semibold text-slate-300">
              No cross-language semantic duplicates discovered yet.
            </p>
          </div>
        ) : (
          <SemanticPairsTable
            mode="hybrid"
            hybridPairs={filteredHybridPairs}
            onInspectPair={onInspectPair}
          />
        )
      ) : isNeuralLoading ? (
        <div className="py-20 text-center text-slate-400 text-xs font-mono animate-pulse bg-slate-900/40 border border-slate-800 rounded-xl">
          Computing in-process dense 256-dimensional subword embedding projections and cosine
          similarity matrices...
        </div>
      ) : filteredNeuralPairs.length === 0 ? (
        <div className="py-16 text-center text-slate-400 text-xs font-mono bg-slate-900/40 border border-slate-800 rounded-xl space-y-2">
          <Cpu className="w-6 h-6 text-indigo-400 mx-auto opacity-80" />
          <p className="font-semibold text-slate-300">
            No neural algorithmic equivalence pairs discovered yet.
          </p>
        </div>
      ) : (
        <SemanticPairsTable mode="neural" neuralPairs={filteredNeuralPairs} />
      )}
    </div>
  );
};
