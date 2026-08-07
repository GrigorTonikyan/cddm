import React from "react";
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

  it("should render DRY health score and clone details when results exist", async () => {
    await useCDDMStore.getState().startScan();
    render(<ScanResults />);
    expect(screen.getByText("DRY Health Score")).toBeDefined();
    expect(screen.getByText("92.7")).toBeDefined();
    expect(screen.getByText("4.85%")).toBeDefined();
    expect(screen.getByText("Language Breakdown")).toBeDefined();
  });

  it("should render clone pair count", async () => {
    await useCDDMStore.getState().startScan();
    render(<ScanResults />);
    // clone count is 3
    expect(screen.getByText("3")).toBeDefined();
    expect(screen.getByText("Clone Pairs")).toBeDefined();
  });
});
