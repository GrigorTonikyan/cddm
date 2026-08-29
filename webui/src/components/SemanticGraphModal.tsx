import React, { useState } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  Network,
  CheckCircle2,
  AlertTriangle,
  Play,
  Code2,
  FileCode2,
  Sparkles,
} from "lucide-react";
import type { CfgNode, CrossLanguageClonePair } from "../types/cddm-types";
import { SingleGraphCard } from "./semantic/SingleGraphCard";
import { CrossLanguageExplorerTab } from "./semantic/CrossLanguageExplorerTab";
import { ModalTabs } from "./ui/ModalTabs";

export interface SemanticGraphModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const SemanticGraphModal: React.FC<SemanticGraphModalProps> = ({ isOpen, onClose }) => {
  const { semanticGraphResponse, isSemanticGraphLoading, semanticGraphError, fetchSemanticGraph } =
    useCDDMStore();

  const [activeTab, setActiveTab] = useState<"visualizer" | "sandbox" | "cross-lang">("visualizer");
  const [showPdgDataEdges, setShowPdgDataEdges] = useState<boolean>(true);
  const [selectedNode, setSelectedNode] = useState<CfgNode | null>(null);

  // Sandbox inputs
  const [sandboxCodeA, setSandboxCodeA] = useState<string>(
    `pub fn calculate_discount(price: f64, is_member: bool) -> f64 {\n    let mut rate = 0.05;\n    if is_member {\n        rate = 0.20;\n    }\n    let discount = price * rate;\n    return discount;\n}`,
  );
  const [sandboxLangA, setSandboxLangA] = useState<string>("Rust");

  const [sandboxCodeB, setSandboxCodeB] = useState<string>(
    `export function calculateDiscount(price: number, isMember: boolean): number {\n    let rate = 0.05;\n    if (isMember) {\n        rate = 0.20;\n    }\n    const discount = price * rate;\n    return discount;\n}`,
  );
  const [sandboxLangB, setSandboxLangB] = useState<string>("TypeScript");

  if (!isOpen) return null;

  const cfgs = semanticGraphResponse?.cfgs || [];
  const pdgs = semanticGraphResponse?.pdgs || [];
  const comparison = semanticGraphResponse?.comparison;

  const handleRunSandbox = () => {
    void fetchSemanticGraph({
      code: sandboxCodeA,
      language: sandboxLangA,
      code_b: sandboxCodeB,
      language_b: sandboxLangB,
    });
    setActiveTab("visualizer");
  };

