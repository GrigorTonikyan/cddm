import { describe, it, expect, vi } from "vite-plus/test";
import { render, screen, fireEvent } from "@testing-library/react";
import { CollapsibleCard } from "./collapsible-card";
import { BADGE_VARIANTS, UI_DATA_ATTRS } from "../../constants/ui-constants";

describe("CollapsibleCard Molecule (Atomic UI)", () => {
  it("renders title, badge, and body when defaultOpen is true", () => {
    render(
      <CollapsibleCard title="Card Title" badgeCount="3" badgeVariant={BADGE_VARIANTS.EMERALD}>
        <div>Inner Card Content</div>
      </CollapsibleCard>,
    );

    expect(screen.getByText("Card Title")).toBeTruthy();
    expect(screen.getByText("3")).toBeTruthy();
    expect(screen.getByText("Inner Card Content")).toBeTruthy();
    expect(document.querySelector(`[${UI_DATA_ATTRS.COLLAPSIBLE_CARD}]`)).toBeTruthy();
  });

  it("toggles body visibility on header click", () => {
    const onToggle = vi.fn();

    render(
      <CollapsibleCard title="Toggle Card" defaultOpen={false} onToggle={onToggle}>
        <div>Hidden Content</div>
      </CollapsibleCard>,
    );

    expect(screen.queryByText("Hidden Content")).toBeNull();

    // Click header to open
    fireEvent.click(screen.getByText("Toggle Card"));
    expect(screen.getByText("Hidden Content")).toBeTruthy();
    expect(onToggle).toHaveBeenCalledWith(true);

    // Click header to close
    fireEvent.click(screen.getByText("Toggle Card"));
    expect(screen.queryByText("Hidden Content")).toBeNull();
    expect(onToggle).toHaveBeenCalledWith(false);
  });
});
