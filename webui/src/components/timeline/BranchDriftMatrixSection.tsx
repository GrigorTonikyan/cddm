import React, { useState } from "react";
import { GitBranch, RefreshCw, AlertCircle, ArrowUpRight, ArrowDownRight } from "lucide-react";
import type { BranchMatrixReport } from "../../types/cddm-types";
import { postJson } from "../../utils/api-client";

export interface BranchDriftMatrixSectionProps {
  initialReport?: BranchMatrixReport | null;
}

export const BranchDriftMatrixSection: React.FC<BranchDriftMatrixSectionProps> = ({
  initialReport = null,
}) => {
  const [branchesInput, setBranchesInput] = useState<string>("main, HEAD");
  const [report, setReport] = useState<BranchMatrixReport | null>(initialReport);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const handleComputeMatrix = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const branchList = branchesInput
        .split(",")
        .map((b) => b.trim())
        .filter(Boolean);

      if (branchList.length < 2) {
        throw new Error("Please specify at least 2 branches or commit hashes separated by commas.");
      }

      const res = await postJson<BranchMatrixReport>(
        "/api/diff/matrix",
        { branches: branchList, min_tokens: 50 },
        "Failed to calculate branch drift matrix",
      );
      setReport(res);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to calculate branch matrix");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="space-y-4">
      <div className="bg-slate-900/60 p-3.5 rounded-xl border border-slate-800/80 flex flex-wrap items-center justify-between gap-3">
        <div className="flex items-center gap-2 flex-1 min-w-[260px]">
          <GitBranch className="w-4 h-4 text-indigo-400 shrink-0" />
          <input
            id="branch-drift-input"
            name="branch_drift_names"
            aria-label="Git branch names for drift matrix comparison"
            type="text"
            value={branchesInput}
            onChange={(e) => setBranchesInput(e.target.value)}
            placeholder="e.g. main, feature/auth, HEAD"
            className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 text-xs text-slate-200 font-mono focus:outline-none focus:border-indigo-500"
          />
        </div>
        <button
          type="button"
          onClick={handleComputeMatrix}
          disabled={isLoading}
          className="px-3.5 py-1.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white text-xs font-semibold flex items-center gap-1.5 transition-colors"
        >
          <RefreshCw className={`w-3.5 h-3.5 ${isLoading ? "animate-spin" : ""}`} />
          <span>Compute Drift Matrix</span>
        </button>
      </div>

      {error && (
        <div className="p-3 bg-rose-950/30 border border-rose-800/50 rounded-xl text-rose-300 text-xs flex items-center gap-2">
          <AlertCircle className="w-4 h-4 shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {report && (
        <div className="space-y-3">
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-3">
              <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                Branches
              </span>
              <p className="text-sm font-mono text-slate-200 mt-0.5">
                {report.branches.length} Compared
              </p>
            </div>
            <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-3">
              <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                Cleanest Branch
              </span>
              <p className="text-sm font-mono text-emerald-400 mt-0.5">
                {report.cleanest_branch || "N/A"}
              </p>
            </div>
            <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-3">
              <span className="text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                Highest Drift
              </span>
              <p className="text-sm font-mono text-amber-400 mt-0.5">
                {report.highest_drift_branch || "N/A"}
              </p>
            </div>
          </div>

          <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl overflow-hidden shadow-lg">
            <div className="overflow-x-auto">
              <table className="w-full text-left text-xs font-mono">
                <thead className="bg-slate-950/80 text-slate-400 uppercase text-[10px] tracking-wider border-b border-slate-800">
                  <tr>
                    <th className="py-2.5 px-3">Base</th>
                    <th className="py-2.5 px-3">Target</th>
                    <th className="py-2.5 px-3 text-right">Base DRY</th>
                    <th className="py-2.5 px-3 text-right">Target DRY</th>
                    <th className="py-2.5 px-3 text-right">Net Delta</th>
                    <th className="py-2.5 px-3 text-right">Changed Files</th>
                    <th className="py-2.5 px-3 text-right">Divergence</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-slate-800/50 text-slate-300">
                  {report.matrix.map((row, idx) => (
                    <tr
                      key={`${row.base_branch}-${row.target_branch}-${idx}`}
                      className="hover:bg-slate-800/30"
                    >
                      <td className="py-2 px-3 text-slate-200">{row.base_branch}</td>
                      <td className="py-2 px-3 text-indigo-300">{row.target_branch}</td>
                      <td className="py-2 px-3 text-right">{row.base_dry_score.toFixed(1)}</td>
                      <td className="py-2 px-3 text-right">{row.target_dry_score.toFixed(1)}</td>
                      <td
                        className={`py-2 px-3 text-right font-bold ${row.net_dry_delta >= 0 ? "text-emerald-400" : "text-rose-400"}`}
                      >
                        <span className="inline-flex items-center gap-0.5">
                          {row.net_dry_delta >= 0 ? (
                            <ArrowUpRight className="w-3 h-3" />
                          ) : (
                            <ArrowDownRight className="w-3 h-3" />
                          )}
                          {row.net_dry_delta >= 0 ? "+" : ""}
                          {row.net_dry_delta.toFixed(2)}%
                        </span>
                      </td>
                      <td className="py-2 px-3 text-right">{row.changed_files_count}</td>
                      <td className="py-2 px-3 text-right text-amber-400">
                        {row.divergence_index.toFixed(1)}%
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
