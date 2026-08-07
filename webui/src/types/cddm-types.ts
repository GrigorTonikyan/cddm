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
}

/**
 * Scan progress event.
 */
export interface ScanProgress {
  scan_id: string;
  phase: string;
  files_processed: number;
  total_files: number;
  progress: number;
  message: string;
}
