import { screen, fireEvent, waitFor } from "@testing-library/react";
import { describe, it, expect } from "vite-plus/test";
import { ClonePairCard } from "./ClonePairCard";
import { createMockClonePair, mockFetchSnippets, renderWithWin2x } from "../test/test-helpers";

describe("ClonePairCard Component", () => {
  const mockPair = createMockClonePair();

  it("should render collapsed summary card with clone type badge", () => {
    renderWithWin2x(<ClonePairCard pair={mockPair} index={1} />);
    expect(screen.getByText("#1")).toBeDefined();
    expect(screen.getByText("a.ts")).toBeDefined();
    expect(screen.getByText("b.ts")).toBeDefined();
    expect(screen.getByText("55 tokens")).toBeDefined();
    expect(screen.getByText("95% match")).toBeDefined();
    expect(screen.getByText("Exact")).toBeDefined();
  });

  it("should expand split details, show Diff Inspector & Refactor Advisor on click", async () => {
    mockFetchSnippets();

    renderWithWin2x(<ClonePairCard pair={mockPair} index={1} />);
    const header = screen.getByText("#1").closest("div");
    fireEvent.click(header!);
    expect(screen.getByText("Fragment Hash: hash123")).toBeDefined();
    expect(screen.getByText("Author A: Grigor")).toBeDefined();
    await waitFor(() => {
      expect(screen.getByText("Diff Inspector")).toBeDefined();
      expect(screen.getByText("Refactor Advisor")).toBeDefined();
    });

    // Click Diff Inspector button to open ClonePairDiffModal
    const diffBtn = screen.getByText("Diff Inspector");
    fireEvent.click(diffBtn);
    expect(screen.getByText("Clone Pair #1 Diff Inspector")).toBeDefined();

    await waitFor(() => {
      expect(screen.getByText("Interactive Code Diff Visualizer")).toBeDefined();
    });
  });
});
