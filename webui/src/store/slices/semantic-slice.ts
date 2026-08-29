import { API_ROUTES } from "../../constants/cddm-constants";
import type {
  CrossLanguageClonePair,
  NeuralScanResult,
  SemanticGraphRequest,
  SemanticGraphResponse,
  SemanticNeuralRequest,
} from "../../types/cddm-types";
import { postJson } from "../../utils/api-client";
import type { GetStoreState, SetStoreState } from "./scan-slice";

export const createSemanticSlice = (set: SetStoreState, get: GetStoreState) => ({
  crossLanguageClones: [] as CrossLanguageClonePair[],
  isCrossLanguageLoading: false,
  neuralResult: null as NeuralScanResult | null,
  isNeuralLoading: false,

  fetchSemanticGraph: async (req: SemanticGraphRequest): Promise<SemanticGraphResponse> => {
    set({ isSemanticGraphLoading: true, semanticGraphError: null });
    try {
      const data = await postJson<SemanticGraphResponse>(
        API_ROUTES.SEMANTIC_GRAPH,
        req,
        "Semantic graph extraction failed",
      );
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
      const pairs = await postJson<CrossLanguageClonePair[]>(
        API_ROUTES.SEMANTIC_SCAN,
        { directory, threshold },
        "Cross-language scan failed",
      );
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

  scanNeuralClones: async (
    req: SemanticNeuralRequest = { directory: ".", threshold: 0.85 },
  ): Promise<NeuralScanResult> => {
    set({ isNeuralLoading: true });
    try {
      const result = await postJson<NeuralScanResult>(
        API_ROUTES.SEMANTIC_NEURAL,
        req,
        "Neural scan failed",
      );
      set({
        neuralResult: result,
        isNeuralLoading: false,
      });
      return result;
    } catch (err) {
      set({ isNeuralLoading: false });
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
