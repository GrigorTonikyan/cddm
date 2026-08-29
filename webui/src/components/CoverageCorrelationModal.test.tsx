import { render, screen, fireEvent, act } from "@testing-library/react";
import { describe, expect, it, vi, beforeEach } from "vite-plus/test";
import { CoverageCorrelationModal } from "./CoverageCorrelationModal";
import { Win2xManagerProvider } from "./ui/win2x-manager/context/win2x-manager-context";
import type { CoverageCorrelationSummary } from "../types/cddm-types";

describe("CoverageCorrelationModal", () => {
  const mockSummary: CoverageCorrelationSummary = {
    total_clone_pairs: 2,
    overall_covered_clones_pct: 75.5,
    dead_code_clones: 1,
    test_gap_clones: 1,
    hot_path_clones: 1,
    total_runtime_hits: 15400,
    metrics: [
      {
        clone_pair_id: 1,
        file_a: "src/auth.ts",
        start_line_a: 10,
        end_line_a: 20,
        hits_a: 15000,
        covered_lines_a: 10,
        total_lines_a: 10,
        coverage_pct_a: 100.0,
        file_b: "src/auth_alt.ts",
        start_line_b: 10,
        end_line_b: 20,
        hits_b: 400,
        covered_lines_b: 5,
        total_lines_b: 10,
        coverage_pct_b: 50.0,
        total_combined_hits: 15400,
        execution_tier: "HotPath",
        has_test_gap: true,
        is_dead_code: false,
        risk_score: 95.0,
      },
      {
        clone_pair_id: 2,
        file_a: "src/legacy.ts",
        start_line_a: 1,
        end_line_a: 15,
        hits_a: 0,
        covered_lines_a: 0,
        total_lines_a: 15,
        coverage_pct_a: 0.0,
        file_b: "src/legacy_old.ts",
        start_line_b: 1,
        end_line_b: 15,
        hits_b: 0,
        covered_lines_b: 0,
        total_lines_b: 15,
        coverage_pct_b: 0.0,
        total_combined_hits: 0,
        execution_tier: "DeadCode",
        has_test_gap: false,
        is_dead_code: true,
        risk_score: 10.0,
      },
    ],
  };

  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("should not render when isOpen is false", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <CoverageCorrelationModal isOpen={false} onClose={vi.fn()} initialSummary={mockSummary} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render overview statistics when open", async () => {
    await act(async () => {
      render(
        <Win2xManagerProvider>
          <CoverageCorrelationModal isOpen={true} onClose={vi.fn()} initialSummary={mockSummary} />
        </Win2xManagerProvider>,
      );
    });

    expect(screen.getByText("Runtime Execution & Coverage-Aware De-duplication")).toBeDefined();
    expect(screen.getByText("75.5%")).toBeDefined();
    expect(screen.getByText("src/auth.ts:10-20")).toBeDefined();
    expect(screen.getByText("src/legacy.ts:1-15")).toBeDefined();
  });

  it("should filter by dead code only", async () => {
    await act(async () => {
      render(
        <Win2xManagerProvider>
          <CoverageCorrelationModal isOpen={true} onClose={vi.fn()} initialSummary={mockSummary} />
        </Win2xManagerProvider>,
      );
    });

    const deadButton = screen.getByText(/Dead Code \(1\)/);
    await act(async () => {
      fireEvent.click(deadButton);
    });

    expect(screen.getByText("src/legacy.ts:1-15")).toBeDefined();
    expect(screen.queryByText("src/auth.ts:10-20")).toBeNull();
  });
});
