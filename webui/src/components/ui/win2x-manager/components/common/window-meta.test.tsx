import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vite-plus/test";
import { WindowMetaDisplay } from "./window-meta";

describe("WindowMetaDisplay", () => {
  it("renders window title and badge", () => {
    render(
      <WindowMetaDisplay
        win={{
          title: "My Test Window",
          subtitle: "Subtitle Text",
          badge: "Active",
        }}
      />,
    );

    expect(screen.getByText("My Test Window")).toBeTruthy();
    expect(screen.getByText("Subtitle Text")).toBeTruthy();
    expect(screen.getByText("Active")).toBeTruthy();
  });
});
