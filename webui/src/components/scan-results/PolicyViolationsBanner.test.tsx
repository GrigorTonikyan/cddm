import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, beforeEach } from "vite-plus/test";
import { PolicyViolationsBanner } from "./PolicyViolationsBanner";
import { useCDDMStore } from "../../store/cddm-store";
import { resetTestStore } from "../../test/test-helpers";

describe("PolicyViolationsBanner Component", () => {
  beforeEach(resetTestStore);

  it("should render nothing when violations are empty", () => {
    const { container } = render(<PolicyViolationsBanner violations={[]} />);
    expect(container.firstChild).toBeNull();
  });

  it("should render max_file_lines violations with actual vs allowed line counts", () => {
    render(
      <PolicyViolationsBanner
        violations={[
          {
            rule_name: "modularity-max-file-lines",
            rule_type: "line_budget",
            severity: "error",
            message: "Line budget violated: 612 lines > 500",
            file_a: "crates/cddm-core/src/big.rs",
            start_line_a: 1,
            end_line_a: 612,
            token_count: 0,
            line_count: 612,
            max_lines: 500,
          },
        ]}
      />,
    );

    expect(screen.getByText("Architectural Policy Violations (1)")).toBeDefined();
    expect(screen.getByText("1 Error(s)")).toBeDefined();
    expect(screen.getByText("modularity-max-file-lines")).toBeDefined();
    expect(screen.getByText("612 / 500 lines")).toBeDefined();
    expect(screen.getByText("crates/cddm-core/src/big.rs:1-612")).toBeDefined();

    fireEvent.click(screen.getByText("Open Policy Studio"));
    expect(useCDDMStore.getState().isPolicyRulesModalOpen).toBe(true);
  });
});
