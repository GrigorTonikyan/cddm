import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { ScanConfigModal } from "./ScanConfigModal";
import { Win2xManagerProvider } from "./ui/win2x-manager/context/win2x-manager-context";

describe("ScanConfigModal Component", () => {
  it("should return null when not open", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <ScanConfigModal isOpen={false} onClose={() => {}} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render Scan Configuration window when open", () => {
    const onClose = vi.fn();

    render(
      <Win2xManagerProvider>
        <ScanConfigModal isOpen={true} onClose={onClose} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Scan Parameters & Engine Configuration")).toBeDefined();
    expect(screen.getByText("M61 Engine")).toBeDefined();
    expect(screen.getByText("Run Duplicate Analysis")).toBeDefined();

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });
});
