import React from "react";
import type { PolicySeverity, PolicyViolation } from "../../types/cddm-types";
import { ShieldCheck } from "lucide-react";

export interface PolicyViolationsTabProps {
  evalViolations: PolicyViolation[];
  isEvaluating: boolean;
  onEvaluate: () => void;
  renderSeverityBadge: (severity: PolicySeverity) => React.ReactNode;
}

export const PolicyViolationsTab: React.FC<PolicyViolationsTabProps> = ({
  evalViolations,
  isEvaluating,
  onEvaluate,
  renderSeverityBadge,
}) => {
  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-4 font-mono text-xs">
      <div className="flex items-center justify-between">
        <span className="text-xs font-medium text-slate-300">
          Total Detected Violations: {evalViolations.length}
        </span>
        <button
          onClick={onEvaluate}
          disabled={isEvaluating}
          className="px-3.5 py-1.5 rounded-lg text-xs font-medium bg-indigo-900/60 text-indigo-200 hover:bg-indigo-800/70 border border-indigo-700/50 transition-colors flex items-center gap-1.5 disabled:opacity-50 cursor-pointer"
        >
          <ShieldCheck className="w-3.5 h-3.5" />
          {isEvaluating ? "Evaluating..." : "Run Policy Check"}
        </button>
      </div>

      {evalViolations.length === 0 ? (
        <div className="flex flex-col items-center justify-center h-64 text-center p-6 rounded-xl bg-slate-900/40 border border-slate-800 font-sans">
          <ShieldCheck className="w-12 h-12 text-emerald-400 mb-3" />
          <h4 className="text-sm font-semibold text-slate-200 mb-1">
            Zero Architectural Policy Violations
          </h4>
          <p className="text-xs text-slate-400 max-w-md">
            All cross-layer boundaries, zero-duplication zones, and token limit rules are currently
            satisfied.
          </p>
        </div>
      ) : (
        <div className="space-y-3">
          {evalViolations.map((v, idx) => (
            <div
              key={idx}
              className="p-4 rounded-lg bg-slate-900/80 border border-rose-900/40 space-y-2 hover:border-rose-700/60 transition-colors"
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  {renderSeverityBadge(v.severity)}
                  <span className="font-semibold text-xs text-slate-100">{v.rule_name}</span>
                  <span className="text-xs px-2 py-0.5 bg-slate-800 rounded text-slate-400">
                    {v.rule_type}
                  </span>
                </div>
                <span className="text-xs text-slate-400">{v.token_count} matching tokens</span>
              </div>

              <p className="text-xs text-slate-300 font-sans">{v.message}</p>

              <div className="grid grid-cols-1 sm:grid-cols-2 gap-2 text-xs pt-1">
                <div className="p-2 rounded bg-slate-950 border border-slate-800/80">
                  <span className="text-slate-500 block text-[10px]">PRIMARY LOCATION:</span>
                  <span className="text-indigo-300 font-medium">
                    {v.file_a}:{v.start_line_a}-{v.end_line_a}
                  </span>
                </div>
                {v.file_b && (
                  <div className="p-2 rounded bg-slate-950 border border-slate-800/80">
                    <span className="text-slate-500 block text-[10px]">OFFENDING COUNTERPART:</span>
                    <span className="text-rose-300 font-medium">
                      {v.file_b}:{v.start_line_b}-{v.end_line_b}
                    </span>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
