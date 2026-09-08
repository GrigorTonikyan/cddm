import { describe, it, expect, vi } from "vite-plus/test";
import { render, screen, fireEvent } from "@testing-library/react";
import { CrossLanguageExplorerTab } from "./CrossLanguageExplorerTab";
import { useCDDMStore } from "../../store/cddm-store";

describe("CrossLanguageExplorerTab", () => {
  it("renders tabs and triggers mode switch to neural embeddings", () => {
    const onInspectPair = vi.fn();
    render(<CrossLanguageExplorerTab onInspectPair={onInspectPair} />);

    expect(screen.getByText("Graph Hybrid (CFG/PDG)")).toBeDefined();
    expect(screen.getByText("Local Neural Embeddings")).toBeDefined();
    expect(screen.getByText("Discover Polyglot Clones")).toBeDefined();

    // Click Local Neural Embeddings button
    fireEvent.click(screen.getByText("Local Neural Embeddings"));
    expect(screen.getByText("Run Neural Embedding Scan")).toBeDefined();
    expect(screen.getByText("Neural Cosine Cutoff:")).toBeDefined();
  });

  it("renders neural scan results table when pairs exist in store", () => {
    const onInspectPair = vi.fn();
    useCDDMStore.setState({
      neuralResult: {
        total_blocks_embedded: 50,
        total_neural_pairs: 1,
        high_confidence_count: 1,
        pairs: [
          {
            file_a: "crates/cddm-core/src/a.rs",
            start_line_a: 5,
            end_line_a: 15,
            language_a: "rs",
            file_b: "crates/cddm-core/src/b.rs",
            start_line_b: 20,
            end_line_b: 30,
            language_b: "rs",
            similarity: 0.94,
            confidence: "High",
            semantic_rationale: "Neural cosine similarity 94.0%",
          },
        ],
      },
    });

    render(<CrossLanguageExplorerTab onInspectPair={onInspectPair} />);

    // Switch to neural mode
    fireEvent.click(screen.getByText("Local Neural Embeddings"));

    expect(screen.getByText("crates/cddm-core/src/a.rs")).toBeDefined();
    expect(screen.getByText("crates/cddm-core/src/b.rs")).toBeDefined();
    expect(screen.getByText("94.0%")).toBeDefined();
    expect(screen.getByText("High")).toBeDefined();
  });

  it("renders HNSW and SQ8 toggles and displays memory compression badge", () => {
    const onInspectPair = vi.fn();
    useCDDMStore.setState({
      neuralResult: {
        total_blocks_embedded: 100,
        total_neural_pairs: 0,
        high_confidence_count: 0,
        pairs: [],
        index_type: "hnsw_sq8",
        memory_reduction_ratio: 4.0,
      },
    });

    render(<CrossLanguageExplorerTab onInspectPair={onInspectPair} />);

    // Switch to neural mode
    fireEvent.click(screen.getByText("Local Neural Embeddings"));

    expect(screen.getByText("HNSW Search")).toBeDefined();
    expect(screen.getByText("SQ8 4x")).toBeDefined();
    expect(screen.getByText("Index: hnsw_sq8")).toBeDefined();
    expect(screen.getByText("4.0x Memory Compression")).toBeDefined();

    // Toggle SQ8 checkbox
    const sq8Checkbox = screen.getByLabelText("SQ8 4x");
    fireEvent.click(sq8Checkbox);
    expect((sq8Checkbox as HTMLInputElement).checked).toBe(true);
  });
});
