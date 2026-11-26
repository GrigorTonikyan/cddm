import React from "react";
import { useCDDMStore } from "../../store/cddm-store";
import type { PolicySeverity, PolicyViolation } from "../../types/cddm-types";
import { AlertTriangle, ShieldAlert } from "lucide-react";

export interface PolicyViolationsBannerProps {
  violations?: PolicyViolation[];
}

function normalizeSeverity(sev: PolicySeverity): "error" | "warning" | "info" {
  const lower = sev.toLowerCase();
  if (lower === "error") return "error";
  if (lower === "warning") return "warning";
  return "info";
}

export const PolicyViolationsBanner: React.FC<PolicyViolationsBannerProps> = ({ violations }) => {
  const { setIsPolicyRulesModalOpen } = useCDDMStore();
  if (!violations || violations.length === 0) return null;

  const errorCount = violations.filter((v) => normalizeSeverity(v.severity) === "error").length;

  return (
    <div className="bg-rose-950/20 border border-rose-800/60 rounded-xl p-4 space-y-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <div className="flex items-center gap-2">
          <ShieldAlert className="w-4 h-4 text-rose-400" />
          <h4 className="text-sm font-semibold text-rose-200">
            Architectural Policy Violations ({violations.length})
          </h4>
          {errorCount > 0 && (
            <span className="px-2 py-0.5 rounded text-[11px] font-mono font-semibold bg-rose-950 text-rose-300 border border-rose-800/60">
              {errorCount} Error(s)
            </span>
          )}
        </div>
        <button
          type="button"
          onClick={() => setIsPolicyRulesModalOpen(true)}
          className="px-3 py-1 rounded-md text-xs font-medium bg-rose-900/50 hover:bg-rose-800/60 text-rose-100 border border-rose-700/50 transition-colors cursor-pointer"
        >
          Open Policy Studio
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-2.5">
        {violations.slice(0, 6).map((v, idx) => (
          <div
            key={`${v.rule_name}-${v.file_a}-${idx}`}
            className="p-3 rounded-lg bg-slate-950/80 border border-slate-800 text-xs font-mono space-y-1"
          >
            <div className="flex items-center justify-between gap-2">
              <span className="text-slate-100 font-semibold flex items-center gap-1.5">
                <AlertTriangle className="w-3.5 h-3.5 text-amber-400" />
                {v.rule_name}
              </span>
              {v.line_count !== undefined && v.max_lines !== undefined ? (
                <span className="px-2 py-0.5 rounded bg-rose-950/70 text-rose-300 border border-rose-800/60">
                  {v.line_count} / {v.max_lines} lines
                </span>
              ) : (
                <span className="px-2 py-0.5 rounded bg-slate-800 text-slate-300">
                  {v.rule_type}
                </span>
              )}
            </div>
            <div className="text-indigo-300 truncate">
              {v.file_a}:{v.start_line_a}-{v.end_line_a}
            </div>
            <p className="text-slate-400 font-sans text-[11px] line-clamp-2">{v.message}</p>
          </div>
        ))}
      </div>
    </div>
  );
};
