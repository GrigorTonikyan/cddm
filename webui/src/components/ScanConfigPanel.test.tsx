import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vite-plus/test";
import { ScanConfigPanel } from "./ScanConfigPanel";
import { useCDDMStore } from "./../store/cddm-store";
import { resetTestStore } from "../test/test-helpers";

describe("ScanConfigPanel Component", () => {
  beforeEach(resetTestStore);

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
    const slider = document.querySelector('input[type="range"]') as HTMLInputElement;
    fireEvent.change(slider, { target: { value: "100" } });
    expect(useCDDMStore.getState().config.min_tokens).toBe(100);
  });

  it("should render git blame toggle", () => {
    render(<ScanConfigPanel />);
    expect(screen.getByText("Git Blame (Authors)")).toBeDefined();
  });

  it("should render Type-3 near-miss toggle and update config state", () => {
    render(<ScanConfigPanel />);
    const type3Label = screen.getByText("Type-3 (Near-Miss Clones)");
    expect(type3Label).toBeDefined();

    const checkboxes = screen.getAllByRole("checkbox") as HTMLInputElement[];
    // Find the checkbox associated with Type-3
    const type3Checkbox = checkboxes.find((cb) =>
      cb.parentElement?.textContent?.includes("Type-3 (Near-Miss Clones)"),
    );
    expect(type3Checkbox).toBeDefined();
    expect(type3Checkbox?.checked).toBe(true);

    if (type3Checkbox) {
      fireEvent.click(type3Checkbox);
      expect(useCDDMStore.getState().config.detect_type3).toBe(false);
    }
  });

  it("should render Type-4 semantic toggle and update config state", () => {
    render(<ScanConfigPanel />);
    const type4Label = screen.getByText("Type-4 (Semantic Clones)");
    expect(type4Label).toBeDefined();

    const checkboxes = screen.getAllByRole("checkbox") as HTMLInputElement[];
    const type4Checkbox = checkboxes.find((cb) =>
      cb.parentElement?.textContent?.includes("Type-4 (Semantic Clones)"),
    );
    expect(type4Checkbox).toBeDefined();
    expect(type4Checkbox?.checked).toBe(false);

    if (type4Checkbox) {
      fireEvent.click(type4Checkbox);
      expect(useCDDMStore.getState().config.detect_type4).toBe(true);
    }
  });
});
