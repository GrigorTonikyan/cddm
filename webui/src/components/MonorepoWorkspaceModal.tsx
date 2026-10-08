import React, { useState, useEffect } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xWindow } from "./ui/win2x-manager";
import { Boxes, FolderGit2, Package, FileCode, RefreshCw, AlertCircle, Folder } from "lucide-react";

export interface MonorepoWorkspaceModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const MonorepoWorkspaceModal: React.FC<MonorepoWorkspaceModalProps> = ({
  isOpen,
  onClose,
}) => {
  const { monorepoData, isMonorepoLoading, monorepoError, runMonorepoScan, config } =
    useCDDMStore();

  const [directory, setDirectory] = useState<string>(config?.directory || ".");
  const [minTokens, setMinTokens] = useState<number>(config?.min_tokens || 50);

  useEffect(() => {
    if (isOpen && !monorepoData && !isMonorepoLoading && runMonorepoScan) {
      void runMonorepoScan(directory, minTokens).catch(() => {});
    }
  }, [isOpen, monorepoData, isMonorepoLoading, runMonorepoScan, directory, minTokens]);

  if (!isOpen) return null;

  const handleScan = async () => {
    try {
      await runMonorepoScan?.(directory, minTokens);
    } catch {
      // Handled via store monorepoError
    }
  };

  const workspaces = monorepoData?.workspaces || [];
  const totalWorkspaces = monorepoData?.total_workspaces ?? workspaces.length;
  const crossWorkspaceClones = monorepoData?.cross_workspace_clones ?? 0;
  const avgDryScore = monorepoData?.average_dry_score ?? 100.0;

  const footerContent = (
    <div className="flex items-center justify-between w-full">
      <div className="flex items-center gap-2 text-xs font-mono text-slate-400">
        <Boxes className="w-3.5 h-3.5 text-indigo-400" />
        <span>Multi-Package AST Isolation</span>
      </div>
      <button
        type="button"
        onClick={onClose}
        className="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold text-xs transition-colors"
      >
        Close
      </button>
    </div>
  );

  return (
    <Win2xWindow
      id="monorepo-workspace-modal"
      windowType="monorepo-workspace"
      isOpen={isOpen}
      onClose={onClose}
      title="Monorepo Workspace & Multi-Package Architecture"
      subtitle="Detect inter-package duplication, cross-workspace clone clusters, and package boundaries"
      badge={`${totalWorkspaces} Package${totalWorkspaces === 1 ? "" : "s"}`}
      icon={<Boxes className="w-4 h-4 text-purple-400" />}
      footer={footerContent}
      initialWidth={880}
      initialHeight={640}
    >
      <div className="space-y-6">
        {/* Controls Bar */}
        <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-4 flex flex-wrap items-center gap-4">
          <div className="flex-1 min-w-[200px]">
            <label
              htmlFor="monorepo-dir"
              className="block text-xs font-semibold text-slate-400 mb-1"
            >
              Monorepo Root Directory:
            </label>
            <div className="flex items-center gap-2 bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 focus-within:border-indigo-500">
              <Folder className="w-3.5 h-3.5 text-slate-500" />
              <input
                id="monorepo-dir"
                type="text"
                value={directory}
                onChange={(e) => setDirectory(e.target.value)}
                placeholder="."
                className="bg-transparent text-xs font-mono text-slate-200 w-full focus:outline-none"
              />
            </div>
          </div>

          <div className="w-[140px]">
            <label
              htmlFor="monorepo-tokens"
              className="block text-xs font-semibold text-slate-400 mb-1"
            >
              Min Tokens:
            </label>
            <input
              id="monorepo-tokens"
              type="number"
              min="10"
              max="500"
              value={minTokens}
              onChange={(e) => setMinTokens(Number(e.target.value))}
              className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 text-xs font-mono text-slate-200 focus:outline-none focus:border-indigo-500"
            />
          </div>

          <div className="self-end">
            <button
              type="button"
              onClick={() => void handleScan()}
              disabled={isMonorepoLoading}
              className="flex items-center gap-2 px-4 py-2 bg-indigo-600 hover:bg-indigo-500 text-white rounded-lg text-xs font-semibold transition-colors disabled:opacity-50"
            >
              <RefreshCw className={`w-3.5 h-3.5 ${isMonorepoLoading ? "animate-spin" : ""}`} />
              <span>{isMonorepoLoading ? "Scanning Monorepo..." : "Scan Workspace"}</span>
            </button>
          </div>
        </div>

        {/* Error Notification */}
        {monorepoError && (
          <div className="p-3 bg-rose-950/60 border border-rose-500/40 rounded-lg text-xs text-rose-200 flex items-center gap-2">
            <AlertCircle className="w-4 h-4 text-rose-400 shrink-0" />
            <span>{monorepoError}</span>
          </div>
        )}

        {/* Summary Stat Cards */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
          <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-3">
            <span className="text-[11px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
              Packages / Crates
            </span>
            <span className="text-xl font-mono font-bold text-indigo-300">{totalWorkspaces}</span>
          </div>

          <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-3">
            <span className="text-[11px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
              Cross-Package Clones
            </span>
            <span
              className={`text-xl font-mono font-bold ${
                crossWorkspaceClones > 0 ? "text-amber-400" : "text-emerald-400"
              }`}
            >
              {crossWorkspaceClones}
            </span>
          </div>

          <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-3">
            <span className="text-[11px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
              Total Clones
            </span>
            <span className="text-xl font-mono font-bold text-slate-200">
              {monorepoData?.total_clones ?? 0}
            </span>
          </div>

          <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-3">
            <span className="text-[11px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
              Avg DRY Health
            </span>
            <span
              className={`text-xl font-mono font-bold ${
                avgDryScore >= 90 ? "text-emerald-400" : "text-rose-400"
              }`}
            >
              {avgDryScore.toFixed(1)}%
            </span>
          </div>
        </div>

        {/* Packages Breakdown */}
        <div className="space-y-3">
          <div className="flex items-center justify-between text-xs font-bold uppercase tracking-wider text-slate-300">
            <div className="flex items-center gap-2">
              <FolderGit2 className="w-4 h-4 text-purple-400" />
              <span>Workspace Packages ({workspaces.length})</span>
            </div>
            {crossWorkspaceClones > 0 && (
              <span className="text-amber-400 text-[11px] normal-case font-normal bg-amber-950/60 border border-amber-800/60 px-2 py-0.5 rounded-full">
                {crossWorkspaceClones} cross-package clone{crossWorkspaceClones === 1 ? "" : "s"}{" "}
                detected
              </span>
            )}
          </div>

          {workspaces.length === 0 ? (
            <div className="text-center py-12 bg-slate-900/40 border border-slate-800/80 rounded-xl">
              <Package className="w-10 h-10 text-slate-600 mx-auto mb-2" />
              <p className="text-sm font-semibold text-slate-400">No workspace packages detected</p>
              <p className="text-xs text-slate-500 mt-1">
                Ensure the target repository contains Cargo workspaces, npm/pnpm/yarn workspaces, or
                Go modules.
              </p>
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3 max-h-[340px] overflow-y-auto pr-1">
              {workspaces.map((pkg) => (
                <div
                  key={pkg.path}
                  className="bg-slate-900/70 border border-slate-800 hover:border-slate-700 rounded-xl p-3.5 flex flex-col justify-between transition-colors"
                >
                  <div>
                    <div className="flex items-center justify-between gap-2 mb-1.5">
                      <span className="font-bold text-xs text-slate-200 truncate">{pkg.name}</span>
                      <span className="text-[10px] uppercase font-mono px-2 py-0.5 rounded bg-indigo-950/80 text-indigo-300 border border-indigo-800/60">
                        {pkg.package_type}
                      </span>
                    </div>
                    <div className="text-[11px] font-mono text-slate-500 truncate mb-1.5">
                      {pkg.path}
                    </div>
                    <div className="flex items-center gap-1.5 text-[10px] font-mono text-slate-400 bg-slate-950/60 px-2 py-1 rounded border border-slate-800/60">
                      <FileCode className="w-3 h-3 text-indigo-400 shrink-0" />
                      <span className="truncate">{pkg.manifest_file}</span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </Win2xWindow>
  );
};
