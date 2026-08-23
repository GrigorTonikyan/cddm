import { render, screen, waitFor, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { DiffViewer } from "../DiffViewer";
import { SnippetResponse } from "../../types/cddm-types";

describe("DiffViewer Component", () => {
  const mockSnippetA: SnippetResponse = {
    file: "src/a.ts",
    start_line: 10,
    end_line: 12,
    context_start_line: 8,
    context_end_line: 14,
    lines: [
      { line_number: 8, content: "const x = 1;", is_target: false },
      { line_number: 9, content: "const y = 2;", is_target: false },
      { line_number: 10, content: "function add(a, b) {", is_target: true },
      { line_number: 11, content: "  return a + b;", is_target: true },
      { line_number: 12, content: "}", is_target: true },
      { line_number: 13, content: "const z = 3;", is_target: false },
    ],
    total_lines: 20,
    language: "TypeScript",
  };

  const mockSnippetB: SnippetResponse = {
    file: "src/b.ts",
    start_line: 20,
    end_line: 22,
    context_start_line: 18,
    context_end_line: 24,
    lines: [
      { line_number: 18, content: "const m = 1;", is_target: false },
      { line_number: 19, content: "const n = 2;", is_target: false },
      { line_number: 20, content: "function add(a, b) {", is_target: true },
      { line_number: 21, content: "  return a + b;", is_target: true },
      { line_number: 22, content: "}", is_target: true },
      { line_number: 23, content: "const k = 3;", is_target: false },
    ],
    total_lines: 30,
    language: "TypeScript",
  };

  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("should display loading state initially and then render snippets", async () => {
    globalThis.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes("src%2Fa.ts") || url.includes("src/a.ts")) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve(mockSnippetA),
        } as Response);
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve(mockSnippetB),
      } as Response);
    });

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

    expect(screen.getByText("Loading synchronized code diff...")).toBeDefined();

    await waitFor(() => {
      expect(screen.getByText("Interactive Code Diff Visualizer")).toBeDefined();
    });

    expect(screen.getByText("TypeScript")).toBeDefined();
    expect(screen.getByText("Side-by-Side")).toBeDefined();
    expect(screen.getByText("Unified")).toBeDefined();
  });

  it("should switch between split and unified view modes", async () => {
    globalThis.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes("src%2Fa.ts") || url.includes("src/a.ts")) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve(mockSnippetA),
        } as Response);
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve(mockSnippetB),
      } as Response);
    });

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
