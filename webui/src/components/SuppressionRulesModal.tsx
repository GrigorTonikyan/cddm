import React, { useState, useEffect } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  ShieldAlert,
  Save,
  RotateCcw,
  Check,
  Filter,
  FileCode,
  FileText,
  AlertCircle,
  HelpCircle,
  Hash,
} from "lucide-react";
import { SuppressionConfig } from "../types/cddm-types";

export interface SuppressionRulesModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const SuppressionRulesModal: React.FC<SuppressionRulesModalProps> = ({
  isOpen,
  onClose,
}) => {
  const {
    suppressionConfig,
    isSuppressionLoading,
    suppressionError,
    fetchSuppressionRules,
    saveSuppressionRules,
  } = useCDDMStore();

  const [activeTab, setActiveTab] = useState<"rules" | "editor" | "directives">("rules");
  const [ignoreTests, setIgnoreTests] = useState<boolean>(false);
  const [ignoreMocks, setIgnoreMocks] = useState<boolean>(false);
  const [ignoreGenerated, setIgnoreGenerated] = useState<boolean>(true);
  const [rawContent, setRawContent] = useState<string>("");
  const [saveSuccess, setSaveSuccess] = useState<boolean>(false);
  const [saveError, setSaveError] = useState<string | null>(null);

  useEffect(() => {
    if (isOpen) {
      void fetchSuppressionRules();
    }
  }, [isOpen, fetchSuppressionRules]);

  useEffect(() => {
    if (suppressionConfig) {
      setIgnoreTests(suppressionConfig.ignore_tests);
      setIgnoreMocks(suppressionConfig.ignore_mocks);
      setIgnoreGenerated(suppressionConfig.ignore_generated);
      setRawContent(
        suppressionConfig.raw_cddmignore ||
          "# .cddmignore — CDDM Code De-Duplication Meister suppression patterns\n\n# Ignore test files\n**/tests/**\n**/*_test.rs\n**/*.test.ts\n**/*.spec.ts\n\n# Ignore mock files\n**/mocks/**\n**/*_mock.rs\n\n# Ignore generated code\n**/generated/**\n**/*.generated.*\n",
      );
    }
  }, [suppressionConfig]);

  if (!isOpen) return null;

  const handleSave = async () => {
    setSaveError(null);
    setSaveSuccess(false);
    try {
      const updatedConfig: SuppressionConfig = {
        rules: suppressionConfig?.rules || [],
        ignore_tests: ignoreTests,
        ignore_mocks: ignoreMocks,
        ignore_generated: ignoreGenerated,
        raw_cddmignore: rawContent,
      };
      await saveSuppressionRules(updatedConfig);
      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 3000);
    } catch (err) {
      setSaveError(err instanceof Error ? err.message : "Failed to save suppression configuration");
    }
  };

  const handleResetDefault = () => {
    const defaultTemplate =
      "# .cddmignore — CDDM Code De-Duplication Meister suppression rules\n\n# Ignore generated code\n**/generated/**\n**/*.generated.*\n**/*.pb.go\n**/*.pb.rs\n\n# Ignore test suites and mocks\n**/tests/**\n**/*_test.*\n**/*.spec.*\n**/*.test.*\n**/mocks/**\n**/fixtures/**\n\n# Per-path minimum token threshold override\n# [threshold] legacy/services/** min_tokens=120\n\n# Per-path clone type filtering\n# [type-filter] vendor/** ignore=Exact,Renamed\n";
    setRawContent(defaultTemplate);
  };

  const footerContent = (
    <div className="flex items-center justify-between w-full">
      <div className="flex items-center gap-2 text-xs font-mono">
        {saveSuccess && (
          <span className="text-emerald-400 flex items-center gap-1.5 font-semibold">
            <Check className="w-3.5 h-3.5" />
            Suppression rules saved successfully
          </span>
        )}
        {(saveError || suppressionError) && (
          <span className="text-rose-400 flex items-center gap-1.5 font-semibold">
            <AlertCircle className="w-3.5 h-3.5" />
            {saveError || suppressionError}
          </span>
        )}
      </div>
      <div className="flex items-center gap-2">
        <button
          type="button"
          onClick={handleSave}
          disabled={isSuppressionLoading}
          className="px-3.5 py-1.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white font-mono text-xs font-semibold flex items-center gap-1.5 transition-colors shadow-lg shadow-indigo-900/30"
        >
          <Save className="w-3.5 h-3.5" />
          Save Rules
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
      id="suppression-rules-modal"
      windowType="suppression-rules"
      isOpen={isOpen}
      onClose={onClose}
      title="Intelligent AST Suppression & .cddmignore Engine"
      subtitle="Glob Rules, Threshold Overrides & Inline Directives"
      badge="Suppression Rules"
      icon={<ShieldAlert className="w-4 h-4 text-indigo-400" />}
      footer={footerContent}
      initialWidth={860}
      initialHeight={620}
    >
      <div className="space-y-4 font-mono text-xs text-slate-300">
        {/* Navigation Tabs */}
        <div className="flex items-center gap-1 border-b border-slate-800 pb-2">
          <button
            type="button"
            onClick={() => setActiveTab("rules")}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 text-xs transition-colors ${
              activeTab === "rules"
                ? "bg-indigo-600/20 text-indigo-300 border border-indigo-500/40"
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
            }`}
          >
            <Filter className="w-3.5 h-3.5" />
            Category & Path Rules
          </button>
          <button
            type="button"
            onClick={() => setActiveTab("editor")}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 text-xs transition-colors ${
              activeTab === "editor"
                ? "bg-indigo-600/20 text-indigo-300 border border-indigo-500/40"
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
            }`}
          >
            <FileText className="w-3.5 h-3.5" />
            .cddmignore Editor
          </button>
          <button
            type="button"
            onClick={() => setActiveTab("directives")}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 text-xs transition-colors ${
              activeTab === "directives"
                ? "bg-indigo-600/20 text-indigo-300 border border-indigo-500/40"
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
            }`}
          >
            <HelpCircle className="w-3.5 h-3.5" />
            Inline Directives Guide
          </button>
        </div>

        {/* Tab 1: Category & Path Rules */}
        {activeTab === "rules" && (
          <div className="space-y-4">
            <div className="p-3.5 bg-slate-900/80 border border-slate-800 rounded-xl space-y-3">
              <span className="text-slate-200 font-semibold text-xs flex items-center gap-2">
                <Filter className="w-3.5 h-3.5 text-indigo-400" />
                Automatic Category Filters
              </span>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
                <label
                  htmlFor="suppression-ignore-tests"
                  className="flex items-center gap-2.5 p-2.5 rounded-lg bg-slate-950/60 border border-slate-800/80 cursor-pointer hover:border-slate-700 transition-colors"
                >
                  <input
                    id="suppression-ignore-tests"
                    name="ignore_tests"
                    aria-label="Ignore Tests (tests/, *_test.*, *.spec.*)"
                    type="checkbox"
                    checked={ignoreTests}
                    onChange={(e) => setIgnoreTests(e.target.checked)}
                    className="rounded border-slate-700 text-indigo-500 focus:ring-indigo-500 bg-slate-900"
                  />
                  <div>
                    <span className="text-slate-200 text-xs font-semibold block">Ignore Tests</span>
                    <span className="text-[10px] text-slate-500">tests/, *_test.*, *.spec.*</span>
                  </div>
                </label>

                <label
                  htmlFor="suppression-ignore-mocks"
                  className="flex items-center gap-2.5 p-2.5 rounded-lg bg-slate-950/60 border border-slate-800/80 cursor-pointer hover:border-slate-700 transition-colors"
                >
                  <input
                    id="suppression-ignore-mocks"
                    name="ignore_mocks"
                    aria-label="Ignore Mocks (mocks/, fixtures/, stubs/)"
                    type="checkbox"
                    checked={ignoreMocks}
                    onChange={(e) => setIgnoreMocks(e.target.checked)}
                    className="rounded border-slate-700 text-indigo-500 focus:ring-indigo-500 bg-slate-900"
                  />
                  <div>
                    <span className="text-slate-200 text-xs font-semibold block">Ignore Mocks</span>
                    <span className="text-[10px] text-slate-500">mocks/, fixtures/, stubs/</span>
                  </div>
                </label>

                <label
                  htmlFor="suppression-ignore-generated"
                  className="flex items-center gap-2.5 p-2.5 rounded-lg bg-slate-950/60 border border-slate-800/80 cursor-pointer hover:border-slate-700 transition-colors"
                >
                  <input
                    id="suppression-ignore-generated"
                    name="ignore_generated"
                    aria-label="Ignore Generated (@generated, DO NOT EDIT)"
                    type="checkbox"
                    checked={ignoreGenerated}
                    onChange={(e) => setIgnoreGenerated(e.target.checked)}
                    className="rounded border-slate-700 text-indigo-500 focus:ring-indigo-500 bg-slate-900"
                  />
                  <div>
                    <span className="text-slate-200 text-xs font-semibold block">
                      Ignore Generated
                    </span>
                    <span className="text-[10px] text-slate-500">@generated, DO NOT EDIT</span>
                  </div>
                </label>
              </div>
            </div>

            {/* Active Parsed Rules Table */}
            <div className="space-y-2">
              <span className="text-slate-200 font-semibold text-xs flex items-center gap-2">
                <FileCode className="w-3.5 h-3.5 text-indigo-400" />
                Active Suppression Rules ({suppressionConfig?.rules.length || 0})
              </span>
              {suppressionConfig?.rules && suppressionConfig.rules.length > 0 ? (
                <div className="overflow-x-auto border border-slate-800 rounded-xl bg-slate-950/60">
                  <table className="w-full text-left text-xs">
                    <thead>
                      <tr className="border-b border-slate-800 bg-slate-900/60 text-slate-400">
                        <th className="py-2 px-3 font-semibold">Pattern</th>
                        <th className="py-2 px-3 font-semibold">Min Tokens Override</th>
                        <th className="py-2 px-3 font-semibold">Ignored Clone Types</th>
                        <th className="py-2 px-3 font-semibold">Comment</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-slate-800/60 text-slate-300">
                      {suppressionConfig.rules.map((rule, idx) => (
                        <tr key={idx} className="hover:bg-slate-900/40 transition-colors">
                          <td className="py-2 px-3 font-mono text-indigo-300">{rule.pattern}</td>
                          <td className="py-2 px-3 font-mono text-amber-400">
                            {rule.min_tokens_override ? `${rule.min_tokens_override} tokens` : "—"}
                          </td>
                          <td className="py-2 px-3 font-mono text-sky-300">
                            {rule.ignored_clone_types && rule.ignored_clone_types.length > 0
                              ? rule.ignored_clone_types.join(", ")
                              : "All Types"}
                          </td>
                          <td className="py-2 px-3 text-slate-500 italic">{rule.comment || "—"}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              ) : (
                <div className="p-4 bg-slate-900/40 border border-slate-800/60 rounded-xl text-center text-slate-500">
                  No custom path rules parsed. Edit .cddmignore to add custom glob rules.
                </div>
              )}
            </div>
          </div>
        )}

        {/* Tab 2: Raw .cddmignore Editor */}
        {activeTab === "editor" && (
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <label
                htmlFor="suppression-raw-editor"
                className="text-slate-400 text-xs flex items-center gap-1.5"
              >
                <Hash className="w-3.5 h-3.5 text-indigo-400" />
                Root workspace configuration (<code className="text-slate-200">.cddmignore</code>)
              </label>
              <button
                type="button"
                onClick={handleResetDefault}
                className="text-xs text-indigo-400 hover:text-indigo-300 flex items-center gap-1 transition-colors"
              >
                <RotateCcw className="w-3 h-3" />
                Reset Template
              </button>
            </div>
            <textarea
              id="suppression-raw-editor"
              name="suppression_raw_content"
              aria-label="Raw .cddmignore glob configuration"
              value={rawContent}
              onChange={(e) => setRawContent(e.target.value)}
              rows={16}
              className="w-full bg-slate-950 border border-slate-800 rounded-xl p-3.5 text-xs font-mono text-slate-200 focus:outline-none focus:border-indigo-500/60 resize-none"
              placeholder="# Enter glob patterns to suppress..."
              spellCheck={false}
            />
          </div>
        )}

        {/* Tab 3: Inline Directives Guide */}
        {activeTab === "directives" && (
          <div className="space-y-3">
            <div className="p-3 bg-slate-900/60 border border-slate-800 rounded-xl space-y-2">
              <span className="text-indigo-300 font-semibold text-xs block">
                Single-Line Suppression Directive
              </span>
              <p className="text-slate-400 text-xs">
                Add an inline comment directly before or on the duplicated line:
              </p>
              <pre className="p-2.5 bg-slate-950 border border-slate-800 rounded-lg text-emerald-300 text-xs overflow-x-auto">
                {`// cddm:ignore [optional reason]\nfn duplicated_helper() {\n    // ...\n}`}
              </pre>
            </div>

            <div className="p-3 bg-slate-900/60 border border-slate-800 rounded-xl space-y-2">
              <span className="text-indigo-300 font-semibold text-xs block">
                Block-Level Suppression Directives
              </span>
              <p className="text-slate-400 text-xs">
                Surround multi-line code regions with start/end directives:
              </p>
              <pre className="p-2.5 bg-slate-950 border border-slate-800 rounded-lg text-emerald-300 text-xs overflow-x-auto">
                {`/* cddm:ignore-start */\n// Code block completely ignored during AST clone tokenization\nexport function complexMatrixMultiplication() {\n    // ...\n}\n/* cddm:ignore-end */`}
              </pre>
            </div>

            <div className="p-3 bg-slate-900/60 border border-slate-800 rounded-xl space-y-2">
              <span className="text-indigo-300 font-semibold text-xs block">
                Rust Attribute & Python Annotation
              </span>
              <pre className="p-2.5 bg-slate-950 border border-slate-800 rounded-lg text-emerald-300 text-xs overflow-x-auto">
                {`#[cddm(allow_duplication)]\nfn legacy_routine() {\n    // ...\n}\n\n# @cddm_ignore\ndef legacy_python_routine():\n    pass`}
              </pre>
            </div>
          </div>
        )}
      </div>
    </Win2xWindow>
  );
};
