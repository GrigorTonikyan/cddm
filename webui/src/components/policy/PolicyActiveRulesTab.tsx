import React from "react";
import type { PolicyConfig, PolicySeverity } from "../../types/cddm-types";
import {
  Ban,
  BookOpen,
  FileX,
  Layers,
  RefreshCw,
  Scale,
  ShieldCheck,
  Sliders,
  Smile,
} from "lucide-react";

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

      <div>
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
          <Ban className="w-4 h-4 text-rose-400" />
          Prohibited Code Patterns (Regex & AST)
        </h3>
        {!policyConfig?.prohibited_patterns || policyConfig.prohibited_patterns.length === 0 ? (
          <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
            No prohibited pattern rules configured.
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-3">
            {policyConfig.prohibited_patterns.map((p, idx) => (
              <div
                key={idx}
                className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
              >
                {renderPolicyRuleHeader(p.name, p.severity, p.description)}
                <div className="flex flex-wrap items-center gap-3 text-xs font-mono">
                  <div className="flex items-center gap-1.5">
                    <span className="text-slate-400">Type:</span>
                    <span className="px-2 py-0.5 bg-slate-800/80 text-purple-300 rounded border border-slate-700/60">
                      {p.pattern_type || "regex"}
                    </span>
                  </div>
                  <div className="flex items-center gap-1.5">
                    <span className="text-slate-400">Pattern:</span>
                    <span className="px-2 py-0.5 bg-slate-800/80 text-rose-300 rounded border border-slate-700/60">
                      {p.pattern}
                    </span>
                  </div>
                  {p.remediation && (
                    <div className="w-full text-slate-400 text-xs mt-1">
                      <span className="text-emerald-400 font-semibold">Remediation:</span>{" "}
                      {p.remediation}
                    </div>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      <div>
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
          <Smile className="w-4 h-4 text-amber-400" />
          Zero-Emoji Sensor
        </h3>
        <div className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 flex items-center justify-between text-xs font-mono">
          <div className="flex items-center gap-2">
            <span className="text-slate-400">Status:</span>
            <span
              className={`px-2 py-0.5 rounded font-semibold ${
                policyConfig?.zero_emojis
                  ? "bg-emerald-950/70 text-emerald-300 border border-emerald-800/60"
                  : "bg-slate-800 text-slate-400 border border-slate-700/60"
              }`}
            >
              {policyConfig?.zero_emojis ? "ENFORCED (Active)" : "DISABLED"}
            </span>
          </div>
          <span className="text-slate-400 text-xs">
            {policyConfig?.zero_emojis
              ? "All files scanned for pictographic glyphs"
              : "Enable 'zero_emojis = true' in .cddmrules.toml"}
          </span>
        </div>
      </div>

      <div>
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
          <Scale className="w-4 h-4 text-sky-400" />
          Token Budgets
        </h3>
        {!policyConfig?.token_budgets || policyConfig.token_budgets.length === 0 ? (
          <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
            No token budget rules configured.
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-3">
            {policyConfig.token_budgets.map((tb, idx) => (
              <div
                key={idx}
                className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
              >
                {renderPolicyRuleHeader(tb.name, tb.severity, tb.description)}
                <div className="flex flex-wrap items-center gap-3 text-xs font-mono">
                  <div className="flex items-center gap-1.5">
                    <span className="text-slate-400">Pattern:</span>
                    <span className="px-2 py-0.5 bg-slate-800/80 text-sky-300 rounded border border-slate-700/60">
                      {tb.pattern}
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
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      <div>
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
          <FileX className="w-4 h-4 text-red-400" />
          Forbidden Files & Conflicting Tooling
        </h3>
        {!policyConfig?.forbidden_files || policyConfig.forbidden_files.length === 0 ? (
          <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
            No forbidden file rules configured.
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-3">
            {policyConfig.forbidden_files.map((ff, idx) => {
              const patterns = [...(ff.pattern ? [ff.pattern] : []), ...(ff.patterns || [])];
              return (
                <div
                  key={idx}
                  className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
                >
                  {renderPolicyRuleHeader(ff.name, ff.severity, ff.description)}
                  <div className="flex flex-wrap items-center gap-2 text-xs font-mono">
                    <span className="text-slate-400">Banned Patterns:</span>
                    {patterns.map((pat, pIdx) => (
                      <span
                        key={pIdx}
                        className="px-2 py-0.5 bg-red-950/50 text-red-300 rounded border border-red-900/50"
                      >
                        {pat}
                      </span>
                    ))}
                  </div>
                  {ff.remediation && (
                    <div className="w-full text-slate-400 text-xs mt-2 font-mono">
                      <span className="text-emerald-400 font-semibold font-sans">Remediation:</span>{" "}
                      {ff.remediation}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        )}
      </div>

      <div>
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
          <RefreshCw className="w-4 h-4 text-teal-400" />
          SSoT Constant & Version Synchronization
        </h3>
        {!policyConfig?.sync_rules || policyConfig.sync_rules.length === 0 ? (
          <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
            No synchronization rules configured.
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-3">
            {policyConfig.sync_rules.map((sr, idx) => {
              const targets = [
                ...(sr.target_files || []),
                ...(sr.target_file && !sr.target_files?.includes(sr.target_file)
                  ? [sr.target_file]
                  : []),
              ];
              return (
                <div
                  key={idx}
                  className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
                >
                  {renderPolicyRuleHeader(sr.name, sr.severity, sr.description)}
                  <div className="space-y-1.5 text-xs font-mono">
                    <div className="flex items-center gap-2">
                      <span className="text-slate-400">Source:</span>
                      <span className="px-2 py-0.5 bg-slate-800 text-teal-300 rounded border border-slate-700">
                        {sr.source_file}
                      </span>
                      {sr.source_key && (
                        <span className="text-slate-400">
                          key: <code className="text-amber-300">{sr.source_key}</code>
                        </span>
                      )}
                      {sr.source_regex && (
                        <span className="text-slate-400">
                          regex: <code className="text-amber-300">{sr.source_regex}</code>
                        </span>
                      )}
                    </div>
                    <div className="flex items-center gap-2">
                      <span className="text-slate-400">Targets:</span>
                      {targets.map((tgt, tIdx) => (
                        <span
                          key={tIdx}
                          className="px-2 py-0.5 bg-slate-800 text-indigo-300 rounded border border-slate-700"
                        >
                          {tgt}
                        </span>
                      ))}
                      {sr.target_regex && (
                        <span className="text-slate-400">
                          match: <code className="text-amber-300">{sr.target_regex}</code>
                        </span>
                      )}
                    </div>
                  </div>
                  {sr.remediation && (
                    <div className="w-full text-slate-400 text-xs mt-2 font-mono">
                      <span className="text-emerald-400 font-semibold font-sans">Remediation:</span>{" "}
                      {sr.remediation}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        )}
      </div>

      <div>
        <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
          <BookOpen className="w-4 h-4 text-emerald-400" />
          Route Documentation Drift & Living Completeness
        </h3>
        {(!policyConfig?.route_docs || policyConfig.route_docs.length === 0) &&
        !policyConfig?.route_docs_sync ? (
          <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
            No route documentation policies configured.
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-3">
            {[
              ...(policyConfig.route_docs_sync ? [policyConfig.route_docs_sync] : []),
              ...(policyConfig.route_docs || []),
            ].map((rd, idx) => (
              <div
                key={idx}
                className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
              >
                {renderPolicyRuleHeader(
                  rd.name || "route-docs-completeness",
                  rd.severity,
                  rd.remediation || "Ensure discovered routes are documented in Markdown",
                )}
                <div className="flex flex-wrap items-center gap-3 text-xs font-mono">
                  <div className="flex items-center gap-1.5">
                    <span className="text-slate-400">Routes Dir:</span>
                    <span className="px-2 py-0.5 bg-slate-800/80 text-emerald-300 rounded border border-slate-700/60">
                      {rd.routes_dir}
                    </span>
                  </div>
                  <div className="flex items-center gap-1.5">
                    <span className="text-slate-400">Doc Target:</span>
                    <span className="px-2 py-0.5 bg-slate-800/80 text-cyan-300 rounded border border-slate-700/60">
                      {rd.doc_file || rd.doc_files?.join(", ") || "README.md"}
                    </span>
                  </div>
                  {rd.framework && (
                    <div className="flex items-center gap-1.5">
                      <span className="text-slate-400">Framework:</span>
                      <span className="text-purple-300 font-semibold">{rd.framework}</span>
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
