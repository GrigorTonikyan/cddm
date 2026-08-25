import { API_ROUTES } from "../../constants/cddm-constants";
import type { ApplyPatchResult, ScanResult } from "../../types/cddm-types";
import type { CDDMStoreState } from "../types";

export type SetStoreState = (
  partial: Partial<CDDMStoreState> | ((state: CDDMStoreState) => Partial<CDDMStoreState>),
) => void;
export type GetStoreState = () => CDDMStoreState;

export const createScanSlice = (set: SetStoreState, get: GetStoreState) => ({
  setConfig: (newConfig: Partial<CDDMStoreState["config"]>) => {
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
      isPolicyRulesModalOpen: false,
      timelineData: null,
      timelineError: null,
      suppressionConfig: null,
      suppressionError: null,
      policyConfig: null,
      policyError: null,
      sandboxRequest: null,
      sandboxResult: null,
      sandboxError: null,
      astRewriteResult: null,
      astError: null,
      verifyResult: null,
      verifyError: null,
    });
  },
});
