import { API_ROUTES } from "../../constants/cddm-constants";
import type {
  CrossLanguageClonePair,
  SemanticGraphRequest,
  SemanticGraphResponse,
} from "../../types/cddm-types";
import type { GetStoreState, SetStoreState } from "./scan-slice";

export const createSemanticSlice = (set: SetStoreState, get: GetStoreState) => ({
  crossLanguageClones: [] as CrossLanguageClonePair[],
  isCrossLanguageLoading: false,

  fetchSemanticGraph: async (req: SemanticGraphRequest): Promise<SemanticGraphResponse> => {
    set({ isSemanticGraphLoading: true, semanticGraphError: null });
    try {
      const res = await fetch(API_ROUTES.SEMANTIC_GRAPH, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(req),
      });
      if (!res.ok) {
        const errText = await res.text().catch(() => res.statusText);
        throw new Error(`Semantic graph extraction failed (${res.status}): ${errText}`);
      }
      const data: SemanticGraphResponse = await res.json();
      set({
        semanticGraphRequest: req,
        semanticGraphResponse: data,
        isSemanticGraphLoading: false,
        semanticGraphError: null,
      });
      return data;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to extract semantic graph";
      set({ semanticGraphError: msg, isSemanticGraphLoading: false });
      throw err;
    }
  },

  scanCrossLanguageClones: async (
    threshold = 0.7,
    directory = ".",
  ): Promise<CrossLanguageClonePair[]> => {
    set({ isCrossLanguageLoading: true });
    try {
      const res = await fetch(API_ROUTES.SEMANTIC_SCAN, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ directory, threshold }),
      });
      if (!res.ok) {
        const errText = await res.text().catch(() => res.statusText);
        throw new Error(`Cross-language scan failed (${res.status}): ${errText}`);
      }
      const pairs: CrossLanguageClonePair[] = await res.json();
      set({
        crossLanguageClones: pairs,
        isCrossLanguageLoading: false,
      });
      return pairs;
    } catch (err) {
      set({ isCrossLanguageLoading: false });
      throw err;
    }
  },

  openSemanticGraphModal: async (req?: SemanticGraphRequest) => {
    set({ isSemanticGraphModalOpen: true });
    if (req) {
      void get().fetchSemanticGraph(req);
    }
  },
});
