/**
 * TypeScript types for CDDM Polyglot Dead Code Detection Subsystem.
 */

export type DeadCodeKind =
  | "unreferenced_function"
  | "unreachable_block"
  | "dead_clone"
  | "uncovered_function"
  | "dead_branch";

export interface DeadCodeItem {
  id: number;
  file_path: string;
  symbol_name: string;
  kind: DeadCodeKind;
  line_start: number;
  line_end: number;
  token_count: number;
  estimated_lines_saved: number;
  reason: string;
  confidence: number;
}

export interface DeadCodeSummary {
  total_dead_items: number;
  dead_functions: number;
  unreachable_blocks: number;
  dead_clones: number;
  uncovered_items: number;
  total_dead_lines: number;
  estimated_savings_pct: number;
  items: DeadCodeItem[];
}

export interface DeadCodeScanRequest {
  directory?: string;
  min_tokens?: number;
  static_only?: boolean;
  report_path?: string;
  report_content?: string;
  languages?: string[];
  ignore?: string[];
}
