/**
 * TypeScript type definitions for CDDM — Code De-Duplication Meister WebUI.
 */

/**
 * Normalized token types.
 */
export type NormalizedToken =
  | { type: "Identifier" }
  | { type: "StringLiteral" }
  | { type: "NumericLiteral" }
  | { type: "Keyword"; id: number }
  | { type: "Punctuation"; id: number };

/**
 * Line span in source file.
 */
export interface LineSpan {
  line_start: number;
  line_end: number;
  byte_offset: number;
}

/**
 * Clone type classification.
 */
export type CloneType = "Exact" | "Renamed" | "NearMiss" | "Semantic";

/**
 * Clone location occurrence.
 */
export interface CloneLocation {
  file: string;
  start_line: number;
  end_line: number;
  author?: string;
}

/**
 * N-way clone cluster (equivalence class).
 */
export interface CloneCluster {
  id: number;
  clone_type: CloneType;
  token_count: number;
  similarity: number;
  fragment_hash: string;
  occurrences: CloneLocation[];
}

/**
 * Clone pair result.
 */
export interface ClonePair {
  file_a: string;
  start_line_a: number;
  end_line_a: number;
  file_b: string;
  start_line_b: number;
  end_line_b: number;
  token_count: number;
  similarity: number;
  fragment_hash: string;
  clone_type: CloneType;
  author_a?: string;
  author_b?: string;
}

/**
 * Language statistics.
 */
export interface LanguageStats {
  language: string;
  files: number;
  tokens: number;
  clones: number;
}

/**
 * Final scan result payload.
 */
export interface ScanResult {
  scan_id: string;
  total_files: number;
  total_tokens: number;
  total_clones: number;
  total_clusters: number;
  duplication_percentage: number;
  dry_health_score: number;
  clone_pairs: ClonePair[];
  clone_clusters: CloneCluster[];
  duration_ms: number;
  language_breakdown: LanguageStats[];
}

/**
 * Scan configuration.
 */
export interface ScanConfig {
  directory: string;
  min_tokens: number;
  languages: string[];
  ignore_patterns: string[];
  detect_type2: boolean;
  scan_self: boolean;
  enable_git_blame?: boolean;
}

/**
 * Scan execution phases matching backend ScanPhase enum.
 */
export type ScanPhase =
  | "Discovery"
  | "Tokenization"
  | "AstAnalysis"
  | "Indexing"
  | "Merging"
  | "Scoring"
  | "Complete"
  | "Cancelled"
  | "Failed";

/**
 * Scan progress event.
 */
export interface ScanProgress {
  scan_id: string;
  phase: ScanPhase;
  files_processed: number;
  total_files: number;
  progress: number;
  message: string;
}

/**
 * A single source line in a snippet response.
 */
export interface SnippetLine {
  line_number: number;
  content: string;
  is_target: boolean;
}

/**
 * Structured response containing source snippet lines with context.
 */
export interface SnippetResponse {
  file: string;
  start_line: number;
  end_line: number;
  context_start_line: number;
  context_end_line: number;
  lines: SnippetLine[];
  total_lines: number;
  language: string;
}

/**
 * Request payload for synthesizing refactoring suggestions.
 */
export interface RefactorRequest {
  file_a: string;
  start_line_a: number;
  end_line_a: number;
  file_b: string;
  start_line_b: number;
  end_line_b: number;
}

/**
 * Request payload for synthesizing multi-site cluster refactoring suggestions.
 */
export interface ClusterRefactorRequest {
  cluster_id: string;
  occurrences: CloneLocation[];
}

/**
 * Represents a variable difference between two clone fragments.
 */
export interface ParameterDifference {
  line_number_a: number;
  line_number_b: number;
  fragment_a_code: string;
  fragment_b_code: string;
}

/**
 * Comprehensive deduplication and refactoring recommendation for a clone pair.
 */
export interface RefactorSuggestion {
  suggested_function_name: string;
  strategy: string;
  common_body_lines: string[];
  parameter_differences: ParameterDifference[];
  target_module_hint: string;
  unified_patch: string;
  lines_saved: number;
}

/**
 * Transformation details at an individual cluster site.
 */
export interface ClusterSiteRefactor {
  file: string;
  start_line: number;
  end_line: number;
  parameter_differences: ParameterDifference[];
  call_site_replacement: string;
}

/**
 * Multi-site refactoring recommendation for an N-way clone cluster.
 */
export interface ClusterRefactorSuggestion {
  cluster_id: string;
  suggested_function_name: string;
  strategy: string;
  common_body_lines: string[];
  target_module_hint: string;
  sites: ClusterSiteRefactor[];
  unified_patch: string;
  total_lines_saved: number;
}

/**
 * Node structure for hierarchical Treemap visualization.
 */
export interface TreemapNode {
  name: string;
  path: string;
  tokens: number;
  clones: number;
  duplicationPercentage: number;
  children?: TreemapNode[];
}

/**
 * Partitioned rectangle layout for Treemap SVG rendering.
 */
export interface TreemapRect {
  x: number;
  y: number;
  width: number;
  height: number;
  node: TreemapNode;
}

/**
 * Request payload for applying a refactoring patch to the workspace.
 */
export interface ApplyPatchRequest {
  patch: string;
  dry_run?: boolean;
}

/**
 * Structured response for a completed patch application.
 */
export interface ApplyPatchResult {
  success: boolean;
  modified_files: string[];
  hunks_applied: number;
  message: string;
}

/**
 * Server-Sent Event payload from backend /api/events.
 */
export type ServerEvent =
  | { type: "scan_started"; payload: { scan_id: string } }
  | { type: "scan_progress"; payload: ScanProgress }
  | { type: "scan_complete"; payload: ScanResult }
  | { type: "patch_applied"; payload: ApplyPatchResult };
