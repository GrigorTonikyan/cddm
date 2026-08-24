import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vite-plus/test";
import { ScanResults } from "../ScanResults";
import { useCDDMStore } from "../../store/cddm-store";
import { resetTestStore, createMockScanResult } from "./test-helpers";
import { Win2xManagerProvider } from "../ui/win2x-manager/context/win2x-manager-context";

describe("ScanResults Component", () => {
  beforeEach(resetTestStore);

  it("should return null when results is null", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <ScanResults />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render DRY health score and visual analytics when results exist", () => {
    useCDDMStore.setState({
      results: createMockScanResult(),
    });
    render(
      <Win2xManagerProvider>
        <ScanResults />
      </Win2xManagerProvider>,
    );
    expect(screen.getByText("DRY Health Score")).toBeDefined();
    expect(screen.getByText("92.7")).toBeDefined();
    expect(screen.getByText(/4\.85/i)).toBeDefined();
    expect(screen.getByText("Visual Analytics")).toBeDefined();
    expect(screen.getByText("Duplication Treemap")).toBeDefined();
    expect(screen.getByText("Language Breakdown")).toBeDefined();

    // Switch to language breakdown
    const langBtn = screen.getByText("Language Breakdown");
    fireEvent.click(langBtn);
    expect(screen.getByText("1 Languages Detected")).toBeDefined();
  });

  it("should open HealthAuditModal when clicking DRY health score card", () => {
    useCDDMStore.setState({
      results: createMockScanResult(),
    });
    render(
      <Win2xManagerProvider>
        <ScanResults />
      </Win2xManagerProvider>,
    );

    const healthCard = screen.getByText("DRY Health Score").closest("div");
    fireEvent.click(healthCard!);
    expect(screen.getByText("DRY Health Score Audit & Diagnostics")).toBeDefined();
  });

  it("should open TreemapExplorerModal when clicking Open in Window on Treemap", () => {
    useCDDMStore.setState({
      results: createMockScanResult(),
    });
    render(
      <Win2xManagerProvider>
        <ScanResults />
      </Win2xManagerProvider>,
    );

    const openInWindowBtn = screen.getByText("Open in Window");
    fireEvent.click(openInWindowBtn);
    expect(screen.getByText("Duplication Treemap Explorer")).toBeDefined();
  });

  it("should open ExportReportModal when clicking Export & Reports button", () => {
    useCDDMStore.setState({
      results: createMockScanResult(),
    });
    render(
      <Win2xManagerProvider>
        <ScanResults />
      </Win2xManagerProvider>,
    );

    const exportBtn = screen.getByText("Export & Reports");
    fireEvent.click(exportBtn);
    expect(screen.getByText("Report Center & SARIF Exporter")).toBeDefined();
  });

  it("should open ScanConfigModal when clicking Scan Settings button", () => {
    useCDDMStore.setState({
      results: createMockScanResult(),
    });
    render(
      <Win2xManagerProvider>
        <ScanResults />
      </Win2xManagerProvider>,
    );

    const settingsBtn = screen.getByText("Scan Settings");
    fireEvent.click(settingsBtn);
    expect(useCDDMStore.getState().isScanConfigOpen).toBe(true);
  });

  it("should render clone pair count", () => {
    useCDDMStore.setState({
      results: createMockScanResult({
        total_clones: 5,
        duplication_percentage: 12.3,
      }),
    });
    render(
      <Win2xManagerProvider>
        <ScanResults />
      </Win2xManagerProvider>,
    );
    expect(screen.getByText("5")).toBeDefined();
    expect(screen.getByText(/12\.30/i)).toBeDefined();
  });
});
