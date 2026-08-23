import { describe, it, expect } from "vite-plus/test";
import { render, screen } from "@testing-library/react";
import { CodeBlock } from "../molecules/CodeBlock";

describe("CodeBlock Component", () => {
  it("should render code with whitespace preservation and filename", () => {
    render(
      <CodeBlock
        filename="utils.ts"
        lineRange="L10-L20"
        code="const total = calculateScore(a, b);"
        variant="added"
      />,
    );

    expect(screen.getByText("utils.ts")).toBeDefined();
    expect(screen.getByText("L10-L20")).toBeDefined();
    expect(screen.getByText("const total = calculateScore(a, b);")).toBeDefined();
  });

  it("should render empty placeholder when code is empty", () => {
    render(<CodeBlock code="" emptyPlaceholder="<no variance>" />);

    expect(screen.getByText("<no variance>")).toBeDefined();
  });
});
