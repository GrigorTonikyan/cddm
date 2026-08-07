import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { ClonePairCard } from "../ClonePairCard";
import { ClonePair } from "../../types/cddm-types";

describe("ClonePairCard Component", () => {
  const mockPair: ClonePair = {
    file_a: "src/a.ts",
    start_line_a: 10,
    end_line_a: 20,
    file_b: "src/b.ts",
    start_line_b: 15,
    end_line_b: 25,
    token_count: 55,
    similarity: 0.95,
    fragment_hash: "hash123",
    clone_type: "Exact",
    author_a: "Grigor",
    author_b: "Grigor",
  };

  it("should render collapsed summary card", () => {
    render(<ClonePairCard pair={mockPair} index={1} />);
    expect(screen.getByText("#1")).toBeDefined();
    expect(screen.getByText("src/a.ts")).toBeDefined();
    expect(screen.getByText("src/b.ts")).toBeDefined();
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
