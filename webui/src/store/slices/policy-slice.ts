import { API_ROUTES } from "../../constants/cddm-constants";
import type { PolicyConfig, SuppressionConfig } from "../../types/cddm-types";
import type { GetStoreState, SetStoreState } from "./scan-slice";

export const createPolicySlice = (set: SetStoreState, get: GetStoreState) => ({
  fetchPolicyRules: async () => {
    set({ isPolicyLoading: true, policyError: null });
    try {
      const res = await fetch(API_ROUTES.POLICY_RULES);
      if (!res.ok) {
        throw new Error(`Failed to fetch policy rules (${res.status})`);
      }
      const data = await res.json();
      set({ policyConfig: data, isPolicyLoading: false, policyError: null });
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to load policy rules";
      set({ policyError: msg, isPolicyLoading: false });
    }
  },

  savePolicyRules: async (config: PolicyConfig) => {
    set({ isPolicyLoading: true, policyError: null });
    try {
      const res = await fetch(API_ROUTES.POLICY_RULES, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(config),
      });
      if (!res.ok) {
        throw new Error(`Failed to save policy rules (${res.status})`);
      }
      set({ policyConfig: config, isPolicyLoading: false, policyError: null });
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to save policy rules";
      set({ policyError: msg, isPolicyLoading: false });
      throw err;
    }
  },

  evaluatePolicyRules: async (directory?: string) => {
    const { config } = get();
    const dir = directory ?? config.directory;
    const res = await fetch(API_ROUTES.POLICY_EVALUATE, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ directory: dir }),
    });
    if (!res.ok) {
      const errorText = await res.text().catch(() => res.statusText);
      throw new Error(`Policy evaluation failed (${res.status}): ${errorText}`);
    }
    return await res.json();
  },

  fetchSuppressionRules: async () => {
    set({ isSuppressionLoading: true, suppressionError: null });
    try {
      const res = await fetch(API_ROUTES.SUPPRESSION_RULES);
      if (!res.ok) {
        throw new Error(`Failed to fetch suppression rules (${res.status})`);
      }
      const data = await res.json();
      set({ suppressionConfig: data, isSuppressionLoading: false, suppressionError: null });
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to load suppression rules";
      set({ suppressionError: msg, isSuppressionLoading: false });
    }
  },

  saveSuppressionRules: async (config: SuppressionConfig) => {
    set({ isSuppressionLoading: true, suppressionError: null });
    try {
      const res = await fetch(API_ROUTES.SUPPRESSION_RULES, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(config),
      });
      if (!res.ok) {
        throw new Error(`Failed to save suppression rules (${res.status})`);
      }
      set({ suppressionConfig: config, isSuppressionLoading: false, suppressionError: null });
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to save suppression rules";
      set({ suppressionError: msg, isSuppressionLoading: false });
      throw err;
    }
  },
});
