import React, { useState } from "react";
import { ClonePair } from "../types/cddm-types";
import { parsePath } from "../utils/path-utils";
import { DiffViewer } from "./DiffViewer";
import { RefactorPatchModal } from "./RefactorPatchModal";
import { Win2xWindow } from "./ui/win2x-manager";
import { Columns2, Sparkles, Wand2, Tag, Hash, User, Copy, Check } from "lucide-react";

export interface ClonePairDiffModalProps {
  isOpen: boolean;
  onClose: () => void;
  pair: ClonePair;
  index?: number;
}

export const ClonePairDiffModal: React.FC<ClonePairDiffModalProps> = ({
  isOpen,
  onClose,
  pair,
  index,
}) => {
  const [isRefactorOpen, setIsRefactorOpen] = useState(false);
  const [copiedPaths, setCopiedPaths] = useState(false);

  if (!isOpen) return null;

  const pathA = parsePath(pair.file_a);
  const pathB = parsePath(pair.file_b);

  const simPct = (pair.similarity * 100).toFixed(0);
  const simNum = pair.similarity * 100;

  const simBadgeStyle =
    simNum >= 95
      ? "bg-emerald-500/15 text-emerald-300 border-emerald-500/40"
      : simNum >= 80
        ? "bg-cyan-500/15 text-cyan-300 border-cyan-500/40"
        : "bg-amber-500/15 text-amber-300 border-amber-500/40";

  const handleCopyPaths = () => {
    const text = `${pair.file_a}:${pair.start_line_a}-${pair.end_line_a} <-> ${pair.file_b}:${pair.start_line_b}-${pair.end_line_b}`;
    void navigator.clipboard.writeText(text);
    setCopiedPaths(true);
    setTimeout(() => setCopiedPaths(false), 2000);
  };

  const footerContent = (
    <>
      <div className="flex items-center gap-3 text-xs font-mono text-slate-400">
        <button
          type="button"
          onClick={handleCopyPaths}
          className="px-2.5 py-1 rounded bg-slate-900 border border-slate-800 hover:bg-slate-800 text-slate-300 transition-colors flex items-center gap-1.5"
        >
          {copiedPaths ? (
            <>
              <Check className="w-3.5 h-3.5 text-emerald-400" />
              <span>Copied Paths</span>
            </>
          ) : (
            <>
              <Copy className="w-3.5 h-3.5" />
              <span>Copy File References</span>
            </>
          )}
        </button>
      </div>

      <div className="flex items-center gap-2">
        <button
          type="button"
          onClick={() => setIsRefactorOpen(true)}
          className="px-3 py-1.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs flex items-center gap-1.5 transition-colors shadow-sm"
        >
          <Wand2 className="w-3.5 h-3.5" />
          <span>Launch Refactor Advisor</span>
        </button>
        <button
          type="button"
          onClick={onClose}
          className="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold text-xs transition-colors"
        >
          Close
        </button>
      </div>
    </>
  );

  return (
    <>
      <Win2xWindow
        id={`clone-diff-inspector-${pair.file_a}:${pair.start_line_a}-${pair.end_line_a}_${pair.file_b}:${pair.start_line_b}-${pair.end_line_b}`}
        windowType="clone-diff-inspector"
        isOpen={isOpen}
        onClose={onClose}
        title={
          index !== undefined ? `Clone Pair #${index} Diff Inspector` : "Clone Pair Diff Inspector"
        }
        subtitle={`${pathA.filename}:${pair.start_line_a} <-> ${pathB.filename}:${pair.start_line_b}`}
        badge={`${simPct}% match`}
        icon={<Columns2 className="w-4 h-4 text-indigo-400" />}
        footer={footerContent}
        initialWidth={1020}
        initialHeight={720}
      >
        <div className="space-y-4">
          {/* Metadata Summary Banner */}
          <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 flex flex-wrap items-center justify-between gap-3 text-xs font-mono">
            <div className="flex flex-wrap items-center gap-3">
              <span className="px-2 py-0.5 rounded bg-indigo-950 text-indigo-300 border border-indigo-800/50 flex items-center gap-1">
                <Tag className="w-3 h-3 opacity-70" />
                {pair.clone_type || "Exact"}
              </span>

              <span className="px-2 py-0.5 rounded bg-slate-900 text-slate-300 border border-slate-800">
                {pair.token_count.toLocaleString()} tokens
              </span>

              <span
                className={`px-2 py-0.5 rounded border font-semibold flex items-center gap-1 ${simBadgeStyle}`}
              >
                <Sparkles className="w-3 h-3" />
                {simPct}% similarity
              </span>
            </div>

            <div className="flex flex-wrap items-center gap-3 text-slate-400">
              <div className="flex items-center gap-1.5">
                <Hash className="w-3.5 h-3.5 text-indigo-400" />
                <span className="text-slate-300 truncate max-w-xs" title={pair.fragment_hash}>
                  Hash: {pair.fragment_hash}
                </span>
              </div>

              {pair.author_a && (
                <div className="flex items-center gap-1 bg-slate-900 px-2 py-0.5 rounded border border-slate-800 text-slate-300">
                  <User className="w-3 h-3 text-indigo-400" />
                  <span>Author A: {pair.author_a}</span>
                </div>
              )}

              {pair.author_b && (
                <div className="flex items-center gap-1 bg-slate-900 px-2 py-0.5 rounded border border-slate-800 text-slate-300">
                  <User className="w-3 h-3 text-indigo-400" />
                  <span>Author B: {pair.author_b}</span>
                </div>
              )}
            </div>
          </div>

          {/* Full Interactive Split Diff Viewer */}
          <DiffViewer
            fileA={pair.file_a}
            startLineA={pair.start_line_a}
            endLineA={pair.end_line_a}
            fileB={pair.file_b}
            startLineB={pair.start_line_b}
            endLineB={pair.end_line_b}
            tokenCount={pair.token_count}
          />
        </div>
      </Win2xWindow>

      {/* Concurrent Refactor Patch Modal */}
      <RefactorPatchModal
        isOpen={isRefactorOpen}
        onClose={() => setIsRefactorOpen(false)}
        fileA={pair.file_a}
        startLineA={pair.start_line_a}
        endLineA={pair.end_line_a}
        fileB={pair.file_b}
        startLineB={pair.start_line_b}
        endLineB={pair.end_line_b}
      />
    </>
  );
};
