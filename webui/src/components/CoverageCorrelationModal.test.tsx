import { screen } from "@testing-library/react";
import { describe, expect, it, vi, beforeEach } from "vite-plus/test";
import { CoverageCorrelationModal } from "./CoverageCorrelationModal";
import {
  clickElementAsync,
  expectDefinedTexts,
  renderAsyncWithWin2x,
  renderWithWin2x,
} from "../test/test-helpers";
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
        hits_a: 5000,
        covered_lines_a: 10,
        total_lines_a: 10,
        coverage_pct_a: 100.0,
        file_b: "src/session.ts",
        start_line_b: 10,
        end_line_b: 20,
        hits_b: 10400,
        covered_lines_b: 10,
        total_lines_b: 10,
        coverage_pct_b: 100.0,
        total_combined_hits: 15400,
        execution_tier: "HotPath",
        has_test_gap: false,
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
        file_b: "src/unused.ts",
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
    vi.clearAllMocks();
  });

  it("should return null when not open", () => {
    const { container } = renderWithWin2x(
      <CoverageCorrelationModal isOpen={false} onClose={vi.fn()} initialSummary={null} />,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render overview statistics when open", async () => {
    await renderAsyncWithWin2x(
      <CoverageCorrelationModal isOpen={true} onClose={vi.fn()} initialSummary={mockSummary} />,
    );

    expectDefinedTexts([
      "Runtime Execution & Coverage-Aware De-duplication",
      "75.5%",
      "src/auth.ts:10-20",
      "src/legacy.ts:1-15",
    ]);
  });

  it("should filter by dead code only", async () => {
    await renderAsyncWithWin2x(
      <CoverageCorrelationModal isOpen={true} onClose={vi.fn()} initialSummary={mockSummary} />,
    );

    await clickElementAsync(/Dead Code \(1\)/);

    expect(screen.getByText("src/legacy.ts:1-15")).toBeDefined();
    expect(screen.queryByText("src/auth.ts:10-20")).toBeNull();
  });
});
