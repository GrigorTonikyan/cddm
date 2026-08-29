import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { LanguageAnalyticsModal } from "./LanguageAnalyticsModal";
import { expectDefinedTexts, renderWithWin2x } from "../test/test-helpers";

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

    expectDefinedTexts([
      "Language & Architectural Composition",
      "2 Languages",
      "2 ecosystems",
      "15 files",
      "7,500 tokens",
      "TypeScript",
      "Rust",
      "66.7%",
      "33.3%",
    ]);

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });
});
