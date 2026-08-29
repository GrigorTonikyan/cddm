import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { ModalFooter } from "./ModalFooter";

describe("ModalFooter", () => {
  it("renders info text and close button", () => {
    const handleClose = vi.fn();
    render(<ModalFooter infoText="Test Information" onClose={handleClose} />);

    expect(screen.getByText("Test Information")).toBeTruthy();
    const btn = screen.getByRole("button", { name: /close/i });
    expect(btn).toBeTruthy();

    fireEvent.click(btn);
    expect(handleClose).toHaveBeenCalledTimes(1);
  });

  it("renders custom action buttons", () => {
    const handleClose = vi.fn();
    render(
      <ModalFooter
        infoText="Test Info"
        onClose={handleClose}
        actionButton={<button type="button">Custom Action</button>}
      />,
    );

    expect(screen.getByText("Custom Action")).toBeTruthy();
  });
});
