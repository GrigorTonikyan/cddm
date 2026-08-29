import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { ModalTabs } from "./ModalTabs";

describe("ModalTabs", () => {
  it("renders tab buttons and responds to clicks", () => {
    const handleChange = vi.fn();
    const tabs = [
      { id: "tab1", label: "Overview", count: 5 },
      { id: "tab2", label: "Details" },
    ];

    render(<ModalTabs tabs={tabs} activeTab="tab1" onTabChange={handleChange} />);

    expect(screen.getByText(/Overview \(5\)/)).toBeTruthy();
    const detailsBtn = screen.getByText("Details");
    expect(detailsBtn).toBeTruthy();

    fireEvent.click(detailsBtn);
    expect(handleChange).toHaveBeenCalledWith("tab2");
  });
});