  const handleInspectPair = (pair: CrossLanguageClonePair) => {
    void fetchSemanticGraph({
      file: pair.file_a,
      file_b: pair.file_b,
      language: pair.language_a,
      language_b: pair.language_b,
      function_a: pair.function_a,
      function_b: pair.function_b,
      lines_a: pair.lines_a,
      lines_b: pair.lines_b,
    });
    setActiveTab("visualizer");
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
      title="Deep Semantic Graph & Polyglot Isomorphism Engine"
      subtitle="Control Flow Graph extraction, Program Dependence def-use chains, and Weisfeiler-Lehman Type-4 clone isomorphism"
      badge={
        comparison
          ? `${(comparison.similarity * 100).toFixed(1)}% Isomorphic`
          : `${cfgs.length} CFG Graphs`
      }
      icon={<Network className="w-4 h-4 text-indigo-400" />}
      footer={footerContent}
      initialWidth={980}
      initialHeight={720}
    >
      <div className="space-y-4">
        {/* Navigation Tabs */}
        <div className="flex items-center justify-between border-b border-slate-800 pb-2">
          <ModalTabs
            tabs={[
              {
                id: "visualizer",
                label: "Graph Visualizer",
                icon: <Network className="w-3.5 h-3.5" />,
              },
              { id: "sandbox", label: "Polyglot Sandbox", icon: <Code2 className="w-3.5 h-3.5" /> },
              {
                id: "cross-lang",
                label: "Cross-Language Explorer",
                icon: <Sparkles className="w-3.5 h-3.5" />,
              },
            ]}
            activeTab={activeTab}
            onTabChange={(id) => setActiveTab(id as "visualizer" | "sandbox" | "cross-lang")}
            activeColorClass="bg-indigo-600 text-white"
          />

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
                <span>
                  {comparison.is_cross_language ? "Polyglot" : "Type-4"} Similarity:{" "}
                  {(comparison.similarity * 100).toFixed(1)}%
                </span>
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
                {cfgs.map((cfg, idx) => (
                  <SingleGraphCard
                    key={`cfg-card-${idx}-${cfg.function_name}`}
                    cfg={cfg}
                    pdg={pdgs[idx]}
                    label={idx === 0 ? "Fragment A" : idx === 1 ? "Fragment B" : `Graph ${idx + 1}`}
                    keyIndex={idx}
                    showPdgDataEdges={showPdgDataEdges}
                    selectedNode={selectedNode}
                    onSelectNode={setSelectedNode}
                  />
                ))}
              </div>
            ) : (
              <div className="py-16 text-center text-slate-400 text-xs font-mono bg-slate-900/40 border border-slate-800 rounded-xl">
                No semantic graphs loaded. Click &quot;Polyglot Sandbox&quot; to test snippets or
                explore cross-language clones.
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
                    className="text-slate-500 hover:text-slate-300 cursor-pointer"
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
            <span className="text-xs text-slate-400 block">
              Paste two function implementations (even across different programming languages!) to
              extract their CFG/PDG structures and compute their structural isomorphism score.
            </span>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <label className="text-xs font-bold text-slate-300 flex items-center gap-1.5">
                    <FileCode2 className="w-3.5 h-3.5 text-indigo-400" />
                    <span>Implementation A:</span>
                  </label>
                  <select
                    value={sandboxLangA}
                    onChange={(e) => setSandboxLangA(e.target.value)}
                    className="bg-slate-950 border border-slate-800 text-[11px] rounded-lg px-2 py-0.5 text-slate-200 font-mono"
                  >
                    <option value="Rust">Rust</option>
                    <option value="TypeScript">TypeScript / JS</option>
                    <option value="Python">Python</option>
                    <option value="Go">Go</option>
                    <option value="Java">Java</option>
                    <option value="C++">C / C++</option>
                  </select>
                </div>
                <textarea
                  value={sandboxCodeA}
                  onChange={(e) => setSandboxCodeA(e.target.value)}
                  rows={8}
                  className="w-full bg-slate-950 border border-slate-800 rounded-xl p-3 font-mono text-xs text-slate-200 focus:outline-none focus:border-indigo-500"
                />
              </div>

              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <label className="text-xs font-bold text-slate-300 flex items-center gap-1.5">
                    <FileCode2 className="w-3.5 h-3.5 text-purple-400" />
                    <span>Implementation B:</span>
                  </label>
                  <select
                    value={sandboxLangB}
                    onChange={(e) => setSandboxLangB(e.target.value)}
                    className="bg-slate-950 border border-slate-800 text-[11px] rounded-lg px-2 py-0.5 text-slate-200 font-mono"
                  >
                    <option value="TypeScript">TypeScript / JS</option>
                    <option value="Rust">Rust</option>
                    <option value="Python">Python</option>
                    <option value="Go">Go</option>
                    <option value="Java">Java</option>
                    <option value="C++">C / C++</option>
                  </select>
                </div>
                <textarea
                  value={sandboxCodeB}
                  onChange={(e) => setSandboxCodeB(e.target.value)}
                  rows={8}
                  className="w-full bg-slate-950 border border-slate-800 rounded-xl p-3 font-mono text-xs text-slate-200 focus:outline-none focus:border-purple-500"
                />
              </div>
            </div>

            <button
              type="button"
              onClick={handleRunSandbox}
              disabled={isSemanticGraphLoading}
              className="w-full py-2.5 bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 disabled:opacity-50 text-white font-bold rounded-xl text-xs flex items-center justify-center gap-2 transition-all shadow-lg shadow-indigo-900/30 cursor-pointer"
            >
              <Play className="w-3.5 h-3.5 fill-current" />
              <span>Extract CFGs & Compare Isomorphism</span>
            </button>
          </div>
        )}

        {/* Tab 3: Cross-Language Explorer */}
        {activeTab === "cross-lang" && (
          <CrossLanguageExplorerTab onInspectPair={handleInspectPair} />
        )}
      </div>
    </Win2xWindow>
  );
};
