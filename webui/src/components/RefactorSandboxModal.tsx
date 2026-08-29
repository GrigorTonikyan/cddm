import React, { useCallback, useEffect, useState } from "react";
import { useCDDMStore } from "../store/cddm-store";
import type {
  AiOccurrenceContext,
  AiRefactorPromptRequest,
  RefactorSandboxRequest,
} from "../types/cddm-types";
import { AstRewritePreview } from "./sandbox/AstRewritePreview";
import { AutoHealTab } from "./sandbox/AutoHealTab";
import { ExtractModuleTab } from "./sandbox/ExtractModuleTab";
import { PatchDiffPreview } from "./sandbox/PatchDiffPreview";
import { SandboxFooterActions } from "./sandbox/SandboxFooterActions";
import { SandboxHeaderControls } from "./sandbox/SandboxHeaderControls";
import { TestVerificationPanel } from "./sandbox/TestVerificationPanel";
import { Win2xWindow } from "./ui/win2x-manager";
import { downloadTextFile } from "../utils/file-download";
import { Bot, Box, FileCode2, Sparkles, TrendingDown } from "lucide-react";

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
    extractResult,
    isExtractLoading,
    extractError,
    previewRefactorSandbox,
    previewAstRefactor,
    verifyRefactorTestSuite,
    applyRefactorBranch,
    generateAiPrompt,
    previewExtractModule,
    applyExtractModule,
  } = useCDDMStore();

  const [activeTab, setActiveTab] = useState<"patch" | "ast" | "heal" | "extract">("patch");
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

  const handleTabChange = async (tab: "patch" | "ast" | "heal" | "extract") => {
    setActiveTab(tab);
    if (tab === "ast" && !astRewriteResult && sandboxRequest) {
      const updatedReq: RefactorSandboxRequest = {
        ...sandboxRequest,
        custom_function_name: customFunctionName.trim() || undefined,
        target_module_path: targetModulePath.trim() || undefined,
      };
      await previewAstRefactor(updatedReq).catch(() => {});
    } else if (tab === "extract" && !extractResult && sandboxRequest) {
      await previewExtractModule({
        occurrences: sandboxRequest.occurrences,
        target_path: targetModulePath.trim() || "crates/shared_utils",
        custom_function_name: customFunctionName.trim() || undefined,
        dry_run: true,
      }).catch(() => {});
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
    const filename = `cddm-refactor-${customFunctionName || "custom"}.patch`;
    downloadTextFile(currentPatch, filename, "text/x-diff;charset=utf-8");
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
    <SandboxFooterActions
      branchAppliedSuccess={branchAppliedSuccess}
      applyError={applyError}
      isVerifying={isVerifying}
      onRunVerification={handleRunVerification}
      copiedPrompt={copiedPrompt}
      isGeneratingPrompt={isGeneratingPrompt}
      onCopyAiPrompt={handleCopyAiPrompt}
      copiedPatch={copiedPatch}
      hasCurrentPatch={Boolean(currentPatch)}
      onCopyPatch={handleCopyPatch}
      downloaded={downloaded}
      onDownloadPatch={handleDownloadPatch}
      isApplyingBranch={isApplyingBranch}
      onApplyToBranch={() => handleApplyToBranch(true)}
      onClose={onClose}
    />
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
        <SandboxHeaderControls
          customFunctionName={customFunctionName}
          onFunctionNameChange={setCustomFunctionName}
          targetModulePath={targetModulePath}
          onTargetModuleChange={setTargetModulePath}
          branchName={branchName}
          onBranchNameChange={setBranchName}
          isSandboxLoading={isSandboxLoading}
          onSimulate={handleSimulate}
        />

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
            <button
              type="button"
              onClick={() => handleTabChange("heal")}
              className={`px-3 py-1.5 rounded-lg text-xs font-semibold flex items-center gap-1.5 transition-colors ${
                activeTab === "heal"
                  ? "bg-emerald-600/30 text-emerald-300 border border-emerald-500/50"
                  : "bg-slate-900/60 text-slate-400 hover:text-slate-200 border border-slate-800"
              }`}
            >
              <Bot className="w-3.5 h-3.5 text-emerald-400" />
              Auto-Heal (AI Surgeon)
            </button>
            <button
              type="button"
              onClick={() => handleTabChange("extract")}
              className={`px-3 py-1.5 rounded-lg text-xs font-semibold flex items-center gap-1.5 transition-colors ${
                activeTab === "extract"
                  ? "bg-cyan-600/30 text-cyan-300 border border-cyan-500/50"
                  : "bg-slate-900/60 text-slate-400 hover:text-slate-200 border border-slate-800"
              }`}
            >
              <Box className="w-3.5 h-3.5 text-cyan-400" />
              Extract Shared Crate/Module
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
          <PatchDiffPreview
            currentPatch={currentPatch}
            isSandboxLoading={isSandboxLoading}
            sandboxError={sandboxError}
          />
        )}

        {/* Tab 2: AST-Native Rewrite Preview */}
        {activeTab === "ast" && (
          <AstRewritePreview
            astRewriteResult={astRewriteResult}
            isAstLoading={isAstLoading}
            astError={astError}
          />
        )}

        {/* Tab 3: AI Code Surgeon Auto-Heal */}
        {activeTab === "heal" && (
          <AutoHealTab
            occurrences={sandboxRequest.occurrences}
            clusterId={sandboxRequest.cluster_id}
            customFunctionName={customFunctionName}
            targetModulePath={targetModulePath}
          />
        )}

        {/* Tab 4: Shared Crate & Module Extraction */}
        {activeTab === "extract" && (
          <ExtractModuleTab
            sandboxRequest={sandboxRequest}
            extractResult={extractResult}
            isExtractLoading={isExtractLoading}
            extractError={extractError}
            onPreview={previewExtractModule}
            onApply={applyExtractModule}
          />
        )}

        {/* Test Verification Panel */}
        <TestVerificationPanel
          showVerifyOutput={showVerifyOutput}
          isVerifying={isVerifying}
          verifyResult={verifyResult}
          verifyError={verifyError}
        />
      </div>
    </Win2xWindow>
  );
};
