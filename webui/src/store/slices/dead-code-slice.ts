import { API_ROUTES } from "../../constants/cddm-constants";
import type {
  DeadClonePruneRequest,
  DeadClonePruneResult,
  DeadCodeScanRequest,
  DeadCodeSummary,
} from "../../types/dead-code-types";
import { postJson } from "../../utils/api-client";
import type { GetStoreState, SetStoreState } from "./scan-slice";

export const createDeadCodeSlice = (set: SetStoreState, _get: GetStoreState) => ({
  isDeadCodeModalOpen: false,
  deadCodeSummary: null as DeadCodeSummary | null,
  isDeadCodeLoading: false,
  deadCodeError: null as string | null,
  isDeadCodePruning: false,
  lastPruneResult: null as DeadClonePruneResult | null,
  deadCodePruneError: null as string | null,

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

  pruneDeadCode: async (req?: DeadClonePruneRequest): Promise<DeadClonePruneResult> => {
    set({ isDeadCodePruning: true, deadCodePruneError: null });
    try {
      const data = await postJson<DeadClonePruneResult>(
        API_ROUTES.DEAD_CODE_PRUNE,
        req ?? {},
        "Dead clone cluster pruning failed",
      );
      set({ lastPruneResult: data, isDeadCodePruning: false });
      return data;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to prune dead clone clusters";
      set({ deadCodePruneError: msg, isDeadCodePruning: false });
      return {
        total_candidates: 0,
        pruned_items: 0,
        skipped_items: 0,
        total_lines_removed: 0,
        dry_run: req?.dry_run ?? false,
        files_affected: [],
        items: [],
      };
    }
  },
});
