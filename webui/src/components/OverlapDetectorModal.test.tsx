import { render, screen, fireEvent, act } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { OverlapDetectorModal } from "./OverlapDetectorModal";
import { Win2xManagerProvider } from "./ui/win2x-manager/context/win2x-manager-context";
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
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes("/api/overlap/catalog")) {
        return Promise.resolve({
          ok: true,
          json: () =>
            Promise.resolve([
              {
                name: "Array Chunking",
                category: "Collections",
                description: "Chunking helper",
                canonical_keywords: ["chunk", "batch"],
                recommendations: [],
              },
            ]),
        });
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve(mockResult),
      });
    });
  });

  it("should not render when isOpen is false", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <OverlapDetectorModal isOpen={false} onClose={vi.fn()} initialScanResult={null} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render modal with matches when open", async () => {
    await act(async () => {
      render(
        <Win2xManagerProvider>
          <OverlapDetectorModal isOpen={true} onClose={vi.fn()} initialScanResult={mockResult} />
        </Win2xManagerProvider>,
      );
    });

    expect(screen.getByText("Ecosystem Library Reimplementation & Overlap Detector")).toBeDefined();
    expect(screen.getByText("Detected Matches (1)")).toBeDefined();
    expect(screen.getByText("Array Chunking")).toBeDefined();
    expect(screen.getByText("Collections")).toBeDefined();
    expect(screen.getByText("95% Confidence")).toBeDefined();
    expect(screen.getByText("cargo add itertools")).toBeDefined();
  });

  it("should switch tabs to algorithm catalog", async () => {
    await act(async () => {
      render(
        <Win2xManagerProvider>
          <OverlapDetectorModal isOpen={true} onClose={vi.fn()} initialScanResult={mockResult} />
        </Win2xManagerProvider>,
      );
    });

    const catalogBtn = screen.getByText(/Algorithm Catalog/);
    await act(async () => {
      fireEvent.click(catalogBtn);
    });

    expect(screen.getByText("Chunking helper")).toBeDefined();
    expect(screen.getByText("Keywords: chunk, batch")).toBeDefined();
  });
});
