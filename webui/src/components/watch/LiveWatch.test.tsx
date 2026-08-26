import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, beforeEach, vi } from "vite-plus/test";
import { LiveWatchBar } from "./LiveWatchBar";
import { LiveEventInspectorModal } from "./LiveEventInspectorModal";
import { useCDDMStore } from "../../store/cddm-store";
import { Win2xManagerProvider } from "../ui/win2x-manager/context/win2x-manager-context";

describe("LiveWatch Components", () => {
  beforeEach(() => {
    useCDDMStore.setState({
      isLiveWatchActive: true,
      isScanning: false,
      liveSyncCount: 3,
      lastLiveSyncTimestamp: Date.now(),
      lastWatchDelta: {
        changed_files: ["src/app.rs"],
        previous_health_score: 90.0,
        new_health_score: 95.0,
        score_delta: 5.0,
        previous_clones: 2,
        new_clones: 1,
        clone_count_delta: -1,
        previous_clusters: 1,
        new_clusters: 1,
        duration_ms: 18,
        timestamp_millis: Date.now(),
      },
      watchEventsLog: [
        {
          changed_files: ["src/app.rs"],
          previous_health_score: 90.0,
          new_health_score: 95.0,
          score_delta: 5.0,
          previous_clones: 2,
          new_clones: 1,
          clone_count_delta: -1,
          previous_clusters: 1,
          new_clusters: 1,
          duration_ms: 18,
          timestamp_millis: Date.now(),
        },
      ],
      isLiveEventInspectorOpen: false,
    });
  });

  it("should render LiveWatchBar with active sync count and score delta", () => {
    render(
      <Win2xManagerProvider>
        <LiveWatchBar />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Live Sync (3)")).toBeDefined();
    expect(screen.getByText("▲ +5.0%")).toBeDefined();
    expect(screen.getByText("Sync Now")).toBeDefined();
    expect(screen.getByText("Events")).toBeDefined();
  });

  it("should toggle watch daemon state on click", () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ status: "ok", is_active: false }),
    } as Response);

    render(
      <Win2xManagerProvider>
        <LiveWatchBar />
      </Win2xManagerProvider>,
    );

    const toggleBtn = screen.getByText("Live Sync (3)");
    fireEvent.click(toggleBtn);
    expect(globalThis.fetch).toHaveBeenCalled();
  });

  it("should open inspector modal when clicking Events button", () => {
    render(
      <Win2xManagerProvider>
        <LiveWatchBar />
      </Win2xManagerProvider>,
    );

    const eventsBtn = screen.getByText("Events");
    fireEvent.click(eventsBtn);
    expect(useCDDMStore.getState().isLiveEventInspectorOpen).toBe(true);
  });

  it("should render LiveEventInspectorModal when open with event details", () => {
    render(
      <Win2xManagerProvider>
        <LiveEventInspectorModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Live Watch & Real-Time Sync Inspector")).toBeDefined();
    expect(screen.getByText("Total Syncs")).toBeDefined();
    expect(screen.getByText("Trigger Manual Sync")).toBeDefined();
    expect(screen.getByText("src/app.rs")).toBeDefined();
    expect(screen.getByText("18ms")).toBeDefined();
  });

  it("should render empty state when no watch events exist", () => {
    useCDDMStore.setState({ watchEventsLog: [] });

    render(
      <Win2xManagerProvider>
        <LiveEventInspectorModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Listening for workspace file changes...")).toBeDefined();
  });
});
