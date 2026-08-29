import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect } from "vite-plus/test";
import { BranchDriftMatrixSection } from "./BranchDriftMatrixSection";
import type { BranchMatrixReport } from "../../types/cddm-types";

describe("BranchDriftMatrixSection Component", () => {
  const mockReport: BranchMatrixReport = {
    workspace_root: ".",
    branches: ["main", "feature/auth"],
    matrix: [
      {
        base_branch: "main",
        target_branch: "feature/auth",
        base_dry_score: 95.0,
        target_dry_score: 96.5,
        net_dry_delta: 1.5,
        changed_files_count: 3,
        new_clones_count: 0,
        divergence_index: 2.1,
      },
    ],
    cleanest_branch: "feature/auth",
    highest_drift_branch: "feature/auth",
    summary: "Computed pairwise branch divergence",
  };

  it("should render initial report data correctly", () => {
    render(<BranchDriftMatrixSection initialReport={mockReport} />);

    expect(screen.getByText("Compute Drift Matrix")).toBeDefined();
    expect(screen.getByText("2 Compared")).toBeDefined();
    expect(screen.getByText("+1.50%")).toBeDefined();
    expect(screen.getByText("2.1%")).toBeDefined();
  });

  it("should handle error when fewer than 2 branches entered", async () => {
    render(<BranchDriftMatrixSection initialReport={null} />);

    const input = screen.getByPlaceholderText(/e\.g\. main/);
    fireEvent.change(input, { target: { value: "main" } });

    const btn = screen.getByText("Compute Drift Matrix");
    fireEvent.click(btn);

    expect(await screen.findByText(/Please specify at least 2 branches/)).toBeDefined();
  });
});
