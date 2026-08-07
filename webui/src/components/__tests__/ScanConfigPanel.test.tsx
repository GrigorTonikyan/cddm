
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

  it("should update min tokens slider", () => {
    render(<ScanConfigPanel />);
    // The range input has min/max/step so we can find it by its role or just any range input.
    // Actually we can find it by finding the input type range or the label.
    // It's easier to find it by value or test-id, but we can just find it by type range.
    const slider = document.querySelector('input[type="range"]') as HTMLInputElement;
    fireEvent.change(slider, { target: { value: "100" } });
    expect(useCDDMStore.getState().config.min_tokens).toBe(100);
  });

  it("should render git blame toggle", () => {
    render(<ScanConfigPanel />);
    expect(screen.getByText("Git Blame (Authors)")).toBeDefined();
  });
});
