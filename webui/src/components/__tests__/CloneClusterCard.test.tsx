import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect } from "vite-plus/test";
import { CloneClusterCard } from "../CloneClusterCard";
import { createMockCluster, renderWithWin2x } from "./test-helpers";

describe("CloneClusterCard Component", () => {
  const mockCluster = createMockCluster();

  it("should render collapsed cluster summary card with badges", () => {
    renderWithWin2x(<CloneClusterCard cluster={mockCluster} index={1} />);

    for (const label of ["#1", "Cluster #1", "3 Sites", "Exact", "100%", "Refactor"]) {
      expect(screen.getByText(label)).toBeDefined();
    }
  });

  it("should expand cluster occurrences list and metadata on accordion click", () => {
    renderWithWin2x(<CloneClusterCard cluster={mockCluster} index={1} />);

    const expandBtn = screen.getByLabelText("Expand cluster details");
    fireEvent.click(expandBtn);

    for (const item of [
      "Occurrences in Codebase (3)",
      "login.ts",
      "register.ts",
      "reset.ts",
      "L10-25",
      "L15-30",
      "L5-20",
    ]) {
      expect(screen.getByText(item)).toBeDefined();
    }
  });
});
