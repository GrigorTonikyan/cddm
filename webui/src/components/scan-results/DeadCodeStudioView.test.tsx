import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vite-plus/test";
import { useCDDMStore } from "../../store/cddm-store";
import type { DeadCodeSummary } from "../../types/dead-code-types";
import { DeadCodeStudioView } from "./DeadCodeStudioView";

describe("DeadCodeStudioView", () => {
  const mockDeadCodeSummary: DeadCodeSummary = {
    total_dead_items: 3,
    dead_functions: 1,
    unreachable_blocks: 1,
    dead_clones: 1,
    uncovered_items: 0,
    total_dead_lines: 48,
    estimated_savings_pct: 4.8,
    items: [
      {
        id: 101,
        file_path: "crates/core/src/unused_fn.rs",
        symbol_name: "legacy_calculator",
        kind: "unreferenced_function",
        line_start: 12,
        line_end: 28,
        token_count: 55,
        estimated_lines_saved: 16,
        reason: "0 callers detected across repository",
        confidence: 0.95,
        package_name: "cddm-core",
      },
      {
        id: 102,
        file_path: "crates/cli/src/dead_branch.rs",
        symbol_name: "unreachable_panic",
        kind: "unreachable_block",
        line_start: 45,
        line_end: 55,
        token_count: 30,
        estimated_lines_saved: 10,
        reason: "Dead conditional branch",
        confidence: 0.9,
        package_name: "cddm-cli",
      },
      {
        id: 103,
        file_path: "src/components/DeadClone.tsx",
        symbol_name: "DuplicateDeadBlock",
        kind: "dead_clone",
        line_start: 80,
        line_end: 102,
        token_count: 70,
        estimated_lines_saved: 22,
        reason: "Clone in unreferenced component",
        confidence: 0.85,
      },
    ],
  };

  beforeEach(() => {
    useCDDMStore.getState().resetScan();
  });

  it("should render cleanly in uninitialized default store state without crashing (Issue #268)", () => {
    render(<DeadCodeStudioView />);
    expect(screen.getByText("Codebase Reachability & Cleanliness")).toBeDefined();
    expect(screen.getByText("No Dead Code Detected")).toBeDefined();
    expect(screen.getByRole("button", { name: /Run Dead Code Scan/i })).toBeDefined();
  });

  it("should render reachability gauge, 4 KPI cards, and dead items when data is populated", () => {
    useCDDMStore.setState({
      deadCodeSummary: mockDeadCodeSummary,
      isDeadCodeLoading: false,
    });

    render(<DeadCodeStudioView />);

    // Gauge
    expect(screen.getByText(/95.2% Reachable/i)).toBeDefined();

    // 4 KPI Cards
    expect(screen.getByText("Unreferenced Funcs")).toBeDefined();
    expect(screen.getByText("Unreachable Blocks")).toBeDefined();
    expect(screen.getByText("Dead Clones")).toBeDefined();
    expect(screen.getByText("Removable Lines")).toBeDefined();

    // Items
    expect(screen.getByText("legacy_calculator")).toBeDefined();
    expect(screen.getByText("unreachable_panic")).toBeDefined();
    expect(screen.getByText("DuplicateDeadBlock")).toBeDefined();
  });

  it("should filter items by kind tab", () => {
    useCDDMStore.setState({
      deadCodeSummary: mockDeadCodeSummary,
      isDeadCodeLoading: false,
    });

    render(<DeadCodeStudioView />);

    const funcButton = screen.getByRole("button", { name: /Functions \(1\)/i });
    fireEvent.click(funcButton);

    expect(screen.getByText("legacy_calculator")).toBeDefined();
    expect(screen.queryByText("unreachable_panic")).toBeNull();
    expect(screen.queryByText("DuplicateDeadBlock")).toBeNull();
  });

  it("should toggle selection and invoke prune function", async () => {
    const pruneMock = vi.fn().mockResolvedValue({
      total_candidates: 1,
      pruned_items: 1,
      skipped_items: 0,
      total_lines_removed: 16,
      dry_run: true,
      files_affected: ["crates/core/src/unused_fn.rs"],
      details: [],
    });

    useCDDMStore.setState({
      deadCodeSummary: mockDeadCodeSummary,
      isDeadCodeLoading: false,
      pruneDeadCode: pruneMock,
    });

    render(<DeadCodeStudioView />);

    const selectAllCheckbox = screen.getByLabelText(/Select All \(3\)/i);
    fireEvent.click(selectAllCheckbox);

    const pruneButton = screen.getByRole("button", { name: /Preview Pruning/i });
    fireEvent.click(pruneButton);

    expect(pruneMock).toHaveBeenCalledWith(
      expect.objectContaining({
        dry_run: true,
        safe_only: true,
      }),
    );
  });
});
