import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vitest";
import { ScanConfigPanel } from "../ScanConfigPanel";
import { useCDDMStore } from "../../store/cddm-store";

describe("ScanConfigPanel Component", () => {
  beforeEach(() => {
    useCDDMStore.getState().resetScan();
  });

  it("should render inputs and controls correctly", () => {
    render(<ScanConfigPanel />);
    expect(screen.getByText("Scan Configuration")).toBeDefined();
    expect(screen.getByPlaceholderText("e.g. ./src or /path/to/repo")).toBeDefined();
    expect(screen.getByText("Run Duplicate Analysis")).toBeDefined();
  });

  it("should update target directory input", () => {
    render(<ScanConfigPanel />);
    const dirInput = screen.getByPlaceholderText("e.g. ./src or /path/to/repo") as HTMLInputElement;
    fireEvent.change(dirInput, { target: { value: "./crates/cddm-core" } });
    expect(useCDDMStore.getState().config.directory).toBe("./crates/cddm-core");
  });
});
