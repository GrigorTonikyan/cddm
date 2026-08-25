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
 * Complete architectural policy configuration.
 */
export interface PolicyConfig {
  boundaries: BoundaryRule[];
  zero_duplication: ZeroDuplicationRule[];
  limits: LimitRule[];
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
