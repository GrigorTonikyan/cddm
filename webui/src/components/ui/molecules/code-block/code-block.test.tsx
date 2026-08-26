import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { render, screen, fireEvent } from "@testing-library/react";
import { CodeBlock } from "./code-block";
import { CODE_BLOCK_VARIANTS, UI_ARIA_LABELS, UI_DATA_ATTRS } from "../../constants/ui-constants";

describe("CodeBlock Molecule (Atomic UI)", () => {
  let writeTextMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    writeTextMock = vi.fn().mockResolvedValue(undefined);
    Object.assign(navigator, {
      clipboard: {
        writeText: writeTextMock,
      },
    });
  });

  it("renders filename, line range, and code snippet", () => {
    render(
      <CodeBlock
        filename="detector.rs"
        lineRange="L50-L75"
        code="pub fn detect_clones() {}"
        variant={CODE_BLOCK_VARIANTS.ADDED}
      />,
    );

    expect(screen.getByText("detector.rs")).toBeTruthy();
    expect(screen.getByText("L50-L75")).toBeTruthy();
    expect(screen.getByText("pub fn detect_clones() {}")).toBeTruthy();
    expect(document.querySelector(`[${UI_DATA_ATTRS.CODE_BLOCK}]`)).toBeTruthy();
  });

  it("copies code snippet to clipboard on click", () => {
    render(<CodeBlock filename="test.ts" code="const x = 42;" showCopy={true} />);

    const copyBtn = screen.getByTitle(UI_ARIA_LABELS.COPY_SNIPPET);
    fireEvent.click(copyBtn);
    expect(writeTextMock).toHaveBeenCalledWith("const x = 42;");
  });

  it("renders empty placeholder when code is empty", () => {
    render(<CodeBlock code="" emptyPlaceholder="<no differences>" />);
    expect(screen.getByText("<no differences>")).toBeTruthy();
  });
});
