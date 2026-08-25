/**
 * Type definitions and message protocols for CDDM VS Code Extension.
 */

export interface WorkspaceHealthStats {
  dryHealthScore: number;
  totalFiles: number;
  totalTokens: number;
  clonePairsCount: number;
  cloneClustersCount: number;
  policyViolationsCount: number;
  duplicationPercentage: number;
  languages: Array<{ language: string; fileCount: number; tokenCount: number }>;
}

export type WebviewIncomingMessage =
  | { type: "openLocation"; file: string; startLine: number; endLine: number }
  | { type: "rescanWorkspace" }
  | { type: "checkPolicies" }
  | { type: "exportSarif" }
  | { type: "openExternalStudio" }
  | { type: "openEmbeddedStudio" }
  | { type: "copyText"; text: string }
  | { type: "requestHealthUpdate" };

export type WebviewOutgoingMessage =
  | { type: "healthUpdate"; stats: WorkspaceHealthStats }
  | { type: "scanProgress"; phase: string; progress: number }
  | { type: "themeChanged"; kind: "dark" | "light" | "high-contrast" };
