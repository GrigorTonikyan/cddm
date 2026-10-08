/**
 * Scan, progress, metrics, and timeline types for CDDM WebUI.
 */

import type { CloneCluster, ClonePair } from "./clone-types";
import type { PolicyViolation } from "./policy-types";
import type { ApplyPatchResult } from "./refactor-types";

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
  policy_violations?: PolicyViolation[];
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
  detect_type3?: boolean;
  detect_type4?: boolean;
  scan_self: boolean;
  enable_git_blame?: boolean;
  cddmignore_path?: string;
  ignore_tests?: boolean;
  ignore_mocks?: boolean;
  ignore_generated?: boolean;
  rules_path?: string;
  enforce_policies?: boolean;
  cross_language?: boolean;
  threads?: number;
  fail_threshold?: number;
  cache_dir?: string;
  enable_cache?: boolean;
  in_tree_cache?: boolean;
  include_ignored?: boolean;
  baseline?: string;
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
 * Comparative delta report for a single workspace watch sync event.
 */
export interface WatchDeltaReport {
  changed_files: string[];
  previous_health_score: number;
  new_health_score: number;
  score_delta: number;
  previous_clones: number;
  new_clones: number;
  clone_count_delta: number;
  previous_clusters: number;
  new_clusters: number;
  duration_ms: number;
  timestamp_millis: number;
}

/**
 * Status response for real-time workspace watch daemon.
 */
export interface WatchStatusResponse {
  is_active: boolean;
  watch_directory: string;
  debounce_ms: number;
  last_sync_timestamp: number | null;
  sync_count: number;
  last_duration_ms: number | null;
  recent_events: WatchDeltaReport[];
}

/**
 * Server-Sent Event payload from backend /api/events.
 */
export type ServerEvent =
  | { type: "scan_started"; payload: { scan_id: string } }
  | { type: "scan_progress"; payload: ScanProgress }
  | { type: "scan_complete"; payload: ScanResult }
  | { type: "patch_applied"; payload: ApplyPatchResult }
  | { type: "watch_file_changed"; payload: { files: string[]; timestamp: number } }
  | { type: "watch_scan_delta"; payload: WatchDeltaReport }
  | { type: "watch_status_changed"; payload: { is_active: boolean } }
  | { type: "file_saved"; payload: { path: string; timestamp: number } };

/**
 * A point-in-time duplication metrics snapshot for a Git commit.
 */
export interface TimelineSnapshot {
  commit_hash: string;
  short_hash: string;
  author: string;
  commit_time: number;
  formatted_date: string;
  message: string;
  tag?: string;
  total_files: number;
  total_tokens: number;
  total_clones: number;
  total_clusters: number;
  duplication_percentage: number;
  dry_health_score: number;
}

/**
 * Metric tracking file modification frequency and duplicate clone association.
 */
export interface FileChurnMetric {
  file_path: string;
  commit_count: number;
  clone_count: number;
}

/**
 * Aggregated historical duplication trend across Git history.
 */
export interface TimelineTrend {
  snapshots: TimelineSnapshot[];
  initial_score: number;
  current_score: number;
  score_delta: number;
  duplication_delta: number;
  churn_hotspots: FileChurnMetric[];
}

/**
 * Status of local Git pre-commit and pre-push hooks.
 */
export interface HookStatus {
  pre_commit_installed: boolean;
  pre_push_installed: boolean;
  hooks_dir: string;
}

/**
 * Pairwise divergence and clone drift metrics between two Git branches.
 */
export interface BranchPairDrift {
  base_branch: string;
  target_branch: string;
  base_dry_score: number;
  target_dry_score: number;
  net_dry_delta: number;
  changed_files_count: number;
  new_clones_count: number;
  divergence_index: number;
}

/**
 * Full N-way matrix report detailing clone drift across multiple Git branches.
 */
export interface BranchMatrixReport {
  workspace_root: string;
  branches: string[];
  matrix: BranchPairDrift[];
  cleanest_branch?: string;
  highest_drift_branch?: string;
  summary: string;
}

/**
 * A detected submodule, package, or workspace crate within a monorepo.
 */
export interface MonorepoWorkspace {
  name: string;
  path: string;
  manifest_file: string;
  package_type: string;
}

/**
 * Comprehensive aggregated summary across multiple monorepo workspaces.
 */
export interface MonorepoScanSummary {
  workspaces: MonorepoWorkspace[];
  total_workspaces: number;
  total_files: number;
  total_tokens: number;
  total_clones: number;
  cross_workspace_clones: number;
  average_dry_score: number;
  scan_result: ScanResult;
}

/**
 * Detailed summary of a persistent cache pack export or import operation.
 */
export interface CachePackSummary {
  success: boolean;
  pack_file: string;
  entry_count: number;
  checksum: string;
  message: string;
}

/**
 * Request payload for exporting persistent cache pack.
 */
export interface CacheExportRequest {
  cache_dir?: string;
  output_pack_path?: string;
}

/**
 * Request payload for importing persistent cache pack.
 */
export interface CacheImportRequest {
  pack_file: string;
  target_cache_dir?: string;
}

/**
 * Status of a clone pair in a differential scan.
 */
export type CloneStatus = "New" | "Legacy" | "Resolved";

/**
 * A clone pair annotated with differential status relative to a base git ref.
 */
export interface DiffClonePair {
  clone_pair: ClonePair;
  status: CloneStatus;
}

/**
 * Summary metrics comparing target state to baseline git reference.
 */
export interface DiffSummary {
  base_ref: string;
  target_ref: string;
  base_dry_score: number;
  target_dry_score: number;
  net_dry_delta: number;
  total_changed_files: number;
  new_clones: number;
  legacy_clones: number;
  resolved_clones: number;
}

/**
 * Full differential scan result returned by /api/diff.
 */
export interface DiffScanResult {
  scan_id: string;
  summary: DiffSummary;
  diff_clones: DiffClonePair[];
  duration_ms: number;
}

/**
 * Request payload for POST /api/diff.
 */
export interface DiffScanRequest {
  base_ref: string;
  target_ref?: string;
  config: ScanConfig;
}
