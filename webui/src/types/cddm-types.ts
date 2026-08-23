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
  duplication_percentage: number;
  dry_health_score: number;
  clone_pairs: ClonePair[];
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
 * Windows 11 persistent window state for draggable/resizable modals.
 */
export interface ModalWindowState {
  x: number;
  y: number;
  width: number;
  height: number;
  isMaximized: boolean;
  isMinimized: boolean;
}

export const DEFAULT_MODAL_WINDOW_STATE: ModalWindowState = {
  x: -1, // -1 denotes center dynamically on first open
  y: -1,
  width: 920,
  height: 680,
  isMaximized: false,
  isMinimized: false,
};
