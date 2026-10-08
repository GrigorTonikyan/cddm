import { fireEvent, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vite-plus/test";
import { useCDDMStore } from "../store/cddm-store";
import { renderWithWin2x } from "../test/test-helpers";
import type { DiffScanResult } from "../types/scan-types";
import { DiffScanResultsModal } from "./DiffScanResultsModal";

const mockDiffScanResult: DiffScanResult = {
  scan_id: "diff-test-1",
  duration_ms: 120,
  summary: {
    base_ref: "main",
    target_ref: "HEAD",
    base_dry_score: 92.5,
    target_dry_score: 95.0,
    net_dry_delta: 2.5,
    total_changed_files: 8,
    new_clones: 1,
    legacy_clones: 3,
    resolved_clones: 0,
  },
  diff_clones: [
    {
      status: "New",
      clone_pair: {
        file_a: "src/auth.ts",
        start_line_a: 10,
        end_line_a: 25,
        file_b: "src/login.ts",
        start_line_b: 12,
        end_line_b: 27,
        token_count: 65,
        similarity: 0.95,
        fragment_hash: "hash-1234",
        clone_type: "Renamed",
      },
    },
    {
      status: "Legacy",
      clone_pair: {
        file_a: "src/util.ts",
        start_line_a: 5,
        end_line_a: 20,
        file_b: "src/helper.ts",
        start_line_b: 5,
        end_line_b: 20,
        token_count: 50,
        similarity: 1.0,
        fragment_hash: "hash-5678",
        clone_type: "Exact",
      },
    },
  ],
};

describe("DiffScanResultsModal", () => {
  beforeEach(() => {
    // Reset store to uninitialized default state to test Issue #268 safety
    useCDDMStore.getState().resetScan();
    useCDDMStore.setState({
      diffScanResult: null,
      isDiffScanning: false,
      diffScanError: null,
    });
  });

  it("should render nothing when isOpen is false", () => {
    const { container } = renderWithWin2x(
      <DiffScanResultsModal isOpen={false} onClose={vi.fn()} />,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render safely in uninitialized default store state without throwing", () => {
    renderWithWin2x(<DiffScanResultsModal isOpen={true} onClose={vi.fn()} />);

    expect(screen.getByText("Differential Codebase Scan & Branch Comparison")).toBeDefined();
    expect(screen.getByText("No Differential Comparison Computed")).toBeDefined();
    expect(screen.getByRole("button", { name: /Run Diff Scan/i })).toBeDefined();
  });

  it("should display summary KPI cards when diffScanResult is populated", () => {
    useCDDMStore.setState({ diffScanResult: mockDiffScanResult });

    renderWithWin2x(<DiffScanResultsModal isOpen={true} onClose={vi.fn()} />);

    expect(screen.getByText("+2.50%")).toBeDefined();
    expect(screen.getByText("+1")).toBeDefined(); // new clones
    expect(screen.getByText("3")).toBeDefined(); // legacy clones
    expect(screen.getByText("92.5%")).toBeDefined(); // base dry score
    expect(screen.getByText("95.0%")).toBeDefined(); // target dry score
  });

  it("should render clone pairs and support status filtering", () => {
    useCDDMStore.setState({ diffScanResult: mockDiffScanResult });

    renderWithWin2x(<DiffScanResultsModal isOpen={true} onClose={vi.fn()} />);

    expect(screen.getByText(/src\/auth\.ts/i)).toBeDefined();
    expect(screen.getByText(/src\/util\.ts/i)).toBeDefined();

    // Filter to New only
    const newFilterBtn = screen.getByRole("button", { name: "New" });
    fireEvent.click(newFilterBtn);

    expect(screen.getByText(/src\/auth\.ts/i)).toBeDefined();
    expect(screen.queryByText(/src\/util\.ts/i)).toBeNull();
  });

  it("should invoke startDiffScan on form submission", async () => {
    const startDiffScanSpy = vi.fn().mockResolvedValue(undefined);
    useCDDMStore.setState({ startDiffScan: startDiffScanSpy });

    renderWithWin2x(<DiffScanResultsModal isOpen={true} onClose={vi.fn()} />);

    const runBtn = screen.getByRole("button", { name: /Run Diff Scan/i });
    fireEvent.click(runBtn);

    expect(startDiffScanSpy).toHaveBeenCalledWith("main", undefined);
  });

  it("should invoke onClose when close button clicked", () => {
    const onClose = vi.fn();
    renderWithWin2x(<DiffScanResultsModal isOpen={true} onClose={onClose} />);

    const closeBtns = screen.getAllByRole("button", { name: "Close" });
    const closeBtn = closeBtns[0];
    expect(closeBtn).toBeDefined();
    if (closeBtn) {
      fireEvent.click(closeBtn);
    }

    expect(onClose).toHaveBeenCalled();
  });
});
