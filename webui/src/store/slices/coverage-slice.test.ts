import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test";
import { createCoverageSlice } from "./coverage-slice";

describe("createCoverageSlice", () => {
  let state: Record<string, unknown> = {};
  const set = (updater: unknown) => {
    if (typeof updater === "function") {
      state = { ...state, ...updater(state) };
    } else {
      state = { ...state, ...(updater as Record<string, unknown>) };
    }
  };
  const get = () => state as never;

  beforeEach(() => {
    state = {};
    vi.restoreAllMocks();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should initialize default state correctly", () => {
    const slice = createCoverageSlice(set as never, get);
    expect(slice.isCoverageModalOpen).toBe(false);
    expect(slice.coverageSummary).toBeNull();
    expect(slice.isCoverageLoading).toBe(false);
    expect(slice.coverageError).toBeNull();
  });

  it("should update isCoverageModalOpen", () => {
    const slice = createCoverageSlice(set as never, get);
    slice.setIsCoverageModalOpen(true);
    expect(state.isCoverageModalOpen).toBe(true);
  });

  it("should correlate coverage successfully", async () => {
    const mockSummary = {
      total_clone_pairs: 2,
      overall_covered_clones_pct: 85.0,
      dead_code_clones: 1,
      test_gap_clones: 0,
      hot_path_clones: 1,
      total_runtime_hits: 1200,
      metrics: [],
    };

    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => mockSummary,
    } as Response);

    const slice = createCoverageSlice(set as never, get);
    const res = await slice.correlateCoverage({ report_content: "SF:a.ts\nDA:1,5\nend_of_record" });

    expect(res.total_clone_pairs).toBe(2);
    expect(state.coverageSummary).toEqual(mockSummary);
    expect(state.isCoverageLoading).toBe(false);
  });
});
