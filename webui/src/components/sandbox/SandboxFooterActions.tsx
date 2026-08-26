import React from "react";
import {
  AlertCircle,
  Check,
  Copy,
  Download,
  GitBranch,
  Play,
  RefreshCw,
  Sparkles,
} from "lucide-react";

export interface SandboxFooterActionsProps {
  branchAppliedSuccess: string | null;
  applyError: string | null;
  isVerifying: boolean;
  onRunVerification: () => void;
  copiedPrompt: boolean;
  isGeneratingPrompt: boolean;
  onCopyAiPrompt: () => void;
  copiedPatch: boolean;
  hasCurrentPatch: boolean;
  onCopyPatch: () => void;
  downloaded: boolean;
  onDownloadPatch: () => void;
  isApplyingBranch: boolean;
  onApplyToBranch: () => void;
  onClose: () => void;
}

export const SandboxFooterActions: React.FC<SandboxFooterActionsProps> = ({
  branchAppliedSuccess,
  applyError,
  isVerifying,
  onRunVerification,
  copiedPrompt,
  isGeneratingPrompt,
  onCopyAiPrompt,
  copiedPatch,
  hasCurrentPatch,
  onCopyPatch,
  downloaded,
  onDownloadPatch,
  isApplyingBranch,
  onApplyToBranch,
  onClose,
}) => {
  return (
    <div className="flex items-center justify-between w-full font-mono text-xs">
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
          onClick={onRunVerification}
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
          onClick={onCopyAiPrompt}
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
          onClick={onCopyPatch}
          disabled={!hasCurrentPatch}
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
          onClick={onDownloadPatch}
          disabled={!hasCurrentPatch}
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
          onClick={onApplyToBranch}
          disabled={!hasCurrentPatch || isApplyingBranch}
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
};
