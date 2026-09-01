import React, { useEffect, useState } from "react";
import { useCDDMStore } from "../store/cddm-store";
import type { PolicyConfig, PolicySeverity, PolicyViolation } from "../types/cddm-types";
import { PolicyActiveRulesTab } from "./policy/PolicyActiveRulesTab";
import { PolicyViolationsTab } from "./policy/PolicyViolationsTab";
import { Win2xWindow } from "./ui/win2x-manager";
import { AlertTriangle, Check, FileCode, Play, RotateCcw, Save, ShieldCheck } from "lucide-react";

export interface PolicyRulesModalProps {
  isOpen: boolean;
  onClose: () => void;
}

const DEFAULT_POLICY_TOML_TEMPLATE = `# CDDM Architectural Rules & Boundary Policy Configuration
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
      setRawContent(policyConfig.raw_toml || DEFAULT_POLICY_TOML_TEMPLATE);
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

  const renderPolicyRuleHeader = (name: string, severity: PolicySeverity, description?: string) => (
    <>
      <div className="flex items-center justify-between mb-1.5">
        <div className="font-semibold text-xs text-slate-200 font-mono flex items-center gap-2">
          {name}
        </div>
        {renderSeverityBadge(severity)}
      </div>
      {description && <p className="text-xs text-slate-400 mb-2">{description}</p>}
    </>
  );

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
            <AlertTriangle className="w-3.5 h-3.5" />
            {saveError || policyError}
          </span>
        )}
      </div>
      <div className="flex items-center gap-2">
        <button
          onClick={handleEvaluate}
          disabled={isEvaluating}
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-medium transition-colors border border-slate-700 cursor-pointer disabled:opacity-50"
        >
          <Play className={`w-3.5 h-3.5 ${isEvaluating ? "animate-spin" : ""}`} />
          {isEvaluating ? "Evaluating..." : "Evaluate Now"}
        </button>
        {activeTab === "editor" && (
          <button
            onClick={handleSave}
            disabled={isPolicyLoading}
            className="flex items-center gap-1.5 px-3.5 py-1.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-medium transition-colors shadow-sm cursor-pointer disabled:opacity-50"
          >
            <Save className="w-3.5 h-3.5" />
            {isPolicyLoading ? "Saving..." : "Save Policies"}
          </button>
        )}
      </div>
    </div>
  );

  return (
    <Win2xWindow
      id="cddm-policy-modal"
      title="Architectural Boundary & Anti-Duplication Policy Studio"
      icon={<ShieldCheck className="w-4 h-4 text-purple-400" />}
      isOpen={isOpen}
      onClose={onClose}
      initialWidth={850}
      initialHeight={620}
      footer={footerContent}
    >
      <div className="flex flex-col h-full bg-slate-950/80 text-slate-200">
        <div className="flex items-center justify-between px-6 py-3 border-b border-slate-800 bg-slate-900/40">
          <div className="flex items-center gap-2">
            <button
              onClick={() => setActiveTab("rules")}
              className={`flex items-center gap-2 px-3 py-1.5 rounded-md text-xs font-medium transition-colors cursor-pointer ${
                activeTab === "rules"
                  ? "bg-purple-950/70 text-purple-300 border border-purple-800/60 shadow-sm"
                  : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
              }`}
            >
              <ShieldCheck className="w-3.5 h-3.5 text-purple-400" />
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

        {activeTab === "rules" && (
          <PolicyActiveRulesTab
            policyConfig={policyConfig}
            renderPolicyRuleHeader={renderPolicyRuleHeader}
          />
        )}

        {activeTab === "violations" && (
          <PolicyViolationsTab
            evalViolations={evalViolations}
            isEvaluating={isEvaluating}
            onEvaluate={handleEvaluate}
            renderSeverityBadge={renderSeverityBadge}
          />
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
                  onClick={() => setRawContent(DEFAULT_POLICY_TOML_TEMPLATE)}
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
              id="policy-rules-raw-editor"
              name="policy_rules_raw"
              aria-label="Policy rules TOML configuration"
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
