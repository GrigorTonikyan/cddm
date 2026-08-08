
import { render, screen } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vitest";
import { ScanResults } from "../ScanResults";
import { useCDDMStore } from "../../store/cddm-store";

describe("ScanResults Component", () => {
  beforeEach(() => {
    useCDDMStore.getState().resetScan();
  });

  it("should return null when results is null", () => {
    const { container } = render(<ScanResults />);
    expect(container.firstChild).toBeNull();
  });

  it("should render DRY health score and clone details when results exist", () => {
    useCDDMStore.setState({
      results: {
        scan_id: "demo-scan-123",
        total_files: 42,
        total_tokens: 15420,
        total_clones: 3,
        duplication_percentage: 4.85,
        dry_health_score: 92.7,
        duration_ms: 12,
        clone_pairs: [],
        language_breakdown: [
          { language: "Rust", files: 10, tokens: 1000, clones: 1 },
        ],
      },
    });
    render(<ScanResults />);
    expect(screen.getByText("DRY Health Score")).toBeDefined();
    expect(screen.getByText("92.7")).toBeDefined();
    expect(screen.getByText(/4\.85/i)).toBeDefined();
    expect(screen.getByText("Language Breakdown")).toBeDefined();
  });

  it("should render clone pair count", () => {
    useCDDMStore.setState({
      results: {
        scan_id: "demo-scan-123",
        total_files: 42,
        total_tokens: 15420,
        total_clones: 3,
        duplication_percentage: 4.85,
        dry_health_score: 92.7,
        duration_ms: 12,
        clone_pairs: [],
        language_breakdown: [],
      },
    });
    render(<ScanResults />);
    // clone count is 3
    expect(screen.getByText("3")).toBeDefined();
    expect(screen.getByText("Clone Pairs")).toBeDefined();
  });
});
