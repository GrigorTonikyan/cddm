import React, { useState } from "react";
import { ClonePair } from "../types/cddm-types";
import { parsePath, FormattedPath } from "../utils/path-utils";
import { DiffViewer } from "./DiffViewer";
import { RefactorPatchModal } from "./RefactorPatchModal";
import { ClonePairDiffModal } from "./ClonePairDiffModal";
import { getIdeDeeplink, getEditorDisplayName } from "../utils/ide-links";
import { useCDDMStore } from "../store/cddm-store";
import {
  ChevronDown,
  ChevronRight,
  FileCode2,
  User,
  Hash,
  Sparkles,
  Wand2,
  Tag,
  Columns2,
  ExternalLink,
  Network,
} from "lucide-react";

export interface ClonePairCardProps {
  pair: ClonePair;
  index: number;
}

interface FilePathSummaryProps {
  parsed: FormattedPath;
  startLine: number;
  endLine: number;
}

const FilePathSummary: React.FC<FilePathSummaryProps> = ({ parsed, startLine, endLine }) => {
  const { preferredEditor } = useCDDMStore();
  const ideLink = getIdeDeeplink(parsed.fullNormalized, startLine, preferredEditor);

  return (
    <div className="flex items-center gap-2 bg-slate-950/90 px-3 py-1.5 rounded-lg border border-slate-800/80 min-w-0 group/item">
      <FileCode2 className="w-4 h-4 shrink-0 text-indigo-400" />
      <div className="min-w-0 flex-1 text-xs font-mono truncate" title={parsed.fullNormalized}>
        <span className="sr-only">{parsed.fullNormalized}</span>
        <span className="text-slate-500 select-none">{parsed.directory}</span>
        <span className="text-slate-100 font-semibold">{parsed.filename}</span>
      </div>
      <a
        href={ideLink}
        onClick={(e) => e.stopPropagation()}
        title={`Open in ${getEditorDisplayName(preferredEditor)} at line ${startLine}`}
        className="shrink-0 p-1 text-slate-500 hover:text-indigo-300 hover:bg-slate-800 rounded transition-colors"
      >
        <ExternalLink className="w-3 h-3" />
      </a>
      <span className="shrink-0 text-[11px] font-mono px-2 py-0.5 bg-slate-800/80 text-slate-300 rounded">
        L{startLine}-{endLine}
      </span>
    </div>
  );
};

