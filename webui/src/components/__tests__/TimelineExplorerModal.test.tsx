import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { TimelineExplorerModal } from "../TimelineExplorerModal";
import { Win2xManagerProvider } from "../ui/win2x-manager/context/win2x-manager-context";
import { useCDDMStore } from "../../store/cddm-store";
import { TimelineTrend } from "../../types/cddm-types";

describe("TimelineExplorerModal Component", () => {
  const mockTrend: TimelineTrend = {
    snapshots: [
      {
        commit_hash: "1111111111111111111111111111111111111111",
        short_hash: "1111111",
        author: "Grigor Tonikyan",
        commit_time: 1700000000,
        formatted_date: "2026-08-20 10:00:00",
        message: "feat: initial commit",
        tag: "v1.0.0",
        total_files: 10,
        total_tokens: 1000,
        total_clones: 4,
        total_clusters: 2,
        duplication_percentage: 8.0,
        dry_health_score: 88.0,
      },
      {
        commit_hash: "2222222222222222222222222222222222222222",
        short_hash: "2222222",
        author: "Grigor Tonikyan",
        commit_time: 1700100000,
        formatted_date: "2026-08-24 10:00:00",
        message: "refactor: eliminate duplication",
        tag: undefined,
        total_files: 10,
        total_tokens: 950,
        total_clones: 1,
        total_clusters: 1,
        duplication_percentage: 2.0,
        dry_health_score: 96.5,
      },
    ],
    initial_score: 88.0,
    current_score: 96.5,
    score_delta: 8.5,
    duplication_delta: -6.0,
    churn_hotspots: [],
  };

  beforeEach(() => {
    useCDDMStore.setState({
      timelineData: mockTrend,
      isTimelineLoading: false,
      timelineError: null,
      hookStatus: {
        pre_commit_installed: false,
        pre_push_installed: false,
        hooks_dir: ".git/hooks",
      },
    });
  });

  it("should return null when not open", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <TimelineExplorerModal isOpen={false} onClose={() => {}} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render timeline trends, trajectory chart, and snapshots table when open", () => {
    const onClose = vi.fn();
    render(
      <Win2xManagerProvider>
        <TimelineExplorerModal isOpen={true} onClose={onClose} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Historical Duplication & Git Timeline Evolution")).toBeDefined();
    expect(screen.getByText("2 Snapshots (+8.5 DRY)")).toBeDefined();
    expect(screen.getByText("+8.5 DRY")).toBeDefined();
    expect(screen.getAllByText("1111111").length).toBeGreaterThanOrEqual(1);
    expect(screen.getAllByText("2222222").length).toBeGreaterThanOrEqual(1);
    expect(screen.getByText("v1.0.0")).toBeDefined();
    expect(screen.getByText("feat: initial commit")).toBeDefined();
    expect(screen.getByText("refactor: eliminate duplication")).toBeDefined();

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should render git hook status and install button", async () => {
    render(
      <Win2xManagerProvider>
        <TimelineExplorerModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Automated Git Hook Quality Gate")).toBeDefined();
    expect(screen.getByText("[INACTIVE]")).toBeDefined();
    expect(screen.getByText("Install Pre-Commit Hook")).toBeDefined();
  });
});
