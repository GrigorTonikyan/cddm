import { render, screen } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vite-plus/test";
import App from "../../App";
import { useCDDMStore } from "../../store/cddm-store";

describe("App Component", () => {
  beforeEach(() => {
    useCDDMStore.getState().resetScan();
  });

  it("should render CDDM Studio header", () => {
    render(<App />);
    expect(screen.getByText("CDDM Studio")).toBeDefined();
  });

  it("should render ScanConfigPanel component", () => {
    render(<App />);
    expect(screen.getByText("Scan Configuration")).toBeDefined();
  });

  it("should show error banner when store has error", () => {
    useCDDMStore.setState({ error: "Something went wrong!" });
    render(<App />);
    expect(screen.getByText("Something went wrong!")).toBeDefined();
  });
});
