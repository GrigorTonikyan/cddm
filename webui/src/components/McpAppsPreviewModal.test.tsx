import { fireEvent, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vite-plus/test";
import { renderWithWin2x } from "../test/test-helpers";
import { McpAppsPreviewModal } from "./McpAppsPreviewModal";

describe("McpAppsPreviewModal", () => {
  it("does not render when isOpen is false", () => {
    const { container } = renderWithWin2x(<McpAppsPreviewModal isOpen={false} onClose={vi.fn()} />);
    expect(container.firstChild).toBeNull();
  });

  it("renders modal header, tabs, and iframe when open", () => {
    renderWithWin2x(<McpAppsPreviewModal isOpen={true} onClose={vi.fn()} />);
    expect(screen.getByText("MCP Apps Generative UI Studio")).toBeDefined();
    expect(screen.getByText("MCP 2026-07-28")).toBeDefined();
    expect(screen.getByText("Diff Split-View Widget")).toBeDefined();
    expect(screen.getByText("Cluster Treemap Widget")).toBeDefined();
    expect(screen.getByText("Raw HTML & Schema")).toBeDefined();
  });

  it("switches to Treemap tab and Raw HTML tab", () => {
    renderWithWin2x(<McpAppsPreviewModal isOpen={true} onClose={vi.fn()} />);

    const treemapTab = screen.getByText("Cluster Treemap Widget");
    fireEvent.click(treemapTab);
    expect(screen.getByText("Active widget: cluster-treemap")).toBeDefined();

    const rawTab = screen.getByText("Raw HTML & Schema");
    fireEvent.click(rawTab);
    expect(screen.getByText(/<!DOCTYPE html>/)).toBeDefined();
  });

  it("calls onClose when close button is clicked", () => {
    const handleClose = vi.fn();
    renderWithWin2x(<McpAppsPreviewModal isOpen={true} onClose={handleClose} />);

    const closeBtn = screen.getByTitle("Close");
    fireEvent.click(closeBtn);
    expect(handleClose).toHaveBeenCalledTimes(1);
  });
});
