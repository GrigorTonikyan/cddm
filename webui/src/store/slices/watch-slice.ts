import { API_ROUTES } from "../../constants/cddm-constants";
import type { WatchStatusResponse } from "../../types/cddm-types";
import type { CDDMStoreState } from "../types";

/**
 * Zustand slice managing real-time filesystem watch daemon interactions.
 */
export const createWatchSlice = (
  set: (
    partial: Partial<CDDMStoreState> | ((state: CDDMStoreState) => Partial<CDDMStoreState>),
  ) => void,
  get: () => CDDMStoreState,
) => ({
  isLiveEventInspectorOpen: false,
  watchEventsLog: [],
  lastWatchDelta: null,
  recentModifiedFiles: [],

  setIsLiveEventInspectorOpen: (isLiveEventInspectorOpen: boolean) =>
    set({ isLiveEventInspectorOpen }),

  clearWatchEventsLog: () => set({ watchEventsLog: [] }),

  fetchWatchStatus: async (): Promise<void> => {
    try {
      const res = await fetch(API_ROUTES.WATCH_STATUS);
      if (res.ok) {
        const data: WatchStatusResponse = await res.json();
        set({
          isLiveWatchActive: data.is_active,
          watchEventsLog: data.recent_events || [],
          lastLiveSyncTimestamp: data.last_sync_timestamp,
          liveSyncCount: data.sync_count,
        });
      }
    } catch {
      // Ignore network errors during offline / disconnected mode
    }
  },

  toggleWatch: async (active?: boolean): Promise<void> => {
    const targetState = active !== undefined ? active : !get().isLiveWatchActive;
    try {
      const res = await fetch(API_ROUTES.WATCH_TOGGLE, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ active: targetState }),
      });
      if (res.ok) {
        const body = await res.json();
        set({ isLiveWatchActive: body.is_active });
      } else {
        set({ isLiveWatchActive: targetState });
      }
    } catch {
      set({ isLiveWatchActive: targetState });
    }
  },

  triggerManualRescan: async (): Promise<void> => {
    set({ isScanning: true, error: null });
    try {
      const res = await fetch(API_ROUTES.WATCH_RESCAN, {
        method: "POST",
      });
      if (!res.ok) {
        throw new Error(`Manual rescan failed with status ${res.status}`);
      }
      const data = await res.json();
      set({
        results: data,
        isScanning: false,
        activeScanId: data.scan_id,
        lastLiveSyncTimestamp: Date.now(),
        liveSyncCount: get().liveSyncCount + 1,
      });
    } catch (err: unknown) {
      set({
        isScanning: false,
        error: err instanceof Error ? err.message : "Failed to execute manual rescan",
      });
    }
  },
});
