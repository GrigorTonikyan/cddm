import React from "react";
import { Cpu } from "lucide-react";
import type { CfgNode, ControlFlowGraph, ProgramDependenceGraph } from "../../types/cddm-types";
import {
  computeGraphLayout,
  generateDataEdgePath,
  generateEdgePath,
} from "../../utils/graph-layout";

export interface SingleGraphCardProps {
  cfg: ControlFlowGraph;
  pdg?: ProgramDependenceGraph;
  label: string;
  keyIndex: number;
  showPdgDataEdges: boolean;
  selectedNode: CfgNode | null;
  onSelectNode: (node: CfgNode | null) => void;
}

const getNodeColorClass = (type: string, isSelected: boolean) => {
  const ring = isSelected ? "ring-2 ring-white scale-105" : "";
  switch (type) {
    case "Entry":
      return `bg-emerald-950/90 border-emerald-500 text-emerald-200 ${ring}`;
    case "Exit":
    case "Return":
      return `bg-rose-950/90 border-rose-500 text-rose-200 ${ring}`;
    case "Branch":
      return `bg-amber-950/90 border-amber-500 text-amber-200 ${ring}`;
    case "LoopHeader":
    case "LoopBody":
      return `bg-purple-950/90 border-purple-500 text-purple-200 ${ring}`;
    default:
      return `bg-indigo-950/90 border-indigo-500 text-indigo-200 ${ring}`;
  }
};

export const SingleGraphCard: React.FC<SingleGraphCardProps> = ({
  cfg,
  pdg,
  label,
  keyIndex,
  showPdgDataEdges,
  selectedNode,
  onSelectNode,
}) => {
  const layout = computeGraphLayout(cfg, 360);

  return (
    <div
      key={`single-graph-${keyIndex}-${cfg.function_name}`}
      className="flex-1 min-w-[340px] bg-slate-900/70 border border-slate-800 rounded-xl p-3.5 flex flex-col"
    >
      <div className="flex items-center justify-between border-b border-slate-800 pb-2 mb-3">
        <div className="flex items-center gap-2">
          <Cpu className="w-4 h-4 text-indigo-400" />
          <span className="font-mono font-bold text-xs text-slate-200 truncate">
            {label}: {cfg.function_name}
          </span>
        </div>
        <span className="text-[10px] font-mono bg-indigo-950 text-indigo-300 px-2 py-0.5 rounded border border-indigo-800/50">
          WL: 0x{cfg.wl_hash.toString(16).slice(0, 8)}
        </span>
      </div>

      <div className="w-full overflow-x-auto">
        <svg
          viewBox={`0 0 ${layout.width} ${layout.height}`}
          className="w-full select-none"
          style={{ minHeight: `${layout.height}px` }}
        >
          <defs>
            <marker
              id="arrow-seq"
              viewBox="0 0 10 10"
              refX="6"
              refY="5"
              markerWidth="6"
              markerHeight="6"
              orient="auto-start-reverse"
            >
              <path d="M 0 0 L 10 5 L 0 10 z" fill="#64748b" />
            </marker>
            <marker
              id="arrow-true"
              viewBox="0 0 10 10"
              refX="6"
              refY="5"
              markerWidth="6"
              markerHeight="6"
              orient="auto-start-reverse"
            >
              <path d="M 0 0 L 10 5 L 0 10 z" fill="#10b981" />
            </marker>
            <marker
              id="arrow-false"
              viewBox="0 0 10 10"
              refX="6"
              refY="5"
              markerWidth="6"
              markerHeight="6"
              orient="auto-start-reverse"
            >
              <path d="M 0 0 L 10 5 L 0 10 z" fill="#f43f5e" />
            </marker>
          </defs>

          {/* CFG Control Flow Edges */}
          {cfg.edges.map((edge, idx) => {
            const fromPos = layout.positions.get(edge.from);
            const toPos = layout.positions.get(edge.to);
            if (!fromPos || !toPos) return null;

            const isLoopBack = edge.edge_type === "LoopBack";
            const pathStr = generateEdgePath(fromPos, toPos, isLoopBack);

            let stroke = "#64748b";
            let marker = "url(#arrow-seq)";
            if (edge.edge_type === "TrueBranch") {
              stroke = "#10b981";
              marker = "url(#arrow-true)";
            } else if (edge.edge_type === "FalseBranch") {
              stroke = "#f43f5e";
              marker = "url(#arrow-false)";
            } else if (edge.edge_type === "LoopBack") {
              stroke = "#a855f7";
            }

            return (
              <path
                key={`edge-${idx}`}
                d={pathStr}
                fill="none"
                stroke={stroke}
                strokeWidth={1.8}
                strokeDasharray={isLoopBack ? "4 3" : undefined}
                markerEnd={marker}
                opacity={0.85}
              />
            );
          })}

          {/* PDG Data Dependency Edges (Def-Use Chains) */}
          {showPdgDataEdges &&
            pdg?.data_edges.map((dataEdge, idx) => {
              const fromPos = layout.positions.get(dataEdge.from);
              const toPos = layout.positions.get(dataEdge.to);
              if (!fromPos || !toPos) return null;

              const pathStr = generateDataEdgePath(fromPos, toPos);
              return (
                <g key={`data-edge-${idx}`}>
                  <path
                    d={pathStr}
                    fill="none"
                    stroke="#38bdf8"
                    strokeWidth={1.5}
                    strokeDasharray="3 3"
                    opacity={0.7}
                  />
                  <text
                    x={(fromPos.x + toPos.x) / 2 + 25}
                    y={(fromPos.y + toPos.y) / 2}
                    fill="#38bdf8"
                    fontSize={8}
                    fontFamily="monospace"
                  >
                    var:{dataEdge.variable}
                  </text>
                </g>
              );
            })}

          {/* CFG Graph Nodes */}
          {Array.from(layout.positions.values()).map(({ id, x, y, node }) => {
            const isSelected = selectedNode?.id === node.id;
            const nodeColor = getNodeColorClass(node.node_type, isSelected);

            return (
              <g
                key={`node-${id}`}
                data-node-id={node.id}
                className="cursor-pointer transition-all"
                onClick={() => onSelectNode(node)}
                style={{ cursor: "pointer", pointerEvents: "all" }}
              >
                <rect
                  x={x - 48}
                  y={y - 18}
                  width={96}
                  height={36}
                  rx={6}
                  onClick={() => onSelectNode(node)}
                  className={`border transition-all ${nodeColor}`}
                  strokeWidth={1.5}
                  style={{ pointerEvents: "all" }}
                />
                <text
                  x={x}
                  y={y - 3}
                  fill="#f8fafc"
                  fontSize={9.5}
                  fontWeight="bold"
                  textAnchor="middle"
                  fontFamily="monospace"
                >
                  {node.node_type}
                </text>
                <text
                  x={x}
                  y={y + 10}
                  fill="#cbd5e1"
                  fontSize={7.5}
                  textAnchor="middle"
                  fontFamily="monospace"
                >
                  {node.label.length > 14 ? `${node.label.slice(0, 12)}..` : node.label}
                </text>
              </g>
            );
          })}
        </svg>
      </div>
    </div>
  );
};
