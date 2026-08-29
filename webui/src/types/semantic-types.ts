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
  graph_similarity?: number;
  token_similarity?: number;
  hybrid_score?: number;
  is_semantic_clone: boolean;
  is_cross_language?: boolean;
  wl_hash_a: number;
  wl_hash_b: number;
}

export interface CrossLanguageClonePair {
  file_a: string;
  language_a: string;
  function_a: string;
  lines_a: [number, number];
  file_b: string;
  language_b: string;
  function_b: string;
  lines_b: [number, number];
  graph_similarity: number;
  token_similarity: number;
  hybrid_score: number;
  clone_type: string;
}

export interface SemanticScanRequest {
  directory?: string;
  threshold?: number;
  min_tokens?: number;
  languages?: string[];
  ignore?: string[];
}

export interface SemanticGraphRequest {
  file?: string;
  code?: string;
  language?: string;
  function_a?: string;
  lines_a?: [number, number];
  file_b?: string;
  code_b?: string;
  language_b?: string;
  function_b?: string;
  lines_b?: [number, number];
}

export interface SemanticGraphResponse {
  cfgs: ControlFlowGraph[];
  pdgs: ProgramDependenceGraph[];
  comparison?: SemanticComparisonResponse | null;
}

export interface RecommendedLibrary {
  language: string;
  package_name: string;
  install_command: string;
  replacement_snippet: string;
}

export interface EcosystemAlgorithm {
  name: string;
  category: string;
  description: string;
  canonical_keywords: string[];
  recommendations: RecommendedLibrary[];
}

export interface OverlapMatch {
  algorithm_name: string;
  category: string;
  file_path: string;
  function_name: string;
  line_span: [number, number];
  confidence: number;
  snippet: string;
  recommended_library: RecommendedLibrary;
}

export interface OverlapScanResult {
  matches: OverlapMatch[];
  total_files_scanned: number;
  scanned_functions: number;
  summary: string;
}

export interface OverlapScanRequest {
  directory?: string;
  threshold?: number;
}

export type EquivalenceConfidence = "High" | "Medium" | "Low";

export interface NeuralClonePair {
  file_a: string;
  start_line_a: number;
  end_line_a: number;
  language_a: string;
  file_b: string;
  start_line_b: number;
  end_line_b: number;
  language_b: string;
  similarity: number;
  confidence: EquivalenceConfidence;
  semantic_rationale: string;
}

export interface NeuralScanResult {
  total_blocks_embedded: number;
  total_neural_pairs: number;
  high_confidence_count: number;
  pairs: NeuralClonePair[];
}

export interface SemanticNeuralRequest {
  directory?: string;
  threshold?: number;
  dimension?: number;
  max_subwords?: number;
}
