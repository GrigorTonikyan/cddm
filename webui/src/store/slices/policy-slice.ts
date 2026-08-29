import { API_ROUTES } from "../../constants/cddm-constants";
import type {
  PolicyConfig,
  PolicyEvaluationResult,
  SuppressionConfig,
} from "../../types/cddm-types";
import { getJson, postJson } from "../../utils/api-client";
import type { GetStoreState, SetStoreState } from "./scan-slice";

export const createPolicySlice = (set: SetStoreState, get: GetStoreState) => ({
  fetchPolicyRules: async () => {
    set({ isPolicyLoading: true, policyError: null });
    try {
      const data = await getJson<PolicyConfig>(
        API_ROUTES.POLICY_RULES,
        "Failed to fetch policy rules",
      );
      set({ policyConfig: data, isPolicyLoading: false, policyError: null });
    } catch (err) {
      set({
        policyError: err instanceof Error ? err.message : "Failed to load policy rules",
        isPolicyLoading: false,
      });
    }
  },

  savePolicyRules: async (config: PolicyConfig) => {
    set({ isPolicyLoading: true, policyError: null });
    try {
      await postJson(API_ROUTES.POLICY_RULES, config, "Failed to save policy rules");
      set({ policyConfig: config, isPolicyLoading: false, policyError: null });
    } catch (err) {
      set({
        policyError: err instanceof Error ? err.message : "Failed to save policy rules",
        isPolicyLoading: false,
      });
      throw err;
    }
  },

  evaluatePolicyRules: async (directory?: string): Promise<PolicyEvaluationResult> => {
    const { config } = get();
    const dir = directory ?? config.directory;
    return await postJson<PolicyEvaluationResult>(
      API_ROUTES.POLICY_EVALUATE,
      { directory: dir },
      "Policy evaluation failed",
    );
  },

  fetchSuppressionRules: async () => {
    set({ isSuppressionLoading: true, suppressionError: null });
    try {
      const data = await getJson<SuppressionConfig>(
        API_ROUTES.SUPPRESSION_RULES,
        "Failed to fetch suppression rules",
      );
      set({ suppressionConfig: data, isSuppressionLoading: false, suppressionError: null });
    } catch (err) {
      set({
        suppressionError: err instanceof Error ? err.message : "Failed to load suppression rules",
        isSuppressionLoading: false,
      });
    }
  },

  saveSuppressionRules: async (config: SuppressionConfig) => {
    set({ isSuppressionLoading: true, suppressionError: null });
    try {
      await postJson(API_ROUTES.SUPPRESSION_RULES, config, "Failed to save suppression rules");
      set({ suppressionConfig: config, isSuppressionLoading: false, suppressionError: null });
    } catch (err) {
      set({
        suppressionError: err instanceof Error ? err.message : "Failed to save suppression rules",
        isSuppressionLoading: false,
      });
      throw err;
    }
  },
});
