import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect } from "vite-plus/test";
import { CloneClusterCard } from "../CloneClusterCard";
import { createMockCluster, renderWithWin2x } from "./test-helpers";

describe("CloneClusterCard Component", () => {
  const mockCluster = createMockCluster();

  it("should render collapsed cluster summary card with badges", () => {
    renderWithWin2x(<CloneClusterCard cluster={mockCluster} index={1} />);

    expect(screen.getByText("#1")).toBeDefined();
    expect(screen.getByText("Cluster #1")).toBeDefined();
    expect(screen.getByText("3 Sites")).toBeDefined();
    expect(screen.getByText("Exact")).toBeDefined();
    expect(screen.getByText("100%")).toBeDefined();
    expect(screen.getByText("Refactor")).toBeDefined();
  });

  it("should expand cluster occurrences list and metadata on accordion click", () => {
    renderWithWin2x(<CloneClusterCard cluster={mockCluster} index={1} />);

    const expandBtn = screen.getByLabelText("Expand cluster details");
    fireEvent.click(expandBtn);

    expect(screen.getByText("Occurrences in Codebase (3)")).toBeDefined();
    expect(screen.getByText("login.ts")).toBeDefined();
    expect(screen.getByText("register.ts")).toBeDefined();
    expect(screen.getByText("reset.ts")).toBeDefined();
    expect(screen.getByText("L10-25")).toBeDefined();
    expect(screen.getByText("L15-30")).toBeDefined();
    expect(screen.getByText("L5-20")).toBeDefined();
  });
});
