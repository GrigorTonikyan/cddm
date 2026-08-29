import React from "react";
import { Eye } from "lucide-react";
import type { CrossLanguageClonePair, NeuralClonePair } from "../../types/cddm-types";

export interface SemanticPairsTableProps {
  mode: "hybrid" | "neural";
  hybridPairs?: CrossLanguageClonePair[];
  neuralPairs?: NeuralClonePair[];
  onInspectPair?: (pair: CrossLanguageClonePair) => void;
}

export const SemanticPairsTable: React.FC<SemanticPairsTableProps> = ({
  mode,
  hybridPairs = [],
  neuralPairs = [],
  onInspectPair,
}) => {
  const isHybrid = mode === "hybrid";

  return (
    <div className="border border-slate-800 rounded-xl overflow-hidden shadow-lg bg-slate-900/70">
      <div className="overflow-x-auto">
        <table className="w-full text-left text-xs font-mono">
          <thead className="bg-slate-950/80 text-slate-400 border-b border-slate-800 uppercase tracking-wider text-[10px]">
            <tr>
              <th className="py-2.5 px-3">Target A</th>
              <th className="py-2.5 px-3">Target B</th>
              <th className="py-2.5 px-3 text-center">
                {isHybrid ? "Graph / Token Sim" : "Cosine Sim"}
              </th>
              <th className="py-2.5 px-3 text-center">
                {isHybrid ? "Hybrid Score" : "Confidence"}
              </th>
              <th className="py-2.5 px-3 text-right">{isHybrid ? "Action" : "Rationale"}</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800/60">
            {isHybrid
              ? hybridPairs.map((pair, idx) => (
                  <tr
                    key={`hybrid-${idx}-${pair.file_a}-${pair.file_b}`}
                    className="hover:bg-slate-800/40 transition-colors group"
                  >
                    <td className="py-2.5 px-3">
                      <div className="flex items-center gap-1.5">
                        <span className="px-1.5 py-0.5 rounded bg-indigo-950 text-indigo-300 border border-indigo-800/50 text-[10px]">
                          {pair.language_a}
                        </span>
                        <span className="font-bold text-slate-200">{pair.function_a}</span>
                      </div>
                      <div className="text-[10px] text-slate-400 truncate max-w-50 mt-0.5">
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
                      <div className="text-[10px] text-slate-400 truncate max-w-50 mt-0.5">
                        {pair.file_b}:{pair.lines_b[0]}-{pair.lines_b[1]}
                      </div>
                    </td>

                    <td className="py-2.5 px-3 text-center text-slate-300">
                      {(pair.graph_similarity * 100).toFixed(1)}% /{" "}
                      {(pair.token_similarity * 100).toFixed(1)}%
                    </td>

                    <td className="py-2.5 px-3 text-center">
                      <span className="px-2 py-0.5 rounded-full font-bold bg-emerald-950 text-emerald-300 border border-emerald-800/60">
                        {(pair.hybrid_score * 100).toFixed(1)}%
                      </span>
                    </td>

                    <td className="py-2.5 px-3 text-right">
                      {onInspectPair && (
                        <button
                          type="button"
                          onClick={() => onInspectPair(pair)}
                          className="px-2.5 py-1 bg-slate-800 hover:bg-indigo-600 text-slate-200 hover:text-white rounded-lg text-[11px] font-semibold transition-all inline-flex items-center gap-1 cursor-pointer"
                          title="Load into CFG Visualizer"
                        >
                          <Eye className="w-3 h-3" />
                          <span>Inspect</span>
                        </button>
                      )}
                    </td>
                  </tr>
                ))
              : neuralPairs.map((pair, idx) => (
                  <tr
                    key={`neural-${idx}-${pair.file_a}-${pair.file_b}`}
                    className="hover:bg-slate-800/40 transition-colors"
                  >
                    <td className="py-2.5 px-3">
                      <div className="flex items-center gap-1.5">
                        <span className="px-1.5 py-0.5 rounded bg-indigo-950 text-indigo-300 border border-indigo-800/50 text-[10px]">
                          {pair.language_a}
                        </span>
                        <span className="font-bold text-slate-200 truncate max-w-45">
                          {pair.file_a}
                        </span>
                      </div>
                      <div className="text-[10px] text-slate-400 mt-0.5">
                        Lines {pair.start_line_a}-{pair.end_line_a}
                      </div>
                    </td>

                    <td className="py-2.5 px-3">
                      <div className="flex items-center gap-1.5">
                        <span className="px-1.5 py-0.5 rounded bg-purple-950 text-purple-300 border border-purple-800/50 text-[10px]">
                          {pair.language_b}
                        </span>
                        <span className="font-bold text-slate-200 truncate max-w-45">
                          {pair.file_b}
                        </span>
                      </div>
                      <div className="text-[10px] text-slate-400 mt-0.5">
                        Lines {pair.start_line_b}-{pair.end_line_b}
                      </div>
                    </td>

                    <td className="py-2.5 px-3 text-center font-bold text-indigo-300">
                      {(pair.similarity * 100).toFixed(1)}%
                    </td>

                    <td className="py-2.5 px-3 text-center">
                      <span
                        className={`px-2 py-0.5 rounded-full font-bold text-[10px] border ${
                          pair.confidence === "High"
                            ? "bg-emerald-950 text-emerald-300 border-emerald-800/60"
                            : pair.confidence === "Medium"
                              ? "bg-amber-950 text-amber-300 border-amber-800/60"
                              : "bg-slate-800 text-slate-300 border-slate-700"
                        }`}
                      >
                        {pair.confidence}
                      </span>
                    </td>

                    <td className="py-2.5 px-3 text-slate-400 text-[11px] truncate max-w-70 text-right">
                      {pair.semantic_rationale}
                    </td>
                  </tr>
                ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};
