import { render, screen } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vite-plus/test";
import { ScanProgressBar } from "../ScanProgressBar";
import { useCDDMStore } from "../../store/cddm-store";
import { resetTestStore, createMockProgress } from "./test-helpers";

describe("ScanProgressBar Component", () => {
  beforeEach(resetTestStore);

  it("should return null when not scanning", () => {
    const { container } = render(<ScanProgressBar />);
    expect(container.firstChild).toBeNull();
  });

  it("should render progress bar when scanning", () => {
    useCDDMStore.setState({
      isScanning: true,
      progress: createMockProgress(),
    });
    render(<ScanProgressBar />);
    expect(screen.getByText(/Phase: Tokenization/i)).toBeDefined();
  });

  it("should display phase name and percentage", () => {
    useCDDMStore.setState({
      isScanning: true,
      progress: createMockProgress({
        progress: 0.75,
        phase: "Indexing",
        message: "Indexing clones...",
        files_processed: 15,
      }),
    });
    render(<ScanProgressBar />);
    expect(screen.getByText(/Phase: Indexing/i)).toBeDefined();
    expect(screen.getByText(/75%/i)).toBeDefined();
  });
});
