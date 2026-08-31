import { fireEvent, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vite-plus/test";
import { DeadCodeExplorerModal } from "./DeadCodeExplorerModal";
import { useCDDMStore } from "../store/cddm-store";
import { expectDefinedTexts, renderAsyncWithWin2x, renderWithWin2x } from "../test/test-helpers";
import type { DeadCodeSummary } from "../types/dead-code-types";

const mockDeadCodeSummary: DeadCodeSummary = {
  total_dead_items: 2,
  dead_functions: 1,
  unreachable_blocks: 1,
  dead_clones: 0,
  uncovered_items: 0,
  total_dead_lines: 32,
  estimated_savings_pct: 4.5,
  items: [
    {
      id: 1,
      file_path: "src/utils/math.rs",
      symbol_name: "unused_calculator",
      kind: "unreferenced_function",
      line_start: 12,
      line_end: 28,
      token_count: 45,
      estimated_lines_saved: 17,
      reason: "Function has 0 references",
      confidence: 0.95,
    },
    {
      id: 2,
      file_path: "src/api/handler.rs",
      symbol_name: "<unreachable_statement>",
      kind: "unreachable_block",
      line_start: 50,
      line_end: 64,
      token_count: 30,
      estimated_lines_saved: 15,
      reason: "Statement follows return",
      confidence: 0.98,
    },
  ],
};

describe("DeadCodeExplorerModal Component with Win2xWindow", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders nothing when isOpen is false", () => {
    const { container } = renderWithWin2x(
      <DeadCodeExplorerModal isOpen={false} onClose={vi.fn()} />,
    );
    expect(container.firstChild).toBeNull();
  });

  it("renders dead code items and KPIs when isOpen is true in win2x-manager", async () => {
    useCDDMStore.setState({
      deadCodeSummary: mockDeadCodeSummary,
      isDeadCodeLoading: false,
      deadCodeError: null,
    });

    await renderAsyncWithWin2x(<DeadCodeExplorerModal isOpen={true} onClose={vi.fn()} />);

    expectDefinedTexts([
      "Polyglot Dead Code Explorer",
      "unused_calculator",
      "<unreachable_statement>",
      "src/utils/math.rs:12-28",
    ]);
  });

  it("filters items by search query", async () => {
    useCDDMStore.setState({
      deadCodeSummary: mockDeadCodeSummary,
      isDeadCodeLoading: false,
      deadCodeError: null,
    });

    await renderAsyncWithWin2x(<DeadCodeExplorerModal isOpen={true} onClose={vi.fn()} />);

    const searchInput = screen.getByPlaceholderText("Search file, symbol...");
    fireEvent.change(searchInput, { target: { value: "math.rs" } });

    expect(screen.getByText("unused_calculator")).toBeDefined();
    expect(screen.queryByText("<unreachable_statement>")).toBeNull();
  });
});
