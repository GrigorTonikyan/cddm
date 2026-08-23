import React, { useState } from "react";
import { ClonePair } from "../types/cddm-types";
import { parsePath, FormattedPath } from "../utils/path-utils";
import {
  ChevronDown,
  ChevronRight,
  FileCode2,
  User,
  Hash,
  Copy,
  Check,
  Sparkles,
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

const FilePathSummary: React.FC<FilePathSummaryProps> = ({ parsed, startLine, endLine }) => (
  <div className="flex items-center gap-2 bg-slate-950/90 px-3 py-1.5 rounded-lg border border-slate-800/80 min-w-0 group/item">
    <FileCode2 className="w-4 h-4 shrink-0 text-indigo-400" />
    <div className="min-w-0 flex-1 text-xs font-mono truncate" title={parsed.fullNormalized}>
      <span className="sr-only">{parsed.fullNormalized}</span>
      <span className="text-slate-500 select-none">{parsed.directory}</span>
      <span className="text-slate-100 font-semibold">{parsed.filename}</span>
    </div>
    <span className="shrink-0 text-[11px] font-mono px-2 py-0.5 bg-slate-800/80 text-slate-300 rounded">
      L{startLine}-{endLine}
    </span>
  </div>
);

interface FragmentDetailPanelProps {
  label: string;
  parsed: FormattedPath;
  startLine: number;
  endLine: number;
  tokens: number;
  isCopied: boolean;
  onCopy: () => void;
}

const FragmentDetailPanel: React.FC<FragmentDetailPanelProps> = ({
  label,
  parsed,
  startLine,
  endLine,
  tokens,
  isCopied,
  onCopy,
}) => (
  <div className="bg-slate-900/90 rounded-lg border border-slate-800/90 overflow-hidden">
    <div className="px-3 py-2 bg-slate-950/80 border-b border-slate-800/80 flex items-center justify-between text-xs font-mono">
      <span className="text-indigo-400 font-semibold flex items-center gap-1.5">
        <FileCode2 className="w-3.5 h-3.5" />
        {label}
      </span>
      <div className="flex items-center gap-2">
        <span className="text-slate-400">
          Lines {startLine}–{endLine}
        </span>
        <button
          type="button"
          onClick={(e) => {
            e.stopPropagation();
            onCopy();
          }}
          className="p-1 hover:bg-slate-800 text-slate-400 hover:text-slate-200 rounded transition-colors"
          title="Copy full path"
        >
          {isCopied ? (
            <Check className="w-3.5 h-3.5 text-emerald-400" />
          ) : (
            <Copy className="w-3.5 h-3.5" />
          )}
        </button>
      </div>
    </div>
    <div className="p-3 font-mono text-xs space-y-2">
      <div className="text-slate-300 break-all bg-slate-950 p-2 rounded border border-slate-800/60">
        <span className="text-slate-500">{parsed.directory}</span>
        <span className="text-indigo-300 font-bold">{parsed.filename}</span>
      </div>
      <div className="text-slate-500 text-[11px] flex items-center justify-between pt-1">
        <span>Tokens: {tokens}</span>
        <span>Range: {endLine - startLine + 1} lines</span>
      </div>
    </div>
  </div>
);

export const ClonePairCard: React.FC<ClonePairCardProps> = ({ pair, index }) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [copiedA, setCopiedA] = useState(false);
  const [copiedB, setCopiedB] = useState(false);

  const pathA = parsePath(pair.file_a);
  const pathB = parsePath(pair.file_b);

  const handleCopy = (path: string, isA: boolean) => {
    void navigator.clipboard.writeText(path);
    if (isA) {
      setCopiedA(true);
      setTimeout(() => setCopiedA(false), 2000);
    } else {
      setCopiedB(true);
      setTimeout(() => setCopiedB(false), 2000);
    }
  };

  const simPct = (pair.similarity * 100).toFixed(0);
  const simNum = pair.similarity * 100;

  const simBadgeStyle =
    simNum >= 95
      ? "bg-emerald-500/15 text-emerald-300 border-emerald-500/40 shadow-emerald-900/20"
      : simNum >= 80
        ? "bg-cyan-500/15 text-cyan-300 border-cyan-500/40 shadow-cyan-900/20"
        : "bg-amber-500/15 text-amber-300 border-amber-500/40 shadow-amber-900/20";

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

            <div className="flex items-center gap-4">
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
            </div>
          </div>

          {/* Side-by-Side Detailed Panels */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <FragmentDetailPanel
              label="Fragment A"
              parsed={pathA}
              startLine={pair.start_line_a}
              endLine={pair.end_line_a}
              tokens={pair.token_count}
              isCopied={copiedA}
              onCopy={() => handleCopy(pathA.fullNormalized, true)}
            />
            <FragmentDetailPanel
              label="Fragment B"
              parsed={pathB}
              startLine={pair.start_line_b}
              endLine={pair.end_line_b}
              tokens={pair.token_count}
              isCopied={copiedB}
              onCopy={() => handleCopy(pathB.fullNormalized, false)}
            />
          </div>
        </div>
      )}
    </div>
  );
};
