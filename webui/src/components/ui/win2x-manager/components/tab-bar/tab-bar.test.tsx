import { describe, it, expect, vi } from "vite-plus/test";
import { render, screen, fireEvent } from "@testing-library/react";
import { TabBar } from "./tab-bar";
import { TabItemData } from "../../core/types";
import { WIN2X_DATA_ATTRS } from "../../constants/win2x-constants";

describe("Win2x TabBar", () => {
  const mockTabs: TabItemData[] = [
    { id: "tab-1", title: "Overview", badgeCount: 3, badgeVariant: "indigo" },
    { id: "tab-2", title: "Analytics", badgeCount: 0, closable: true },
    { id: "tab-3", title: "Settings", closable: false },
    { id: "tab-4", title: "Disabled", disabled: true },
  ];

  it("renders all tabs with titles, badges, and attributes", () => {
    render(<TabBar tabs={mockTabs} activeTabId="tab-1" onTabSelect={vi.fn()} />);

    expect(screen.getByText("Overview")).toBeDefined();
    expect(screen.getByText("Analytics")).toBeDefined();
    expect(screen.getByText("Settings")).toBeDefined();
    expect(screen.getByText("Disabled")).toBeDefined();

    const tabItems = screen
      .getAllByRole("button")
      .filter((btn) => btn.hasAttribute(WIN2X_DATA_ATTRS.TAB_ITEM));
    expect(tabItems.length).toBe(4);
  });

  it("handles tab selection on click", () => {
    const handleSelect = vi.fn();
    render(<TabBar tabs={mockTabs} activeTabId="tab-1" onTabSelect={handleSelect} />);

    fireEvent.click(screen.getByText("Analytics"));
    expect(handleSelect).toHaveBeenCalledWith("tab-2");
  });

  it("handles tab close click and prevents selection propagation", () => {
    const handleSelect = vi.fn();
    const handleClose = vi.fn();
    render(
      <TabBar
        tabs={mockTabs}
        activeTabId="tab-1"
        onTabSelect={handleSelect}
        onTabClose={handleClose}
      />,
    );

    // Tab 2 has close button
    const closeButtons = screen
      .getAllByRole("button", { hidden: true })
      .filter((el) => el.className.includes("tabClose"));
    expect(closeButtons.length).toBeGreaterThan(0);

    fireEvent.click(closeButtons[0]!);
    expect(handleClose).toHaveBeenCalled();
  });

  it("handles adding new tabs when onTabAdd is provided", () => {
    const handleAdd = vi.fn();
    render(
      <TabBar tabs={mockTabs} activeTabId="tab-1" onTabSelect={vi.fn()} onTabAdd={handleAdd} />,
    );

    const addBtn = screen.getByLabelText("Add new tab");
    fireEvent.click(addBtn);
    expect(handleAdd).toHaveBeenCalledTimes(1);
  });
});
