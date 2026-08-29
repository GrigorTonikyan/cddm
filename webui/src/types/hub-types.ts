/**
 * TypeScript domain type definitions for Organization Federation Hub (.cddmhub.toml).
 */

export interface HubRepoConfig {
  name: string;
  path: string;
  tech_stack?: string;
  min_tokens?: number;
  fail_threshold?: number;
}

export interface HubConfig {
  name: string;
  repositories: HubRepoConfig[];
  min_tokens: number;
  fail_threshold: number;
  ignore_patterns: string[];
}

export interface CrossRepoOccurrence {
  repo_name: string;
  file_path: string;
  start_line: number;
  end_line: number;
  snippet?: string;
}

export interface CrossRepoClonePair {
  repo_a: string;
  file_a: string;
  start_line_a: number;
  end_line_a: number;
  repo_b: string;
  file_b: string;
  start_line_b: number;
  end_line_b: number;
  token_count: number;
  similarity: number;
}

export interface CrossRepoCluster {
  id: number;
  repos: string[];
  occurrences: CrossRepoOccurrence[];
  token_count: number;
  similarity: number;
  suggested_package: string;
}

export interface RepoDuplicationMetric {
  name: string;
  path: string;
  tech_stack: string;
  total_files: number;
  total_tokens: number;
  internal_duplication_percentage: number;
  cross_repo_duplication_percentage: number;
  dry_health_score: number;
}

export interface RepoPairDuplication {
  repo_a: string;
  repo_b: string;
  shared_clones: number;
  shared_tokens: number;
}

export interface HubScanSummary {
  hub_name: string;
  total_repos: number;
  total_files: number;
  total_tokens: number;
  organization_dry_score: number;
  repos: RepoDuplicationMetric[];
  duplication_matrix: RepoPairDuplication[];
  clusters: CrossRepoCluster[];
}

export interface HubExtractRequest {
  hub_config?: HubConfig;
  cluster_id: number;
  target_package_name: string;
  package_type: string;
  target_dir: string;
  dry_run: boolean;
}

export interface HubRepoUpdate {
  repo_name: string;
  repo_path: string;
  manifest_changes: Array<{ manifest_path: string; added_dependency: string }>;
  callsite_rewrites: Array<{
    file: string;
    line: number;
    original_call: string;
    replacement_call: string;
  }>;
  branch_name: string;
}

export interface HubExtractResult {
  package_name: string;
  package_type: string;
  package_dir: string;
  generated_files: Array<{ file_path: string; content: string }>;
  repo_updates: HubRepoUpdate[];
  lines_saved: number;
  summary: string;
}
