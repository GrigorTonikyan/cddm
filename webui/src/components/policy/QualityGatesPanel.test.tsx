import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vite-plus/test";
import type { PolicyConfig } from "../../types/cddm-types";
import { QualityGatesPanel } from "./QualityGatesPanel";

describe("QualityGatesPanel", () => {
  it("renders unconfigured guidance when neither [scan] nor [dead_code] is present", () => {
    render(<QualityGatesPanel policyConfig={null} />);
    expect(screen.getByText(/No \[scan\] or \[dead_code\] gates configured/i)).toBeDefined();
  });

  it("renders [scan] and [dead_code] settings when configured", () => {
    const cfg: PolicyConfig = {
      boundaries: [],
      zero_duplication: [],
      limits: [],
      scan: {
        max_duplication_pct: 5,
        min_dry_score: 90,
        detect_type4: true,
      },
      dead_code: {
        fail_on_findings: true,
        max_items: 0,
        static_only: true,
      },
    };
    render(<QualityGatesPanel policyConfig={cfg} />);
    expect(screen.getByText("5%")).toBeDefined();
    expect(screen.getByText("90")).toBeDefined();
    expect(screen.getByText("enabled")).toBeDefined();
    expect(screen.getByText("[dead_code] Gate")).toBeDefined();
    expect(screen.getByText(/Edit|Modify \[scan\]/i)).toBeDefined();
  });
});
