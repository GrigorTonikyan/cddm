import { API_ROUTES } from "../../constants/cddm-constants";
import type {
  HubConfig,
  HubExtractRequest,
  HubExtractResult,
  HubScanSummary,
} from "../../types/cddm-types";
import { getJson, postJson } from "../../utils/api-client";
import type { GetStoreState, SetStoreState } from "./scan-slice";

export const createHubSlice = (set: SetStoreState, _get: GetStoreState) => ({
  isHubModalOpen: false,
  hubConfig: null as HubConfig | null,
  hubSummary: null as HubScanSummary | null,
  isHubLoading: false,
  hubError: null as string | null,

  setIsHubModalOpen: (open: boolean) => set({ isHubModalOpen: open }),

  fetchHubConfig: async (): Promise<void> => {
    set({ isHubLoading: true, hubError: null });
    try {
      const data = await getJson<HubConfig>(
        API_ROUTES.HUB_CONFIG,
        "Failed to load Hub configuration",
      );
      set({ hubConfig: data, isHubLoading: false });
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to load Hub configuration";
      set({ hubError: msg, isHubLoading: false });
    }
  },

  saveHubConfig: async (config: HubConfig): Promise<void> => {
    set({ isHubLoading: true, hubError: null });
    try {
      await postJson<unknown>(API_ROUTES.HUB_CONFIG, config, "Failed to save Hub configuration");
      set({ hubConfig: config, isHubLoading: false });
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to save Hub configuration";
      set({ hubError: msg, isHubLoading: false });
      throw err;
    }
  },

  runHubScan: async (config?: HubConfig): Promise<HubScanSummary> => {
    set({ isHubLoading: true, hubError: null });
    try {
      const data = await postJson<HubScanSummary>(
        API_ROUTES.HUB_SCAN,
        { hub_config: config },
        "Hub scan failed",
      );
      set({ hubSummary: data, isHubLoading: false });
      return data;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to execute Organization Hub scan";
      set({ hubError: msg, isHubLoading: false });
      return {
        hub_name: "Federation Hub",
        total_repos: 0,
        total_files: 0,
        total_tokens: 0,
        organization_dry_score: 100.0,
        repos: [],
        duplication_matrix: [],
        clusters: [],
      };
    }
  },

  extractHubPackage: async (req: HubExtractRequest): Promise<HubExtractResult> => {
    set({ isHubLoading: true, hubError: null });
    try {
      const data = await postJson<HubExtractResult>(
        API_ROUTES.HUB_EXTRACT,
        req,
        "Hub package extraction failed",
      );
      set({ isHubLoading: false });
      return data;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to extract Hub shared package";
      set({ hubError: msg, isHubLoading: false });
      throw err;
    }
  },
});
