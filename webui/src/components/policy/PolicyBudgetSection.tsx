import React from "react";
import type { PolicyConfig, PolicySeverity } from "../../types/cddm-types";
import { FileText, Scale } from "lucide-react";

export interface PolicyBudgetSectionProps {
  policyConfig: PolicyConfig | null;
  renderPolicyRuleHeader: (
    name: string,
    severity: PolicySeverity,
    description?: string,
  ) => React.ReactNode;
}

export const PolicyBudgetSection: React.FC<PolicyBudgetSectionProps> = ({
  policyConfig,
  renderPolicyRuleHeader,
}) => {
  const tokenBudgets = policyConfig?.token_budgets ?? [];
  const lineBudgets = policyConfig?.line_budgets ?? [];

  return (
    <>
      <div>
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
          <Scale className="w-4 h-4 text-sky-400" />
          Token & Line Budgets
        </h3>
        {tokenBudgets.length === 0 ? (
          <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
            No token budget rules configured.
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-3">
            {tokenBudgets.map((tb, idx) => (
              <div
                key={idx}
                className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
              >
                {renderPolicyRuleHeader(tb.name, tb.severity, tb.description)}
                <div className="flex flex-wrap items-center gap-3 text-xs font-mono">
                  <div className="flex items-center gap-1.5">
                    <span className="text-slate-400">Pattern:</span>
                    <span className="px-2 py-0.5 bg-slate-800/80 text-sky-300 rounded border border-slate-700/60">
                      {tb.pattern || tb.patterns?.join(", ") || "*"}
                    </span>
                  </div>
                  {tb.max_file_tokens !== undefined && (
                    <div className="flex items-center gap-1.5">
                      <span className="text-slate-400">Max File Tokens:</span>
                      <span className="text-amber-300 font-bold">{tb.max_file_tokens}</span>
                    </div>
                  )}
                  {tb.max_total_tokens !== undefined && (
                    <div className="flex items-center gap-1.5">
                      <span className="text-slate-400">Max Total Tokens:</span>
                      <span className="text-amber-300 font-bold">{tb.max_total_tokens}</span>
                    </div>
                  )}
                  {tb.max_file_lines !== undefined && (
                    <div className="flex items-center gap-1.5">
                      <span className="text-slate-400">Max File Lines:</span>
                      <span className="text-emerald-300 font-bold">{tb.max_file_lines}</span>
                    </div>
                  )}
                  {tb.ignore_tests === false && (
                    <span className="px-2 py-0.5 bg-emerald-950/60 text-emerald-300 rounded border border-emerald-800/50">
                      Tests Enforced
                    </span>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      <div>
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
          <FileText className="w-4 h-4 text-emerald-400" />
          File Line Budgets (max_file_lines)
        </h3>
        {lineBudgets.length === 0 ? (
          <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
            No dedicated line budget rules configured.
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-3">
            {lineBudgets.map((lb, idx) => (
              <div
                key={idx}
                className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
              >
                {renderPolicyRuleHeader(lb.name, lb.severity, lb.description)}
                <div className="flex flex-wrap items-center gap-3 text-xs font-mono">
                  <div className="flex items-center gap-1.5">
                    <span className="text-slate-400">Pattern:</span>
                    <span className="px-2 py-0.5 bg-slate-800/80 text-sky-300 rounded border border-slate-700/60">
                      {lb.pattern || lb.patterns?.join(", ") || "*"}
                    </span>
                  </div>
                  <div className="flex items-center gap-1.5">
                    <span className="text-slate-400">Max File Lines:</span>
                    <span className="text-emerald-300 font-bold">{lb.max_file_lines}</span>
                  </div>
                  {lb.ignore_tests === false && (
                    <span className="px-2 py-0.5 bg-emerald-950/60 text-emerald-300 rounded border border-emerald-800/50">
                      Tests Enforced
                    </span>
                  )}
                  {lb.ignore_mocks === false && (
                    <span className="px-2 py-0.5 bg-emerald-950/60 text-emerald-300 rounded border border-emerald-800/50">
                      Mocks Enforced
                    </span>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </>
  );
};
