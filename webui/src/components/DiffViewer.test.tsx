import { render, screen, waitFor, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { DiffViewer } from "./DiffViewer";
import { DEFAULT_TEST_CLONE_PAIR_PROPS, mockFetchSnippets } from "../test/test-helpers";

describe("DiffViewer Component", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("should display loading state initially and then render snippets", async () => {
    mockFetchSnippets();

    render(<DiffViewer {...DEFAULT_TEST_CLONE_PAIR_PROPS} />);

    expect(screen.getByText("Loading synchronized code diff...")).toBeDefined();

    await waitFor(() => {
      expect(screen.getByText("Interactive Code Diff Visualizer")).toBeDefined();
    });

    expect(screen.getByText("TypeScript")).toBeDefined();
    expect(screen.getByText("Side-by-Side")).toBeDefined();
    expect(screen.getByText("Unified")).toBeDefined();
  });

  it("should switch between split and unified view modes", async () => {
    mockFetchSnippets();

    render(<DiffViewer {...DEFAULT_TEST_CLONE_PAIR_PROPS} />);

    await waitFor(() => {
      expect(screen.getByText("Interactive Code Diff Visualizer")).toBeDefined();
    });

    const unifiedBtn = screen.getByText("Unified");
    fireEvent.click(unifiedBtn);

    expect(screen.getByText(/Fragment A: a\.ts/i)).toBeDefined();
    expect(screen.getByText(/Fragment B: b\.ts/i)).toBeDefined();
  });

  it("should show error alert when API fetch fails", async () => {
    globalThis.fetch = vi.fn().mockRejectedValue(new Error("Network connection error"));

    render(
      <DiffViewer
        fileA="src/a.ts"
        startLineA={10}
        endLineA={12}
        fileB="src/b.ts"
        startLineB={20}
        endLineB={22}
      />,
    );

    await waitFor(() => {
      expect(screen.getByText("Snippet Retrieval Notice")).toBeDefined();
      expect(screen.getByText("Network connection error")).toBeDefined();
    });
  });
});
