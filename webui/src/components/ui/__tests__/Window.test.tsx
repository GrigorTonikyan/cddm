import { describe, it, expect, vi } from "vite-plus/test";
import { render, screen, fireEvent, act } from "@testing-library/react";
import { Window } from "../organisms/Window";

describe("Window Organism Component", () => {
  it("should render window when isOpen is true", () => {
    const onClose = vi.fn();
    render(
      <Window isOpen={true} onClose={onClose} title="Test Window" subtitle="Test Subtitle">
        <div data-testid="window-content">Window Body Content</div>
      </Window>,
    );

    expect(screen.getByText("Test Window")).toBeDefined();
    expect(screen.getByText("Test Subtitle")).toBeDefined();
    expect(screen.getByTestId("window-content")).toBeDefined();
  });

  it("should not render when isOpen is false", () => {
    const onClose = vi.fn();
    render(
      <Window isOpen={false} onClose={onClose} title="Hidden Window">
        <div>Content</div>
      </Window>,
    );

    expect(screen.queryByText("Hidden Window")).toBeNull();
  });

  it("should trigger onClose when close button is clicked", () => {
    const onClose = vi.fn();
    render(
      <Window isOpen={true} onClose={onClose} title="Closable Window">
        <div>Content</div>
      </Window>,
    );

    const closeBtn = screen.getByRole("button", { name: "Close" });
    act(() => {
      fireEvent.click(closeBtn);
    });
    expect(onClose).toHaveBeenCalled();
  });

  it("should minimize to pill dock and restore on click", () => {
    const onClose = vi.fn();
    render(
      <Window isOpen={true} onClose={onClose} title="Minimizable Window">
        <div data-testid="window-inner-content">Main Body</div>
      </Window>,
    );

    const minBtn = screen.getByRole("button", { name: "Minimize" });
    act(() => {
      fireEvent.click(minBtn);
    });

    // Should now show the minimized pill
    const pill = screen.getByText("Minimizable Window");
    expect(pill).toBeDefined();

    // Click pill to restore
    act(() => {
      fireEvent.click(pill);
    });
    expect(screen.getByTestId("window-inner-content")).toBeDefined();
  });
});
