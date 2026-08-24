import { screen, fireEvent, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { ClonePairDiffModal } from "../ClonePairDiffModal";
import { createMockClonePair, mockFetchSnippets, renderWithWin2x } from "./test-helpers";

describe("ClonePairDiffModal Component", () => {
  const mockPair = createMockClonePair();

  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("should return null when not open", () => {
    const { container } = renderWithWin2x(
      <ClonePairDiffModal isOpen={false} onClose={() => {}} pair={mockPair} />,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render Diff Inspector window with metadata when open", async () => {
    mockFetchSnippets();

    const onClose = vi.fn();

    renderWithWin2x(
      <ClonePairDiffModal isOpen={true} onClose={onClose} pair={mockPair} index={1} />,
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

  it("should support copying file references", async () => {
    mockFetchSnippets();

    const writeTextMock = vi.fn().mockResolvedValue(undefined);
    Object.assign(navigator, {
      clipboard: {
        writeText: writeTextMock,
      },
    });

    renderWithWin2x(<ClonePairDiffModal isOpen={true} onClose={() => {}} pair={mockPair} />);

    await waitFor(() => {
      expect(screen.getByText("Interactive Code Diff Visualizer")).toBeDefined();
    });

    const copyBtn = screen.getByText("Copy File References");
    fireEvent.click(copyBtn);
    expect(writeTextMock).toHaveBeenCalledWith("src/a.ts:10-20 <-> src/b.ts:15-25");
  });
});