export const ClonePairCard: React.FC<ClonePairCardProps> = ({ pair, index }) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [isRefactorOpen, setIsRefactorOpen] = useState(false);
  const [isDiffModalOpen, setIsDiffModalOpen] = useState(false);

  const pathA = parsePath(pair.file_a);
  const pathB = parsePath(pair.file_b);

  const simPct = (pair.similarity * 100).toFixed(0);
  const simNum = pair.similarity * 100;

  const simBadgeStyle =
    simNum >= 95
      ? "bg-emerald-500/15 text-emerald-300 border-emerald-500/40 shadow-emerald-900/20"
      : simNum >= 80
        ? "bg-cyan-500/15 text-cyan-300 border-cyan-500/40 shadow-cyan-900/20"
        : "bg-amber-500/15 text-amber-300 border-amber-500/40 shadow-amber-900/20";

  const cloneTypeBadge =
    pair.clone_type === "Exact"
      ? "bg-emerald-950/80 text-emerald-300 border-emerald-800/50"
      : pair.clone_type === "Renamed"
        ? "bg-indigo-950/80 text-indigo-300 border-indigo-800/50"
        : pair.clone_type === "NearMiss"
          ? "bg-amber-950/80 text-amber-300 border-amber-800/50"
          : "bg-purple-950/80 text-purple-300 border-purple-800/50";

  return (
    <div className="group bg-slate-900/70 border border-slate-800/80 hover:border-indigo-500/40 rounded-xl overflow-hidden shadow-lg transition-all duration-200 backdrop-blur-sm">
      {/* Card Header Header Bar */}
      <div
        onClick={() => setIsExpanded(!isExpanded)}
        className="p-4 bg-slate-950/60 cursor-pointer flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-slate-800/60 hover:bg-slate-800/40 transition-colors"
      >
        {/* Left Side: Index & Split Path Cards */}
        <div className="flex items-center gap-3 min-w-0 flex-1">
          <span className="shrink-0 w-7 h-7 rounded-lg bg-indigo-950/80 text-indigo-300 font-mono text-xs flex items-center justify-center font-bold border border-indigo-800/50">
            #{index}
          </span>

          {/* Files Grid Comparison */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-3 min-w-0 flex-1">
            <FilePathSummary
              parsed={pathA}
              startLine={pair.start_line_a}
              endLine={pair.end_line_a}
            />
            <FilePathSummary
              parsed={pathB}
              startLine={pair.start_line_b}
              endLine={pair.end_line_b}
            />
          </div>
        </div>

        {/* Right Side: Badges & Controls */}
        <div className="flex items-center justify-between md:justify-end gap-3 shrink-0">
          <div className="flex items-center gap-2">
            {/* Clone Type Badge */}
            <span
              className={`text-[11px] font-mono px-2 py-0.5 rounded border flex items-center gap-1 ${cloneTypeBadge}`}
            >
              <Tag className="w-3 h-3 opacity-70" />
              {pair.clone_type || "Exact"}
            </span>

            <span className="text-xs font-mono bg-slate-800/80 text-slate-300 px-2.5 py-1 rounded-md border border-slate-700/50">
              {pair.token_count.toLocaleString()} tokens
            </span>
            <span
              className={`text-xs font-mono px-2.5 py-1 rounded-md border font-semibold flex items-center gap-1 ${simBadgeStyle}`}
            >
              <Sparkles className="w-3 h-3" />
              {simPct}% match
            </span>
          </div>

          <div className="p-1 rounded-lg hover:bg-slate-800 text-slate-400 transition-colors">
            {isExpanded ? (
              <ChevronDown className="w-5 h-5" />
            ) : (
              <ChevronRight className="w-5 h-5" />
            )}
          </div>
        </div>
      </div>

      {/* Expanded Split View Details */}
      {isExpanded && (
        <div className="p-4 bg-slate-950/90 space-y-4 border-t border-slate-800/80">
          {/* Metadata Toolbar */}
          <div className="flex flex-wrap items-center justify-between gap-3 text-xs text-slate-400 pb-3 border-b border-slate-800/60 font-mono">
            <div className="flex items-center gap-2">
              <Hash className="w-3.5 h-3.5 text-indigo-400" />
              <span className="text-slate-300 font-semibold">
                Fragment Hash: {pair.fragment_hash}
              </span>
            </div>

            <div className="flex items-center gap-3">
              {pair.author_a && (
                <div className="flex items-center gap-1.5 bg-slate-900 px-2.5 py-1 rounded border border-slate-800">
                  <User className="w-3.5 h-3.5 text-indigo-400" />
                  <span className="text-slate-300">Author A: {pair.author_a}</span>
                </div>
              )}
              {pair.author_b && (
                <div className="flex items-center gap-1.5 bg-slate-900 px-2.5 py-1 rounded border border-slate-800">
                  <User className="w-3.5 h-3.5 text-indigo-400" />
                  <span className="text-slate-300">Author B: {pair.author_b}</span>
                </div>
              )}
              {/* Diff Inspector Modal Button */}
              <button
                type="button"
                onClick={(e) => {
                  e.stopPropagation();
                  setIsDiffModalOpen(true);
                }}
                className="px-3 py-1 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold text-xs flex items-center gap-1.5 transition-colors border border-slate-700/60 shadow-sm"
              >
                <Columns2 className="w-3.5 h-3.5 text-indigo-400" />
                Diff Inspector
              </button>

              {/* Refactor Advisor Button */}
              <button
                type="button"
                onClick={(e) => {
                  e.stopPropagation();
                  setIsRefactorOpen(true);
                }}
                className="px-3 py-1 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs flex items-center gap-1.5 transition-colors shadow-sm"
              >
                <Wand2 className="w-3.5 h-3.5" />
                Refactor Advisor
              </button>

              {/* Refactor Sandbox Button */}
              <button
                type="button"
                onClick={(e) => {
                  e.stopPropagation();
                  void useCDDMStore.getState().openRefactorSandbox({
                    occurrences: [
                      {
                        file: pair.file_a,
                        start_line: pair.start_line_a,
                        end_line: pair.end_line_a,
                        author: pair.author_a,
                      },
                      {
                        file: pair.file_b,
                        start_line: pair.start_line_b,
                        end_line: pair.end_line_b,
                        author: pair.author_b,
                      },
                    ],
                  });
                }}
                className="px-3 py-1 rounded-lg bg-purple-600/30 hover:bg-purple-600/50 text-purple-200 border border-purple-500/40 font-semibold text-xs flex items-center gap-1.5 transition-colors shadow-sm"
              >
                <Sparkles className="w-3.5 h-3.5 text-purple-300" />
                Sandbox
              </button>

              {/* Semantic Graph Button */}
              <button
                type="button"
                onClick={(e) => {
                  e.stopPropagation();
                  void useCDDMStore.getState().openSemanticGraphModal({
                    file: pair.file_a,
                    file_b: pair.file_b,
                  });
                }}
                className="px-3 py-1 rounded-lg bg-cyan-950/80 hover:bg-cyan-900/60 text-cyan-300 border border-cyan-800/60 font-semibold text-xs flex items-center gap-1.5 transition-colors shadow-sm"
              >
                <Network className="w-3.5 h-3.5 text-cyan-400" />
                Semantic Graph
              </button>
            </div>
          </div>

          {/* Interactive Synchronized Split Diff Viewer */}
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
      )}

      {/* Standalone Diff Inspector Modal */}
      <ClonePairDiffModal
        isOpen={isDiffModalOpen}
        onClose={() => setIsDiffModalOpen(false)}
        pair={pair}
        index={index}
      />

      {/* Refactor Patch Synthesis Modal */}
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
    </div>
  );
};
