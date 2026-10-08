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
      if (results && Array.isArray(results.clone_pairs)) {
        set({ results, isScanning: false, activeScanId: results.scan_id, error: null });
      } else {
        set({ isScanning: false, results: null });
      }
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
      isHookManagerModalOpen: false,
      isMonorepoModalOpen: false,
      monorepoData: null,
      monorepoError: null,
    });
  },

  runMonorepoScan: async (directory?: string, minTokens?: number) => {
    set({ isMonorepoLoading: true, monorepoError: null });
    const { config } = get();
    const dir = directory ?? config.directory;
    const tokens = minTokens ?? config.min_tokens;

    try {
      const res = await fetch(API_ROUTES.MONOREPO, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ directory: dir, min_tokens: tokens }),
      });

      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Monorepo scan failed (${res.status}): ${errorText || res.statusText}`);
      }

      const summary = (await res.json()) as import("../../types/cddm-types").MonorepoScanSummary;
      set({ monorepoData: summary, isMonorepoLoading: false, monorepoError: null });
      return summary;
    } catch (err) {
      const message = err instanceof Error ? err.message : "Monorepo scan failed";
      set({ isMonorepoLoading: false, monorepoError: message });
      throw err;
    }
  },

  exportCachePack: (cacheDir?: string, outputPackPath?: string) =>
    sendCachePackRequest(
      API_ROUTES.CACHE_EXPORT,
      { cache_dir: cacheDir, output_pack_path: outputPackPath },
      "Cache export failed",
    ),

  importCachePack: (packFile: string, targetCacheDir?: string) =>
    sendCachePackRequest(
      API_ROUTES.CACHE_IMPORT,
      { pack_file: packFile, target_cache_dir: targetCacheDir },
      "Cache import failed",
    ),
});

async function sendCachePackRequest(
  url: string,
  payload: Record<string, unknown>,
  errorContext: string,
): Promise<import("../../types/cddm-types").CachePackSummary> {
  const res = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    const errorText = await res.text().catch(() => res.statusText);
    throw new Error(`${errorContext} (${res.status}): ${errorText || res.statusText}`);
  }

  return (await res.json()) as import("../../types/cddm-types").CachePackSummary;
}
