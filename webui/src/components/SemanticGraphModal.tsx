import React, { useState } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xWindow } from "./ui/win2x-manager";
import { Network, Cpu, CheckCircle2, AlertTriangle, Play, Code2, FileCode2 } from "lucide-react";
import type { CfgNode, ControlFlowGraph, ProgramDependenceGraph } from "../types/cddm-types";
import { computeGraphLayout, generateDataEdgePath, generateEdgePath } from "../utils/graph-layout";

export interface SemanticGraphModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const SemanticGraphModal: React.FC<SemanticGraphModalProps> = ({ isOpen, onClose }) => {
  const { semanticGraphResponse, isSemanticGraphLoading, semanticGraphError, fetchSemanticGraph } =
    useCDDMStore();

  const [activeTab, setActiveTab] = useState<"visualizer" | "sandbox">("visualizer");
  const [showPdgDataEdges, setShowPdgDataEdges] = useState<boolean>(true);
  const [selectedNode, setSelectedNode] = useState<CfgNode | null>(null);

  // Sandbox inputs
  const [sandboxCodeA, setSandboxCodeA] = useState<string>(
    `pub fn process_items(items: &[i32]) -> i32 {\n    let mut total = 0;\n    for x in items {\n        if *x > 0 {\n            total += *x;\n        }\n    }\n    return total;\n}`,
  );
  const [sandboxCodeB, setSandboxCodeB] = useState<string>(
    `pub fn calculate_sum(nums: &[i32]) -> i32 {\n    let mut acc = 0;\n    for val in nums {\n        if *val > 0 {\n            acc += *val;\n        }\n    }\n    return acc;\n}`,
  );
  const [sandboxLang, setSandboxLang] = useState<string>("Rust");

  if (!isOpen) return null;

  const cfgs = semanticGraphResponse?.cfgs || [];
  const pdgs = semanticGraphResponse?.pdgs || [];
  const comparison = semanticGraphResponse?.comparison;

  const handleRunSandbox = () => {
    void fetchSemanticGraph({
      code: sandboxCodeA,
      language: sandboxLang,
      code_b: sandboxCodeB,
      language_b: sandboxLang,
    });
    setActiveTab("visualizer");
  };

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

