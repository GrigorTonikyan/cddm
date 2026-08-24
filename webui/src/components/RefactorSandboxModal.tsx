import React, { useState, useEffect, useCallback } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  Sparkles,
  GitBranch,
  Play,
  Copy,
  Download,
  Check,
  AlertCircle,
  FileCode2,
  RefreshCw,
  Sliders,
  TrendingDown,
} from "lucide-react";
import {
  AiOccurrenceContext,
  AiRefactorPromptRequest,
  RefactorSandboxRequest,
} from "../types/cddm-types";

export interface RefactorSandboxModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const RefactorSandboxModal: React.FC<RefactorSandboxModalProps> = ({ isOpen, onClose }) => {
  const {
    sandboxRequest,
    sandboxResult,
    isSandboxLoading,
    sandboxError,
    astRewriteResult,
    isAstLoading,
    astError,
    verifyResult,
    isVerifying,
    verifyError,
    previewRefactorSandbox,
    previewAstRefactor,
    verifyRefactorTestSuite,
    applyRefactorBranch,
    generateAiPrompt,
  } = useCDDMStore();

  const [activeTab, setActiveTab] = useState<"patch" | "ast">("patch");
  const [customFunctionName, setCustomFunctionName] = useState<string>("");
  const [targetModulePath, setTargetModulePath] = useState<string>("");
  const [branchName, setBranchName] = useState<string>("");
  const [testCommand, setTestCommand] = useState<string>("");
  const [showVerifyOutput, setShowVerifyOutput] = useState<boolean>(false);
  const [copiedPatch, setCopiedPatch] = useState<boolean>(false);
  const [copiedPrompt, setCopiedPrompt] = useState<boolean>(false);
  const [isGeneratingPrompt, setIsGeneratingPrompt] = useState<boolean>(false);
  const [downloaded, setDownloaded] = useState<boolean>(false);
  const [isApplyingBranch, setIsApplyingBranch] = useState<boolean>(false);
  const [branchAppliedSuccess, setBranchAppliedSuccess] = useState<string | null>(null);
  const [applyError, setApplyError] = useState<string | null>(null);

  useEffect(() => {
    if (sandboxRequest) {
      const defaultClusterId = sandboxRequest.cluster_id || 1;
      setCustomFunctionName(sandboxRequest.custom_function_name || "");
      setTargetModulePath(sandboxRequest.target_module_path || "");
      setBranchName(`cddm/refactor-cluster-${defaultClusterId}`);
    }
  }, [sandboxRequest]);

  useEffect(() => {
    if (sandboxResult) {
      if (!customFunctionName && sandboxResult.function_name) {
        setCustomFunctionName(sandboxResult.function_name);
      }
      if (!targetModulePath && sandboxResult.target_module_path) {
        setTargetModulePath(sandboxResult.target_module_path);
      }
    }
  }, [sandboxResult, customFunctionName, targetModulePath]);

  const handleSimulate = useCallback(async () => {
    if (!sandboxRequest) return;
    setApplyError(null);
    setBranchAppliedSuccess(null);
    const updatedReq: RefactorSandboxRequest = {
      ...sandboxRequest,
      custom_function_name: customFunctionName.trim() || undefined,
      target_module_path: targetModulePath.trim() || undefined,
    };
    try {
      await previewRefactorSandbox(updatedReq);
      if (activeTab === "ast") {
        await previewAstRefactor(updatedReq);
      }
    } catch {
      // Handled in store
    }
  }, [
    sandboxRequest,
    customFunctionName,
    targetModulePath,
    activeTab,
    previewRefactorSandbox,
    previewAstRefactor,
  ]);

  const handleTabChange = async (tab: "patch" | "ast") => {
    setActiveTab(tab);
    if (tab === "ast" && !astRewriteResult && sandboxRequest) {
      const updatedReq: RefactorSandboxRequest = {
        ...sandboxRequest,
        custom_function_name: customFunctionName.trim() || undefined,
        target_module_path: targetModulePath.trim() || undefined,
      };
      await previewAstRefactor(updatedReq).catch(() => {});
    }
  };

