import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { ClonePairCard } from "../ClonePairCard";
import { createMockClonePair } from "./test-helpers";

describe("ClonePairCard Component", () => {
  const mockPair = createMockClonePair();

  it("should render collapsed summary card with clone type badge", () => {
    render(<ClonePairCard pair={mockPair} index={1} />);
    expect(screen.getByText("#1")).toBeDefined();
    expect(screen.getByText("a.ts")).toBeDefined();
    expect(screen.getByText("b.ts")).toBeDefined();
    expect(screen.getByText("55 tokens")).toBeDefined();
    expect(screen.getByText("95% match")).toBeDefined();
    expect(screen.getByText("Exact")).toBeDefined();
  });

  it("should expand split details and show Refactor Advisor on click", async () => {
    // Mock fetch for DiffViewer
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () =>
        Promise.resolve({
          file: "src/a.ts",
          start_line: 10,
          end_line: 20,
          context_start_line: 8,
          context_end_line: 22,
          lines: [],
          total_lines: 30,
          language: "TypeScript",
        }),
    } as Response);

    render(<ClonePairCard pair={mockPair} index={1} />);
    const header = screen.getByText("#1").closest("div");
    fireEvent.click(header!);
    expect(screen.getByText("Fragment Hash: hash123")).toBeDefined();
    expect(screen.getByText("Author A: Grigor")).toBeDefined();
    await waitFor(() => {
      expect(screen.getByText("Refactor Advisor")).toBeDefined();
    });
  });
});
