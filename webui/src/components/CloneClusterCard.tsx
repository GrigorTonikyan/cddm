import React, { useState } from "react";
import { CloneCluster, CloneLocation } from "../types/cddm-types";
import { parsePath } from "../utils/path-utils";
import { RefactorPatchModal } from "./RefactorPatchModal";
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
  Layers,
  ExternalLink,
} from "lucide-react";

export interface CloneClusterCardProps {
  cluster: CloneCluster;
  index: number;
}

interface LocationItemProps {
  location: CloneLocation;
  idx: number;
}

const LocationItem: React.FC<LocationItemProps> = ({ location, idx }) => {
  const parsed = parsePath(location.file);
  const { preferredEditor } = useCDDMStore();
  const ideLink = getIdeDeeplink(parsed.fullNormalized, location.start_line, preferredEditor);

  return (
    <div className="flex items-center justify-between gap-3 bg-slate-950/80 px-3.5 py-2 rounded-lg border border-slate-800/80 min-w-0">
      <div className="flex items-center gap-2 min-w-0 flex-1">
        <span className="shrink-0 w-5 h-5 rounded bg-slate-800 text-slate-300 font-mono text-[10px] flex items-center justify-center font-bold">
          {idx + 1}
        </span>
        <FileCode2 className="w-4 h-4 shrink-0 text-indigo-400" />
        <div className="min-w-0 flex-1 text-xs font-mono truncate" title={parsed.fullNormalized}>
          <span className="sr-only">{parsed.fullNormalized}</span>
          <span className="text-slate-500 select-none">{parsed.directory}</span>
          <span className="text-slate-100 font-semibold">{parsed.filename}</span>
        </div>
      </div>

      <div className="flex items-center gap-2 shrink-0">
        <a
          href={ideLink}
          onClick={(e) => e.stopPropagation()}
          title={`Open in ${getEditorDisplayName(preferredEditor)} at line ${location.start_line}`}
          className="p-1 text-slate-500 hover:text-indigo-300 hover:bg-slate-800 rounded transition-colors"
        >
          <ExternalLink className="w-3 h-3" />
        </a>
        <span className="text-[11px] font-mono px-2 py-0.5 bg-slate-800/90 text-slate-300 rounded border border-slate-700/50">
          L{location.start_line}-{location.end_line}
        </span>
        {location.author && (
          <span className="hidden sm:flex items-center gap-1 text-[11px] text-slate-400 bg-slate-800/50 px-2 py-0.5 rounded border border-slate-800">
            <User className="w-3 h-3 text-slate-500" />
            <span className="truncate max-w-[100px]">{location.author}</span>
          </span>
        )}
      </div>
    </div>
  );
};

