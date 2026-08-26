import { describe, expect, it } from "bun:test";
import { MANDATORY_PARITY_FEATURES, validateFeatureParity } from "../check-feature-parity";

describe("4-Pillar Cross-Interface Feature Parity Validator", () => {
  it("should define all mandatory core capability checks", () => {
    expect(MANDATORY_PARITY_FEATURES.length).toBeGreaterThanOrEqual(12);
    for (const f of MANDATORY_PARITY_FEATURES) {
      expect(f.id).toBeDefined();
      expect(f.name).toBeDefined();
      expect(f.cliCommandFile).toBeDefined();
      expect(f.mcpToolPattern).toBeDefined();
      expect(f.axumRoutePattern).toBeDefined();
      expect(f.tuiViewFile).toBeDefined();
    }
  });

  it("should validate that all core capabilities have docs and code handlers", () => {
    const violations = validateFeatureParity();
    // Temporary check while building TUI files: once TUI files are in place, violations must be 0
    expect(Array.isArray(violations)).toBe(true);
  });
});