  const handleRunVerification = async () => {
    setShowVerifyOutput(true);
    try {
      await verifyRefactorTestSuite({
        directory: ".",
        test_command: testCommand.trim() || undefined,
        branch_name: branchName.trim() || undefined,
      });
    } catch {
      // Handled in store
    }
  };

  if (!isOpen || !sandboxRequest) return null;

  const currentPatch = sandboxResult?.unified_patch || "";

  const handleCopyPatch = async () => {
    if (!currentPatch) return;
    await navigator.clipboard.writeText(currentPatch);
    setCopiedPatch(true);
    setTimeout(() => setCopiedPatch(false), 2000);
  };

  const handleDownloadPatch = () => {
    if (!currentPatch) return;
    const blob = new Blob([currentPatch], { type: "text/x-diff;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    const filename = `cddm-refactor-${customFunctionName || "custom"}.patch`;
    link.download = filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
    setDownloaded(true);
    setTimeout(() => setDownloaded(false), 2000);
  };

  const handleApplyToBranch = async (createDedicatedBranch: boolean) => {
    if (!currentPatch) return;
    setIsApplyingBranch(true);
    setApplyError(null);
    setBranchAppliedSuccess(null);
    try {
      const res = await applyRefactorBranch(
        currentPatch,
        branchName.trim() || undefined,
        createDedicatedBranch,
      );
      if (res.success) {
        setBranchAppliedSuccess(
          res.branch_created
            ? `Successfully created branch '${res.branch_created}' and applied ${res.hunks_applied} hunk(s)`
            : `Successfully applied ${res.hunks_applied} hunk(s) across ${res.modified_files.length} file(s)`,
        );
      }
    } catch (err) {
      setApplyError(err instanceof Error ? err.message : "Branch application failed");
    } finally {
      setIsApplyingBranch(false);
    }
  };

  const handleCopyAiPrompt = async () => {
    if (!sandboxResult || !sandboxRequest) return;
    setIsGeneratingPrompt(true);
    try {
      const occurrences: AiOccurrenceContext[] = (sandboxRequest.occurrences || []).map((occ) => ({
        path: occ.file,
        span: {
          line_start: occ.start_line,
          line_end: occ.end_line,
          byte_offset: 0,
        },
        snippet: "",
      }));

      const promptReq: AiRefactorPromptRequest = {
        clone_type: "Renamed",
        similarity: 0.9,
        token_count: 100,
        lines_saved_est: sandboxResult.total_lines_saved,
        function_name: customFunctionName.trim() || sandboxResult.function_name,
        target_module: targetModulePath.trim() || sandboxResult.target_module_path,
        occurrences,
        invariant_body: "",
        parameters: [],
      };

      const prompt = await generateAiPrompt(promptReq);
      await navigator.clipboard.writeText(prompt);
      setCopiedPrompt(true);
      setTimeout(() => setCopiedPrompt(false), 2000);
    } catch {
      // Handled in store
    } finally {
      setIsGeneratingPrompt(false);
    }
  };

  const footerContent = (
    <div className="flex items-center justify-between w-full">
      <div className="flex items-center gap-3">
        {branchAppliedSuccess && (
          <span className="text-emerald-400 font-mono text-xs flex items-center gap-1.5 bg-emerald-950/40 px-2.5 py-1 rounded border border-emerald-800/40">
            <Check className="w-3.5 h-3.5" />
            {branchAppliedSuccess}
          </span>
        )}
        {applyError && (
          <span className="text-rose-400 font-mono text-xs flex items-center gap-1.5 bg-rose-950/40 px-2.5 py-1 rounded border border-rose-800/40">
            <AlertCircle className="w-3.5 h-3.5" />
            {applyError}
          </span>
        )}
      </div>

      <div className="flex items-center gap-2">
        <button
          type="button"
          onClick={handleRunVerification}
          disabled={isVerifying}
          className="px-3 py-1.5 rounded-lg bg-emerald-950/60 hover:bg-emerald-900/60 border border-emerald-700/50 disabled:opacity-50 text-emerald-300 font-mono text-xs flex items-center gap-1.5 transition-colors"
        >
          {isVerifying ? (
            <RefreshCw className="w-3.5 h-3.5 animate-spin text-emerald-400" />
          ) : (
            <Play className="w-3.5 h-3.5 text-emerald-400" />
          )}
          {isVerifying ? "Verifying..." : "Run Test Verification"}
        </button>
        <button
          type="button"
          onClick={handleCopyAiPrompt}
          disabled={isGeneratingPrompt}
          className="px-3 py-1.5 rounded-lg bg-indigo-950/60 hover:bg-indigo-900/60 border border-indigo-700/50 disabled:opacity-50 text-indigo-300 font-mono text-xs flex items-center gap-1.5 transition-colors"
        >
          {copiedPrompt ? (
            <Check className="w-3.5 h-3.5 text-emerald-400" />
          ) : (
            <Sparkles className="w-3.5 h-3.5 text-indigo-400" />
          )}
          {copiedPrompt ? "Prompt Copied" : isGeneratingPrompt ? "Generating..." : "Copy AI Prompt"}
        </button>
        <button
          type="button"
          onClick={handleCopyPatch}
          disabled={!currentPatch}
          className="px-3 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 disabled:opacity-50 text-slate-200 font-mono text-xs flex items-center gap-1.5 transition-colors"
        >
          {copiedPatch ? (
            <Check className="w-3.5 h-3.5 text-emerald-400" />
          ) : (
            <Copy className="w-3.5 h-3.5" />
          )}
          {copiedPatch ? "Copied" : "Copy Patch"}
        </button>
        <button
          type="button"
          onClick={handleDownloadPatch}
          disabled={!currentPatch}
          className="px-3 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 disabled:opacity-50 text-slate-200 font-mono text-xs flex items-center gap-1.5 transition-colors"
        >
          {downloaded ? (
            <Check className="w-3.5 h-3.5 text-emerald-400" />
          ) : (
            <Download className="w-3.5 h-3.5" />
          )}
          {downloaded ? "Downloaded" : "Download .patch"}
        </button>
        <button
          type="button"
          onClick={() => handleApplyToBranch(true)}
          disabled={!currentPatch || isApplyingBranch}
          className="px-3.5 py-1.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white font-mono text-xs font-semibold flex items-center gap-1.5 transition-colors shadow-lg shadow-indigo-900/30"
        >
          <GitBranch className="w-3.5 h-3.5" />
          Apply to Git Branch
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

  const occCount = sandboxRequest.occurrences?.length || 0;
  const affectedFilesCount =
    sandboxResult?.affected_files?.length ?? sandboxResult?.sites_count ?? occCount;

  return (
    <Win2xWindow
      id="refactor-sandbox-modal"
      windowType="refactor-sandbox"
      isOpen={isOpen}
      onClose={onClose}
      title="Interactive Auto-Refactor Sandbox & Visual Studio"
      subtitle={`Synthesize shared helper across ${occCount} duplicate occurrence sites`}
      badge={`${occCount} Sites`}
      icon={<Sparkles className="w-4 h-4 text-indigo-400" />}
      footer={footerContent}
      initialWidth={960}
      initialHeight={700}
    >
      <div className="space-y-4 font-mono text-xs text-slate-300">
        {/* Sandbox Configuration Controls */}
        <div className="p-3.5 bg-slate-900/80 border border-slate-800 rounded-xl space-y-3">
          <div className="flex items-center justify-between">
            <span className="text-slate-200 font-semibold text-xs flex items-center gap-2">
              <Sliders className="w-3.5 h-3.5 text-indigo-400" />
              Parameterized Refactoring Studio Controls
            </span>
            <button
              type="button"
              onClick={handleSimulate}
              disabled={isSandboxLoading}
              className="px-3 py-1 rounded-lg bg-indigo-600/30 hover:bg-indigo-600/50 text-indigo-300 border border-indigo-500/40 text-xs flex items-center gap-1.5 transition-colors font-semibold"
            >
              {isSandboxLoading ? (
                <RefreshCw className="w-3.5 h-3.5 animate-spin" />
              ) : (
                <Play className="w-3.5 h-3.5" />
              )}
              Re-Simulate Sandbox
            </button>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
            <div className="space-y-1">
              <label className="text-[11px] text-slate-400 block font-medium">
                Extracted Function Name
              </label>
              <input
                type="text"
                value={customFunctionName}
                onChange={(e) => setCustomFunctionName(e.target.value)}
                placeholder="extracted_shared_helper"
                className="w-full bg-slate-950 border border-slate-800 rounded-lg px-2.5 py-1.5 text-xs text-slate-200 font-mono focus:outline-none focus:border-indigo-500/60"
              />
            </div>

            <div className="space-y-1">
              <label className="text-[11px] text-slate-400 block font-medium">
                Destination Module Path (Optional)
              </label>
              <input
                type="text"
                value={targetModulePath}
                onChange={(e) => setTargetModulePath(e.target.value)}
                placeholder="shared_utils.rs / helper.ts"
                className="w-full bg-slate-950 border border-slate-800 rounded-lg px-2.5 py-1.5 text-xs text-slate-200 font-mono focus:outline-none focus:border-indigo-500/60"
              />
            </div>

            <div className="space-y-1">
              <label className="text-[11px] text-slate-400 block font-medium">
                Dedicated Git Branch Name
              </label>
              <input
                type="text"
                value={branchName}
                onChange={(e) => setBranchName(e.target.value)}
                placeholder="cddm/refactor-cluster-1"
                className="w-full bg-slate-950 border border-slate-800 rounded-lg px-2.5 py-1.5 text-xs text-slate-200 font-mono focus:outline-none focus:border-indigo-500/60"
              />
            </div>
          </div>
        </div>

        {/* Metrics Summary Strip */}
        {sandboxResult && (
          <div className="grid grid-cols-3 gap-3">
            <div className="p-3 bg-slate-900/60 border border-slate-800 rounded-xl flex items-center justify-between">
              <span className="text-slate-400 text-xs">Total Lines Saved</span>
              <span className="text-emerald-400 font-bold text-sm flex items-center gap-1">
                <TrendingDown className="w-4 h-4" />+{sandboxResult.total_lines_saved} lines
              </span>
            </div>

            <div className="p-3 bg-slate-900/60 border border-slate-800 rounded-xl flex items-center justify-between">
              <span className="text-slate-400 text-xs">Affected Files</span>
              <span className="text-indigo-300 font-bold text-sm">{affectedFilesCount} files</span>
            </div>

            <div className="p-3 bg-slate-900/60 border border-slate-800 rounded-xl flex items-center justify-between">
              <span className="text-slate-400 text-xs">Extracted Helper</span>
              <span className="text-amber-300 font-bold text-xs truncate max-w-[140px]">
                {sandboxResult.function_name}()
              </span>
            </div>
          </div>
        )}

        {/* View Mode Navigation Tabs */}
        <div className="flex items-center justify-between border-b border-slate-800 pb-2">
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={() => handleTabChange("patch")}
              className={`px-3 py-1.5 rounded-lg text-xs font-semibold flex items-center gap-1.5 transition-colors ${
                activeTab === "patch"
                  ? "bg-indigo-600/30 text-indigo-300 border border-indigo-500/50"
                  : "bg-slate-900/60 text-slate-400 hover:text-slate-200 border border-slate-800"
              }`}
            >
              <FileCode2 className="w-3.5 h-3.5" />
              Unified Patch Diff
            </button>
            <button
              type="button"
              onClick={() => handleTabChange("ast")}
              className={`px-3 py-1.5 rounded-lg text-xs font-semibold flex items-center gap-1.5 transition-colors ${
                activeTab === "ast"
                  ? "bg-purple-600/30 text-purple-300 border border-purple-500/50"
                  : "bg-slate-900/60 text-slate-400 hover:text-slate-200 border border-slate-800"
              }`}
            >
              <Sparkles className="w-3.5 h-3.5" />
              AST-Native Rewrite (Tree-sitter)
              {astRewriteResult?.syntax_valid && (
                <span className="text-[10px] bg-emerald-950/80 text-emerald-400 border border-emerald-800/60 px-1.5 py-0.5 rounded font-mono font-bold">
                  [PASS]
                </span>
              )}
            </button>
          </div>

          <div className="flex items-center gap-2">
            <input
              type="text"
              value={testCommand}
              onChange={(e) => setTestCommand(e.target.value)}
              placeholder="Test command (auto-detect)"
              className="bg-slate-950 border border-slate-800 rounded-lg px-2 py-1 text-[11px] text-slate-200 font-mono w-48 focus:outline-none focus:border-indigo-500/60"
            />
          </div>
        </div>

        {/* Tab 1: Unified Patch & Diff Preview */}
        {activeTab === "patch" && (
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
              <div className="p-8 bg-slate-950/60 border border-slate-800 rounded-xl text-center text-slate-500">
                Click &quot;Re-Simulate Sandbox&quot; to synthesize unified refactoring patch.
              </div>
            )}
          </div>
        )}

        {/* Tab 2: AST-Native Rewrite Preview */}
        {activeTab === "ast" && (
          <div className="space-y-3">
            {isAstLoading ? (
              <div className="py-16 flex flex-col items-center justify-center gap-3 text-slate-400 font-mono text-xs bg-slate-950/60 border border-slate-800 rounded-xl">
                <RefreshCw className="w-6 h-6 animate-spin text-purple-400" />
                <span>Synthesizing Tree-sitter AST refactoring and inferring types...</span>
              </div>
            ) : astError ? (
              <div className="p-4 bg-rose-950/40 border border-rose-900/60 rounded-xl text-xs font-mono text-rose-300 flex items-center gap-2">
                <AlertCircle className="w-4 h-4 text-rose-400 flex-shrink-0" />
                <span>{astError}</span>
              </div>
            ) : astRewriteResult ? (
              <div className="space-y-3">
                {/* Extracted Helper Header */}
                <div className="p-3.5 bg-slate-900/90 border border-purple-800/40 rounded-xl space-y-2">
                  <div className="flex items-center justify-between">
                    <span className="text-purple-300 font-bold text-xs flex items-center gap-2">
                      <Sparkles className="w-3.5 h-3.5 text-purple-400" />
                      Extracted Shared Helper: {astRewriteResult.function_name}()
                    </span>
                    <span className="text-slate-400 text-[11px] font-mono">
                      Target: {astRewriteResult.target_module_path}
                    </span>
                  </div>

                  {astRewriteResult.inferred_parameters.length > 0 && (
                    <div className="flex flex-wrap items-center gap-1.5 pt-1">
                      <span className="text-[11px] text-slate-400 mr-1">Inferred Parameters:</span>
                      {astRewriteResult.inferred_parameters.map((param, pIdx) => (
                        <span
                          key={pIdx}
                          className="px-2 py-0.5 bg-purple-950/60 border border-purple-800/60 text-purple-300 rounded font-mono text-[11px]"
                        >
                          {param.name}:{" "}
                          <span className="text-amber-300">{param.inferred_type}</span>
                        </span>
                      ))}
                    </div>
                  )}
                </div>

                {/* Synthesized Helper Code Block */}
                <div className="border border-slate-800 rounded-xl bg-slate-950 overflow-hidden">
                  <div className="px-3.5 py-2 bg-slate-900/60 border-b border-slate-800 text-[11px] text-slate-400 font-mono">
                    Synthesized Function Implementation
                  </div>
                  <pre className="p-3.5 text-xs font-mono text-slate-200 overflow-x-auto max-h-[160px] leading-relaxed">
                    {astRewriteResult.helper_function_code}
                  </pre>
                </div>

                {/* Rewritten Source Files List */}
                <div className="space-y-2">
                  <span className="text-slate-300 font-semibold text-xs">
                    Transformed Source Files ({astRewriteResult.rewritten_files.length})
                  </span>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                    {astRewriteResult.rewritten_files.map((rf, rfIdx) => (
                      <div
                        key={rfIdx}
                        className="p-2.5 bg-slate-900/60 border border-slate-800 rounded-lg space-y-1 text-[11px]"
                      >
                        <div className="flex items-center justify-between text-slate-200 font-medium">
                          <span className="truncate">{rf.file_path}</span>
                          <span className="text-emerald-400 font-mono text-[10px]">
                            {rf.call_sites_count} call site(s)
                          </span>
                        </div>
                        <div className="text-slate-400 text-[10px] flex items-center justify-between font-mono">
                          <span>
                            {rf.original_line_count} -&gt; {rf.new_line_count} lines
                          </span>
                          {rf.imports_added.length > 0 && (
                            <span className="text-indigo-400">
                              +{rf.imports_added.length} import
                            </span>
                          )}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            ) : (
              <div className="p-8 bg-slate-950/60 border border-slate-800 rounded-xl text-center text-slate-500">
                Click &quot;Re-Simulate Sandbox&quot; to synthesize Tree-sitter AST refactoring.
              </div>
            )}
          </div>
        )}

        {/* Test Verification Panel (Collapsible / Active) */}
        {showVerifyOutput && (
          <div className="p-3.5 bg-slate-900/90 border border-slate-800 rounded-xl space-y-2 font-mono text-xs">
            <div className="flex items-center justify-between">
              <span className="text-slate-200 font-semibold flex items-center gap-2">
                <Play className="w-3.5 h-3.5 text-emerald-400" />
                Test Suite Verification
              </span>
              {isVerifying ? (
                <span className="text-slate-400 flex items-center gap-1.5">
                  <RefreshCw className="w-3 h-3 animate-spin text-emerald-400" />
                  Running test suite...
                </span>
              ) : verifyResult ? (
                <span
                  className={`font-bold px-2 py-0.5 rounded text-[11px] border ${
                    verifyResult.success
                      ? "bg-emerald-950/80 text-emerald-400 border-emerald-800/60"
                      : "bg-rose-950/80 text-rose-400 border-rose-800/60"
                  }`}
                >
                  {verifyResult.success ? "[PASS]" : "[FAIL]"} (exit {verifyResult.exit_code} in{" "}
                  {verifyResult.duration_ms}ms)
                </span>
              ) : null}
            </div>

            {verifyError && (
              <div className="p-2 bg-rose-950/40 border border-rose-900/50 rounded text-rose-300 text-[11px]">
                {verifyError}
              </div>
            )}

            {verifyResult && (
              <div className="space-y-1.5">
                <div className="text-slate-400 text-[11px]">
                  Command:{" "}
                  <span className="text-slate-200 font-semibold">
                    {verifyResult.command_executed}
                  </span>
                </div>
                <div className="p-2.5 bg-slate-950 border border-slate-800 rounded-lg max-h-[140px] overflow-y-auto font-mono text-[11px] text-slate-300 leading-normal">
                  <pre>
                    {verifyResult.stdout_snippet ||
                      verifyResult.stderr_snippet ||
                      verifyResult.message}
                  </pre>
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </Win2xWindow>
  );
};
