import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { TreemapExplorerModal } from "../TreemapExplorerModal";
import { createMockClonePair } from "./test-helpers";
import { Win2xManagerProvider } from "../ui/win2x-manager/context/win2x-manager-context";

describe("TreemapExplorerModal Component", () => {
  const mockPairs = [createMockClonePair({ file_a: "src/engine/a.ts", file_b: "src/engine/b.ts" })];

  it("should return null when not open", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <TreemapExplorerModal
          isOpen={false}
          onClose={() => {}}
          clonePairs={mockPairs}
          totalTokens={1000}
        />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render Treemap Explorer window when open", () => {
    const onClose = vi.fn();
    const onSelectFilter = vi.fn();

    render(
      <Win2xManagerProvider>
        <TreemapExplorerModal
          isOpen={true}
          onClose={onClose}
          clonePairs={mockPairs}
          totalTokens={1000}
          onSelectFilterPath={onSelectFilter}
        />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Duplication Treemap Explorer")).toBeDefined();
    expect(screen.getByText("1 Clones")).toBeDefined();
    expect(screen.getByPlaceholderText(/Filter treemap/i)).toBeDefined();
    expect(screen.getByText("Duplication Treemap Visualizer")).toBeDefined();

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should support updating internal filter text", () => {
    const onSelectFilter = vi.fn();

    render(
      <Win2xManagerProvider>
        <TreemapExplorerModal
          isOpen={true}
          onClose={() => {}}
          clonePairs={mockPairs}
          totalTokens={1000}
          onSelectFilterPath={onSelectFilter}
        />
      </Win2xManagerProvider>,
    );

    const input = screen.getByPlaceholderText(/Filter treemap/i);
    fireEvent.change(input, { target: { value: "src/engine" } });
    expect(onSelectFilter).toHaveBeenCalledWith("src/engine");
    expect(screen.getAllByText("Clear Filter").length).toBeGreaterThan(0);
  });
});
