import React, { useState, useEffect } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  Scale,
  Save,
  RotateCcw,
  Check,
  AlertCircle,
  ShieldCheck,
  AlertTriangle,
  Layers,
  FileCode,
  Sliders,
} from "lucide-react";
import { PolicyConfig, PolicyViolation, PolicySeverity } from "../types/cddm-types";

export interface PolicyRulesModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const PolicyRulesModal: React.FC<PolicyRulesModalProps> = ({ isOpen, onClose }) => {
  const {
    policyConfig,
    isPolicyLoading,
    policyError,
    fetchPolicyRules,
    savePolicyRules,
    evaluatePolicyRules,
    results,
  } = useCDDMStore();

  const [activeTab, setActiveTab] = useState<"rules" | "violations" | "editor">("rules");
  const [rawContent, setRawContent] = useState<string>("");
  const [saveSuccess, setSaveSuccess] = useState<boolean>(false);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [evalViolations, setEvalViolations] = useState<PolicyViolation[]>([]);
  const [isEvaluating, setIsEvaluating] = useState<boolean>(false);

  useEffect(() => {
    if (isOpen) {
      void fetchPolicyRules();
    }
  }, [isOpen, fetchPolicyRules]);

  useEffect(() => {
    if (policyConfig) {
      setRawContent(
        policyConfig.raw_toml ||
          `# CDDM Architectural Rules & Boundary Policy Configuration
# Schema Reference: docs/ARCHITECTURE.md

# [[boundaries]]
# name = "domain-isolation"
# description = "Domain core logic must not be duplicated into presentation or infrastructure layers"
# source = "src/domain/**"
# forbidden_targets = ["src/presentation/**", "src/infra/**"]
# severity = "error"

# [[zero_duplication]]
# name = "auth-security-zone"
# description = "Authentication and cryptography modules must have zero code duplication"
# pattern = "src/auth/**"
# severity = "error"

# [[limits]]
# name = "api-cluster-limit"
# description = "API handlers must not exceed 100 duplicate tokens or 3 multi-site occurrences"
# pattern = "src/api/**"
# max_tokens = 100
# max_occurrences = 3
# severity = "warning"
`,
      );
    }
  }, [policyConfig]);

  useEffect(() => {
    if (results?.policy_violations) {
      setEvalViolations(results.policy_violations);
    }
  }, [results]);

  if (!isOpen) return null;

