import { API_ROUTES } from "../../constants/cddm-constants";
import type { DeadCodeScanRequest, DeadCodeSummary } from "../../types/dead-code-types";
import { postJson } from "../../utils/api-client";
import type { GetStoreState, SetStoreState } from "./scan-slice";

export const createDeadCodeSlice = (set: SetStoreState, _get: GetStoreState) => ({
  isDeadCodeModalOpen: false,
  deadCodeSummary: null as DeadCodeSummary | null,
  isDeadCodeLoading: false,
  deadCodeError: null as string | null,

  setIsDeadCodeModalOpen: (open: boolean) => set({ isDeadCodeModalOpen: open }),

  scanDeadCode: async (req?: DeadCodeScanRequest): Promise<DeadCodeSummary> => {
    set({ isDeadCodeLoading: true, deadCodeError: null });
    try {
      const data = await postJson<DeadCodeSummary>(
        API_ROUTES.DEAD_CODE_SCAN,
        req ?? {},
        "Dead code analysis failed",
      );
      set({ deadCodeSummary: data, isDeadCodeLoading: false });
      return data;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to scan dead code";
      set({ deadCodeError: msg, isDeadCodeLoading: false });
      return {
        total_dead_items: 0,
        dead_functions: 0,
        unreachable_blocks: 0,
        dead_clones: 0,
        uncovered_items: 0,
        total_dead_lines: 0,
        estimated_savings_pct: 0,
        items: [],
      };
    }
  },
});
