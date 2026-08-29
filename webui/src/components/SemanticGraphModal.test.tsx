import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { SemanticGraphModal } from "./SemanticGraphModal";
import {
  assertModalClosesOnButtonClick,
  createMockControlFlowGraph,
  expectDefinedTexts,
  renderWithWin2x,
} from "../test/test-helpers";
import { useCDDMStore } from "./../store/cddm-store";
import type { SemanticGraphResponse } from "./../types/cddm-types";

describe("SemanticGraphModal Component", () => {
  const mockResponse: SemanticGraphResponse = {
    cfgs: [
      {
        ...createMockControlFlowGraph(),
        function_name: "compute_a",
        line_end: 8,
      },
    ],
    pdgs: [
      {
        cfg: {
          file_path: "src/calc.rs",
          function_name: "compute_a",
          line_start: 1,
          line_end: 8,
          nodes: [],
          edges: [],
          wl_hash: 0x12345678,
        },
        data_edges: [
          {
            from: 0,
            to: 2,
            variable: "x",
            kind: "DataDependency",
          },
        ],
      },
    ],
    comparison: {
      similarity: 0.95,
      is_semantic_clone: true,
      wl_hash_a: 0x12345678,
      wl_hash_b: 0x12345678,
    },
  };

  beforeEach(() => {
    useCDDMStore.setState({
      semanticGraphResponse: mockResponse,
      isSemanticGraphLoading: false,
      semanticGraphError: null,
    });
  });

  it("should return null when not open", () => {
    const { container } = renderWithWin2x(<SemanticGraphModal isOpen={false} onClose={() => {}} />);
    expect(container.firstChild).toBeNull();
  });

  it("should render semantic graph modal with CFG nodes, similarity badge, and controls", () => {
    const onClose = vi.fn();
    renderWithWin2x(<SemanticGraphModal isOpen={true} onClose={onClose} />);

    expectDefinedTexts([
      "Deep Semantic Graph & Polyglot Isomorphism Engine",
      "95.0% Isomorphic",
      "Type-4 Similarity: 95.0%",
      "Fragment A: compute_a",
      "Entry",
      "Branch",
      "Return",
    ]);

    assertModalClosesOnButtonClick(onClose);
  });

  it("should switch between visualizer, sandbox, and cross-language explorer tabs", () => {
    renderWithWin2x(<SemanticGraphModal isOpen={true} onClose={() => {}} />);

    const sandboxTab = screen.getByText("Polyglot Sandbox");
    fireEvent.click(sandboxTab);

    expectDefinedTexts([
      "Implementation A:",
      "Implementation B:",
      "Extract CFGs & Compare Isomorphism",
    ]);

    const crossLangTab = screen.getByText("Cross-Language Explorer");
    fireEvent.click(crossLangTab);
    expect(screen.getByText("Discover Polyglot Clones")).toBeDefined();
  });
});