  const renderSingleGraph = (
    cfg: ControlFlowGraph,
    pdg: ProgramDependenceGraph | undefined,
    label: string,
    keyIndex: number,
  ) => {
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
                  onClick={() => setSelectedNode(node)}
                  style={{ cursor: "pointer", pointerEvents: "all" }}
                >
                  <rect
                    x={x - 48}
                    y={y - 18}
                    width={96}
                    height={36}
                    rx={6}
                    onClick={() => setSelectedNode(node)}
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

  const footerContent = (
    <>
      <div className="flex items-center gap-3 text-xs font-mono text-slate-400">
        <label className="flex items-center gap-2 cursor-pointer">
          <input
            type="checkbox"
            checked={showPdgDataEdges}
            onChange={(e) => setShowPdgDataEdges(e.target.checked)}
            className="rounded bg-slate-950 border-slate-700 text-indigo-600 focus:ring-0"
          />
          <span className="text-slate-300">Show PDG Data Dependencies (Def-Use Chains)</span>
        </label>
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
      id="cddm-semantic-graph-window"
      windowType="semantic-graph"
      isOpen={isOpen}
      onClose={onClose}
      title="Deep Semantic Graph & CFG/PDG Visualizer"
      subtitle="Control Flow Graph extraction, Program Dependence def-use chains, and Weisfeiler-Lehman Type-4 clone isomorphism"
      badge={
        comparison
          ? `${(comparison.similarity * 100).toFixed(1)}% Isomorphic`
          : `${cfgs.length} CFG Graphs`
      }
      icon={<Network className="w-4 h-4 text-indigo-400" />}
      footer={footerContent}
      initialWidth={960}
      initialHeight={700}
    >
      <div className="space-y-4">
        {/* Navigation Tabs */}
        <div className="flex items-center justify-between border-b border-slate-800 pb-2">
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={() => setActiveTab("visualizer")}
              className={`px-3.5 py-1.5 rounded-lg text-xs font-semibold flex items-center gap-2 transition-colors ${
                activeTab === "visualizer"
                  ? "bg-indigo-600 text-white"
                  : "bg-slate-900 text-slate-400 hover:text-slate-200"
              }`}
            >
              <Network className="w-3.5 h-3.5" />
              <span>Graph Visualizer & Comparator</span>
            </button>
            <button
              type="button"
              onClick={() => setActiveTab("sandbox")}
              className={`px-3.5 py-1.5 rounded-lg text-xs font-semibold flex items-center gap-2 transition-colors ${
                activeTab === "sandbox"
                  ? "bg-indigo-600 text-white"
                  : "bg-slate-900 text-slate-400 hover:text-slate-200"
              }`}
            >
              <Code2 className="w-3.5 h-3.5" />
              <span>Semantic Comparison Sandbox</span>
            </button>
          </div>

          {comparison && (
            <div className="flex items-center gap-2">
              <span
                className={`text-xs font-mono font-bold px-2.5 py-1 rounded-full flex items-center gap-1.5 border ${
                  comparison.is_semantic_clone
                    ? "bg-emerald-950/80 text-emerald-300 border-emerald-800/80"
                    : "bg-amber-950/80 text-amber-300 border-amber-800/80"
                }`}
              >
                <CheckCircle2 className="w-3.5 h-3.5" />
                <span>Type-4 Similarity: {(comparison.similarity * 100).toFixed(1)}%</span>
              </span>
            </div>
          )}
        </div>

        {/* Error Alert */}
        {semanticGraphError && (
          <div className="p-3 bg-rose-950/40 border border-rose-800/60 rounded-xl text-rose-300 text-xs flex items-center gap-2">
            <AlertTriangle className="w-4 h-4 text-rose-400 shrink-0" />
            <span>{semanticGraphError}</span>
          </div>
        )}

        {/* Tab 1: Visualizer View */}
        {activeTab === "visualizer" && (
          <div className="space-y-4">
            {isSemanticGraphLoading ? (
              <div className="py-20 text-center text-slate-400 text-xs font-mono animate-pulse">
                Extracting Control Flow Graphs and computing Weisfeiler-Lehman kernels...
              </div>
            ) : cfgs.length > 0 ? (
              <div className="flex flex-wrap gap-4 items-start">
                {cfgs.map((cfg, idx) =>
                  renderSingleGraph(
                    cfg,
                    pdgs[idx],
                    idx === 0 ? "Fragment A" : idx === 1 ? "Fragment B" : `Graph ${idx + 1}`,
                    idx,
                  ),
                )}
              </div>
            ) : (
              <div className="py-16 text-center text-slate-400 text-xs font-mono bg-slate-900/40 border border-slate-800 rounded-xl">
                No semantic graphs loaded. Click &quot;Semantic Comparison Sandbox&quot; to test
                snippets or inspect a clone pair from results.
              </div>
            )}

            {/* Selected Node Details Card */}
            {selectedNode && (
              <div className="p-3.5 bg-slate-900/90 border border-indigo-900/60 rounded-xl flex flex-wrap items-center justify-between gap-3 text-xs font-mono text-slate-300">
                <div className="flex items-center gap-3">
                  <div className="p-1.5 bg-indigo-950 text-indigo-300 rounded-lg border border-indigo-800/50 font-bold">
                    Node #{selectedNode.id}
                  </div>
                  <div>
                    <span className="text-slate-400">Type:</span>{" "}
                    <span className="text-indigo-300 font-semibold">{selectedNode.node_type}</span>{" "}
                    | <span className="text-slate-400">Label:</span> &quot;{selectedNode.label}
                    &quot;
                  </div>
                </div>
                <div className="flex items-center gap-3 text-slate-400">
                  <span>
                    Lines: {selectedNode.line_start}-{selectedNode.line_end}
                  </span>
                  <span>Statements: {selectedNode.statement_count}</span>
                  <button
                    type="button"
                    onClick={() => setSelectedNode(null)}
                    className="text-slate-500 hover:text-slate-300"
                  >
                    Clear
                  </button>
                </div>
              </div>
            )}
          </div>
        )}

        {/* Tab 2: Custom Sandbox */}
        {activeTab === "sandbox" && (
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-xs text-slate-400">
                Paste two function implementations to extract their CFG/PDG structures and compute
                their structural isomorphism score.
              </span>
              <div className="flex items-center gap-2">
                <span className="text-xs text-slate-400">Language:</span>
                <select
                  value={sandboxLang}
                  onChange={(e) => setSandboxLang(e.target.value)}
                  className="bg-slate-950 border border-slate-800 text-xs rounded-lg px-2.5 py-1 text-slate-200 font-mono"
                >
                  <option value="Rust">Rust</option>
                  <option value="TypeScript">TypeScript / JS</option>
                  <option value="Python">Python</option>
                  <option value="Go">Go</option>
                  <option value="Java">Java</option>
                  <option value="C++">C / C++</option>
                </select>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-1.5">
                <label className="text-xs font-bold text-slate-300 flex items-center gap-1.5">
                  <FileCode2 className="w-3.5 h-3.5 text-indigo-400" />
                  <span>Function Implementation A:</span>
                </label>
                <textarea
                  value={sandboxCodeA}
                  onChange={(e) => setSandboxCodeA(e.target.value)}
                  rows={8}
                  className="w-full bg-slate-950 border border-slate-800 rounded-xl p-3 font-mono text-xs text-slate-200 focus:outline-none focus:border-indigo-500"
                />
              </div>

              <div className="space-y-1.5">
                <label className="text-xs font-bold text-slate-300 flex items-center gap-1.5">
                  <FileCode2 className="w-3.5 h-3.5 text-indigo-400" />
                  <span>Function Implementation B:</span>
                </label>
                <textarea
                  value={sandboxCodeB}
                  onChange={(e) => setSandboxCodeB(e.target.value)}
                  rows={8}
                  className="w-full bg-slate-950 border border-slate-800 rounded-xl p-3 font-mono text-xs text-slate-200 focus:outline-none focus:border-indigo-500"
                />
              </div>
            </div>

            <button
              type="button"
              onClick={handleRunSandbox}
              disabled={isSemanticGraphLoading}
              className="w-full py-2.5 bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white font-bold rounded-xl text-xs flex items-center justify-center gap-2 transition-colors shadow-lg shadow-indigo-900/30"
            >
              <Play className="w-3.5 h-3.5" />
              <span>Extract CFGs & Compare Isomorphism</span>
            </button>
          </div>
        )}
      </div>
    </Win2xWindow>
  );
};
