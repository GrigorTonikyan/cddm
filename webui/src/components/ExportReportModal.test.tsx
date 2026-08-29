import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { ExportReportModal } from "./ExportReportModal";
import {
  createMockScanResult,
  expectDefinedTexts,
  expectNullWhenClosed,
  renderWithWin2x,
} from "../test/test-helpers";

describe("ExportReportModal Component", () => {
  const mockResult = createMockScanResult();

  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("should return null when not open", () => {
    expectNullWhenClosed(
      <ExportReportModal isOpen={false} onClose={() => {}} results={mockResult} />,
    );
  });

  it("should render SARIF, JSON, Markdown, and CI tabs when open", () => {
    const onClose = vi.fn();

    renderWithWin2x(<ExportReportModal isOpen={true} onClose={onClose} results={mockResult} />);

    expectDefinedTexts([
      "Report Center & SARIF Exporter",
      "OASIS SARIF v2.1.0",
      "Scan JSON",
      "Markdown Summary",
      "CI / GitHub Actions",
      "Copy SARIF",
      "Download .sarif",
    ]);

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

    renderWithWin2x(<ExportReportModal isOpen={true} onClose={() => {}} results={mockResult} />);

    const copyBtn = screen.getByText("Copy SARIF");
    fireEvent.click(copyBtn);
    expect(writeTextMock).toHaveBeenCalled();
  });
});
