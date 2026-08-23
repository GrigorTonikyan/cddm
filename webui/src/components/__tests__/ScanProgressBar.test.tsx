import { render, screen } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vite-plus/test";
import { ScanProgressBar } from "../ScanProgressBar";
import { useCDDMStore } from "../../store/cddm-store";

describe("ScanProgressBar Component", () => {
  beforeEach(() => {
    useCDDMStore.getState().resetScan();
  });

  it("should return null when not scanning", () => {
    const { container } = render(<ScanProgressBar />);
    expect(container.firstChild).toBeNull();
  });

  it("should render progress bar when scanning", () => {
    useCDDMStore.setState({
      isScanning: true,
      progress: {
        progress: 0.5,
        phase: "Tokenization",
        message: "Tokenizing files...",
        files_processed: 10,
        total_files: 20,
        scan_id: "123",
      },
    });
    render(<ScanProgressBar />);
    expect(screen.getByText(/Phase: Tokenization/i)).toBeDefined();
  });

  it("should display phase name and percentage", () => {
    useCDDMStore.setState({
      isScanning: true,
      progress: {
        progress: 0.75,
        phase: "Indexing",
        message: "Indexing clones...",
        files_processed: 15,
        total_files: 20,
        scan_id: "123",
      },
    });
    render(<ScanProgressBar />);
    expect(screen.getByText(/Phase: Indexing/i)).toBeDefined();
    expect(screen.getByText("75%")).toBeDefined();
  });
});
