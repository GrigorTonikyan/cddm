import { API_ROUTES } from "../../constants/cddm-constants";
import type {
  CoverageCorrelateRequest,
  CoverageCorrelationSummary,
  CoverageIngestRequest,
} from "../../types/cddm-types";
import { postJson } from "../../utils/api-client";
import type { GetStoreState, SetStoreState } from "./scan-slice";

export const createCoverageSlice = (set: SetStoreState, _get: GetStoreState) => ({
  isCoverageModalOpen: false,
  coverageSummary: null as CoverageCorrelationSummary | null,
  isCoverageLoading: false,
  coverageError: null as string | null,

  setIsCoverageModalOpen: (open: boolean) => set({ isCoverageModalOpen: open }),

  ingestCoverageReport: async (req: CoverageIngestRequest): Promise<void> => {
    set({ isCoverageLoading: true, coverageError: null });
    try {
      await postJson<unknown>(
        API_ROUTES.COVERAGE_INGEST,
        req,
        "Coverage tracefile ingestion failed",
      );
      set({ isCoverageLoading: false });
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to ingest coverage tracefile";
      set({ coverageError: msg, isCoverageLoading: false });
      throw err;
    }
  },

  correlateCoverage: async (
    req?: CoverageCorrelateRequest,
  ): Promise<CoverageCorrelationSummary> => {
    set({ isCoverageLoading: true, coverageError: null });
    try {
      const data = await postJson<CoverageCorrelationSummary>(
        API_ROUTES.COVERAGE_CORRELATE,
        req ?? {},
        "Coverage correlation failed",
      );
      set({ coverageSummary: data, isCoverageLoading: false });
      return data;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to correlate coverage with clones";
      set({ coverageError: msg, isCoverageLoading: false });
      return {
        total_clone_pairs: 0,
        overall_covered_clones_pct: 0,
        dead_code_clones: 0,
        test_gap_clones: 0,
        hot_path_clones: 0,
        total_runtime_hits: 0,
        metrics: [],
      };
    }
  },
});
