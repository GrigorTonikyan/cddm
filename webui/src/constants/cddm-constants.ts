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
  POLICY_RULES: "/api/policy/rules",
  POLICY_EVALUATE: "/api/policy/evaluate",
} as const;

/**
 * Default fallback parameters for scanning codebases.
 */
export const DEFAULT_SCAN_CONFIG: ScanConfig = {
  directory: ".",
  min_tokens: 50,
  languages: [],
  ignore_patterns: ["node_modules", "target", ".git", "dist", "build", ".logs"],
  detect_type2: true,
  scan_self: true,
  enable_git_blame: true,
  ignore_tests: false,
  ignore_mocks: false,
  ignore_generated: true,
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
