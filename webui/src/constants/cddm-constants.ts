/**
 * Centralized constants and protocol definitions for CDDM WebUI.
 */

import pkg from "../../package.json";
import { CloneType, ScanConfig, ScanPhase } from "../types/cddm-types";

/**
 * Dynamic application version derived from package.json.
 */
export const APP_VERSION: string = pkg.version;

/**
 * REST API endpoint routes exposed by the Axum server.
 */
export const API_ROUTES = {
  HEALTH: "/api/health",
  SCAN: "/api/scan",
  SNIPPET: "/api/snippet",
  REFACTOR: "/api/refactor",
  REFACTOR_CLUSTER: "/api/refactor-cluster",
  APPLY_PATCH: "/api/apply-patch",
  EVENTS: "/api/events",
  TIMELINE: "/api/timeline",
  HOOKS: "/api/workflow/hooks",
  HOOKS_INSTALL: "/api/workflow/hooks/install",
  SUPPRESSION_RULES: "/api/suppression/rules",
  REFACTOR_SANDBOX: "/api/refactor/sandbox",
  REFACTOR_APPLY_BRANCH: "/api/refactor/apply-branch",
  REFACTOR_AI_PROMPT: "/api/refactor/ai-prompt",
  REFACTOR_AST: "/api/refactor/ast",
  REFACTOR_VERIFY: "/api/refactor/verify",
  REFACTOR_HEAL: "/api/refactor/heal",
  CACHE_EXPORT: "/api/cache/export",
  CACHE_IMPORT: "/api/cache/import",
  MONOREPO_SCAN: "/api/monorepo/scan",
  POLICY_RULES: "/api/policy/rules",
  POLICY_EVALUATE: "/api/policy/evaluate",
  SEMANTIC_GRAPH: "/api/semantic-graph",
  SEMANTIC_SCAN: "/api/semantic/scan",
  SEMANTIC_NEURAL: "/api/semantic/neural",
  WATCH_STATUS: "/api/watch/status",
  WATCH_TOGGLE: "/api/watch/toggle",
  WATCH_RESCAN: "/api/watch/rescan",
  EXTRACT_PREVIEW: "/api/extract/preview",
  EXTRACT_APPLY: "/api/extract/apply",
  HUB_CONFIG: "/api/hub/config",
  HUB_SCAN: "/api/hub/scan",
  HUB_EXTRACT: "/api/hub/extract",
  COVERAGE_INGEST: "/api/coverage/ingest",
  COVERAGE_CORRELATE: "/api/coverage/correlate",
  DEAD_CODE_SCAN: "/api/dead-code/scan",
  DEAD_CODE: "/api/dead-code",
} as const;

/**
 * Default parameters for autonomous AI healing refactoring.
 */
export const DEFAULT_HEAL_CONFIG = {
  max_iterations: 3,
  verify: true,
  default_provider: "Mock" as const,
  default_gemini_model: "gemini-2.5-pro",
  default_claude_model: "claude-3-7-sonnet",
  default_openai_model: "gpt-4.5-preview",
  default_ollama_model: "qwen2.5-coder",
  default_ollama_endpoint: "http://localhost:11434",
} as const;

/**
 * Default quality gate threshold (5.0%).
 */
export const DEFAULT_FAIL_THRESHOLD = 5.0;

/**
 * Default fallback parameters for scanning codebases.
 */
export const DEFAULT_SCAN_CONFIG: ScanConfig = {
  directory: ".",
  min_tokens: 50,
  languages: [],
  ignore_patterns: [
    "node_modules",
    "target",
    ".git",
    "dist",
    "build",
    ".logs",
    "packaging",
    "npm",
    "editors",
    "test-results",
    "tests",
    ".test.",
    ".spec.",
  ],
  detect_type2: true,
  detect_type3: true,
  scan_self: true,
  enable_git_blame: true,
  ignore_tests: false,
  ignore_mocks: false,
  ignore_generated: true,
  cross_language: true,
};

/**
 * All supported clone type classifications.
 */
export const ALL_CLONE_TYPES: CloneType[] = ["Exact", "Renamed", "NearMiss", "Semantic"];

/**
 * Scan execution phases in chronological order.
 */
export const ALL_SCAN_PHASES: ScanPhase[] = [
  "Discovery",
  "Tokenization",
  "AstAnalysis",
  "Indexing",
  "Merging",
  "Scoring",
  "Complete",
  "Cancelled",
  "Failed",
];
