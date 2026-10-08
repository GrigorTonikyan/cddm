import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vite-plus/test";
import { useCDDMStore } from "../../store/cddm-store";
import { Win2xManagerProvider } from "../ui/win2x-manager/context/win2x-manager-context";
import { LiveEventInspectorModal } from "./LiveEventInspectorModal";

describe("LiveEventInspectorModal", () => {
  beforeEach(() => {
    useCDDMStore.getState().resetScan();
    useCDDMStore.setState({
      isLiveWatchActive: false,
      isScanning: false,
      liveSyncCount: 0,
      lastLiveSyncTimestamp: null,
      watchEventsLog: [],
    });
  });

  it("should return null when closed", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <LiveEventInspectorModal isOpen={false} onClose={vi.fn()} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render safely in uninitialized default store state without throwing", () => {
    render(
      <Win2xManagerProvider>
        <LiveEventInspectorModal isOpen={true} onClose={vi.fn()} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Live Watch & Real-Time Sync Inspector")).toBeDefined();
    expect(screen.getByText("Total Syncs")).toBeDefined();
    expect(screen.getByText("Listening for workspace file changes...")).toBeDefined();
    expect(screen.getByText("0 event(s) recorded")).toBeDefined();
  });

  it("should render populated event log entries", () => {
    useCDDMStore.setState({
      isLiveWatchActive: true,
      liveSyncCount: 2,
      lastLiveSyncTimestamp: 1700000000000,
      watchEventsLog: [
        {
          changed_files: ["crates/cddm-core/src/scan.rs"],
          previous_health_score: 90.0,
          new_health_score: 95.0,
          score_delta: 5.0,
          previous_clones: 3,
          new_clones: 1,
          clone_count_delta: -2,
          previous_clusters: 2,
          new_clusters: 1,
          duration_ms: 14,
          timestamp_millis: 1700000000000,
        },
      ],
    });

    render(
      <Win2xManagerProvider>
        <LiveEventInspectorModal isOpen={true} onClose={vi.fn()} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("crates/cddm-core/src/scan.rs")).toBeDefined();
    expect(screen.getByText("14ms")).toBeDefined();
    expect(screen.getByText(/DRY: 95.0%/)).toBeDefined();
    expect(screen.getByText("1 event(s) recorded")).toBeDefined();
  });

  it("should trigger clear watch events log", () => {
    const mockClear = vi.fn();
    useCDDMStore.setState({
      watchEventsLog: [
        {
          changed_files: ["src/a.ts"],
          previous_health_score: 90.0,
          new_health_score: 90.0,
          score_delta: 0,
          previous_clones: 1,
          new_clones: 1,
          clone_count_delta: 0,
          previous_clusters: 1,
          new_clusters: 1,
          duration_ms: 10,
          timestamp_millis: 1700000000000,
        },
      ],
      clearWatchEventsLog: mockClear,
    });

    render(
      <Win2xManagerProvider>
        <LiveEventInspectorModal isOpen={true} onClose={vi.fn()} />
      </Win2xManagerProvider>,
    );

    const clearBtn = screen.getByText("Clear History");
    fireEvent.click(clearBtn);
    expect(mockClear).toHaveBeenCalledTimes(1);
  });
});
