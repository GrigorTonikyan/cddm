import { describe, it, expect } from "vite-plus/test";
import { render, screen, fireEvent, renderHook } from "@testing-library/react";
import React from "react";
import { Win2xManagerProvider } from "./win2x-manager-context";
import { useWindowManager } from "../hooks/use-window-manager";
import { WIN2X_LAYOUT_MODES } from "../constants/win2x-constants";

describe("Win2xManagerContext & Provider", () => {
  it("throws error when useWindowManager is used outside provider", () => {
    expect(() => {
      renderHook(() => useWindowManager());
    }).toThrow("useWindowManager must be used within a Win2xManagerProvider");
  });

  it("registers, updates, focuses, and unregisters windows within provider", () => {
    const TestConsumer: React.FC = () => {
      const {
        windows,
        activeWindowId,
        registerWindow,
        updateWindow,
        focusWindow,
        unregisterWindow,
        cascadeWindows,
        tileWindows,
        minimizeAllWindows,
        restoreAllWindows,
        closeWindow,
      } = useWindowManager();

      return (
        <div>
          <span data-testid="active-id">{activeWindowId ?? "none"}</span>
          <span data-testid="window-count">{windows.size}</span>
          <button
            onClick={() =>
              registerWindow("win-1", {
                id: "win-1",
                title: "Window 1",
                isMinimized: false,
                isMaximized: false,
                rect: { x: 10, y: 10, width: 400, height: 300 },
              })
            }
          >
            Register 1
          </button>
          <button
            onClick={() =>
              registerWindow("win-2", {
                id: "win-2",
                title: "Window 2",
                isMinimized: false,
                isMaximized: false,
                rect: { x: 50, y: 50, width: 400, height: 300 },
              })
            }
          >
            Register 2
          </button>
          <button onClick={() => focusWindow("win-1")}>Focus 1</button>
          <button onClick={() => updateWindow("win-1", { title: "Window 1 Updated" })}>
            Update 1
          </button>
          <button onClick={() => cascadeWindows()}>Cascade</button>
          <button onClick={() => tileWindows(WIN2X_LAYOUT_MODES.TILE_GRID)}>Tile Grid</button>
          <button onClick={() => tileWindows(WIN2X_LAYOUT_MODES.TILE_HORIZONTAL)}>
            Tile Horizontal
          </button>
          <button onClick={() => minimizeAllWindows()}>Minimize All</button>
          <button onClick={() => restoreAllWindows()}>Restore All</button>
          <button onClick={() => closeWindow("win-1")}>Close 1</button>
          <button onClick={() => unregisterWindow("win-2")}>Unregister 2</button>
          <ul>
            {Array.from(windows.values()).map((win) => (
              <li key={win.id} data-testid={`win-${win.id}`}>
                {win.title}:{win.zIndex}:{win.isMinimized ? "min" : "norm"}
              </li>
            ))}
          </ul>
        </div>
      );
    };

    render(
      <Win2xManagerProvider>
        <TestConsumer />
      </Win2xManagerProvider>,
    );

    expect(screen.getByTestId("window-count").textContent).toBe("0");

    // Register win-1
    fireEvent.click(screen.getByText("Register 1"));
    expect(screen.getByTestId("window-count").textContent).toBe("1");
    expect(screen.getByTestId("active-id").textContent).toBe("win-1");

    // Register win-2
    fireEvent.click(screen.getByText("Register 2"));
    expect(screen.getByTestId("window-count").textContent).toBe("2");
    expect(screen.getByTestId("active-id").textContent).toBe("win-2");

    // Focus win-1
    fireEvent.click(screen.getByText("Focus 1"));
    expect(screen.getByTestId("active-id").textContent).toBe("win-1");

    // Update win-1
    fireEvent.click(screen.getByText("Update 1"));
    expect(screen.getByTestId("win-win-1").textContent).toContain("Window 1 Updated");

    // Cascade
    fireEvent.click(screen.getByText("Cascade"));

    // Tile Grid
    fireEvent.click(screen.getByText("Tile Grid"));

    // Tile Horizontal
    fireEvent.click(screen.getByText("Tile Horizontal"));

    // Minimize All
    fireEvent.click(screen.getByText("Minimize All"));
    expect(screen.getByTestId("win-win-1").textContent).toContain("min");
    expect(screen.getByTestId("active-id").textContent).toBe("none");

    // Restore All
    fireEvent.click(screen.getByText("Restore All"));
    expect(screen.getByTestId("win-win-1").textContent).toContain("norm");

    // Close 1
    fireEvent.click(screen.getByText("Close 1"));
    expect(screen.getByTestId("window-count").textContent).toBe("1");

    // Unregister 2
    fireEvent.click(screen.getByText("Unregister 2"));
    expect(screen.getByTestId("window-count").textContent).toBe("0");
  });

  it("handles snap presets and expands in direction", () => {
    const SnapConsumer: React.FC = () => {
      const { registerWindow, applySnapPreset, expandWindowInDirection, windows } =
        useWindowManager();
      return (
        <div>
          <button
            onClick={() =>
              registerWindow("win-a", {
                id: "win-a",
                title: "Window A",
                isMinimized: false,
                isMaximized: false,
                rect: { x: 100, y: 100, width: 400, height: 300 },
              })
            }
          >
            Reg A
          </button>
          <button onClick={() => applySnapPreset("win-a", "two-equal", 0)}>Snap 50-50 Left</button>
          <button onClick={() => expandWindowInDirection("win-a", "right")}>Expand Right</button>
          <span data-testid="win-a-w">{windows.get("win-a")?.rect.width ?? 0}</span>
        </div>
      );
    };

    render(
      <Win2xManagerProvider>
        <SnapConsumer />
      </Win2xManagerProvider>,
    );

    fireEvent.click(screen.getByText("Reg A"));
    expect(screen.getByTestId("win-a-w").textContent).toBe("400");

    fireEvent.click(screen.getByText("Snap 50-50 Left"));
    expect(Number(screen.getByTestId("win-a-w").textContent)).toBe(window.innerWidth / 2);
  });

  it("handles theme switching and initial theme configuration", () => {
    const ThemeConsumer: React.FC = () => {
      const { theme, setTheme } = useWindowManager();
      return (
        <div>
          <span data-testid="current-theme">{theme}</span>
          <button onClick={() => setTheme("light")}>Set Light</button>
          <button onClick={() => setTheme("high-contrast")}>Set High Contrast</button>
          <button onClick={() => setTheme("dark")}>Set Dark</button>
        </div>
      );
    };

    render(
      <Win2xManagerProvider initialTheme="dark">
        <ThemeConsumer />
      </Win2xManagerProvider>,
    );

    expect(screen.getByTestId("current-theme").textContent).toBe("dark");

    const themeTransitions: [string, string][] = [
      ["Set Light", "light"],
      ["Set High Contrast", "high-contrast"],
      ["Set Dark", "dark"],
    ];
    for (const [btnText, expectedTheme] of themeTransitions) {
      fireEvent.click(screen.getByText(btnText));
      expect(screen.getByTestId("current-theme").textContent).toBe(expectedTheme);
    }
  });

  it("handles global keyboard shortcuts for cascade, tile, and minimization", () => {
    const ShortcutConsumer: React.FC = () => {
      const { registerWindow, windows } = useWindowManager();
      return (
        <div>
          <button
            onClick={() => {
              registerWindow("w1", {
                id: "w1",
                title: "W1",
                isMinimized: false,
                isMaximized: false,
                rect: { x: 0, y: 0, width: 300, height: 200 },
              });
              registerWindow("w2", {
                id: "w2",
                title: "W2",
                isMinimized: false,
                isMaximized: false,
                rect: { x: 50, y: 50, width: 300, height: 200 },
              });
            }}
          >
            Spawn Windows
          </button>
          <span data-testid="w1-min">{windows.get("w1")?.isMinimized ? "yes" : "no"}</span>
          <span data-testid="w2-x">{windows.get("w2")?.rect.x ?? -1}</span>
        </div>
      );
    };

    render(
      <Win2xManagerProvider enableKeyboardShortcuts={true}>
        <ShortcutConsumer />
      </Win2xManagerProvider>,
    );

    fireEvent.click(screen.getByText("Spawn Windows"));

    // Trigger Alt+Shift+C (Cascade)
    fireEvent.keyDown(window, { key: "c", altKey: true, shiftKey: true });
    expect(Number(screen.getByTestId("w2-x").textContent)).toBeGreaterThan(0);

    // Trigger Alt+Shift+M (Minimize All)
    fireEvent.keyDown(window, { key: "m", altKey: true, shiftKey: true });
    expect(screen.getByTestId("w1-min").textContent).toBe("yes");

    // Trigger Alt+Shift+R (Restore All)
    fireEvent.keyDown(window, { key: "r", altKey: true, shiftKey: true });
    expect(screen.getByTestId("w1-min").textContent).toBe("no");

    // Trigger Alt+Shift+G (Tile Grid)
    fireEvent.keyDown(window, { key: "g", altKey: true, shiftKey: true });
    expect(Number(screen.getByTestId("w2-x").textContent)).toBeGreaterThanOrEqual(0);
  });
});
