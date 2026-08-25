/**
 * Type definitions for Control Flow Graph (CFG), Program Dependence Graph (PDG),
 * and Weisfeiler-Lehman semantic graph matching.
 */

export type CfgNodeType =
  | "Entry"
  | "Exit"
  | "BasicBlock"
  | "Branch"
  | "LoopHeader"
  | "LoopBody"
  | "Return";

export interface CfgNode {
  id: number;
  node_type: CfgNodeType;
  label: string;
  statement_count: number;
  line_start: number;
  line_end: number;
}

export type CfgEdgeType = "Sequential" | "TrueBranch" | "FalseBranch" | "LoopBack" | "LoopExit";

export interface CfgEdge {
  from: number;
  to: number;
  edge_type: CfgEdgeType;
}

export interface ControlFlowGraph {
  file_path: string;
  function_name: string;
  line_start: number;
  line_end: number;
  nodes: CfgNode[];
  edges: CfgEdge[];
  wl_hash: number;
}

export type PdgEdgeKind = "DataDependency" | "ControlDependency";

export interface PdgEdge {
  from: number;
  to: number;
  variable: string;
  kind: PdgEdgeKind;
}

export interface ProgramDependenceGraph {
  cfg: ControlFlowGraph;
  data_edges: PdgEdge[];
}

export interface SemanticComparisonResponse {
  similarity: number;
  is_semantic_clone: boolean;
  wl_hash_a: number;
  wl_hash_b: number;
}

export interface SemanticGraphRequest {
  file?: string;
  code?: string;
  language?: string;
  file_b?: string;
  code_b?: string;
  language_b?: string;
}

export interface SemanticGraphResponse {
  cfgs: ControlFlowGraph[];
  pdgs: ProgramDependenceGraph[];
  comparison?: SemanticComparisonResponse | null;
}
