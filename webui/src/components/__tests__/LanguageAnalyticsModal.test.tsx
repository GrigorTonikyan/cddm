import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { LanguageAnalyticsModal } from "../LanguageAnalyticsModal";
import { renderWithWin2x } from "./test-helpers";

describe("LanguageAnalyticsModal Component", () => {
  const mockLanguages = [
    { language: "TypeScript", files: 10, tokens: 5000, clones: 4 },
    { language: "Rust", files: 5, tokens: 2500, clones: 1 },
  ];

  it("should return null when not open", () => {
    const { container } = renderWithWin2x(
      <LanguageAnalyticsModal
        isOpen={false}
        onClose={() => {}}
        languages={mockLanguages}
        totalTokens={7500}
        totalFiles={15}
      />,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render multi-language stats and table when open", () => {
    const onClose = vi.fn();

    renderWithWin2x(
      <LanguageAnalyticsModal
        isOpen={true}
        onClose={onClose}
        languages={mockLanguages}
        totalTokens={7500}
        totalFiles={15}
      />,
    );

    expect(screen.getByText("Language & Architectural Composition")).toBeDefined();
    expect(screen.getByText("2 Languages")).toBeDefined();
    expect(screen.getByText("2 ecosystems")).toBeDefined();
    expect(screen.getByText("15 files")).toBeDefined();
    expect(screen.getByText("7,500 tokens")).toBeDefined();
    expect(screen.getByText("TypeScript")).toBeDefined();
    expect(screen.getByText("Rust")).toBeDefined();
    expect(screen.getByText("66.7%")).toBeDefined();
    expect(screen.getByText("33.3%")).toBeDefined();

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });
});
