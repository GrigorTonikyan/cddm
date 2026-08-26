import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { ExportReportModal } from "./ExportReportModal";
import { createMockScanResult } from "../test/test-helpers";
import { Win2xManagerProvider } from "./ui/win2x-manager/context/win2x-manager-context";

describe("ExportReportModal Component", () => {
  const mockResult = createMockScanResult();

  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("should return null when not open", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <ExportReportModal isOpen={false} onClose={() => {}} results={mockResult} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render SARIF, JSON, Markdown, and CI tabs when open", () => {
    const onClose = vi.fn();

    render(
      <Win2xManagerProvider>
        <ExportReportModal isOpen={true} onClose={onClose} results={mockResult} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Report Center & SARIF Exporter")).toBeDefined();
    expect(screen.getByText("OASIS SARIF v2.1.0")).toBeDefined();
    expect(screen.getByText("Scan JSON")).toBeDefined();
    expect(screen.getByText("Markdown Summary")).toBeDefined();
    expect(screen.getByText("CI / GitHub Actions")).toBeDefined();

    // Verify SARIF tab is default
    expect(screen.getByText("Copy SARIF")).toBeDefined();
    expect(screen.getByText("Download .sarif")).toBeDefined();

    // Switch to JSON tab
    fireEvent.click(screen.getByText("Scan JSON"));
    expect(screen.getByText("Copy JSON")).toBeDefined();
    expect(screen.getByText("Download .json")).toBeDefined();

    // Switch to Markdown tab
    fireEvent.click(screen.getByText("Markdown Summary"));
    expect(screen.getByText("Copy Markdown")).toBeDefined();

    // Switch to CI tab
    fireEvent.click(screen.getByText("CI / GitHub Actions"));
    expect(screen.getByText("Copy YAML")).toBeDefined();

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should support copying SARIF text to clipboard", () => {
    const writeTextMock = vi.fn().mockResolvedValue(undefined);
    Object.assign(navigator, {
      clipboard: {
        writeText: writeTextMock,
      },
    });

    render(
      <Win2xManagerProvider>
        <ExportReportModal isOpen={true} onClose={() => {}} results={mockResult} />
      </Win2xManagerProvider>,
    );

    const copyBtn = screen.getByText("Copy SARIF");
    fireEvent.click(copyBtn);
    expect(writeTextMock).toHaveBeenCalled();
  });
});