  const handleSave = async () => {
    setSaveError(null);
    setSaveSuccess(false);
    try {
      const updatedConfig: PolicyConfig = {
        boundaries: policyConfig?.boundaries || [],
        zero_duplication: policyConfig?.zero_duplication || [],
        limits: policyConfig?.limits || [],
        raw_toml: rawContent,
      };
      await savePolicyRules(updatedConfig);
      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 3000);
    } catch (err) {
      setSaveError(
        err instanceof Error ? err.message : "Failed to save architectural policy rules",
      );
    }
  };

  const handleEvaluate = async () => {
    setIsEvaluating(true);
    try {
      const res = await evaluatePolicyRules();
      setEvalViolations(res.violations || []);
    } catch (err) {
      setSaveError(err instanceof Error ? err.message : "Failed to evaluate policy rules");
    } finally {
      setIsEvaluating(false);
    }
  };

  const handleResetDefault = () => {
    const defaultTemplate = `# CDDM Architectural Rules & Boundary Policy Configuration
# Schema Reference: docs/ARCHITECTURE.md

[[boundaries]]
name = "domain-isolation"
description = "Domain core logic must not be duplicated into presentation or infrastructure layers"
source = "src/domain/**"
forbidden_targets = ["src/presentation/**", "src/infra/**"]
severity = "error"

[[zero_duplication]]
name = "auth-security-zone"
description = "Authentication and cryptography modules must have zero code duplication"
pattern = "src/auth/**"
severity = "error"

[[limits]]
name = "api-cluster-limit"
description = "API handlers must not exceed 100 duplicate tokens or 3 multi-site occurrences"
pattern = "src/api/**"
max_tokens = 100
max_occurrences = 3
severity = "warning"
`;
    setRawContent(defaultTemplate);
  };

  const renderSeverityBadge = (sev: PolicySeverity) => {
    switch (sev) {
      case "Error":
        return (
          <span className="px-2 py-0.5 text-xs font-mono font-semibold rounded bg-rose-950/70 text-rose-300 border border-rose-800/60">
            [ERROR]
          </span>
        );
      case "Warning":
        return (
          <span className="px-2 py-0.5 text-xs font-mono font-semibold rounded bg-amber-950/70 text-amber-300 border border-amber-800/60">
            [WARN]
          </span>
        );
      case "Info":
        return (
          <span className="px-2 py-0.5 text-xs font-mono font-semibold rounded bg-sky-950/70 text-sky-300 border border-sky-800/60">
            [INFO]
          </span>
        );
    }
  };

  const footerContent = (
    <div className="flex items-center justify-between w-full">
      <div className="flex items-center gap-2 text-xs font-mono">
        {saveSuccess && (
          <span className="text-emerald-400 flex items-center gap-1.5 font-semibold">
            <Check className="w-3.5 h-3.5" />
            Architectural policies saved successfully
          </span>
        )}
        {(saveError || policyError) && (
          <span className="text-rose-400 flex items-center gap-1.5 font-semibold">
            <AlertCircle className="w-3.5 h-3.5" />
            {saveError || policyError}
          </span>
        )}
        {!saveSuccess && !saveError && !policyError && (
          <span className="text-slate-500">
            .cddmrules.toml active — boundary isolation &amp; zero-duplication policies
          </span>
        )}
      </div>

      <div className="flex items-center gap-2">
        <button
          type="button"
          onClick={handleSave}
          disabled={isPolicyLoading}
          className="px-3.5 py-1.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-mono text-xs font-semibold flex items-center gap-1.5 disabled:opacity-50 transition-colors shadow-lg shadow-indigo-900/30"
        >
          <Save className="w-3.5 h-3.5" />
          {isPolicyLoading ? "Saving..." : "Save Policies"}
        </button>
        <button
          type="button"
          onClick={onClose}
          className="px-3.5 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-mono text-xs font-semibold transition-colors"
        >
          Close
        </button>
      </div>
    </div>
  );

  return (
    <Win2xWindow
      id="cddm-policy-rules-modal"
      windowType="policy-rules"
      title="Architectural Boundary & Anti-Duplication Policy Studio"
      subtitle="Boundary Isolation, Zero Duplication & Token Limits"
      badge="Policy Studio"
      icon={<Scale className="w-4 h-4 text-purple-400" />}
      isOpen={isOpen}
      onClose={onClose}
      initialWidth={880}
      initialHeight={640}
      footer={footerContent}
    >
      <div className="flex flex-col h-full bg-slate-950 text-slate-100">
        {/* Navigation Tabs Header */}
        <div className="flex items-center justify-between border-b border-slate-800/80 bg-slate-900/60 px-4 py-2.5">
          <div className="flex items-center gap-1.5">
            <button
              onClick={() => setActiveTab("rules")}
              className={`flex items-center gap-2 px-3 py-1.5 rounded-md text-xs font-medium transition-colors cursor-pointer ${
                activeTab === "rules"
                  ? "bg-purple-950/70 text-purple-300 border border-purple-800/60 shadow-sm"
                  : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
              }`}
            >
              <Layers className="w-3.5 h-3.5" />
              Active Policies (
              {(policyConfig?.boundaries?.length || 0) +
                (policyConfig?.zero_duplication?.length || 0) +
                (policyConfig?.limits?.length || 0)}
              )
            </button>
            <button
              onClick={() => setActiveTab("violations")}
              className={`flex items-center gap-2 px-3 py-1.5 rounded-md text-xs font-medium transition-colors cursor-pointer ${
                activeTab === "violations"
                  ? "bg-purple-950/70 text-purple-300 border border-purple-800/60 shadow-sm"
                  : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
              }`}
            >
              <AlertTriangle className="w-3.5 h-3.5 text-amber-400" />
              Violations Inspector ({evalViolations.length})
            </button>
            <button
              onClick={() => setActiveTab("editor")}
              className={`flex items-center gap-2 px-3 py-1.5 rounded-md text-xs font-medium transition-colors cursor-pointer ${
                activeTab === "editor"
                  ? "bg-purple-950/70 text-purple-300 border border-purple-800/60 shadow-sm"
                  : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
              }`}
            >
              <FileCode className="w-3.5 h-3.5" />
              .cddmrules.toml Editor
            </button>
          </div>
        </div>

        {/* Tab 1: Rules Overview */}
        {activeTab === "rules" && (
          <div className="flex-1 overflow-y-auto p-6 space-y-6">
            <div>
              <h3 className="text-sm font-semibold text-slate-200 flex items-center gap-2 mb-3">
                <Layers className="w-4 h-4 text-indigo-400" />
                Cross-Layer Boundary Isolation Rules ({policyConfig?.boundaries?.length || 0})
              </h3>
              {!policyConfig?.boundaries || policyConfig.boundaries.length === 0 ? (
                <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
                  No architectural boundary isolation rules active. Edit .cddmrules.toml to define
                  layer boundaries.
                </div>
              ) : (
                <div className="grid grid-cols-1 gap-3">
                  {policyConfig.boundaries.map((b, idx) => (
                    <div
                      key={idx}
                      className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
                    >
                      <div className="flex items-center justify-between mb-1.5">
                        <div className="font-semibold text-xs text-slate-200 font-mono flex items-center gap-2">
                          {b.name}
                        </div>
                        {renderSeverityBadge(b.severity)}
                      </div>
                      {b.description && (
                        <p className="text-xs text-slate-400 mb-2">{b.description}</p>
                      )}
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
                Zero Duplication Critical Zones ({policyConfig?.zero_duplication?.length || 0})
              </h3>
              {!policyConfig?.zero_duplication || policyConfig.zero_duplication.length === 0 ? (
                <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
                  No zero-duplication zones configured.
                </div>
              ) : (
                <div className="grid grid-cols-1 gap-3">
                  {policyConfig.zero_duplication.map((z, idx) => (
                    <div
                      key={idx}
                      className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
                    >
                      <div className="flex items-center justify-between mb-1.5">
                        <div className="font-semibold text-xs text-slate-200 font-mono flex items-center gap-2">
                          {z.name}
                        </div>
                        {renderSeverityBadge(z.severity)}
                      </div>
                      {z.description && (
                        <p className="text-xs text-slate-400 mb-2">{z.description}</p>
                      )}
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
                Clone Token &amp; Occurrence Limits ({policyConfig?.limits?.length || 0})
              </h3>
              {!policyConfig?.limits || policyConfig.limits.length === 0 ? (
                <div className="p-4 rounded-lg bg-slate-900/60 border border-slate-800/80 text-xs text-slate-400">
                  No clone limit rules configured.
                </div>
              ) : (
                <div className="grid grid-cols-1 gap-3">
                  {policyConfig.limits.map((l, idx) => (
                    <div
                      key={idx}
                      className="p-3.5 rounded-lg bg-slate-900/70 border border-slate-800 hover:border-slate-700 transition-colors"
                    >
                      <div className="flex items-center justify-between mb-1.5">
                        <div className="font-semibold text-xs text-slate-200 font-mono flex items-center gap-2">
                          {l.name}
                        </div>
                        {renderSeverityBadge(l.severity)}
                      </div>
                      {l.description && (
                        <p className="text-xs text-slate-400 mb-2">{l.description}</p>
                      )}
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
        )}

        {/* Tab 2: Violations Inspector */}
        {activeTab === "violations" && (
          <div className="flex-1 overflow-y-auto p-6 space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-xs font-medium text-slate-300 font-mono">
                Total Detected Violations: {evalViolations.length}
              </span>
              <button
                onClick={handleEvaluate}
                disabled={isEvaluating}
                className="px-3.5 py-1.5 rounded-lg text-xs font-medium bg-indigo-900/60 text-indigo-200 hover:bg-indigo-800/70 border border-indigo-700/50 transition-colors flex items-center gap-1.5 disabled:opacity-50 cursor-pointer"
              >
                <ShieldCheck className="w-3.5 h-3.5" />
                {isEvaluating ? "Evaluating..." : "Run Policy Check"}
              </button>
            </div>

            {evalViolations.length === 0 ? (
              <div className="flex flex-col items-center justify-center h-64 text-center p-6 rounded-xl bg-slate-900/40 border border-slate-800">
                <ShieldCheck className="w-12 h-12 text-emerald-400 mb-3" />
                <h4 className="text-sm font-semibold text-slate-200 mb-1">
                  Zero Architectural Policy Violations
                </h4>
                <p className="text-xs text-slate-400 max-w-md">
                  All cross-layer boundaries, zero-duplication zones, and token limit rules are
                  currently satisfied.
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
                        <span className="font-semibold text-xs font-mono text-slate-100">
                          {v.rule_name}
                        </span>
                        <span className="text-xs px-2 py-0.5 bg-slate-800 rounded text-slate-400 font-mono">
                          {v.rule_type}
                        </span>
                      </div>
                      <span className="text-xs text-slate-400 font-mono">
                        {v.token_count} matching tokens
                      </span>
                    </div>

                    <p className="text-xs text-slate-300">{v.message}</p>

                    <div className="grid grid-cols-1 sm:grid-cols-2 gap-2 text-xs font-mono pt-1">
                      <div className="p-2 rounded bg-slate-950 border border-slate-800/80">
                        <span className="text-slate-500 block text-[10px]">PRIMARY LOCATION:</span>
                        <span className="text-indigo-300 font-medium">
                          {v.file_a}:{v.start_line_a}-{v.end_line_a}
                        </span>
                      </div>
                      {v.file_b && (
                        <div className="p-2 rounded bg-slate-950 border border-slate-800/80">
                          <span className="text-slate-500 block text-[10px]">
                            OFFENDING COUNTERPART:
                          </span>
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
        )}

        {/* Tab 3: Raw TOML Editor */}
        {activeTab === "editor" && (
          <div className="flex-1 flex flex-col p-4 bg-slate-950 overflow-hidden">
            <div className="flex items-center justify-between mb-2">
              <span className="text-xs font-mono text-slate-400">
                Editing: <span className="text-indigo-300 font-semibold">.cddmrules.toml</span>
              </span>
              <div className="flex items-center gap-2">
                <button
                  type="button"
                  onClick={handleResetDefault}
                  className="px-3 py-1 rounded text-xs font-mono bg-slate-800 text-slate-300 hover:bg-slate-700 border border-slate-700 transition-colors flex items-center gap-1.5 cursor-pointer"
                >
                  <RotateCcw className="w-3.5 h-3.5" />
                  Reset Starter Rules
                </button>
                <span className="text-xs text-slate-500 font-mono">
                  TOML Format &bull; Strict Schema Enforced
                </span>
              </div>
            </div>
            <textarea
              value={rawContent}
              onChange={(e) => setRawContent(e.target.value)}
              placeholder="# Enter architectural policy rules in TOML format..."
              spellCheck={false}
              className="flex-1 w-full bg-slate-900 text-slate-200 font-mono text-xs p-4 rounded-lg border border-slate-800 focus:outline-none focus:border-indigo-500 resize-none leading-relaxed"
            />
          </div>
        )}
      </div>
    </Win2xWindow>
  );
};
