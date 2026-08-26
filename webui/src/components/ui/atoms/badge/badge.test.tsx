import { describe, it, expect } from "vite-plus/test";
import { render, screen } from "@testing-library/react";
import { Badge } from "./badge";
import { BADGE_VARIANTS, BADGE_SIZES, UI_DATA_ATTRS } from "../../constants/ui-constants";

describe("Badge Atom (Atomic UI)", () => {
  it("renders with default variant and size", () => {
    render(<Badge>Default Badge</Badge>);
    const badge = screen.getByText("Default Badge");
    expect(badge).toBeTruthy();
    expect(badge.getAttribute(UI_DATA_ATTRS.BADGE)).toBe("true");
  });

  it("renders with custom variant and size", () => {
    render(
      <Badge variant={BADGE_VARIANTS.EMERALD} size={BADGE_SIZES.SM}>
        Saved
      </Badge>,
    );
    expect(screen.getByText("Saved")).toBeTruthy();
  });
});
