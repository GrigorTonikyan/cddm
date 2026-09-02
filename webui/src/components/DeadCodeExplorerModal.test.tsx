import { fireEvent, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vite-plus/test";
import { DeadCodeExplorerModal } from "./DeadCodeExplorerModal";
import { useCDDMStore } from "../store/cddm-store";
import { expectDefinedTexts, renderAsyncWithWin2x, renderWithWin2x } from "../test/test-helpers";
import type { DeadClonePruneResult, DeadCodeItem, DeadCodeSummary } from "../types/dead-code-types";

const mockItem1: DeadCodeItem = {
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
};

const mockItem2: DeadCodeItem = {
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
};

const mockDeadCodeSummary: DeadCodeSummary = {
  total_dead_items: 2,
  dead_functions: 1,
  unreachable_blocks: 1,
  dead_clones: 0,
  uncovered_items: 0,
  total_dead_lines: 32,
  estimated_savings_pct: 4.5,
  items: [mockItem1, mockItem2],
};

const mockPruneResult: DeadClonePruneResult = {
  total_candidates: 2,
  pruned_items: 2,
  skipped_items: 0,
  total_lines_removed: 32,
  dry_run: true,
  files_affected: ["src/utils/math.rs", "src/api/handler.rs"],
  items: [
    {
      id: 1,
      file_path: "src/utils/math.rs",
      symbol_name: "unused_calculator",
      line_start: 12,
      line_end: 28,
      lines_removed: 17,
      status: "dry_run_pruned",
      confidence: 0.95,
      reason: "Function has 0 references",
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
      lastPruneResult: null,
    });

    await renderAsyncWithWin2x(<DeadCodeExplorerModal isOpen={true} onClose={vi.fn()} />);

    expectDefinedTexts([
      "Polyglot Dead Code Explorer & Safe Pruner",
      "unused_calculator",
      "<unreachable_statement>",
      "src/utils/math.rs:12-28",
      "Dry Run Preview",
      "Strict Safe-Only (≥90%)",
    ]);
  });

  it("filters items by search query", async () => {
    useCDDMStore.setState({
      deadCodeSummary: mockDeadCodeSummary,
      isDeadCodeLoading: false,
      deadCodeError: null,
      lastPruneResult: null,
    });

    await renderAsyncWithWin2x(<DeadCodeExplorerModal isOpen={true} onClose={vi.fn()} />);

    const searchInput = screen.getByPlaceholderText("Search file, symbol...");
    fireEvent.change(searchInput, { target: { value: "math.rs" } });

    expect(screen.getByText("unused_calculator")).toBeDefined();
    expect(screen.queryByText("<unreachable_statement>")).toBeNull();
  });

  it("handles item selection and pruning execution", async () => {
    const pruneMock = vi.fn().mockResolvedValue(mockPruneResult);
    useCDDMStore.setState({
      deadCodeSummary: mockDeadCodeSummary,
      isDeadCodeLoading: false,
      deadCodeError: null,
      isDeadCodePruning: false,
      lastPruneResult: null,
      pruneDeadCode: pruneMock,
    });

    await renderAsyncWithWin2x(<DeadCodeExplorerModal isOpen={true} onClose={vi.fn()} />);

    const selectAllBtn = screen.getByText("Select All");
    fireEvent.click(selectAllBtn);

    const pruneBtn = screen.getByRole("button", { name: /Preview Pruning/i });
    expect(pruneBtn).toBeDefined();
    fireEvent.click(pruneBtn);

    expect(pruneMock).toHaveBeenCalled();
  });

  it("displays last prune result notification banner", async () => {
    useCDDMStore.setState({
      deadCodeSummary: mockDeadCodeSummary,
      isDeadCodeLoading: false,
      deadCodeError: null,
      lastPruneResult: mockPruneResult,
    });

    await renderAsyncWithWin2x(<DeadCodeExplorerModal isOpen={true} onClose={vi.fn()} />);

    expect(screen.getByText(/\[DRY RUN\]/i)).toBeDefined();
    expect(screen.getByText(/Pruned 2 items \(32 LOC saved\)/i)).toBeDefined();
  });

  it("filters items by workspace package and renders reachability badges", async () => {
    const summaryWithPackages: DeadCodeSummary = {
      ...mockDeadCodeSummary,
      items: [
        {
          ...mockItem1,
          package_name: "core",
          is_exported: true,
          cross_package_callers: ["cli", "mcp"],
        },
        {
          ...mockItem2,
          package_name: "webui",
        },
      ],
    };

    useCDDMStore.setState({
      deadCodeSummary: summaryWithPackages,
      isDeadCodeLoading: false,
      deadCodeError: null,
      lastPruneResult: null,
    });

    await renderAsyncWithWin2x(<DeadCodeExplorerModal isOpen={true} onClose={vi.fn()} />);

    expect(screen.getAllByText("core").length).toBeGreaterThanOrEqual(1);
    expect(screen.getByText("Exported")).toBeDefined();
    expect(screen.getByText("2 callers")).toBeDefined();
    expect(screen.getAllByText("webui").length).toBeGreaterThanOrEqual(1);

    const pkgFilter = screen.getByLabelText("Filter dead code by workspace package");
    fireEvent.change(pkgFilter, { target: { value: "core" } });

    expect(screen.getByText("unused_calculator")).toBeDefined();
    expect(screen.queryByText("<unreachable_statement>")).toBeNull();
  });
});
