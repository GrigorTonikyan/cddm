import React from "react";
import { AlertCircle, FileCode2, RefreshCw } from "lucide-react";

export interface PatchDiffPreviewProps {
  currentPatch: string;
  isSandboxLoading: boolean;
  sandboxError: string | null;
}

export const PatchDiffPreview: React.FC<PatchDiffPreviewProps> = ({
  currentPatch,
  isSandboxLoading,
  sandboxError,
}) => {
  return (
    <div className="space-y-2">
      <span className="text-slate-200 font-semibold text-xs flex items-center gap-2">
        <FileCode2 className="w-3.5 h-3.5 text-indigo-400" />
        Live Synthesized Unified Diff Patch
      </span>

      {isSandboxLoading ? (
        <div className="py-16 flex flex-col items-center justify-center gap-3 text-slate-400 font-mono text-xs bg-slate-950/60 border border-slate-800 rounded-xl">
          <RefreshCw className="w-6 h-6 animate-spin text-indigo-400" />
          <span>Synthesizing multi-site unified refactoring patch...</span>
        </div>
      ) : sandboxError ? (
        <div className="p-4 bg-rose-950/40 border border-rose-900/60 rounded-xl text-xs font-mono text-rose-300 flex items-center gap-2">
          <AlertCircle className="w-4 h-4 text-rose-400 flex-shrink-0" />
          <span>{sandboxError}</span>
        </div>
      ) : currentPatch ? (
        <div className="border border-slate-800 rounded-xl bg-slate-950 overflow-hidden">
          <pre className="p-4 text-xs font-mono overflow-x-auto max-h-[340px] leading-relaxed">
            {currentPatch.split("\n").map((line, idx) => {
              let lineClass = "text-slate-400";
              if (line.startsWith("+") && !line.startsWith("+++")) {
                lineClass = "text-emerald-400 bg-emerald-950/30 -mx-4 px-4 block";
              } else if (line.startsWith("-") && !line.startsWith("---")) {
                lineClass = "text-rose-400 bg-rose-950/30 -mx-4 px-4 block";
              } else if (line.startsWith("@@")) {
                lineClass = "text-indigo-400 font-semibold bg-indigo-950/20 -mx-4 px-4 block";
              } else if (line.startsWith("---") || line.startsWith("+++")) {
                lineClass = "text-slate-200 font-bold";
              }
              return (
                <span key={idx} className={lineClass}>
                  {line}
                  {"\n"}
                </span>
              );
            })}
          </pre>
        </div>
      ) : (
        <div className="p-8 bg-slate-950/60 border border-slate-800 rounded-xl text-center text-slate-500 font-mono text-xs">
          Click &quot;Re-Simulate Sandbox&quot; to synthesize unified refactoring patch.
        </div>
      )}
    </div>
  );
};
