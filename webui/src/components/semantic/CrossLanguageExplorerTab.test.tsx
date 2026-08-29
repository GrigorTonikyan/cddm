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
});
