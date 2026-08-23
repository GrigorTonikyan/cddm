import React, { useState, useMemo } from "react";
import { ClonePair, TreemapNode, TreemapRect } from "../types/cddm-types";
import { Folder, FileCode, ChevronRight, Filter, Layers, Info } from "lucide-react";

export interface DuplicationTreemapProps {
  clonePairs: ClonePair[];
  totalTokens: number;
  onSelectFilterPath?: (path: string) => void;
  selectedFilterPath?: string;
  className?: string;
}

interface HierarchyRawNode {
  name: string;
  path: string;
  tokens: number;
  clones: number;
  children: Map<string, HierarchyRawNode>;
}

/**
 * Builds a hierarchical directory tree from clone pairs and file paths.
 */
export const buildTreemapHierarchy = (clonePairs: ClonePair[]): TreemapNode => {
  const root: HierarchyRawNode = {
    name: "root",
    path: "",
    tokens: 0,
    clones: 0,
    children: new Map(),
  };

  // Collect duplicate instances and aggregate token counts
  for (const pair of clonePairs) {
    const recordFile = (filePath: string, tokens: number) => {
      const normalized = filePath.replace(/\\/g, "/").replace(/^\.\//, "");
      const segments = normalized.split("/");
      let curr = root;

      let currentPath = "";
      for (let i = 0; i < segments.length; i++) {
        const seg = segments[i];
        if (!seg) continue;
        currentPath = currentPath ? `${currentPath}/${seg}` : seg;

        if (!curr.children.has(seg)) {
          curr.children.set(seg, {
            name: seg,
            path: currentPath,
            tokens: 0,
            clones: 0,
            children: new Map(),
          });
        }
        curr = curr.children.get(seg)!;
        curr.tokens += tokens;
        curr.clones += 1;
      }
    };

    recordFile(pair.file_a, pair.token_count);
    recordFile(pair.file_b, pair.token_count);
  }

  const convertRawToTreemap = (raw: HierarchyRawNode): TreemapNode => {
    const childrenList = Array.from(raw.children.values()).map(convertRawToTreemap);
    const totalChildTokens = childrenList.reduce((sum, c) => sum + c.tokens, 0);
    const totalTokens = Math.max(raw.tokens, totalChildTokens, 1);
    const totalClones = Math.max(
      raw.clones,
      childrenList.reduce((sum, c) => sum + c.clones, 0),
    );

    // Calculate relative duplication density
    const duplicationPercentage = Math.min(
      100,
      Math.max(5, (totalClones * 12) / Math.max(1, childrenList.length || 1)),
    );

    return {
      name: raw.name,
      path: raw.path,
      tokens: totalTokens,
      clones: totalClones,
      duplicationPercentage,
      children: childrenList.length > 0 ? childrenList : undefined,
    };
  };

  return convertRawToTreemap(root);
};

/**
 * Standard Squarified Treemap layout algorithm (Bruls et al.).
 */
export const computeSquarifiedLayout = (
  nodes: TreemapNode[],
  x: number,
  y: number,
  width: number,
  height: number,
): TreemapRect[] => {
  if (nodes.length === 0 || width <= 0 || height <= 0) return [];

  const totalValue = nodes.reduce((sum, n) => sum + n.tokens, 0);
  if (totalValue <= 0) return [];

  const rects: TreemapRect[] = [];
  const sorted = [...nodes].sort((a, b) => b.tokens - a.tokens);

  let remainingNodes = [...sorted];
  let curX = x;
  let curY = y;
  let curW = width;
  let curH = height;

  while (remainingNodes.length > 0) {
    const isHorizontal = curW >= curH;
    const remainingTotal = remainingNodes.reduce((sum, n) => sum + n.tokens, 0);

    const row: TreemapNode[] = [remainingNodes[0]!];
    remainingNodes = remainingNodes.slice(1);

    let rowTotal = row[0]!.tokens;

    // Worst aspect ratio calculation
    const worst = (r: TreemapNode[]) => {
      const s = r.reduce((sum, item) => sum + item.tokens, 0);
      const rArea = (s / remainingTotal) * (curW * curH);
      const rowLen = isHorizontal ? rArea / curH : rArea / curW;
      if (rowLen <= 0) return Infinity;

      let maxAspect = 0;
      for (const item of r) {
        const itemArea = (item.tokens / remainingTotal) * (curW * curH);
        const itemLen = itemArea / rowLen;
        const aspect = Math.max(rowLen / itemLen, itemLen / rowLen);
        if (aspect > maxAspect) maxAspect = aspect;
      }
      return maxAspect;
    };

    while (remainingNodes.length > 0) {
      const next = remainingNodes[0]!;
      const newRow = [...row, next];
      if (worst(newRow) <= worst(row)) {
        row.push(next);
        rowTotal += next.tokens;
        remainingNodes = remainingNodes.slice(1);
      } else {
        break;
      }
    }

    // Lay out current row
    const rowArea = (rowTotal / remainingTotal) * (curW * curH);
    if (isHorizontal) {
      const rowWidth = Math.max(1, rowArea / curH);
      let itemY = curY;
      for (const item of row) {
        const itemHeight = (item.tokens / rowTotal) * curH;
        rects.push({
          x: curX,
          y: itemY,
          width: rowWidth,
          height: itemHeight,
          node: item,
        });
        itemY += itemHeight;
      }
      curX += rowWidth;
      curW -= rowWidth;
    } else {
      const rowHeight = Math.max(1, rowArea / curW);
      let itemX = curX;
      for (const item of row) {
        const itemWidth = (item.tokens / rowTotal) * curW;
        rects.push({
          x: itemX,
          y: curY,
          width: itemWidth,
          height: rowHeight,
          node: item,
        });
        itemX += itemWidth;
      }
      curY += rowHeight;
      curH -= rowHeight;
    }
  }

  return rects;
};

export const DuplicationTreemap: React.FC<DuplicationTreemapProps> = ({
  clonePairs,
  onSelectFilterPath,
  selectedFilterPath,
  className = "",
}) => {
  const [currentPath, setCurrentPath] = useState<string>("");
  const [hoveredNode, setHoveredNode] = useState<TreemapNode | null>(null);

  const fullHierarchy = useMemo(() => {
    return buildTreemapHierarchy(clonePairs);
  }, [clonePairs]);

  // Find the active node for current drilldown path
  const activeNode = useMemo(() => {
    if (!currentPath) return fullHierarchy;
    const segments = currentPath.split("/");
    let curr: TreemapNode | undefined = fullHierarchy;
    for (const seg of segments) {
      if (!curr?.children) break;
      curr = curr.children.find((c) => c.name === seg);
    }
    return curr || fullHierarchy;
  }, [fullHierarchy, currentPath]);

  // Compute SVG layout rectangles
  const layoutRects = useMemo(() => {
    const nodes = activeNode.children || [activeNode];
    return computeSquarifiedLayout(nodes, 0, 0, 800, 360);
  }, [activeNode]);

  // Breadcrumbs navigation
  const breadcrumbs = useMemo(() => {
    if (!currentPath) return [{ name: "Root", path: "" }];
    const segments = currentPath.split("/");
    const crumbs = [{ name: "Root", path: "" }];
    let accum = "";
    for (const seg of segments) {
      accum = accum ? `${accum}/${seg}` : seg;
      crumbs.push({ name: seg, path: accum });
    }
    return crumbs;
  }, [currentPath]);

  const handleNodeClick = (node: TreemapNode) => {
    if (node.children && node.children.length > 0) {
      setCurrentPath(node.path);
    } else if (onSelectFilterPath) {
      onSelectFilterPath(node.path);
    }
  };

  const getNodeColor = (node: TreemapNode) => {
    if (node.clones >= 10 || node.duplicationPercentage > 15) {
      return "fill-rose-500/25 stroke-rose-500/60 hover:fill-rose-500/40 text-rose-300";
    }
    if (node.clones >= 4 || node.duplicationPercentage > 8) {
      return "fill-amber-500/25 stroke-amber-500/60 hover:fill-amber-500/40 text-amber-300";
    }
    return "fill-indigo-500/20 stroke-indigo-500/50 hover:fill-indigo-500/35 text-indigo-300";
  };

  return (
    <div
      className={`bg-slate-900/80 border border-slate-800/80 rounded-xl p-5 shadow-lg space-y-4 ${className}`}
    >
      {/* Treemap Header & Breadcrumbs */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <div>
          <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider flex items-center gap-2">
            <Layers className="w-4 h-4 text-indigo-400" />
            <span>Duplication Treemap Visualizer</span>
          </h3>
          {/* Breadcrumb path */}
          <div className="flex items-center gap-1 text-xs font-mono text-slate-400 mt-1 flex-wrap">
            {breadcrumbs.map((crumb, idx) => (
              <React.Fragment key={crumb.path}>
                {idx > 0 && <ChevronRight className="w-3.5 h-3.5 text-slate-600 shrink-0" />}
                <button
                  type="button"
                  onClick={() => setCurrentPath(crumb.path)}
                  className={`hover:text-indigo-300 transition-colors ${
                    idx === breadcrumbs.length - 1 ? "text-indigo-400 font-bold" : "text-slate-400"
                  }`}
                >
                  {crumb.name}
                </button>
              </React.Fragment>
            ))}
          </div>
        </div>

        {/* Legend */}
        <div className="flex items-center gap-3 text-[11px] font-mono shrink-0">
          <div className="flex items-center gap-1.5 text-slate-400">
            <span className="w-2.5 h-2.5 rounded bg-indigo-500/50 border border-indigo-400/80" />
            <span>Low Duplication</span>
          </div>
          <div className="flex items-center gap-1.5 text-slate-400">
            <span className="w-2.5 h-2.5 rounded bg-amber-500/50 border border-amber-400/80" />
            <span>Moderate</span>
          </div>
          <div className="flex items-center gap-1.5 text-slate-400">
            <span className="w-2.5 h-2.5 rounded bg-rose-500/50 border border-rose-400/80" />
            <span>High Density</span>
          </div>
        </div>
      </div>

      {/* SVG Treemap Canvas */}
      <div className="relative w-full aspect-[20/9] bg-slate-950 rounded-xl border border-slate-800/90 overflow-hidden shadow-inner">
        {layoutRects.length === 0 ? (
          <div className="absolute inset-0 flex flex-col items-center justify-center text-xs font-mono text-slate-500 gap-2">
            <Info className="w-5 h-5 opacity-60" />
            <span>No hierarchical duplication data available for current selection</span>
          </div>
        ) : (
          <svg
            viewBox="0 0 800 360"
            className="w-full h-full select-none"
            preserveAspectRatio="none"
          >
            {layoutRects.map((rect) => {
              const colorCls = getNodeColor(rect.node);
              const isSelected = selectedFilterPath && rect.node.path.includes(selectedFilterPath);

              return (
                <g
                  key={rect.node.path}
                  className="cursor-pointer transition-all duration-150"
                  onClick={() => handleNodeClick(rect.node)}
                  onMouseEnter={() => setHoveredNode(rect.node)}
                  onMouseLeave={() => setHoveredNode(null)}
                >
                  <rect
                    x={rect.x + 1}
                    y={rect.y + 1}
                    width={Math.max(2, rect.width - 2)}
                    height={Math.max(2, rect.height - 2)}
                    rx={4}
                    className={`transition-all duration-200 ${colorCls} ${
                      isSelected ? "stroke-2 stroke-indigo-400 fill-indigo-500/50" : "stroke-1"
                    }`}
                  />
                  {rect.width > 50 && rect.height > 26 && (
                    <text
                      x={rect.x + 6}
                      y={rect.y + 18}
                      className="fill-slate-100 font-mono text-[11px] font-semibold pointer-events-none"
                    >
                      {rect.node.name.length > Math.floor(rect.width / 8)
                        ? `${rect.node.name.slice(0, Math.floor(rect.width / 8) - 1)}…`
                        : rect.node.name}
                    </text>
                  )}
                  {rect.width > 60 && rect.height > 44 && (
                    <text
                      x={rect.x + 6}
                      y={rect.y + 34}
                      className="fill-slate-400 font-mono text-[10px] pointer-events-none opacity-80"
                    >
                      {rect.node.tokens.toLocaleString()} tokens
                    </text>
                  )}
                </g>
              );
            })}
          </svg>
        )}

        {/* Hover Tooltip Overlay */}
        {hoveredNode && (
          <div className="absolute bottom-3 left-3 bg-slate-900/95 border border-slate-700/80 rounded-lg p-2.5 shadow-2xl text-xs font-mono backdrop-blur-md pointer-events-none flex items-center gap-3 z-10">
            <div className="p-1.5 rounded bg-indigo-950 text-indigo-400 border border-indigo-800/50">
              {hoveredNode.children ? (
                <Folder className="w-3.5 h-3.5" />
              ) : (
                <FileCode className="w-3.5 h-3.5" />
              )}
            </div>
            <div>
              <div className="text-slate-100 font-bold">{hoveredNode.path}</div>
              <div className="text-slate-400 text-[11px] flex items-center gap-2 mt-0.5">
                <span>{hoveredNode.tokens.toLocaleString()} tokens</span>
                <span>•</span>
                <span className="text-indigo-300">
                  {hoveredNode.clones} duplicate clone references
                </span>
                {hoveredNode.children && (
                  <>
                    <span>•</span>
                    <span className="text-slate-500">
                      Click to zoom ({hoveredNode.children.length} items)
                    </span>
                  </>
                )}
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Filter Action Bar */}
      {selectedFilterPath && onSelectFilterPath && (
        <div className="flex items-center justify-between p-2.5 bg-indigo-950/40 border border-indigo-800/50 rounded-lg text-xs font-mono">
          <div className="flex items-center gap-2 text-indigo-300">
            <Filter className="w-3.5 h-3.5" />
            <span>
              Active Treemap Filter: <strong className="text-white">{selectedFilterPath}</strong>
            </span>
          </div>
          <button
            type="button"
            onClick={() => onSelectFilterPath("")}
            className="text-slate-400 hover:text-slate-200 text-[11px] underline"
          >
            Clear Filter
          </button>
        </div>
      )}
    </div>
  );
};
