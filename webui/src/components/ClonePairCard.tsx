import React, { useState } from "react";
import { ClonePair } from "../types/cddm-types";
import { ChevronDown, ChevronRight, FileCode, User, Hash } from "lucide-react";

/**
 * Props for ClonePairCard component.
 */
export interface ClonePairCardProps {
  /** Clone pair data object */
  pair: ClonePair;
  /** Index number in current scan list */
  index: number;
}

/**
 * Side-by-Side Clone Pair Card component for CDDM WebUI.
 *
 * @param {ClonePairCardProps} props - Component props
 */
export const ClonePairCard: React.FC<ClonePairCardProps> = ({ pair, index }) => {
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <div className="bg-gray-900 border border-gray-800 rounded-xl overflow-hidden shadow-lg transition-all hover:border-gray-700">
      {/* Header Bar */}
      <div
        onClick={() => setIsExpanded(!isExpanded)}
        className="p-4 bg-gray-950/80 cursor-pointer flex items-center justify-between border-b border-gray-800 hover:bg-gray-800/50 transition-colors"
      >
        <div className="flex items-center gap-3">
          <span className="w-6 h-6 rounded-full bg-indigo-950 text-indigo-300 font-mono text-xs flex items-center justify-center font-bold">
            #{index}
          </span>

          <div className="flex items-center gap-2 text-sm font-mono">
            <span className="text-gray-200 font-semibold">{pair.file_a}</span>
            <span className="text-gray-500 font-sans">
              (lines {pair.start_line_a}-{pair.end_line_a})
            </span>
            <span className="text-indigo-400 font-bold px-2">↔</span>
            <span className="text-gray-200 font-semibold">{pair.file_b}</span>
            <span className="text-gray-500 font-sans">
              (lines {pair.start_line_b}-{pair.end_line_b})
            </span>
          </div>
        </div>

        <div className="flex items-center gap-4">
          <span className="text-xs font-mono bg-indigo-950 text-indigo-300 px-2 py-1 rounded">
            {pair.token_count} tokens
          </span>
          <span className="text-xs font-mono bg-emerald-950 text-emerald-300 px-2 py-1 rounded">
            {(pair.similarity * 100).toFixed(0)}% match
          </span>

          {isExpanded ? (
            <ChevronDown className="w-5 h-5 text-gray-400" />
          ) : (
            <ChevronRight className="w-5 h-5 text-gray-400" />
          )}
        </div>
      </div>

      {/* Expanded Split View Details */}
      {isExpanded && (
        <div className="p-4 bg-gray-950 space-y-4">
          {/* Metadata Row */}
          <div className="flex items-center justify-between text-xs text-gray-400 border-b border-gray-800 pb-3 font-mono">
            <div className="flex items-center gap-2">
              <Hash className="w-3.5 h-3.5 text-indigo-400" />
              <span>Fragment Hash: {pair.fragment_hash}</span>
            </div>

            <div className="flex items-center gap-4">
              {pair.author_a && (
                <div className="flex items-center gap-1">
                  <User className="w-3.5 h-3.5 text-indigo-400" />
                  <span>Author A: {pair.author_a}</span>
                </div>
              )}
              {pair.author_b && (
                <div className="flex items-center gap-1">
                  <User className="w-3.5 h-3.5 text-indigo-400" />
                  <span>Author B: {pair.author_b}</span>
                </div>
              )}
            </div>
          </div>

          {/* Side by Side File Spec */}
          <div className="grid grid-cols-2 gap-4">
            <div className="bg-gray-900 p-3 rounded-lg border border-gray-800 font-mono text-xs">
              <div className="flex items-center justify-between text-indigo-300 font-bold mb-2 pb-1 border-b border-gray-800">
                <span className="flex items-center gap-1.5">
                  <FileCode className="w-4 h-4" />
                  Fragment A
                </span>
                <span>
                  L{pair.start_line_a} - L{pair.end_line_a}
                </span>
              </div>
              <div className="text-gray-400">Path: {pair.file_a}</div>
            </div>

            <div className="bg-gray-900 p-3 rounded-lg border border-gray-800 font-mono text-xs">
              <div className="flex items-center justify-between text-indigo-300 font-bold mb-2 pb-1 border-b border-gray-800">
                <span className="flex items-center gap-1.5">
                  <FileCode className="w-4 h-4" />
                  Fragment B
                </span>
                <span>
                  L{pair.start_line_b} - L{pair.end_line_b}
                </span>
              </div>
              <div className="text-gray-400">Path: {pair.file_b}</div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
