import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, beforeEach, vi } from "vite-plus/test";
import App from "./App";
import { useCDDMStore } from "./store/cddm-store";
import { createMockScanResult } from "./test/test-helpers";
import { Win2xManagerProvider } from "./components/ui/win2x-manager/context/win2x-manager-context";

describe("App Component", () => {
  beforeEach(() => {
    useCDDMStore.getState().resetScan();
    global.fetch = vi.fn().mockImplementation(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({}),
        text: () => Promise.resolve(""),
      }),
    );
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

  it("should open ScanConfigModal when clicking Config Window in header", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const configBtn = screen.getByText("Config Window");
    fireEvent.click(configBtn);
    expect(
      await screen.findByText("Scan Parameters & Engine Configuration", {}, { timeout: 15000 }),
    ).toBeDefined();
  });

  it("should open HealthAuditModal and ExportReportModal from header when results exist", async () => {
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
    expect(
      await screen.findByText("DRY Health Score Audit & Diagnostics", {}, { timeout: 15000 }),
    ).toBeDefined();

    // Test Reports modal
    const reportsBtns = screen.getAllByText("Reports");
    fireEvent.click(reportsBtns[0]!);
    expect(
      await screen.findByText("Report Center & SARIF Exporter", {}, { timeout: 15000 }),
    ).toBeDefined();
  });

  it("should open OverlapDetectorModal when clicking Overlap Detector in header", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const overlapBtn = screen.getByText("Overlap Detector");
    fireEvent.click(overlapBtn);
    expect(
      await screen.findByText(
        "Ecosystem Library Reimplementation & Overlap Detector",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });

  it("should open HubFederationModal when clicking Org Hub in header", async () => {
    render(
      <Win2xManagerProvider>
        <App />
      </Win2xManagerProvider>,
    );

    const hubBtn = screen.getByText("Org Hub");
    fireEvent.click(hubBtn);
    expect(
      await screen.findByText(
        "Organization Federation Hub (.cddmhub.toml)",
        {},
        { timeout: 15000 },
      ),
    ).toBeDefined();
  });
});
