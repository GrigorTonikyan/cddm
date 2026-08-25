/**
 * Refactoring, AST rewrite, and sandbox types for CDDM WebUI.
 */

import type { CloneLocation, CloneType } from "./clone-types";

/**
 * Interactive refactor sandbox simulation request.
 */
export interface RefactorSandboxRequest {
  cluster_id?: number;
  occurrences: CloneLocation[];
  custom_function_name?: string;
  target_module_path?: string;
  custom_parameter_names?: string[];
}

/**
 * Interactive refactor sandbox simulation result.
 */
export interface RefactorSandboxResult {
  function_name: string;
  target_module_path: string;
  parameter_names?: string[];
  unified_patch: string;
  total_lines_saved: number;
  sites_count?: number;
  affected_files?: string[];
  preview_diff_hunks?: string[];
}

/**
 * Request payload for applying refactor patch to a dedicated Git branch.
 */
export interface ApplyRefactorBranchRequest {
  patch: string;
  branch_name?: string;
  create_branch?: boolean;
}

/**
 * Response payload for refactor patch branch application.
 */
export interface ApplyRefactorBranchResult {
  success: boolean;
  branch_created?: string;
  modified_files: string[];
  hunks_applied: number;
  message: string;
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
 * Contextual code fragment occurrence for AI refactoring prompt synthesis.
 */
export interface AiOccurrenceContext {
  path: string;
  span: import("./scan-types").LineSpan;
  snippet: string;
}

/**
 * Structured request for synthesizing an AI refactoring prompt.
 */
export interface AiRefactorPromptRequest {
  clone_type: CloneType;
  similarity: number;
  token_count: number;
  lines_saved_est: number;
  function_name: string;
  target_module: string;
  occurrences: AiOccurrenceContext[];
  invariant_body: string;
  parameters: string[];
  custom_instructions?: string;
}

/**
 * Response payload containing the synthesized AI prompt specification.
 */
export interface AiPromptResponse {
  prompt: string;
}

/**
 * Inferred parameter with extracted identifier name and language-specific type.
 */
export interface InferredParameter {
  name: string;
  inferred_type: string;
  original_values: string[];
}

/**
 * Source file rewritten via Tree-sitter AST node substitution.
 */
export interface AstRewrittenFile {
  file_path: string;
  original_line_count: number;
  new_line_count: number;
  call_sites_count: number;
  rewritten_source: string;
  imports_added: string[];
}

/**
 * Complete result of an AST-native refactoring transformation across multiple files.
 */
export interface AstRewriteResult {
  cluster_id?: number;
  function_name: string;
  target_module_path: string;
  helper_signature: string;
  helper_function_code: string;
  inferred_parameters: InferredParameter[];
  rewritten_files: AstRewrittenFile[];
  unified_patch: string;
  total_lines_saved: number;
  syntax_valid: boolean;
}

/**
 * Request payload to run closed-loop test suite verification.
 */
export interface VerifyRefactorRequest {
  directory: string;
  test_command?: string;
  branch_name?: string;
  timeout_seconds?: number;
}

/**
 * Structured result of closed-loop test suite verification.
 */
export interface VerifyRefactorResult {
  success: boolean;
  exit_code: number;
  duration_ms: number;
  command_executed: string;
  stdout_snippet: string;
  stderr_snippet: string;
  message: string;
}
