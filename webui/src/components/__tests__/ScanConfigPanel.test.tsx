import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vite-plus/test";
import { ScanConfigPanel } from "../ScanConfigPanel";
import { useCDDMStore } from "../../store/cddm-store";
import { resetTestStore } from "./test-helpers";

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
});
