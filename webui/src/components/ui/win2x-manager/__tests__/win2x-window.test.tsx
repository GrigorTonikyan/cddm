import { describe, it, expect, vi } from "vite-plus/test";
import { render, screen, fireEvent } from "@testing-library/react";
import { Win2xWindow } from "../components/win2x-window/win2x-window";
import { Win2xManagerProvider } from "../context/win2x-manager-context";
import { WIN2X_DATA_ATTRS } from "../constants/win2x-constants";
import { UI_DATA_ATTRS } from "../../constants/ui-constants";

describe("Win2xWindow Organism", () => {
  it("renders when isOpen is true and hides when false", () => {
    const onClose = vi.fn();

    const { rerender } = render(
      <Win2xManagerProvider>
        <Win2xWindow id="test-win" isOpen={true} onClose={onClose} title="Test Window">
          <div>Window Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Test Window")).toBeTruthy();
    expect(screen.getByText("Window Content")).toBeTruthy();
    expect(document.querySelector(`[${WIN2X_DATA_ATTRS.WINDOW}]`)).toBeTruthy();

    rerender(
      <Win2xManagerProvider>
        <Win2xWindow id="test-win" isOpen={false} onClose={onClose} title="Test Window">
          <div>Window Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    expect(screen.queryByText("Test Window")).toBeNull();
  });

  it("calls onClose when Escape key is pressed and window is active", () => {
    const onClose = vi.fn();

    render(
      <Win2xManagerProvider>
        <Win2xWindow id="test-win-esc" isOpen={true} onClose={onClose} title="Test Window">
          <div>Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    fireEvent.keyDown(window, { key: "Escape" });
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("minimizes window to dock pill when clicking outside modal backdrop", () => {
    const onClose = vi.fn();

    render(
      <Win2xManagerProvider>
        <Win2xWindow
          id="test-win-minimize"
          isOpen={true}
          isModal={true}
          onClose={onClose}
          title="Active Window"
        >
          <div>Window Body Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    expect(document.querySelector(`[${WIN2X_DATA_ATTRS.WINDOW}]`)).toBeTruthy();

    // Click outside on the backdrop
    const backdrop = document.querySelector(`[${UI_DATA_ATTRS.BACKDROP}]`);
    expect(backdrop).toBeTruthy();
    fireEvent.click(backdrop!);

    // Should NOT close, but instead minimize to dock pill
    expect(onClose).not.toHaveBeenCalled();
    expect(document.querySelector(`[${WIN2X_DATA_ATTRS.WINDOW}]`)).toBeNull(); // window unmounts when minimized
    expect(document.querySelector(`[${WIN2X_DATA_ATTRS.MINIMIZED_PILL}]`)).toBeTruthy();
    expect(screen.getByText("Active Window")).toBeTruthy(); // inside the pill
  });

  it("restores window from minimized dock pill via click", () => {
    const onClose = vi.fn();

    render(
      <Win2xManagerProvider>
        <Win2xWindow
          id="test-win-restore"
          isOpen={true}
          isModal={true}
          onClose={onClose}
          title="Active Window"
        >
          <div>Window Body Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    // Click backdrop to minimize first
    const backdrop = document.querySelector(`[${UI_DATA_ATTRS.BACKDROP}]`);
    fireEvent.click(backdrop!);

    const pill = document.querySelector(`[${WIN2X_DATA_ATTRS.MINIMIZED_PILL}]`);
    expect(pill).toBeTruthy();

    // Click pill to restore
    fireEvent.click(pill!);
    expect(document.querySelector(`[${WIN2X_DATA_ATTRS.WINDOW}]`)).toBeTruthy();
    expect(screen.getByText("Window Body Content")).toBeTruthy();
  });

  it("calls onClose when closeOnOutsideClick is true for modal", () => {
    const onClose = vi.fn();

    render(
      <Win2xManagerProvider>
        <Win2xWindow
          id="test-win-close-outside"
          isOpen={true}
          isModal={true}
          onClose={onClose}
          minimizeOnOutsideClick={false}
          closeOnOutsideClick={true}
          title="Close-on-Outside Window"
        >
          <div>Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    const backdrop = document.querySelector(`[${UI_DATA_ATTRS.BACKDROP}]`);
    expect(backdrop).toBeTruthy();
    fireEvent.click(backdrop!);

    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("invokes custom onOutsideClick callback when provided for modal", () => {
    const customOutside = vi.fn();
    const onClose = vi.fn();

    render(
      <Win2xManagerProvider>
        <Win2xWindow
          id="test-win-custom-outside"
          isOpen={true}
          isModal={true}
          onClose={onClose}
          onOutsideClick={customOutside}
          title="Custom Outside Window"
        >
          <div>Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    const backdrop = document.querySelector(`[${UI_DATA_ATTRS.BACKDROP}]`);
    expect(backdrop).toBeTruthy();
    fireEvent.click(backdrop!);

    expect(customOutside).toHaveBeenCalledTimes(1);
    expect(onClose).not.toHaveBeenCalled();
  });

  it("renders optional footer when provided", () => {
    render(
      <Win2xManagerProvider>
        <Win2xWindow
          id="test-win-footer"
          isOpen={true}
          onClose={vi.fn()}
          title="Window with Footer"
          footer={<span>Custom Footer Area</span>}
        >
          <div>Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Custom Footer Area")).toBeTruthy();
  });

  it("supports simultaneous non-modal windows and brings clicked window to foreground", () => {
    render(
      <Win2xManagerProvider>
        <Win2xWindow id="win-1" isOpen={true} onClose={vi.fn()} title="First Non-Modal Window">
          <div>Content 1</div>
        </Win2xWindow>
        <Win2xWindow id="win-2" isOpen={true} onClose={vi.fn()} title="Second Non-Modal Window">
          <div>Content 2</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    const win1 = screen
      .getByText("First Non-Modal Window")
      .closest(`[${WIN2X_DATA_ATTRS.WINDOW}]`) as HTMLElement;
    const win2 = screen
      .getByText("Second Non-Modal Window")
      .closest(`[${WIN2X_DATA_ATTRS.WINDOW}]`) as HTMLElement;

    expect(win1).toBeTruthy();
    expect(win2).toBeTruthy();

    // win-2 was registered second, so it starts active
    expect(win2.getAttribute(WIN2X_DATA_ATTRS.ACTIVE)).toBe("true");

    // Click win-1 -> should elevate win-1 to active
    fireEvent.pointerDown(win1);
    expect(win1.getAttribute(WIN2X_DATA_ATTRS.ACTIVE)).toBe("true");
  });

  it("opens titlebar context menu on right click and supports cascade/tile actions", () => {
    render(
      <Win2xManagerProvider>
        <Win2xWindow id="win-ctx" isOpen={true} onClose={vi.fn()} title="Context Window">
          <div>Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    const titlebar = document.querySelector(`[${WIN2X_DATA_ATTRS.TITLEBAR}]`);
    expect(titlebar).toBeTruthy();

    // Right click titlebar
    fireEvent.contextMenu(titlebar!);

    // Context menu should appear
    expect(document.querySelector(`[${WIN2X_DATA_ATTRS.CONTEXT_MENU}]`)).toBeTruthy();
    expect(screen.getByText("Cascade All")).toBeTruthy();
    expect(screen.getByText("Tile All")).toBeTruthy();
  });

  it("renders tab bar inside window when tabs prop is provided", () => {
    const handleSelect = vi.fn();
    const handleAdd = vi.fn();

    render(
      <Win2xManagerProvider>
        <Win2xWindow
          id="win-tabs"
          isOpen={true}
          onClose={vi.fn()}
          title="Tabbed Window"
          tabs={[
            { id: "t1", title: "Tab 1", badgeCount: 2 },
            { id: "t2", title: "Tab 2" },
          ]}
          activeTabId="t1"
          onTabSelect={handleSelect}
          onTabAdd={handleAdd}
        >
          <div>Tab Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Tab 1")).toBeTruthy();
    expect(screen.getByText("Tab 2")).toBeTruthy();
    expect(document.querySelector(`[${WIN2X_DATA_ATTRS.TAB_BAR}]`)).toBeTruthy();

    fireEvent.click(screen.getByText("Tab 2"));
    expect(handleSelect).toHaveBeenCalledWith("t2");

    const addBtn = screen.getByLabelText("Add new tab");
    fireEvent.click(addBtn);
    expect(handleAdd).toHaveBeenCalledTimes(1);
  });

  it("applies theme attribute to the window frame", () => {
    const { rerender } = render(
      <Win2xManagerProvider initialTheme="dark">
        <Win2xWindow id="win-theme" isOpen={true} onClose={vi.fn()} title="Themed Window">
          <div>Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    const winElem = document.querySelector(`[${WIN2X_DATA_ATTRS.WINDOW}]`);
    expect(winElem?.getAttribute(WIN2X_DATA_ATTRS.THEME)).toBe("dark");

    rerender(
      <Win2xManagerProvider initialTheme="dark">
        <Win2xWindow
          id="win-theme"
          isOpen={true}
          onClose={vi.fn()}
          title="Themed Window"
          theme="light"
        >
          <div>Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    expect(winElem?.getAttribute(WIN2X_DATA_ATTRS.THEME)).toBe("light");
  });

  it("elevates focus to window on pointer enter (hover focus)", () => {
    render(
      <Win2xManagerProvider>
        <Win2xWindow id="win-hover-1" isOpen={true} onClose={vi.fn()} title="Hover 1">
          <div>Content 1</div>
        </Win2xWindow>
        <Win2xWindow id="win-hover-2" isOpen={true} onClose={vi.fn()} title="Hover 2">
          <div>Content 2</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    const win1 = screen.getByText("Hover 1").closest(`[${WIN2X_DATA_ATTRS.WINDOW}]`) as HTMLElement;
    const win2 = screen.getByText("Hover 2").closest(`[${WIN2X_DATA_ATTRS.WINDOW}]`) as HTMLElement;

    // win2 was registered last, so active initially
    expect(win2.getAttribute(WIN2X_DATA_ATTRS.ACTIVE)).toBe("true");

    // Hover over win1
    fireEvent.pointerEnter(win1);
    expect(win1.getAttribute(WIN2X_DATA_ATTRS.ACTIVE)).toBe("true");
  });

  it("stops wheel event propagation to prevent background scroll chaining", () => {
    render(
      <Win2xManagerProvider>
        <Win2xWindow id="win-wheel" isOpen={true} onClose={vi.fn()} title="Wheel Window">
          <div>Scroll Content</div>
        </Win2xWindow>
      </Win2xManagerProvider>,
    );

    const winElem = screen
      .getByText("Wheel Window")
      .closest(`[${WIN2X_DATA_ATTRS.WINDOW}]`) as HTMLElement;
    const wheelEvent = new WheelEvent("wheel", { bubbles: true, cancelable: true, deltaY: 100 });
    const stopPropagationSpy = vi.spyOn(wheelEvent, "stopPropagation");

    winElem.dispatchEvent(wheelEvent);
    expect(stopPropagationSpy).toHaveBeenCalled();
  });
});
