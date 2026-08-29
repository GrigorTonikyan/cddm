import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { HealthAuditModal } from "./HealthAuditModal";
import { createMockScanResult, renderWithWin2x } from "../test/test-helpers";

describe("HealthAuditModal Component", () => {
  it("should return null when not open", () => {
    const { container } = renderWithWin2x(
      <HealthAuditModal isOpen={false} onClose={() => {}} results={createMockScanResult()} />,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render health audit diagnostics and pass badge when duplication is low", () => {
    const onClose = vi.fn();
    const mockResult = createMockScanResult({
      dry_health_score: 94.5,
      duplication_percentage: 3.5,
    });

    renderWithWin2x(<HealthAuditModal isOpen={true} onClose={onClose} results={mockResult} />);

    expect(screen.getByText("DRY Health Score Audit & Diagnostics")).toBeDefined();
    expect(screen.getByText("Score: 94.5/100")).toBeDefined();
    expect(screen.getByText("94.5")).toBeDefined();
    expect(screen.getByText("[PASS] Threshold")).toBeDefined();
    expect(screen.getByText(/3\.50% Duplication/i)).toBeDefined();

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should render fail badge when duplication exceeds threshold", () => {
    const mockResult = createMockScanResult({
      dry_health_score: 52.0,
      duplication_percentage: 22.4,
    });

    renderWithWin2x(<HealthAuditModal isOpen={true} onClose={() => {}} results={mockResult} />);

    expect(screen.getByText("[FAIL] Threshold")).toBeDefined();
    expect(screen.getByText(/22\.40% Duplication/i)).toBeDefined();
  });
});
