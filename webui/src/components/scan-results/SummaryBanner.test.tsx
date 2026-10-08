import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vite-plus/test";
import { useCDDMStore } from "../../store/cddm-store";
import { createMockScanResult } from "../../test/test-helpers";
import { SummaryBanner } from "./SummaryBanner";

describe("SummaryBanner", () => {
  const mockResult = createMockScanResult({
    dry_health_score: 92.5,
    duplication_percentage: 4.5,
    total_files: 42,
  });

  beforeEach(() => {
    useCDDMStore.getState().resetScan();
  });

  it("should render cleanly in uninitialized default store state (Issue #268)", () => {
    render(<SummaryBanner results={mockResult} onOpenHealthAudit={vi.fn()} />);

    expect(screen.getByText("DRY Health Score")).toBeDefined();
    expect(screen.getByText("92.5")).toBeDefined();
    expect(screen.getByText("Duplication Rate")).toBeDefined();
    expect(screen.getByText("4.50%")).toBeDefined();
    expect(screen.getByText("Files Scanned")).toBeDefined();
    expect(screen.getByText("Dead Code")).toBeDefined();
  });

  it("should display dead code telemetry and switch mode on click", () => {
    useCDDMStore.setState({
      deadCodeSummary: {
        total_dead_items: 7,
        dead_functions: 3,
        unreachable_blocks: 2,
        dead_clones: 2,
        uncovered_items: 0,
        total_dead_lines: 145,
        estimated_savings_pct: 3.2,
        items: [],
      },
    });

    render(<SummaryBanner results={mockResult} onOpenHealthAudit={vi.fn()} />);

    expect(screen.getByText("7")).toBeDefined();
    expect(screen.getByText("~145 removable LOC")).toBeDefined();

    const deadCodeCard = screen.getByTitle("Click to switch to Polyglot Dead Code Studio");
    fireEvent.click(deadCodeCard);

    expect(useCDDMStore.getState().viewMode).toBe("dead-code");
  });

  it("should invoke onOpenHealthAudit when clicking DRY score card", () => {
    const onOpenHealthAudit = vi.fn();
    render(<SummaryBanner results={mockResult} onOpenHealthAudit={onOpenHealthAudit} />);

    const dryCard = screen.getByTitle("Click to open full DRY Health Score Audit Window");
    fireEvent.click(dryCard);

    expect(onOpenHealthAudit).toHaveBeenCalledTimes(1);
  });
});
