import React from "react";
import { Play, RefreshCw, Sliders } from "lucide-react";

export interface SandboxHeaderControlsProps {
  customFunctionName: string;
  onFunctionNameChange: (name: string) => void;
  targetModulePath: string;
  onTargetModuleChange: (path: string) => void;
  branchName: string;
  onBranchNameChange: (branch: string) => void;
  isSandboxLoading: boolean;
  onSimulate: () => void;
}

export const SandboxHeaderControls: React.FC<SandboxHeaderControlsProps> = ({
  customFunctionName,
  onFunctionNameChange,
  targetModulePath,
  onTargetModuleChange,
  branchName,
  onBranchNameChange,
  isSandboxLoading,
  onSimulate,
}) => {
  return (
    <div className="p-3.5 bg-slate-900/80 border border-slate-800 rounded-xl space-y-3 font-mono text-xs text-slate-300">
      <div className="flex items-center justify-between">
        <span className="text-slate-200 font-semibold text-xs flex items-center gap-2">
          <Sliders className="w-3.5 h-3.5 text-indigo-400" />
          Parameterized Refactoring Studio Controls
        </span>
        <button
          type="button"
          onClick={onSimulate}
          disabled={isSandboxLoading}
          className="px-3 py-1 rounded-lg bg-indigo-600/30 hover:bg-indigo-600/50 text-indigo-300 border border-indigo-500/40 text-xs flex items-center gap-1.5 transition-colors font-semibold"
        >
          {isSandboxLoading ? (
            <RefreshCw className="w-3.5 h-3.5 animate-spin" />
          ) : (
            <Play className="w-3.5 h-3.5" />
          )}
          Re-Simulate Sandbox
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
        <div className="space-y-1">
          <label className="text-[11px] text-slate-400 block font-medium">
            Extracted Function Name
          </label>
          <input
            type="text"
            value={customFunctionName}
            onChange={(e) => onFunctionNameChange(e.target.value)}
            placeholder="extracted_shared_helper"
            className="w-full bg-slate-950 border border-slate-800 rounded-lg px-2.5 py-1.5 text-xs text-slate-200 font-mono focus:outline-none focus:border-indigo-500/60"
          />
        </div>

        <div className="space-y-1">
          <label className="text-[11px] text-slate-400 block font-medium">
            Destination Module Path (Optional)
          </label>
          <input
            type="text"
            value={targetModulePath}
            onChange={(e) => onTargetModuleChange(e.target.value)}
            placeholder="shared_utils.rs / helper.ts"
            className="w-full bg-slate-950 border border-slate-800 rounded-lg px-2.5 py-1.5 text-xs text-slate-200 font-mono focus:outline-none focus:border-indigo-500/60"
          />
        </div>

        <div className="space-y-1">
          <label className="text-[11px] text-slate-400 block font-medium">
            Dedicated Git Branch Name
          </label>
          <input
            type="text"
            value={branchName}
            onChange={(e) => onBranchNameChange(e.target.value)}
            placeholder="cddm/refactor-cluster-1"
            className="w-full bg-slate-950 border border-slate-800 rounded-lg px-2.5 py-1.5 text-xs text-slate-200 font-mono focus:outline-none focus:border-indigo-500/60"
          />
        </div>
      </div>
    </div>
  );
};
