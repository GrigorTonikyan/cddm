import { describe, it, expect, beforeEach, vi } from "vite-plus/test";
import { useCDDMStore } from "./cddm-store";
import type { SemanticGraphResponse } from "./../types/cddm-types";

describe("useCDDMStore - Semantic Slice", () => {
  beforeEach(() => {
    useCDDMStore.setState({
      isSemanticGraphModalOpen: false,
      semanticGraphRequest: null,
      semanticGraphResponse: null,
      isSemanticGraphLoading: false,
      semanticGraphError: null,
    });
  });

  it("should initialize with default semantic state", () => {
    const state = useCDDMStore.getState();
    expect(state.isSemanticGraphModalOpen).toBe(false);
    expect(state.semanticGraphResponse).toBeNull();
    expect(state.isSemanticGraphLoading).toBe(false);
    expect(state.semanticGraphError).toBeNull();
  });

  it("should successfully fetch semantic graph data", async () => {
    const mockResponse: SemanticGraphResponse = {
      cfgs: [
        {
          file_path: "src/a.rs",
          function_name: "test_a",
          line_start: 1,
          line_end: 10,
          nodes: [],
          edges: [],
          wl_hash: 999,
        },
      ],
      pdgs: [],
      comparison: {
        similarity: 0.9,
        is_semantic_clone: true,
        wl_hash_a: 999,
        wl_hash_b: 999,
      },
    };

    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockResponse),
    } as Response);

    const req = { code: "fn test() {}", language: "Rust" };
    const res = await useCDDMStore.getState().fetchSemanticGraph(req);

    const state = useCDDMStore.getState();
    expect(res).toEqual(mockResponse);
    expect(state.semanticGraphResponse).toEqual(mockResponse);
    expect(state.isSemanticGraphLoading).toBe(false);
    expect(state.semanticGraphError).toBeNull();
  });

  it("should handle semantic graph fetch errors cleanly", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 400,
      text: () => Promise.resolve("Invalid code"),
    } as Response);

    await expect(useCDDMStore.getState().fetchSemanticGraph({ code: "bad code" })).rejects.toThrow(
      "Semantic graph extraction failed",
    );

    const state = useCDDMStore.getState();
    expect(state.isSemanticGraphLoading).toBe(false);
    expect(state.semanticGraphError).toContain("Semantic graph extraction failed");
  });

  it("should open modal and trigger fetch when request is provided", async () => {
    const mockResponse: SemanticGraphResponse = {
      cfgs: [],
      pdgs: [],
      comparison: null,
    };

    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockResponse),
    } as Response);

    await useCDDMStore.getState().openSemanticGraphModal({ code: "fn foo() {}" });
    const state = useCDDMStore.getState();
    expect(state.isSemanticGraphModalOpen).toBe(true);
  });

  it("should successfully scan cross-language clones", async () => {
    const mockClones = [
      {
        file_a: "src/calc.rs",
        language_a: "Rust",
        function_a: "add",
        lines_a: [1, 5] as [number, number],
        file_b: "webui/src/calc.ts",
        language_b: "TypeScript",
        function_b: "add",
        lines_b: [1, 5] as [number, number],
        graph_similarity: 0.95,
        token_similarity: 0.85,
        hybrid_score: 0.91,
        clone_type: "Semantic",
      },
    ];

    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockClones),
    } as Response);

    const res = await useCDDMStore.getState().scanCrossLanguageClones(0.75, ".");
    expect(res).toEqual(mockClones);
    const state = useCDDMStore.getState();
    expect(state.crossLanguageClones).toEqual(mockClones);
    expect(state.isCrossLanguageLoading).toBe(false);
  });

  it("should successfully execute in-process neural code scan", async () => {
    const mockNeuralResult = {
      total_blocks_embedded: 42,
      total_neural_pairs: 3,
      high_confidence_count: 2,
      pairs: [
        {
          file_a: "src/a.rs",
          start_line_a: 10,
          end_line_a: 25,
          language_a: "rs",
          file_b: "src/b.py",
          start_line_b: 15,
          end_line_b: 30,
          language_b: "py",
          similarity: 0.93,
          confidence: "High" as const,
          semantic_rationale: "Neural cosine similarity 93.0%",
        },
      ],
    };

    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockNeuralResult),
    } as Response);

    const res = await useCDDMStore.getState().scanNeuralClones({ directory: ".", threshold: 0.85 });
    expect(res).toEqual(mockNeuralResult);
    const state = useCDDMStore.getState();
    expect(state.neuralResult).toEqual(mockNeuralResult);
    expect(state.isNeuralLoading).toBe(false);
  });
});
