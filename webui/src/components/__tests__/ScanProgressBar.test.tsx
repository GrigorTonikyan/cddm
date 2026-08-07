import React from "react";
import { render, screen } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vitest";
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
    useCDDMStore.setState({ isScanning: true, progress: { progress: 0.5, phase: "Hashing", message: "Hashing files...", files_processed: 10, total_files: 20, scan_id: "123" } });
    render(<ScanProgressBar />);
    expect(screen.getByText(/Phase: Hashing/i)).toBeDefined();
  });

  it("should display phase name and percentage", () => {
    useCDDMStore.setState({ isScanning: true, progress: { progress: 0.75, phase: "Matching", message: "Finding clones...", files_processed: 15, total_files: 20, scan_id: "123" } });
    render(<ScanProgressBar />);
    expect(screen.getByText(/Phase: Matching/i)).toBeDefined();
    expect(screen.getByText("75%")).toBeDefined();
  });
});
