import React from "react";
import type { PolicyConfig, PolicySeverity } from "../../types/cddm-types";
import { Layers, ShieldCheck, Sliders } from "lucide-react";

export interface PolicyActiveRulesTabProps {
  policyConfig: PolicyConfig | null;
  renderPolicyRuleHeader: (
    name: string,
    severity: PolicySeverity,
    description?: string,
  ) => React.ReactNode;
}

export const PolicyActiveRulesTab: React.FC<PolicyActiveRulesTabProps> = ({
  policyConfig,
  renderPolicyRuleHeader,
}) => {
  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6">
      <div>
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
          <Layers className="w-4 h-4 text-indigo-400" />
          Cross-Layer Boundary Isolation
        </h3>
        {!policyConfig?.boundaries || policyConfig.boundaries.length === 0 ? (
          <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
            No boundary rules configured.
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-3">
            {policyConfig.boundaries.map((b, idx) => (
              <div
                key={idx}
                className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
              >
                {renderPolicyRuleHeader(b.name, b.severity, b.description)}
                <div className="flex items-center gap-2 text-xs font-mono">
                  <span className="text-slate-400">Source:</span>
                  <span className="px-2 py-0.5 bg-slate-800/80 text-indigo-300 rounded border border-slate-700/60">
                    {b.source}
                  </span>
                  <span className="text-slate-500">-&gt;</span>
                  <span className="text-slate-400">Forbidden:</span>
                  <div className="flex flex-wrap gap-1">
                    {b.forbidden_targets.map((tgt, tIdx) => (
                      <span
                        key={tIdx}
                        className="px-2 py-0.5 bg-rose-950/50 text-rose-300 rounded border border-rose-900/50"
                      >
                        {tgt}
                      </span>
                    ))}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      <div>
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
          <ShieldCheck className="w-4 h-4 text-emerald-400" />
          Zero Duplication Zones
        </h3>
        {!policyConfig?.zero_duplication || policyConfig.zero_duplication.length === 0 ? (
          <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
            No zero-duplication zones.
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-3">
            {policyConfig.zero_duplication.map((z, idx) => (
              <div
                key={idx}
                className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
              >
                {renderPolicyRuleHeader(z.name, z.severity, z.description)}
                <div className="flex items-center gap-2 text-xs font-mono">
                  <span className="text-slate-400">Protected Pattern:</span>
                  <span className="px-2 py-0.5 bg-slate-800/80 text-emerald-300 rounded border border-slate-700/60">
                    {z.pattern}
                  </span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      <div>
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
          <Sliders className="w-4 h-4 text-cyan-400" />
          Clone & Occurrence Limits
        </h3>
        {!policyConfig?.limits || policyConfig.limits.length === 0 ? (
          <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
            No limit rules configured.
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-3">
            {policyConfig.limits.map((l, idx) => (
              <div
                key={idx}
                className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
              >
                {renderPolicyRuleHeader(l.name, l.severity, l.description)}
                <div className="flex flex-wrap items-center gap-3 text-xs font-mono">
                  <div className="flex items-center gap-1.5">
                    <span className="text-slate-400">Pattern:</span>
                    <span className="px-2 py-0.5 bg-slate-800/80 text-cyan-300 rounded border border-slate-700/60">
                      {l.pattern}
                    </span>
                  </div>
                  {l.max_tokens !== undefined && (
                    <div className="flex items-center gap-1.5">
                      <span className="text-slate-400">Max Tokens:</span>
                      <span className="text-amber-300 font-bold">{l.max_tokens}</span>
                    </div>
                  )}
                  {l.max_occurrences !== undefined && (
                    <div className="flex items-center gap-1.5">
                      <span className="text-slate-400">Max Cluster Occurrences:</span>
                      <span className="text-amber-300 font-bold">{l.max_occurrences}</span>
                    </div>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};
