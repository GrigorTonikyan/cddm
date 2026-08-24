import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { ClonePairDiffModal } from "../ClonePairDiffModal";
import { createMockClonePair } from "./test-helpers";
import { Win2xManagerProvider } from "../ui/win2x-manager/context/win2x-manager-context";

describe("ClonePairDiffModal Component", () => {
  const mockPair = createMockClonePair();

  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("should return null when not open", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <ClonePairDiffModal isOpen={false} onClose={() => {}} pair={mockPair} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render Diff Inspector window with metadata when open", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () =>
        Promise.resolve({
          file: "src/a.ts",
          start_line: 10,
          end_line: 20,
          context_start_line: 8,
          context_end_line: 22,
          lines: [{ line_number: 10, content: "const x = 1;", is_target: true }],
          total_lines: 30,
          language: "TypeScript",
        }),
    } as Response);

    const onClose = vi.fn();

    render(
      <Win2xManagerProvider>
        <ClonePairDiffModal isOpen={true} onClose={onClose} pair={mockPair} index={1} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Clone Pair #1 Diff Inspector")).toBeDefined();
    expect(screen.getByText("Exact")).toBeDefined();
    expect(screen.getByText("55 tokens")).toBeDefined();
    expect(screen.getByText("95% similarity")).toBeDefined();
    expect(screen.getByText("Hash: hash123")).toBeDefined();
    expect(screen.getByText("Author A: Grigor")).toBeDefined();

    await waitFor(() => {
      expect(screen.getByText("Interactive Code Diff Visualizer")).toBeDefined();
    });

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should support copying file references", () => {
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

    const writeTextMock = vi.fn().mockResolvedValue(undefined);
    Object.assign(navigator, {
      clipboard: {
        writeText: writeTextMock,
      },
    });

    render(
      <Win2xManagerProvider>
        <ClonePairDiffModal isOpen={true} onClose={() => {}} pair={mockPair} />
      </Win2xManagerProvider>,
    );

    const copyBtn = screen.getByText("Copy File References");
    fireEvent.click(copyBtn);
    expect(writeTextMock).toHaveBeenCalledWith("src/a.ts:10-20 <-> src/b.ts:15-25");
  });
});
