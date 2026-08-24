import { create } from "zustand";
import { API_ROUTES, DEFAULT_SCAN_CONFIG } from "../constants/cddm-constants";
import {
  ApplyPatchResult,
  CloneCluster,
  ScanConfig,
  ScanProgress,
  ScanResult,
  ServerEvent,
} from "../types/cddm-types";
import { DEFAULT_EDITOR, SupportedEditor } from "../utils/ide-links";

/**
 * Interface for CDDM Zustand Store State and Actions.
 */
export interface CDDMStoreState {
  /** Current scan configuration */
  config: ScanConfig;
  /** Active scan ID or null if idle */
  activeScanId: string | null;
  /** Active scan progress details */
  progress: ScanProgress | null;
  /** Final scan results if completed */
  results: ScanResult | null;
  /** Whether a scan is currently running */
  isScanning: boolean;
  /** Error message if scan failed */
  error: string | null;

  /** Active view mode for results list (pairwise vs n-way clusters) */
  viewMode: "pairs" | "clusters";
  /** Currently selected cluster for inspection or refactoring */
  selectedCluster: CloneCluster | null;

  /** Real-time live watch & push sync status */
  isLiveWatchActive: boolean;
  /** Preferred IDE editor for protocol deeplinks */
  preferredEditor: SupportedEditor;
  /** Timestamp of the most recent live push synchronization */
  lastLiveSyncTimestamp: number | null;
  /** Whether a patch is currently being applied to workspace */
  isPatching: boolean;
  /** Status notification message for patch operations */
  patchStatusMessage: string | null;

  /** Global window modal visibility states */
  isScanConfigOpen: boolean;
  isHealthAuditOpen: boolean;
  isExportReportOpen: boolean;
  isTreemapModalOpen: boolean;
  isLanguageModalOpen: boolean;
  isClusterRefactorModalOpen: boolean;

  /** Updates the scan configuration */
  setConfig: (config: Partial<ScanConfig>) => void;
  /** Initiates a new code duplication scan */
  startScan: () => Promise<void>;
  /** Cancels an ongoing scan */
  cancelScan: () => void;
  /** Resets state to idle */
  resetScan: () => void;

  /** View mode and cluster setters */
  setViewMode: (viewMode: "pairs" | "clusters") => void;
  setSelectedCluster: (selectedCluster: CloneCluster | null) => void;

  /** Live watch & IDE preferences setters */
  setIsLiveWatchActive: (active: boolean) => void;
  setPreferredEditor: (editor: SupportedEditor) => void;
  setPatchStatusMessage: (msg: string | null) => void;
  /** Applies synthesized refactoring patch directly to workspace */
  applyPatch: (patch: string, dryRun?: boolean) => Promise<ApplyPatchResult>;

  /** Modal visibility setters */
  setIsScanConfigOpen: (open: boolean) => void;
  setIsHealthAuditOpen: (open: boolean) => void;
  setIsExportReportOpen: (open: boolean) => void;
  setIsTreemapModalOpen: (open: boolean) => void;
  setIsLanguageModalOpen: (open: boolean) => void;
  setIsClusterRefactorModalOpen: (open: boolean) => void;
}

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
  isPatching: false,
  patchStatusMessage: null,

  isScanConfigOpen: false,
  isHealthAuditOpen: false,
  isExportReportOpen: false,
  isTreemapModalOpen: false,
  isLanguageModalOpen: false,
  isClusterRefactorModalOpen: false,

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

  setConfig: (newConfig) => {
    set((state) => ({
      config: { ...state.config, ...newConfig },
    }));
  },

  startScan: async () => {
    set({ isScanning: true, error: null, results: null, progress: null });
    const { config } = get();

    try {
      const res = await fetch(API_ROUTES.SCAN, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(config),
      });

      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Scan request failed (${res.status}): ${errorText || res.statusText}`);
      }

      const results: ScanResult = await res.json();
      set({ results, isScanning: false, activeScanId: results.scan_id, error: null });
    } catch (err) {
      set({
        isScanning: false,
        results: null,
        error: err instanceof Error ? err.message : "Scan execution failed",
      });
    }
  },

  applyPatch: async (patch: string, dryRun: boolean = false) => {
    set({ isPatching: true, error: null });
    try {
      const res = await fetch(API_ROUTES.APPLY_PATCH, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ patch, dry_run: dryRun }),
      });

      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Patch application failed (${res.status}): ${errorText || res.statusText}`);
      }

      const result: ApplyPatchResult = await res.json();
      set({
        isPatching: false,
        patchStatusMessage: result.message,
      });
      return result;
    } catch (err) {
      const message = err instanceof Error ? err.message : "Patch application failed";
      set({ isPatching: false, error: message });
      throw err;
    }
  },

  cancelScan: () => {
    set({ isScanning: false, progress: null, error: "Scan cancelled" });
  },

  resetScan: () => {
    set({
      results: null,
      progress: null,
      isScanning: false,
      error: null,
      selectedCluster: null,
      patchStatusMessage: null,
      isScanConfigOpen: false,
      isHealthAuditOpen: false,
      isExportReportOpen: false,
      isTreemapModalOpen: false,
      isLanguageModalOpen: false,
      isClusterRefactorModalOpen: false,
    });
  },
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
          useCDDMStore.setState({
            results: event.payload,
            isScanning: false,
            activeScanId: event.payload.scan_id,
            lastLiveSyncTimestamp: Date.now(),
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
