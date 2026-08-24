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
  isTimelineModalOpen: boolean;
  isSuppressionModalOpen: boolean;
  isRefactorSandboxOpen: boolean;

  /** Historical timeline data and loading state */
  timelineData: import("../types/cddm-types").TimelineTrend | null;
  isTimelineLoading: boolean;
  timelineError: string | null;
  hookStatus: import("../types/cddm-types").HookStatus | null;

  /** Suppression rules state */
  suppressionConfig: import("../types/cddm-types").SuppressionConfig | null;
  isSuppressionLoading: boolean;
  suppressionError: string | null;

  /** Refactor sandbox state */
  sandboxRequest: import("../types/cddm-types").RefactorSandboxRequest | null;
  sandboxResult: import("../types/cddm-types").RefactorSandboxResult | null;
  isSandboxLoading: boolean;
  sandboxError: string | null;

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

  /** Fetches Git timeline historical duplication data */
  fetchTimeline: (directory?: string, maxSamples?: number, minTokens?: number) => Promise<void>;
  /** Fetches Git hook installation status */
  fetchHookStatus: (directory?: string) => Promise<void>;
  /** Installs a Git hook for automated quality enforcement */
  installHook: (hookType: string, failThreshold?: number, minTokens?: number) => Promise<string>;

  /** Fetches active suppression configuration */
  fetchSuppressionRules: () => Promise<void>;
  /** Saves updated suppression rules and .cddmignore */
  saveSuppressionRules: (config: import("../types/cddm-types").SuppressionConfig) => Promise<void>;
  /** Opens the interactive refactor sandbox modal and loads preview */
  openRefactorSandbox: (req: import("../types/cddm-types").RefactorSandboxRequest) => Promise<void>;
  /** Runs sandbox preview with updated parameters */
  previewRefactorSandbox: (
    req: import("../types/cddm-types").RefactorSandboxRequest,
  ) => Promise<import("../types/cddm-types").RefactorSandboxResult>;
  /** Applies refactoring patch to workspace or dedicated Git branch */
  applyRefactorBranch: (
    patch: string,
    branchName?: string,
    createBranch?: boolean,
  ) => Promise<import("../types/cddm-types").ApplyRefactorBranchResult>;

  /** Modal visibility setters */
  setIsScanConfigOpen: (open: boolean) => void;
  setIsHealthAuditOpen: (open: boolean) => void;
  setIsExportReportOpen: (open: boolean) => void;
  setIsTreemapModalOpen: (open: boolean) => void;
  setIsLanguageModalOpen: (open: boolean) => void;
  setIsClusterRefactorModalOpen: (open: boolean) => void;
  setIsTimelineModalOpen: (open: boolean) => void;
  setIsSuppressionModalOpen: (open: boolean) => void;
  setIsRefactorSandboxOpen: (open: boolean) => void;
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
  isTimelineModalOpen: false,
  isSuppressionModalOpen: false,
  isRefactorSandboxOpen: false,

  timelineData: null,
  isTimelineLoading: false,
  timelineError: null,
  hookStatus: null,

  suppressionConfig: null,
  isSuppressionLoading: false,
  suppressionError: null,

  sandboxRequest: null,
  sandboxResult: null,
  isSandboxLoading: false,
  sandboxError: null,

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

  fetchSuppressionRules: async () => {
    set({ isSuppressionLoading: true, suppressionError: null });
    try {
      const res = await fetch(API_ROUTES.SUPPRESSION_RULES);
      if (!res.ok) {
        throw new Error(`Failed to fetch suppression rules (${res.status})`);
      }
      const data = await res.json();
      set({ suppressionConfig: data, isSuppressionLoading: false, suppressionError: null });
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to load suppression rules";
      set({ suppressionError: msg, isSuppressionLoading: false });
    }
  },

  saveSuppressionRules: async (config) => {
    set({ isSuppressionLoading: true, suppressionError: null });
    try {
      const res = await fetch(API_ROUTES.SUPPRESSION_RULES, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(config),
      });
      if (!res.ok) {
        throw new Error(`Failed to save suppression rules (${res.status})`);
      }
      set({ suppressionConfig: config, isSuppressionLoading: false, suppressionError: null });
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to save suppression rules";
      set({ suppressionError: msg, isSuppressionLoading: false });
      throw err;
    }
  },

  openRefactorSandbox: async (req) => {
    set({
      isRefactorSandboxOpen: true,
      sandboxRequest: req,
      sandboxResult: null,
      isSandboxLoading: true,
      sandboxError: null,
    });
    try {
      const res = await fetch(API_ROUTES.REFACTOR_SANDBOX, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(req),
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Sandbox simulation failed (${res.status}): ${errorText}`);
      }
      const result = await res.json();
      set({ sandboxResult: result, isSandboxLoading: false, sandboxError: null });
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to run sandbox simulation";
      set({ sandboxError: msg, isSandboxLoading: false });
    }
  },

  previewRefactorSandbox: async (req) => {
    set({ isSandboxLoading: true, sandboxError: null });
    try {
      const res = await fetch(API_ROUTES.REFACTOR_SANDBOX, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(req),
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Sandbox simulation failed (${res.status}): ${errorText}`);
      }
      const result = await res.json();
      set({
        sandboxRequest: req,
        sandboxResult: result,
        isSandboxLoading: false,
        sandboxError: null,
      });
      return result;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to run sandbox simulation";
      set({ sandboxError: msg, isSandboxLoading: false });
      throw err;
    }
  },

  applyRefactorBranch: async (patch, branchName, createBranch = true) => {
    try {
      const res = await fetch(API_ROUTES.REFACTOR_APPLY_BRANCH, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          patch,
          branch_name: branchName,
          create_branch: createBranch,
        }),
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Branch application failed (${res.status}): ${errorText}`);
      }
      const result = await res.json();
      set({ patchStatusMessage: result.message });
      return result;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to apply refactor branch";
      set({ patchStatusMessage: msg });
      throw err;
    }
  },

  fetchTimeline: async (directory?: string, maxSamples: number = 10, minTokens?: number) => {
    set({ isTimelineLoading: true, timelineError: null });
    const { config } = get();
    const dir = directory ?? config.directory;
    const tokens = minTokens ?? config.min_tokens;

    try {
      const params = new URLSearchParams({
        directory: dir,
        max_samples: maxSamples.toString(),
        min_tokens: tokens.toString(),
      });
      const res = await fetch(`${API_ROUTES.TIMELINE}?${params.toString()}`);
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Failed to fetch timeline (${res.status}): ${errorText}`);
      }
      const data = await res.json();
      set({ timelineData: data, isTimelineLoading: false, timelineError: null });
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to load timeline trend";
      set({ timelineError: message, isTimelineLoading: false });
    }
  },

  fetchHookStatus: async (directory?: string) => {
    const { config } = get();
    const dir = directory ?? config.directory;
    try {
      const params = new URLSearchParams({ directory: dir });
      const res = await fetch(`${API_ROUTES.HOOKS}?${params.toString()}`);
      if (res.ok) {
        const data = await res.json();
        set({ hookStatus: data });
      }
    } catch {
      // ignore
    }
  },

  installHook: async (hookType: string, failThreshold: number = 15.0, minTokens?: number) => {
    const { config } = get();
    const tokens = minTokens ?? config.min_tokens;
    const res = await fetch(API_ROUTES.HOOKS_INSTALL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        directory: config.directory,
        hook_type: hookType,
        fail_threshold: failThreshold,
        min_tokens: tokens,
      }),
    });

    if (!res.ok) {
      const errorText = await res.text().catch(() => res.statusText);
      throw new Error(`Hook installation failed (${res.status}): ${errorText}`);
    }

    const data = await res.json();
    await get().fetchHookStatus();
    return data.message || "Hook installed successfully";
  },

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
      isTimelineModalOpen: false,
      isSuppressionModalOpen: false,
      isRefactorSandboxOpen: false,
      timelineData: null,
      timelineError: null,
      suppressionConfig: null,
      suppressionError: null,
      sandboxRequest: null,
      sandboxResult: null,
      sandboxError: null,
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
