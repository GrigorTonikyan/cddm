import { create } from "zustand";
import { API_ROUTES, DEFAULT_SCAN_CONFIG } from "../constants/cddm-constants";
import { CloneCluster, ScanConfig, ScanProgress, ScanResult } from "../types/cddm-types";

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

  isScanConfigOpen: false,
  isHealthAuditOpen: false,
  isExportReportOpen: false,
  isTreemapModalOpen: false,
  isLanguageModalOpen: false,
  isClusterRefactorModalOpen: false,

  setViewMode: (viewMode) => set({ viewMode }),
  setSelectedCluster: (selectedCluster) => set({ selectedCluster }),

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
      isScanConfigOpen: false,
      isHealthAuditOpen: false,
      isExportReportOpen: false,
      isTreemapModalOpen: false,
      isLanguageModalOpen: false,
      isClusterRefactorModalOpen: false,
    });
  },
}));
