import { API_ROUTES, DEFAULT_FAIL_THRESHOLD } from "../../constants/cddm-constants";
import type { GetStoreState, SetStoreState } from "./scan-slice";

export const createTimelineSlice = (set: SetStoreState, get: GetStoreState) => ({
  fetchTimeline: async (directory?: string, maxSamples: number = 10, minTokens?: number) => {
    set({ isTimelineLoading: true, timelineError: null });
    const { config } = get();
    const dir = directory ?? config.directory;
    const tokens = minTokens ?? config.min_tokens;

    try {
      const params = new URLSearchParams({
        directory: dir,
        max_samples: maxSamples.toString(),
        min_tokens: tokens.toString(),
      });
      const res = await fetch(`${API_ROUTES.TIMELINE}?${params.toString()}`);
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Failed to fetch timeline (${res.status}): ${errorText}`);
      }
      const data = await res.json();
      set({ timelineData: data, isTimelineLoading: false, timelineError: null });
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to load timeline trend";
      set({ timelineError: message, isTimelineLoading: false });
    }
  },

  fetchHookStatus: async (directory?: string) => {
    const { config } = get();
    const dir = directory ?? config.directory;
    try {
      const params = new URLSearchParams({ directory: dir });
      const res = await fetch(`${API_ROUTES.HOOKS}?${params.toString()}`);
      if (res.ok) {
        const data = await res.json();
        set({ hookStatus: data });
      }
    } catch {
      // ignore
    }
  },

  installHook: async (
    hookType: string,
    failThreshold: number = DEFAULT_FAIL_THRESHOLD,
    minTokens?: number,
  ) => {
    const { config } = get();
    const tokens = minTokens ?? config.min_tokens;
    const res = await fetch(API_ROUTES.HOOKS_INSTALL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        directory: config.directory,
        hook_type: hookType,
        fail_threshold: failThreshold,
        min_tokens: tokens,
      }),
    });

    if (!res.ok) {
      const errorText = await res.text().catch(() => res.statusText);
      throw new Error(`Hook installation failed (${res.status}): ${errorText}`);
    }

    const data = await res.json();
    await get().fetchHookStatus();
    return data.message || "Hook installed successfully";
  },
});
