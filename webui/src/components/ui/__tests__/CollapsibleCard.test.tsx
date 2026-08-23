import { describe, it, expect } from "vite-plus/test";
import { render, screen, fireEvent } from "@testing-library/react";
import { CollapsibleCard } from "../molecules/CollapsibleCard";

describe("CollapsibleCard Component", () => {
  it("should render title and badge count", () => {
    render(
      <CollapsibleCard title="Section Title" badgeCount="3 items" defaultOpen={true}>
        <div>Card Inner Body</div>
      </CollapsibleCard>,
    );

    expect(screen.getByText("Section Title")).toBeDefined();
    expect(screen.getByText("3 items")).toBeDefined();
    expect(screen.getByText("Card Inner Body")).toBeDefined();
  });

  it("should toggle open and closed on header click", () => {
    render(
      <CollapsibleCard title="Toggle Section" defaultOpen={true}>
        <div>Collapsible Content</div>
      </CollapsibleCard>,
    );

    expect(screen.getByText("Collapsible Content")).toBeDefined();

    // Click header to collapse
    const header = screen.getByText("Toggle Section");
    fireEvent.click(header);
    expect(screen.queryByText("Collapsible Content")).toBeNull();

    // Click header again to expand
    fireEvent.click(header);
    expect(screen.getByText("Collapsible Content")).toBeDefined();
  });
});
