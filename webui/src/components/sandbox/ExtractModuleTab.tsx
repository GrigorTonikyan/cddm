import React, { useState } from "react";
import type {
  ExtractRequest,
  ExtractResult,
  ExtractTargetKind,
  RefactorSandboxRequest,
} from "../../types/cddm-types";
import {
  AlertCircle,
  Box,
  Check,
  FileCode,
  FilePlus,
  FolderGit2,
  Play,
  RefreshCw,
  TrendingDown,
} from "lucide-react";

export interface ExtractModuleTabProps {
  sandboxRequest: RefactorSandboxRequest | null;
  extractResult: ExtractResult | null;
  isExtractLoading: boolean;
  extractError: string | null;
  onPreview: (req: ExtractRequest) => Promise<ExtractResult>;
  onApply: (req: ExtractRequest) => Promise<ExtractResult>;
}

export const ExtractModuleTab: React.FC<ExtractModuleTabProps> = ({
  sandboxRequest,
  extractResult,
  isExtractLoading,
  extractError,
  onPreview,
  onApply,
}) => {
  const [targetPath, setTargetPath] = useState<string>("crates/shared_utils");
  const [customFnName, setCustomFnName] = useState<string>("");
  const [strategy, setStrategy] = useState<ExtractTargetKind>("auto");
  const [activeFileTab, setActiveFileTab] = useState<number>(0);
  const [isApplying, setIsApplying] = useState<boolean>(false);
  const [appliedSuccess, setAppliedSuccess] = useState<string | null>(null);

  const handleGeneratePreview = async () => {
    if (!sandboxRequest) return;
    setAppliedSuccess(null);
    const req: ExtractRequest = {
      occurrences: sandboxRequest.occurrences,
      target_path: targetPath.trim() || "crates/shared_utils",
      custom_function_name: customFnName.trim() || undefined,
      target_kind: strategy,
      dry_run: true,
    };
    try {
      await onPreview(req);
    } catch {
      // Handled in store
    }
  };

  const handleApplyToWorkspace = async () => {
    if (!sandboxRequest) return;
    setIsApplying(true);
    setAppliedSuccess(null);
    const req: ExtractRequest = {
      occurrences: sandboxRequest.occurrences,
      target_path: targetPath.trim() || "crates/shared_utils",
      custom_function_name: customFnName.trim() || undefined,
      target_kind: strategy,
      dry_run: false,
    };
    try {
      const res = await onApply(req);
      setAppliedSuccess(res.message);
    } catch {
      // Handled in store
    } finally {
      setIsApplying(false);
    }
  };

  return (
    <div className="space-y-4 font-mono text-xs text-slate-300">
      {/* Configuration Header Card */}
      <div className="p-3.5 bg-slate-900/90 border border-cyan-800/40 rounded-xl space-y-3">
        <div className="flex items-center justify-between">
          <span className="text-cyan-300 font-bold text-xs flex items-center gap-2">
            <Box className="w-4 h-4 text-cyan-400" />
            Automated Shared Crate &amp; Module Extraction
          </span>
          <span className="text-slate-400 text-[11px]">
            Occurrences: {sandboxRequest?.occurrences.length || 0}
          </span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-2.5 pt-1">
          <div>
            <label className="block text-[11px] text-slate-400 mb-1">Target Path / Name</label>
            <input
              type="text"
              aria-label="Target Path"
              value={targetPath}
              onChange={(e) => setTargetPath(e.target.value)}
              placeholder="e.g. crates/shared_utils or src/common/utils.rs"
              className="w-full px-2.5 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 text-xs focus:outline-none focus:border-cyan-500"
            />
          </div>

          <div>
            <label className="block text-[11px] text-slate-400 mb-1">Custom Function Name</label>
            <input
              type="text"
              aria-label="Function Name"
              value={customFnName}
              onChange={(e) => setCustomFnName(e.target.value)}
              placeholder="e.g. compute_shared_total"
              className="w-full px-2.5 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 text-xs focus:outline-none focus:border-cyan-500"
            />
          </div>

          <div>
            <label className="block text-[11px] text-slate-400 mb-1">Packaging Strategy</label>
            <select
              aria-label="Packaging Strategy"
              value={strategy}
              onChange={(e) => setStrategy(e.target.value as ExtractTargetKind)}
              className="w-full px-2.5 py-1.5 bg-slate-950 border border-slate-700 rounded-lg text-slate-200 text-xs focus:outline-none focus:border-cyan-500"
            >
              <option value="auto">Auto-Detect</option>
              <option value="new_crate">New Standalone Crate/Package</option>
              <option value="new_module">New Shared Module File</option>
              <option value="existing_module">Existing Module</option>
            </select>
          </div>
        </div>

        <div className="flex items-center justify-between pt-1 border-t border-slate-800/80">
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={handleGeneratePreview}
              disabled={isExtractLoading || !sandboxRequest}
              className="px-3 py-1.5 bg-cyan-900/60 hover:bg-cyan-800/70 text-cyan-200 border border-cyan-700/60 rounded-lg font-medium text-xs flex items-center gap-1.5 transition-colors disabled:opacity-50"
            >
              <Play className="w-3.5 h-3.5" />
              <span>Preview Extraction Plan</span>
            </button>

            {extractResult && (
              <button
                type="button"
                onClick={handleApplyToWorkspace}
                disabled={isApplying || isExtractLoading}
                className="px-3 py-1.5 bg-emerald-900/60 hover:bg-emerald-800/70 text-emerald-200 border border-emerald-700/60 rounded-lg font-medium text-xs flex items-center gap-1.5 transition-colors disabled:opacity-50"
              >
                {isApplying ? (
                  <RefreshCw className="w-3.5 h-3.5 animate-spin" />
                ) : (
                  <Check className="w-3.5 h-3.5" />
                )}
                <span>Apply to Workspace</span>
              </button>
            )}
          </div>

          {extractResult && (
            <div className="flex items-center gap-2 text-[11px] text-slate-400 font-mono">
              <span className="px-2 py-0.5 bg-emerald-950/60 border border-emerald-800/60 text-emerald-300 rounded flex items-center gap-1">
                <TrendingDown className="w-3 h-3 text-emerald-400" />~
                {extractResult.total_lines_saved} lines saved
              </span>
              <span className="px-2 py-0.5 bg-slate-950 border border-slate-800 text-slate-300 rounded">
                Strategy: {extractResult.target_kind}
              </span>
            </div>
          )}
        </div>
      </div>

      {appliedSuccess && (
        <div className="p-3 bg-emerald-950/40 border border-emerald-800/60 rounded-xl text-emerald-300 text-xs flex items-center gap-2">
          <Check className="w-4 h-4 text-emerald-400 flex-shrink-0" />
          <span>{appliedSuccess}</span>
        </div>
      )}

      {isExtractLoading ? (
        <div className="py-16 flex flex-col items-center justify-center gap-3 text-slate-400 font-mono text-xs bg-slate-950/60 border border-slate-800 rounded-xl">
          <RefreshCw className="w-6 h-6 animate-spin text-cyan-400" />
          <span>Synthesizing shared crate structure and manifest updates...</span>
        </div>
      ) : extractError ? (
        <div className="p-4 bg-rose-950/40 border border-rose-900/60 rounded-xl text-xs font-mono text-rose-300 flex items-center gap-2">
          <AlertCircle className="w-4 h-4 text-rose-400 flex-shrink-0" />
          <span>{extractError}</span>
        </div>
      ) : extractResult ? (
        <div className="space-y-3">
          {/* Target Generated Files Tabs */}
          {extractResult.generated_files.length > 0 && (
            <div className="border border-slate-800 rounded-xl bg-slate-950 overflow-hidden">
              <div className="flex items-center gap-1 px-2.5 py-1.5 bg-slate-900/80 border-b border-slate-800 overflow-x-auto">
                <span className="text-[11px] text-slate-400 font-bold mr-2 flex items-center gap-1">
                  <FilePlus className="w-3.5 h-3.5 text-cyan-400" />
                  Generated Files ({extractResult.generated_files.length}):
                </span>
                {extractResult.generated_files.map((file, fIdx) => (
                  <button
                    key={fIdx}
                    type="button"
                    onClick={() => setActiveFileTab(fIdx)}
                    className={`px-2.5 py-1 rounded text-[11px] font-mono transition-colors ${
                      activeFileTab === fIdx
                        ? "bg-cyan-950/80 text-cyan-300 border border-cyan-800/60 font-semibold"
                        : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/40"
                    }`}
                  >
                    {file.file_path}
                  </button>
                ))}
              </div>

              {extractResult.generated_files[activeFileTab] && (
                <pre className="p-3.5 text-xs font-mono text-slate-200 overflow-x-auto max-h-[180px] leading-relaxed">
                  {extractResult.generated_files[activeFileTab].content}
                </pre>
              )}
            </div>
          )}

          {/* Manifest Updates Preview */}
          {extractResult.manifest_updates.length > 0 && (
            <div className="space-y-2">
              <span className="text-slate-300 font-semibold text-xs flex items-center gap-1.5">
                <FolderGit2 className="w-3.5 h-3.5 text-amber-400" />
                Manifest Updates ({extractResult.manifest_updates.length})
              </span>
              <div className="space-y-2">
                {extractResult.manifest_updates.map((mu, muIdx) => (
                  <div
                    key={muIdx}
                    className="p-3 bg-slate-900/60 border border-amber-900/40 rounded-xl space-y-1.5"
                  >
                    <div className="flex items-center justify-between text-slate-200 text-xs">
                      <span className="font-semibold text-amber-300">{mu.manifest_path}</span>
                      <span className="text-slate-400 text-[10px] font-mono">
                        Added Dependency: {mu.dependency_name}
                      </span>
                    </div>
                    <pre className="p-2.5 bg-slate-950 border border-slate-800 rounded-lg text-[11px] font-mono text-emerald-300 overflow-x-auto">
                      {mu.diff_preview}
                    </pre>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Caller File Rewrites */}
          {extractResult.caller_rewrites.length > 0 && (
            <div className="space-y-2">
              <span className="text-slate-300 font-semibold text-xs flex items-center gap-1.5">
                <FileCode className="w-3.5 h-3.5 text-indigo-400" />
                Occurrence Caller Rewrites ({extractResult.caller_rewrites.length})
              </span>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                {extractResult.caller_rewrites.map((cr, crIdx) => (
                  <div
                    key={crIdx}
                    className="p-2.5 bg-slate-900/60 border border-slate-800 rounded-lg space-y-1 text-[11px]"
                  >
                    <div className="flex items-center justify-between text-slate-200 font-medium">
                      <span className="truncate">{cr.file_path}</span>
                      {cr.injected_import && (
                        <span className="text-cyan-400 font-mono text-[10px] truncate max-w-[160px]">
                          {cr.injected_import}
                        </span>
                      )}
                    </div>
                    <pre className="p-2 bg-slate-950 border border-slate-800 rounded text-[10px] font-mono text-slate-300 overflow-x-auto max-h-[80px]">
                      {cr.diff_patch}
                    </pre>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      ) : (
        <div className="p-8 bg-slate-950/60 border border-slate-800 rounded-xl text-center text-slate-500 font-mono text-xs">
          Click &quot;Preview Extraction Plan&quot; to synthesize shared crate and manifest updates.
        </div>
      )}
    </div>
  );
};
