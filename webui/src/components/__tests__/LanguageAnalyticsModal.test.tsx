import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { LanguageAnalyticsModal } from "../LanguageAnalyticsModal";
import { Win2xManagerProvider } from "../ui/win2x-manager/context/win2x-manager-context";

describe("LanguageAnalyticsModal Component", () => {
  const mockLanguages = [
    { language: "TypeScript", files: 10, tokens: 5000, clones: 4 },
    { language: "Rust", files: 5, tokens: 2500, clones: 1 },
  ];

  it("should return null when not open", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <LanguageAnalyticsModal
          isOpen={false}
          onClose={() => {}}
          languages={mockLanguages}
          totalTokens={7500}
          totalFiles={15}
        />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render multi-language stats and table when open", () => {
    const onClose = vi.fn();

    render(
      <Win2xManagerProvider>
        <LanguageAnalyticsModal
          isOpen={true}
          onClose={onClose}
          languages={mockLanguages}
          totalTokens={7500}
          totalFiles={15}
        />
      </Win2xManagerProvider>,
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