export const CloneClusterCard: React.FC<CloneClusterCardProps> = ({ cluster, index }) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [isRefactorOpen, setIsRefactorOpen] = useState(false);

  const simPct = (cluster.similarity * 100).toFixed(0);
  const simNum = cluster.similarity * 100;

  const simBadgeStyle =
    simNum >= 95
      ? "bg-emerald-500/15 text-emerald-300 border-emerald-500/40 shadow-emerald-900/20"
      : simNum >= 80
        ? "bg-cyan-500/15 text-cyan-300 border-cyan-500/40 shadow-cyan-900/20"
        : "bg-amber-500/15 text-amber-300 border-amber-500/40 shadow-amber-900/20";

  const cloneTypeBadge =
    cluster.clone_type === "Exact"
      ? "bg-emerald-950/80 text-emerald-300 border-emerald-800/50"
      : cluster.clone_type === "Renamed"
        ? "bg-indigo-950/80 text-indigo-300 border-indigo-800/50"
        : cluster.clone_type === "NearMiss"
          ? "bg-amber-950/80 text-amber-300 border-amber-800/50"
          : "bg-purple-950/80 text-purple-300 border-purple-800/50";

  return (
    <div className="group bg-slate-900/70 border border-slate-800/80 hover:border-indigo-500/40 rounded-xl overflow-hidden shadow-lg transition-all duration-200 backdrop-blur-sm">
      {/* Header Bar */}
      <div
        onClick={() => setIsExpanded(!isExpanded)}
        className="p-4 bg-slate-950/60 cursor-pointer flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-slate-800/60 hover:bg-slate-800/40 transition-colors"
      >
        {/* Left: Cluster ID & Primary File Previews */}
        <div className="flex items-center gap-3 min-w-0 flex-1">
          <span className="shrink-0 w-8 h-8 rounded-lg bg-purple-950/90 text-purple-300 font-mono text-xs flex items-center justify-center font-bold border border-purple-800/50">
            #{index}
          </span>

          <div className="flex flex-col min-w-0 flex-1 gap-1">
            <div className="flex items-center gap-2">
              <span className="text-xs font-semibold text-slate-200 flex items-center gap-1.5">
                <Layers className="w-3.5 h-3.5 text-purple-400" />
                Cluster #{cluster.id}
              </span>
              <span className="text-[11px] px-2 py-0.5 rounded-full bg-purple-500/10 text-purple-300 border border-purple-500/20 font-medium">
                {cluster.occurrences.length} Sites
              </span>
            </div>

            <div className="text-xs font-mono text-slate-400 truncate">
              {cluster.occurrences.map((loc) => parsePath(loc.file).filename).join(" <-> ")}
            </div>
          </div>
        </div>

        {/* Right: Badges and Actions */}
        <div className="flex items-center justify-between md:justify-end gap-3 shrink-0">
          <div className="flex items-center gap-2">
            <span
              className={`text-xs px-2.5 py-0.5 rounded-full font-semibold border ${cloneTypeBadge}`}
            >
              {cluster.clone_type}
            </span>

            <span
              className={`text-xs px-2.5 py-0.5 rounded-full font-mono font-semibold border ${simBadgeStyle}`}
            >
              {simPct}%
            </span>
          </div>

          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={(e) => {
                e.stopPropagation();
                setIsRefactorOpen(true);
              }}
              title="Synthesize Multi-Site Refactoring Patch"
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-gradient-to-r from-purple-600/20 to-indigo-600/20 hover:from-purple-600/30 hover:to-indigo-600/30 text-purple-300 border border-purple-500/30 hover:border-purple-500/50 shadow-sm transition-all"
            >
              <Wand2 className="w-3.5 h-3.5" />
              <span>Refactor</span>
            </button>

            <button
              type="button"
              onClick={(e) => {
                e.stopPropagation();
                void useCDDMStore.getState().openRefactorSandbox({
                  cluster_id: cluster.id,
                  occurrences: cluster.occurrences,
                });
              }}
              title="Open in Interactive Refactoring Studio Sandbox"
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-indigo-600/20 hover:bg-indigo-600/30 text-indigo-300 border border-indigo-500/30 hover:border-indigo-500/50 shadow-sm transition-all"
            >
              <Sparkles className="w-3.5 h-3.5" />
              <span>Sandbox</span>
            </button>

            <button
              type="button"
              onClick={(e) => {
                e.stopPropagation();
                setIsExpanded(!isExpanded);
              }}
              aria-label={isExpanded ? "Collapse cluster details" : "Expand cluster details"}
              className="p-1 text-slate-400 hover:text-slate-200 transition-colors"
            >
              {isExpanded ? (
                <ChevronDown className="w-5 h-5" />
              ) : (
                <ChevronRight className="w-5 h-5" />
              )}
            </button>
          </div>
        </div>
      </div>

      {/* Expanded Accordion Body */}
      {isExpanded && (
        <div className="p-4 space-y-4 bg-slate-950/40">
          {/* Metadata Grid */}
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 bg-slate-900/50 p-3 rounded-lg border border-slate-800/60 text-xs">
            <div className="flex items-center gap-2">
              <Sparkles className="w-4 h-4 text-purple-400 shrink-0" />
              <span className="text-slate-400">Tokens:</span>
              <span className="font-mono text-slate-200 font-bold">{cluster.token_count}</span>
            </div>

            <div className="flex items-center gap-2">
              <Layers className="w-4 h-4 text-indigo-400 shrink-0" />
              <span className="text-slate-400">Equivalence:</span>
              <span className="text-slate-200 font-semibold">
                {cluster.occurrences.length} Sites
              </span>
            </div>

            <div className="flex items-center gap-2 col-span-2">
              <Hash className="w-4 h-4 text-slate-500 shrink-0" />
              <span className="text-slate-400">Hash:</span>
              <span className="font-mono text-slate-400 truncate" title={cluster.fragment_hash}>
                {cluster.fragment_hash.slice(0, 16)}...
              </span>
            </div>
          </div>

          {/* All Occurrences Sites List */}
          <div className="space-y-2">
            <h4 className="text-xs font-semibold text-slate-400 uppercase tracking-wider">
              Occurrences in Codebase ({cluster.occurrences.length})
            </h4>
            <div className="space-y-1.5">
              {cluster.occurrences.map((loc, idx) => (
                <LocationItem
                  key={`${loc.file}-${loc.start_line}-${loc.end_line}-${idx}`}
                  location={loc}
                  idx={idx}
                />
              ))}
            </div>
          </div>
        </div>
      )}

      {/* Cluster Multi-Site Refactor Patch Modal */}
      {isRefactorOpen && (
        <RefactorPatchModal
          cluster={cluster}
          isOpen={isRefactorOpen}
          onClose={() => setIsRefactorOpen(false)}
        />
      )}
    </div>
  );
};
