import { render, screen } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vite-plus/test";
import { ScanResults } from "../ScanResults";
import { useCDDMStore } from "../../store/cddm-store";
import { resetTestStore, createMockScanResult } from "./test-helpers";

describe("ScanResults Component", () => {
  beforeEach(resetTestStore);

  it("should return null when results is null", () => {
    const { container } = render(<ScanResults />);
    expect(container.firstChild).toBeNull();
  });

  it("should render DRY health score and clone details when results exist", () => {
    useCDDMStore.setState({
      results: createMockScanResult(),
    });
    render(<ScanResults />);
    expect(screen.getByText("DRY Health Score")).toBeDefined();
    expect(screen.getByText("92.7")).toBeDefined();
    expect(screen.getByText(/4\.85/i)).toBeDefined();
    expect(screen.getByText("Language Breakdown")).toBeDefined();
  });

  it("should render clone pair count", () => {
    useCDDMStore.setState({
      results: createMockScanResult({
        total_clones: 5,
        duplication_percentage: 12.3,
      }),
    });
    render(<ScanResults />);
    expect(screen.getByText("5")).toBeDefined();
    expect(screen.getByText(/12\.30/i)).toBeDefined();
  });
});
