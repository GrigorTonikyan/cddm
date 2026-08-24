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
    previewRefactorSandbox,
    applyRefactorBranch,
    generateAiPrompt,
  } = useCDDMStore();

  const [customFunctionName, setCustomFunctionName] = useState<string>("");
  const [targetModulePath, setTargetModulePath] = useState<string>("");
  const [branchName, setBranchName] = useState<string>("");
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
    } catch {
      // Handled in store
    }
  }, [sandboxRequest, customFunctionName, targetModulePath, previewRefactorSandbox]);

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
        createDedicatedBranch ? branchName.trim() || undefined : undefined,
        createDedicatedBranch,
      );
      if (res.success) {
        setBranchAppliedSuccess(res.message);
        setTimeout(() => {
          setBranchAppliedSuccess(null);
        }, 5000);
      } else {
        setApplyError(res.message);
      }
    } catch (err) {
      setApplyError(err instanceof Error ? err.message : "Failed to apply refactor patch");
    } finally {
      setIsApplyingBranch(false);
    }
  };

  const handleCopyAiPrompt = async () => {
    if (!sandboxRequest) return;
    setIsGeneratingPrompt(true);
    try {
      const occurrences: AiOccurrenceContext[] = (sandboxRequest.occurrences || []).map((occ) => ({
        path: occ.file,
        span: { line_start: occ.start_line, line_end: occ.end_line, byte_offset: 0 },
        snippet: "",
      }));
      const promptReq: AiRefactorPromptRequest = {
        clone_type: "Renamed",
        similarity: 0.95,
        token_count: 100,
        lines_saved_est: sandboxResult?.total_lines_saved ?? occurrences.length * 10,
        function_name:
          customFunctionName.trim() || sandboxResult?.function_name || "extracted_helper",
        target_module:
          targetModulePath.trim() || sandboxResult?.target_module_path || "src/utils.rs",
        occurrences,
        invariant_body: currentPatch || "",
        parameters: [],
        custom_instructions: undefined,
      };
      const promptText = await generateAiPrompt(promptReq);
      await navigator.clipboard.writeText(promptText);
      setCopiedPrompt(true);
      setTimeout(() => setCopiedPrompt(false), 2000);
    } catch {
      // ignore
    } finally {
      setIsGeneratingPrompt(false);
    }
  };

  const footerContent = (
    <div className="flex items-center justify-between w-full">
      <div className="flex items-center gap-2 text-xs font-mono">
        {branchAppliedSuccess && (
          <span className="text-emerald-400 flex items-center gap-1.5 font-semibold">
            <Check className="w-3.5 h-3.5" />
            {branchAppliedSuccess}
          </span>
        )}
        {applyError && (
          <span className="text-rose-400 flex items-center gap-1.5 font-semibold">
            <AlertCircle className="w-3.5 h-3.5" />
            {applyError}
          </span>
        )}
      </div>
      <div className="flex items-center gap-2">
        <button
          type="button"
          onClick={handleCopyAiPrompt}
          disabled={isGeneratingPrompt}
          className="px-3 py-1.5 rounded-lg bg-purple-950/80 hover:bg-purple-900 border border-purple-700/50 disabled:opacity-50 text-purple-200 font-mono text-xs flex items-center gap-1.5 transition-colors shadow-sm"
        >
          {copiedPrompt ? (
            <Check className="w-3.5 h-3.5 text-emerald-400" />
          ) : (
            <Sparkles className="w-3.5 h-3.5 text-purple-400" />
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

        {/* Unified Patch & Diff Preview */}
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
      </div>
    </Win2xWindow>
  );
};
