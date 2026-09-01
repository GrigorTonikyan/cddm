import React, { useState } from "react";
import { ClonePair } from "../types/cddm-types";
import { DuplicationTreemap } from "./DuplicationTreemap";
import { Win2xWindow } from "./ui/win2x-manager";
import { LayoutGrid, Layers, Search, Filter } from "lucide-react";

export interface TreemapExplorerModalProps {
  isOpen: boolean;
  onClose: () => void;
  clonePairs: ClonePair[];
  totalTokens: number;
  selectedFilterPath?: string;
  onSelectFilterPath?: (path: string) => void;
}

export const TreemapExplorerModal: React.FC<TreemapExplorerModalProps> = ({
  isOpen,
  onClose,
  clonePairs,
  totalTokens,
  selectedFilterPath = "",
  onSelectFilterPath,
}) => {
  const [internalFilter, setInternalFilter] = useState<string>(selectedFilterPath);

  if (!isOpen) return null;

  const handleSelectPath = (path: string) => {
    setInternalFilter(path);
    if (onSelectFilterPath) {
      onSelectFilterPath(path);
    }
  };

  const footerContent = (
    <>
      <div className="flex items-center gap-2 text-xs font-mono text-slate-400">
        <Layers className="w-3.5 h-3.5 text-indigo-400" />
        <span>
          Visualizing {clonePairs.length.toLocaleString()} duplicate fragments across{" "}
          {totalTokens.toLocaleString()} tokens
        </span>
      </div>
      <button
        type="button"
        onClick={onClose}
        className="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold text-xs transition-colors"
      >
        Close
      </button>
    </>
  );

  return (
    <Win2xWindow
      id="cddm-treemap-explorer-window"
      windowType="treemap-explorer"
      isOpen={isOpen}
      onClose={onClose}
      title="Duplication Treemap Explorer"
      subtitle="Interactive Codebase Duplication Distribution & Hierarchy"
      badge={`${clonePairs.length} Clones`}
      icon={<LayoutGrid className="w-4 h-4 text-indigo-400" />}
      footer={footerContent}
      initialWidth={980}
      initialHeight={680}
    >
      <div className="space-y-4">
        {/* Search & Filter Bar */}
        <div className="flex items-center justify-between gap-3 bg-slate-950/80 p-3 rounded-xl border border-slate-800 text-xs font-mono">
          <div className="flex items-center gap-2 flex-1 relative">
            <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
            <input
              id="treemap-filter-input"
              name="treemap_filter"
              aria-label="Filter treemap by directory or file path"
              type="text"
              value={internalFilter}
              onChange={(e) => handleSelectPath(e.target.value)}
              placeholder="Filter treemap by directory or file path..."
              className="w-full bg-slate-900 border border-slate-800 rounded-lg pl-9 pr-3 py-1.5 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 transition-colors"
            />
          </div>

          {internalFilter && (
            <button
              type="button"
              onClick={() => handleSelectPath("")}
              className="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 hover:bg-slate-800 text-slate-300 text-xs flex items-center gap-1.5 transition-colors shrink-0"
            >
              <Filter className="w-3.5 h-3.5 text-indigo-400" />
              <span>Clear Filter</span>
            </button>
          )}
        </div>

        {/* Embedded Duplication Treemap */}
        <DuplicationTreemap
          clonePairs={clonePairs}
          totalTokens={totalTokens}
          selectedFilterPath={internalFilter}
          onSelectFilterPath={handleSelectPath}
        />
      </div>
    </Win2xWindow>
  );
};
