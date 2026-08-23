import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect } from "vite-plus/test";
import { ClonePairCard } from "../ClonePairCard";
import { createMockClonePair } from "./test-helpers";

describe("ClonePairCard Component", () => {
  const mockPair = createMockClonePair();

  it("should render collapsed summary card", () => {
    render(<ClonePairCard pair={mockPair} index={1} />);
    expect(screen.getByText("#1")).toBeDefined();
    expect(screen.getByText("a.ts")).toBeDefined();
    expect(screen.getByText("b.ts")).toBeDefined();
    expect(screen.getByText("55 tokens")).toBeDefined();
    expect(screen.getByText("95% match")).toBeDefined();
  });

  it("should expand split details on click", () => {
    render(<ClonePairCard pair={mockPair} index={1} />);
    const header = screen.getByText("#1").closest("div");
    fireEvent.click(header!);
    expect(screen.getByText("Fragment Hash: hash123")).toBeDefined();
    expect(screen.getByText("Author A: Grigor")).toBeDefined();
  });
});
