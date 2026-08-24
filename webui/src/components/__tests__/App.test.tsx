import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vite-plus/test";
import App from "../../App";
import { useCDDMStore } from "../../store/cddm-store";
import { createMockScanResult } from "./test-helpers";
import { Win2xManagerProvider } from "../ui/win2x-manager/context/win2x-manager-context";

describe("App Component", () => {
  beforeEach(() => {
    useCDDMStore.getState().resetScan();
  });

  it("should render CDDM Studio header", () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );
    expect(screen.getByText("CDDM Studio")).toBeDefined();
  });

  it("should render ScanConfigPanel component", () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );
    expect(screen.getByText("Scan Configuration")).toBeDefined();
  });

  it("should show error banner when store has error", () => {
    useCDDMStore.setState({ error: "Something went wrong!" });
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );
    expect(screen.getByText("Something went wrong!")).toBeDefined();
  });

  it("should open ScanConfigModal when clicking Config Window in header", () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const configBtn = screen.getByText("Config Window");
    fireEvent.click(configBtn);
    expect(screen.getByText("Scan Parameters & Engine Configuration")).toBeDefined();
  });

  it("should open HealthAuditModal and ExportReportModal from header when results exist", () => {
    useCDDMStore.setState({
      results: createMockScanResult(),
    });

    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    // Test Health Audit modal
    const healthBtns = screen.getAllByText("Health Audit");
    fireEvent.click(healthBtns[0]!);
    expect(screen.getByText("DRY Health Score Audit & Diagnostics")).toBeDefined();

    // Test Reports modal
    const reportsBtns = screen.getAllByText("Reports");
    fireEvent.click(reportsBtns[0]!);
    expect(screen.getByText("Report Center & SARIF Exporter")).toBeDefined();
  });
});
