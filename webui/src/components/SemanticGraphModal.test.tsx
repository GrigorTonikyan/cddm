import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { SemanticGraphModal } from "./SemanticGraphModal";
import { Win2xManagerProvider } from "./ui/win2x-manager/context/win2x-manager-context";
import { useCDDMStore } from "./../store/cddm-store";
import type { SemanticGraphResponse } from "./../types/cddm-types";

describe("SemanticGraphModal Component", () => {
  const mockResponse: SemanticGraphResponse = {
    cfgs: [
      {
        file_path: "src/calc.rs",
        function_name: "compute_a",
        line_start: 1,
        line_end: 8,
        nodes: [
          {
            id: 0,
            node_type: "Entry",
            label: "entry",
            statement_count: 1,
            line_start: 1,
            line_end: 1,
          },
          {
            id: 1,
            node_type: "Branch",
            label: "if x > 0",
            statement_count: 1,
            line_start: 2,
            line_end: 2,
          },
          {
            id: 2,
            node_type: "Return",
            label: "return x",
            statement_count: 1,
            line_start: 3,
            line_end: 3,
          },
        ],
        edges: [
          { from: 0, to: 1, edge_type: "Sequential" },
          { from: 1, to: 2, edge_type: "TrueBranch" },
        ],
        wl_hash: 0x12345678,
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
    const { container } = render(
      <Win2xManagerProvider>
        <SemanticGraphModal isOpen={false} onClose={() => {}} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render semantic graph modal with CFG nodes, similarity badge, and controls", () => {
    const onClose = vi.fn();
    render(
      <Win2xManagerProvider>
        <SemanticGraphModal isOpen={true} onClose={onClose} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Deep Semantic Graph & Polyglot Isomorphism Engine")).toBeDefined();
    expect(screen.getByText("95.0% Isomorphic")).toBeDefined();
    expect(screen.getByText("Type-4 Similarity: 95.0%")).toBeDefined();
    expect(screen.getByText("Fragment A: compute_a")).toBeDefined();

    // Node types inside SVG text
    expect(screen.getByText("Entry")).toBeDefined();
    expect(screen.getByText("Branch")).toBeDefined();
    expect(screen.getByText("Return")).toBeDefined();

    // Close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should switch between visualizer, sandbox, and cross-language explorer tabs", () => {
    render(
      <Win2xManagerProvider>
        <SemanticGraphModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    const sandboxTab = screen.getByText("Polyglot Sandbox");
    fireEvent.click(sandboxTab);

    expect(screen.getByText("Implementation A:")).toBeDefined();
    expect(screen.getByText("Implementation B:")).toBeDefined();
    expect(screen.getByText("Extract CFGs & Compare Isomorphism")).toBeDefined();

    const crossLangTab = screen.getByText("Cross-Language Explorer");
    fireEvent.click(crossLangTab);
    expect(screen.getByText("Discover Polyglot Clones")).toBeDefined();
  });
});
