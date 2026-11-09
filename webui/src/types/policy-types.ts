/**
 * Policy and suppression domain types for CDDM WebUI.
 */

import type { CloneType } from "./clone-types";

/**
 * A suppression rule parsed from .cddmignore or configured via WebUI.
 */
export interface SuppressionRule {
  pattern: string;
  comment?: string;
  min_tokens_override?: number;
  ignored_clone_types?: CloneType[];
}

/**
 * Inline suppression directive parsed from source comments.
 */
export interface SuppressionDirective {
  file_path: string;
  line_start: number;
  line_end: number;
  directive_type: string;
  reason?: string;
}

/**
 * Complete suppression configuration.
 */
export interface SuppressionConfig {
  rules: SuppressionRule[];
  ignore_tests: boolean;
  ignore_mocks: boolean;
  ignore_generated: boolean;
  raw_cddmignore?: string;
}

/**
 * Architectural policy violation severity.
 */
export type PolicySeverity = "Error" | "Warning" | "Info";

/**
 * Architectural boundary rule.
 */
export interface BoundaryRule {
  name: string;
  description?: string;
  source: string;
  forbidden_targets: string[];
  severity: PolicySeverity;
}

/**
 * Zero duplication zone rule.
 */
export interface ZeroDuplicationRule {
  name: string;
  description?: string;
  pattern: string;
  severity: PolicySeverity;
}

/**
 * Clone token limit and cluster occurrence rule.
 */
export interface LimitRule {
  name: string;
  description?: string;
  pattern: string;
  max_tokens?: number;
  max_occurrences?: number;
  severity: PolicySeverity;
}

/**
 * Prohibited pattern rule for regex or Tree-sitter AST queries.
 */
export interface ProhibitedPatternRule {
  name: string;
  description?: string;
  pattern: string;
  pattern_type?: "regex" | "ast";
  include?: string[];
  exclude?: string[];
  severity: PolicySeverity;
  remediation?: string;
}

/**
 * Token budget rule constraining file or total tokens across a glob pattern.
 */
export interface TokenBudgetRule {
  name: string;
  description?: string;
  pattern: string;
  max_file_tokens?: number;
  max_total_tokens?: number;
  severity: PolicySeverity;
}

/**
 * Forbidden file rule banning conflicting tooling or deprecated lockfiles.
 */
export interface ForbiddenFileRule {
  name: string;
  description?: string;
  pattern?: string;
  patterns?: string[];
  severity: PolicySeverity;
  remediation?: string;
}

/**
 * SSoT constant and version synchronization rule.
 */
export interface SyncRule {
  name: string;
  description?: string;
  source_file: string;
  source_key?: string;
  source_regex?: string;
  target_files?: string[];
  target_file?: string;
  target_key?: string;
  target_regex?: string;
  severity: PolicySeverity;
  remediation?: string;
}

/**
 * Route documentation completeness policy.
 */
export interface RouteDocsPolicy {
  name?: string;
  framework?: string;
  routes_dir: string;
  doc_file?: string;
  doc_files?: string[];
  ignore_routes?: string[];
  severity: PolicySeverity;
  remediation?: string;
}

/**
 * Complete architectural policy configuration.
 */
export interface PolicyConfig {
  boundaries: BoundaryRule[];
  zero_duplication: ZeroDuplicationRule[];
  limits: LimitRule[];
  prohibited_patterns?: ProhibitedPatternRule[];
  zero_emojis?: boolean;
  token_budgets?: TokenBudgetRule[];
  forbidden_files?: ForbiddenFileRule[];
  sync_rules?: SyncRule[];
  route_docs_sync?: RouteDocsPolicy;
  route_docs?: RouteDocsPolicy[];
  raw_toml?: string;
}

/**
 * Architectural policy violation record.
 */
export interface PolicyViolation {
  rule_name: string;
  rule_type: string;
  severity: PolicySeverity;
  message: string;
  file_a: string;
  start_line_a: number;
  end_line_a: number;
  file_b?: string;
  start_line_b?: number;
  end_line_b?: number;
  cluster_id?: number;
  token_count: number;
  snippet?: string;
  remediation?: string;
}

/**
 * Architectural policy evaluation result.
 */
export interface PolicyEvaluationResult {
  passed: boolean;
  total_violations: number;
  error_count: number;
  warning_count: number;
  info_count: number;
  violations: PolicyViolation[];
}
