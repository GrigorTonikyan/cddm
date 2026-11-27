import { Gauge } from "lucide-react";
import React from "react";
import type { PolicyConfig } from "../../types/cddm-types";

export interface QualityGatesPanelProps {
  policyConfig: PolicyConfig | null;
}

export const QualityGatesPanel: React.FC<QualityGatesPanelProps> = ({ policyConfig }) => {
  const scan = policyConfig?.scan;
  const deadCode = policyConfig?.dead_code;
  const hasAnyGate = Boolean(scan || deadCode);

  return (
    <div>
      <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
        <Gauge className="w-4 h-4 text-sky-400" />
        Quality Gates &amp; Dead-Code Policy ([scan] / [dead_code])
      </h3>
      {!hasAnyGate ? (
        <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
          No [scan] or [dead_code] gates configured. Edit .cddmrules.toml in the TOML Editor tab to
          enable native quality gates.
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-xs font-mono">
          <div className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 space-y-1.5">
            <div className="font-semibold text-sky-300">[scan] Quality &amp; Detection</div>
            {scan ? (
              <>
                <div className="text-slate-300">
                  Max Duplication:{" "}
                  <span className="text-amber-300">
                    {scan.max_duplication_pct !== undefined
                      ? `${scan.max_duplication_pct}%`
                      : "unset"}
                  </span>
                </div>
                <div className="text-slate-300">
                  Min DRY Score:{" "}
                  <span className="text-emerald-300">
                    {scan.min_dry_score !== undefined ? scan.min_dry_score : "unset"}
                  </span>
                </div>
                <div className="text-slate-300">
                  Type-4 Semantic:{" "}
                  <span className="text-indigo-300">
                    {scan.detect_type4 === undefined
                      ? "default"
                      : scan.detect_type4
                        ? "enabled"
                        : "disabled"}
                  </span>
                </div>
              </>
            ) : (
              <div className="text-slate-500">Not configured</div>
            )}
          </div>

          <div className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 space-y-1.5">
            <div className="font-semibold text-rose-300">[dead_code] Gate</div>
            {deadCode ? (
              <>
                <div className="text-slate-300">
                  Fail on Findings:{" "}
                  <span className="text-amber-300">
                    {deadCode.fail_on_findings ? "true" : "false"}
                  </span>
                </div>
                <div className="text-slate-300">
                  Max Items:{" "}
                  <span className="text-emerald-300">
                    {deadCode.max_items !== undefined
                      ? deadCode.max_items
                      : deadCode.fail_on_findings
                        ? 0
                        : "unset"}
                  </span>
                </div>
                <div className="text-slate-300">
                  Static Only:{" "}
                  <span className="text-indigo-300">{deadCode.static_only ? "true" : "false"}</span>
                </div>
              </>
            ) : (
              <div className="text-slate-500">Not configured</div>
            )}
          </div>
          <div className="md:col-span-2 text-slate-500">
            Read-only summary. Modify [scan] and [dead_code] sections in the .cddmrules.toml Editor
            tab.
          </div>
        </div>
      )}
    </div>
  );
};
