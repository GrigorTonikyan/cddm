/**
 * TypeScript types for CDDM Polyglot Dead Code Detection Subsystem.
 */

export type DeadCodeKind =
  | "unreferenced_function"
  | "unreachable_block"
  | "dead_clone"
  | "uncovered_function"
  | "dead_branch";

export type ReachabilityStatus =
  | "live_cross_package"
  | "live_internal"
  | "unused_export"
  | "dead_internal";

export interface SymbolReachability {
  symbol_name: string;
  declaring_package: string;
  declaring_file: string;
  is_exported: boolean;
  status: ReachabilityStatus;
  direct_callers: string[];
  transitive_callers: string[];
  caller_packages: string[];
  total_references: number;
}

export interface CrossPackageReachabilitySummary {
  total_packages: number;
  packages: string[];
  live_cross_package_symbols: number;
  live_internal_symbols: number;
  unused_exported_symbols: number;
  dead_internal_symbols: number;
  total_cross_package_calls: number;
  symbol_traces: SymbolReachability[];
}

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
  package_name?: string;
  is_exported?: boolean;
  cross_package_callers?: string[];
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
  reachability_summary?: CrossPackageReachabilitySummary;
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

export type PruneActionStatus = "pruned" | "dry_run_pruned" | "skipped_unsafe" | "failed";

export interface PrunedItem {
  id: number;
  file_path: string;
  symbol_name: string;
  line_start: number;
  line_end: number;
  lines_removed: number;
  status: PruneActionStatus;
  confidence: number;
  reason: string;
  diff_preview?: string;
}

export interface DeadClonePruneResult {
  total_candidates: number;
  pruned_items: number;
  skipped_items: number;
  total_lines_removed: number;
  dry_run: boolean;
  files_affected: string[];
  items: PrunedItem[];
}

export interface DeadClonePruneRequest {
  directory?: string;
  min_tokens?: number;
  dry_run?: boolean;
  safe_only?: boolean;
  threshold?: number;
  item_ids?: number[];
  languages?: string[];
  ignore?: string[];
}
