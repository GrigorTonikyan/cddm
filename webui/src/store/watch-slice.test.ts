import { describe, it, expect, beforeEach, vi } from "vite-plus/test";
import { useCDDMStore } from "./cddm-store";
import type { WatchStatusResponse, ScanResult } from "./../types/cddm-types";

describe("useCDDMStore - Watch Slice", () => {
  beforeEach(() => {
    useCDDMStore.setState({
      isLiveWatchActive: true,
      isLiveEventInspectorOpen: false,
      watchEventsLog: [],
      lastWatchDelta: null,
      recentModifiedFiles: [],
      liveSyncCount: 0,
      lastLiveSyncTimestamp: null,
    });
  });

  it("should initialize with default live watch state", () => {
    const state = useCDDMStore.getState();
    expect(state.isLiveWatchActive).toBe(true);
    expect(state.isLiveEventInspectorOpen).toBe(false);
    expect(state.watchEventsLog).toEqual([]);
    expect(state.lastWatchDelta).toBeNull();
    expect(state.recentModifiedFiles).toEqual([]);
  });

  it("should toggle live event inspector modal", () => {
    useCDDMStore.getState().setIsLiveEventInspectorOpen(true);
    expect(useCDDMStore.getState().isLiveEventInspectorOpen).toBe(true);

    useCDDMStore.getState().setIsLiveEventInspectorOpen(false);
    expect(useCDDMStore.getState().isLiveEventInspectorOpen).toBe(false);
  });

  it("should fetch watch daemon status and update store", async () => {
    const mockStatus: WatchStatusResponse = {
      is_active: true,
      watch_directory: ".",
      debounce_ms: 300,
      last_sync_timestamp: 1700000000000,
      sync_count: 5,
      last_duration_ms: 22,
      recent_events: [
        {
          changed_files: ["src/lib.rs"],
          previous_health_score: 90.0,
          new_health_score: 92.5,
          score_delta: 2.5,
          previous_clones: 3,
          new_clones: 2,
          clone_count_delta: -1,
          previous_clusters: 1,
          new_clusters: 1,
          duration_ms: 22,
          timestamp_millis: 1700000000000,
        },
      ],
    };

    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockStatus),
    } as Response);

    await useCDDMStore.getState().fetchWatchStatus();

    const state = useCDDMStore.getState();
    expect(state.isLiveWatchActive).toBe(true);
    expect(state.liveSyncCount).toBe(5);
    expect(state.lastLiveSyncTimestamp).toBe(1700000000000);
    expect(state.watchEventsLog.length).toBe(1);
    expect(state.watchEventsLog[0]!.score_delta).toBe(2.5);
  });

  it("should toggle watch active state via API", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ status: "ok", is_active: false }),
    } as Response);

    await useCDDMStore.getState().toggleWatch(false);
    expect(useCDDMStore.getState().isLiveWatchActive).toBe(false);

    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ status: "ok", is_active: true }),
    } as Response);

    await useCDDMStore.getState().toggleWatch();
    expect(useCDDMStore.getState().isLiveWatchActive).toBe(true);
  });

  it("should execute manual rescan successfully", async () => {
    const mockScanResult: Partial<ScanResult> = {
      scan_id: "manual-rescan-1",
      total_files: 10,
      total_tokens: 500,
      total_clones: 0,
      total_clusters: 0,
      duplication_percentage: 0.0,
      dry_health_score: 100.0,
      duration_ms: 15,
    };

    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockScanResult),
    } as Response);

    await useCDDMStore.getState().triggerManualRescan();

    const state = useCDDMStore.getState();
    expect(state.results?.scan_id).toBe("manual-rescan-1");
    expect(state.isScanning).toBe(false);
    expect(state.activeScanId).toBe("manual-rescan-1");
    expect(state.liveSyncCount).toBe(1);
  });

  it("should clear watch events log", () => {
    useCDDMStore.setState({
      watchEventsLog: [
        {
          changed_files: ["src/main.rs"],
          previous_health_score: 80.0,
          new_health_score: 85.0,
          score_delta: 5.0,
          previous_clones: 2,
          new_clones: 1,
          clone_count_delta: -1,
          previous_clusters: 1,
          new_clusters: 1,
          duration_ms: 10,
          timestamp_millis: Date.now(),
        },
      ],
    });

    expect(useCDDMStore.getState().watchEventsLog.length).toBe(1);
    useCDDMStore.getState().clearWatchEventsLog();
    expect(useCDDMStore.getState().watchEventsLog.length).toBe(0);
  });
});
