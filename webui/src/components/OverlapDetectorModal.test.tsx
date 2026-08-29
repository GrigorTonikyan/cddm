import { screen } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { OverlapDetectorModal } from "./OverlapDetectorModal";
import {
  clickElementAsync,
  expectDefinedTexts,
  expectNullWhenClosed,
  mockSuccessResponse,
  renderAsyncWithWin2x,
} from "../test/test-helpers";
import type { OverlapScanResult } from "../types/cddm-types";

describe("OverlapDetectorModal Component", () => {
  const mockResult: OverlapScanResult = {
    matches: [
      {
        algorithm_name: "Array Chunking",
        category: "Collections",
        file_path: "src/utils.rs",
        function_name: "chunk_items",
        line_span: [10, 25],
        confidence: 0.95,
        snippet: "pub fn chunk_items() {}",
        recommended_library: {
          language: "rust",
          package_name: "itertools",
          install_command: "cargo add itertools",
          replacement_snippet: "items.chunks(size)",
        },
      },
    ],
    total_files_scanned: 15,
    scanned_functions: 42,
    summary: "Discovered 1 ecosystem library overlap match",
  };

  beforeEach(() => {
    vi.restoreAllMocks();
    globalThis.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes("/api/overlap/catalog")) {
        return mockSuccessResponse([
          {
            name: "Array Chunking",
            category: "Collections",
            description: "Chunking helper",
            canonical_keywords: ["chunk", "batch"],
            recommendations: [],
          },
        ]);
      }
      return mockSuccessResponse(mockResult);
    });
  });

  it("should not render when isOpen is false", () => {
    expectNullWhenClosed(
      <OverlapDetectorModal isOpen={false} onClose={vi.fn()} initialScanResult={null} />,
    );
  });

  it("should render modal with scan results when open", async () => {
    await renderAsyncWithWin2x(
      <OverlapDetectorModal isOpen={true} onClose={vi.fn()} initialScanResult={mockResult} />,
    );

    expectDefinedTexts([
      "Ecosystem Library Reimplementation & Overlap Detector",
      "Detected Matches (1)",
      "Array Chunking",
      "Collections",
      "95% Confidence",
      "cargo add itertools",
    ]);
  });

  it("should switch tabs to algorithm catalog", async () => {
    await renderAsyncWithWin2x(
      <OverlapDetectorModal isOpen={true} onClose={vi.fn()} initialScanResult={mockResult} />,
    );

    await clickElementAsync(/Algorithm Catalog/);

    expect(screen.getByText("Chunking helper")).toBeDefined();
    expect(screen.getByText("Keywords: chunk, batch")).toBeDefined();
  });
});
