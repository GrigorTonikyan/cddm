import { lazyModal } from "../utils/lazy-modal";

export const CoverageCorrelationModal = lazyModal(
  () => import("./CoverageCorrelationModal"),
  "CoverageCorrelationModal",
);
export const DeadCodeExplorerModal = lazyModal(
  () => import("./DeadCodeExplorerModal"),
  "DeadCodeExplorerModal",
);
export const HubFederationModal = lazyModal(
  () => import("./HubFederationModal"),
  "HubFederationModal",
);
export const OverlapDetectorModal = lazyModal(
  () => import("./OverlapDetectorModal"),
  "OverlapDetectorModal",
);
export const PolicyRulesModal = lazyModal(() => import("./PolicyRulesModal"), "PolicyRulesModal");
export const RefactorSandboxModal = lazyModal(
  () => import("./RefactorSandboxModal"),
  "RefactorSandboxModal",
);
export const ScanConfigModal = lazyModal(() => import("./ScanConfigModal"), "ScanConfigModal");
export const SemanticGraphModal = lazyModal(
  () => import("./SemanticGraphModal"),
  "SemanticGraphModal",
);
export const SuppressionRulesModal = lazyModal(
  () => import("./SuppressionRulesModal"),
  "SuppressionRulesModal",
);
export const TimelineExplorerModal = lazyModal(
  () => import("./TimelineExplorerModal"),
  "TimelineExplorerModal",
);
export const HookManagerModal = lazyModal(() => import("./HookManagerModal"), "HookManagerModal");
export const MonorepoWorkspaceModal = lazyModal(
  () => import("./MonorepoWorkspaceModal"),
  "MonorepoWorkspaceModal",
);
export const LiveEventInspectorModal = lazyModal(
  () => import("./watch/LiveEventInspectorModal"),
  "LiveEventInspectorModal",
);
export const DiffScanResultsModal = lazyModal(
  () => import("./DiffScanResultsModal"),
  "DiffScanResultsModal",
);
export const McpAppsPreviewModal = lazyModal(
  () => import("./McpAppsPreviewModal"),
  "McpAppsPreviewModal",
);
export const CodeEditorModal = lazyModal(() => import("./CodeEditorModal"), "CodeEditorModal");
