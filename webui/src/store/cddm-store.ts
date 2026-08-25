import { create } from "zustand";
import { API_ROUTES, DEFAULT_SCAN_CONFIG } from "../constants/cddm-constants";
import type { ServerEvent } from "../types/cddm-types";
import { DEFAULT_EDITOR } from "../utils/ide-links";
import { createPolicySlice } from "./slices/policy-slice";
import { createRefactorSlice } from "./slices/refactor-slice";
import { createScanSlice } from "./slices/scan-slice";
import { createSemanticSlice } from "./slices/semantic-slice";
import { createTimelineSlice } from "./slices/timeline-slice";
import type { CDDMStoreState } from "./types";

export type { CDDMStoreState } from "./types";

/**
 * Global Zustand store for CDDM WebUI control plane.
 */
export const useCDDMStore = create<CDDMStoreState>((set, get) => ({
  config: DEFAULT_SCAN_CONFIG,
  activeScanId: null,
  progress: null,
  results: null,
  isScanning: false,
  error: null,

  viewMode: "pairs",
  selectedCluster: null,

  isLiveWatchActive: true,
  preferredEditor: DEFAULT_EDITOR,
  lastLiveSyncTimestamp: null,
  liveSyncCount: 0,
  isPatching: false,
  patchStatusMessage: null,

  isScanConfigOpen: false,
  isHealthAuditOpen: false,
  isExportReportOpen: false,
  isTreemapModalOpen: false,
  isLanguageModalOpen: false,
  isClusterRefactorModalOpen: false,
  isTimelineModalOpen: false,
  isSuppressionModalOpen: false,
  isRefactorSandboxOpen: false,
  isPolicyRulesModalOpen: false,
  isSemanticGraphModalOpen: false,

  semanticGraphRequest: null,
  semanticGraphResponse: null,
  isSemanticGraphLoading: false,
  semanticGraphError: null,

  timelineData: null,
  isTimelineLoading: false,
  timelineError: null,
  hookStatus: null,

  suppressionConfig: null,
  isSuppressionLoading: false,
  suppressionError: null,

  policyConfig: null,
  isPolicyLoading: false,
  policyError: null,

  sandboxRequest: null,
  sandboxResult: null,
  isSandboxLoading: false,
  sandboxError: null,

  astRewriteResult: null,
  isAstLoading: false,
  astError: null,

  verifyResult: null,
  isVerifying: false,
  verifyError: null,

  setViewMode: (viewMode) => set({ viewMode }),
  setSelectedCluster: (selectedCluster) => set({ selectedCluster }),

  setIsLiveWatchActive: (isLiveWatchActive) => set({ isLiveWatchActive }),
  setPreferredEditor: (preferredEditor) => set({ preferredEditor }),
  setPatchStatusMessage: (patchStatusMessage) => set({ patchStatusMessage }),

  setIsScanConfigOpen: (isScanConfigOpen) => set({ isScanConfigOpen }),
  setIsHealthAuditOpen: (isHealthAuditOpen) => set({ isHealthAuditOpen }),
  setIsExportReportOpen: (isExportReportOpen) => set({ isExportReportOpen }),
  setIsTreemapModalOpen: (isTreemapModalOpen) => set({ isTreemapModalOpen }),
  setIsLanguageModalOpen: (isLanguageModalOpen) => set({ isLanguageModalOpen }),
  setIsClusterRefactorModalOpen: (isClusterRefactorModalOpen) =>
    set({ isClusterRefactorModalOpen }),
  setIsTimelineModalOpen: (isTimelineModalOpen) => {
    set({ isTimelineModalOpen });
    if (isTimelineModalOpen && !get().timelineData && !get().isTimelineLoading) {
      void get().fetchTimeline();
      void get().fetchHookStatus();
    }
  },
  setIsSuppressionModalOpen: (isSuppressionModalOpen) => {
    set({ isSuppressionModalOpen });
    if (isSuppressionModalOpen && !get().suppressionConfig && !get().isSuppressionLoading) {
      void get().fetchSuppressionRules();
    }
  },
  setIsRefactorSandboxOpen: (isRefactorSandboxOpen) => set({ isRefactorSandboxOpen }),
  setIsPolicyRulesModalOpen: (isPolicyRulesModalOpen) => {
    set({ isPolicyRulesModalOpen });
    if (isPolicyRulesModalOpen && !get().policyConfig && !get().isPolicyLoading) {
      void get().fetchPolicyRules();
    }
  },
  setIsSemanticGraphModalOpen: (isSemanticGraphModalOpen) => set({ isSemanticGraphModalOpen }),

  ...createScanSlice(set, get),
  ...createTimelineSlice(set, get),
  ...createPolicySlice(set, get),
  ...createRefactorSlice(set, get),
  ...createSemanticSlice(set, get),
}));

let eventSourceInstance: EventSource | null = null;

/**
 * Initializes auto-reconnecting Server-Sent Events live watch subscription.
 */
export function connectLiveWatchSSE(): void {
  if (typeof window === "undefined" || !("EventSource" in window)) return;
  if (eventSourceInstance) {
    eventSourceInstance.close();
  }

  try {
    const es = new EventSource(API_ROUTES.EVENTS);
    eventSourceInstance = es;

    es.onmessage = (e) => {
      try {
        const event: ServerEvent = JSON.parse(e.data);
        const { isLiveWatchActive } = useCDDMStore.getState();
        if (!isLiveWatchActive) return;

        if (event.type === "scan_started") {
          useCDDMStore.setState({
            isScanning: true,
            activeScanId: event.payload.scan_id,
            error: null,
          });
        } else if (event.type === "scan_progress") {
          useCDDMStore.setState({ progress: event.payload });
        } else if (event.type === "scan_complete") {
          const prevCount = useCDDMStore.getState().liveSyncCount;
          useCDDMStore.setState({
            results: event.payload,
            isScanning: false,
            activeScanId: event.payload.scan_id,
            lastLiveSyncTimestamp: Date.now(),
            liveSyncCount: prevCount + 1,
            error: null,
          });
        } else if (event.type === "patch_applied") {
          useCDDMStore.setState({
            patchStatusMessage: event.payload.message,
          });
        }
      } catch {
        // ignore parse error
      }
    };
  } catch {
    // SSE initialization error fallback
  }
}

if (typeof window !== "undefined") {
  connectLiveWatchSSE();
}
