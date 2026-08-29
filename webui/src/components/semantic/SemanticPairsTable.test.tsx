import { describe, it, expect, vi } from "vite-plus/test";
import { render, screen, fireEvent } from "@testing-library/react";
import { SemanticPairsTable } from "./SemanticPairsTable";

describe("SemanticPairsTable", () => {
  it("renders hybrid pairs with inspection callback", () => {
    const onInspect = vi.fn();
    render(
      <SemanticPairsTable
        mode="hybrid"
        hybridPairs={[
          {
            file_a: "a.rs",
            function_a: "fn_a",
            lines_a: [1, 10],
            language_a: "rs",
            file_b: "b.py",
            function_b: "fn_b",
            lines_b: [5, 15],
            language_b: "py",
            graph_similarity: 0.9,
            token_similarity: 0.85,
            hybrid_score: 0.88,
            clone_type: "Type4Semantic",
          },
        ]}
        onInspectPair={onInspect}
      />,
    );

    expect(screen.getByText("fn_a")).toBeDefined();
    expect(screen.getByText("fn_b")).toBeDefined();
    expect(screen.getByText("Inspect")).toBeDefined();

    fireEvent.click(screen.getByText("Inspect"));
    expect(onInspect).toHaveBeenCalled();
  });

  it("renders neural pairs with confidence and rationale", () => {
    render(
      <SemanticPairsTable
        mode="neural"
        neuralPairs={[
          {
            file_a: "pkg/core.rs",
            start_line_a: 10,
            end_line_a: 20,
            language_a: "rs",
            file_b: "pkg/lib.ts",
            start_line_b: 30,
            end_line_b: 40,
            language_b: "ts",
            similarity: 0.95,
            confidence: "High",
            semantic_rationale: "Algorithmic clone across Rust and TypeScript",
          },
        ]}
      />,
    );

    expect(screen.getByText("pkg/core.rs")).toBeDefined();
    expect(screen.getByText("pkg/lib.ts")).toBeDefined();
    expect(screen.getByText("95.0%")).toBeDefined();
    expect(screen.getByText("High")).toBeDefined();
  });
});
