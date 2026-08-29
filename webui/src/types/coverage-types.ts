/**
 * TypeScript domain type definitions for Runtime Execution & Coverage-Aware De-duplication.
 */

export type ExecutionTier = "DeadCode" | "Cold" | "Warm" | "HotPath";

export interface CloneCoverageMetric {
  clone_pair_id: number;
  file_a: string;
  start_line_a: number;
  end_line_a: number;
  hits_a: number;
  covered_lines_a: number;
  total_lines_a: number;
  coverage_pct_a: number;
  file_b: string;
  start_line_b: number;
  end_line_b: number;
  hits_b: number;
  covered_lines_b: number;
  total_lines_b: number;
  coverage_pct_b: number;
  total_combined_hits: number;
  execution_tier: ExecutionTier;
  has_test_gap: boolean;
  is_dead_code: boolean;
  risk_score: number;
}

export interface CoverageCorrelationSummary {
  total_clone_pairs: number;
  overall_covered_clones_pct: number;
  dead_code_clones: number;
  test_gap_clones: number;
  hot_path_clones: number;
  total_runtime_hits: number;
  metrics: CloneCoverageMetric[];
}

export interface CoverageIngestRequest {
  report_content?: string;
  report_path?: string;
  format?: string;
}

export interface CoverageCorrelateRequest {
  report_path?: string;
  report_content?: string;
  format?: string;
  directory?: string;
  min_tokens?: number;
  dead_code_only?: boolean;
  min_hits?: number;
  risk_threshold?: number;
}
