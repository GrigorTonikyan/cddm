import { describe, it, expect, vi } from "vite-plus/test";
import { render, screen, fireEvent } from "@testing-library/react";
import { IconButton } from "../atoms/icon-button/icon-button";
import { Sparkles } from "lucide-react";
import { BUTTON_VARIANTS, UI_DATA_ATTRS } from "../constants/ui-constants";

describe("IconButton Atom (Atomic UI)", () => {
  it("renders with accessible title and handles click", () => {
    const onClick = vi.fn();
    render(
      <IconButton
        icon={<Sparkles data-testid="icon" />}
        title="Trigger Action"
        onClick={onClick}
      />,
    );

    const btn = screen.getByTitle("Trigger Action");
    expect(btn).toBeTruthy();
    expect(btn.getAttribute(UI_DATA_ATTRS.ICON_BUTTON)).toBe("true");

    fireEvent.click(btn);
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it("renders danger variant when specified", () => {
    render(<IconButton icon={<Sparkles />} title="Delete" variant={BUTTON_VARIANTS.DANGER} />);
    expect(screen.getByTitle("Delete")).toBeTruthy();
  });
});
