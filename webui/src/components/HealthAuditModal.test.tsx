import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { HealthAuditModal } from "./HealthAuditModal";
import { createMockScanResult } from "../test/test-helpers";
import { Win2xManagerProvider } from "./ui/win2x-manager/context/win2x-manager-context";

describe("HealthAuditModal Component", () => {
  it("should return null when not open", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <HealthAuditModal isOpen={false} onClose={() => {}} results={createMockScanResult()} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render health audit diagnostics and pass badge when duplication is low", () => {
    const onClose = vi.fn();
    const mockResult = createMockScanResult({
      dry_health_score: 94.5,
      duplication_percentage: 5.5,
    });

    render(
      <Win2xManagerProvider>
        <HealthAuditModal isOpen={true} onClose={onClose} results={mockResult} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("DRY Health Score Audit & Diagnostics")).toBeDefined();
    expect(screen.getByText("Score: 94.5/100")).toBeDefined();
    expect(screen.getByText("94.5")).toBeDefined();
    expect(screen.getByText("[PASS] Threshold")).toBeDefined();
    expect(screen.getByText(/5\.50% Duplication/i)).toBeDefined();

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

    render(
      <Win2xManagerProvider>
        <HealthAuditModal isOpen={true} onClose={() => {}} results={mockResult} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("[FAIL] Threshold")).toBeDefined();
    expect(screen.getByText(/22\.40% Duplication/i)).toBeDefined();
  });
});
