import { describe, expect, it } from "bun:test";
import { checkAndTriggerMilestoneRelease } from "./milestone-release-engine";

describe("milestone-release-engine", () => {
  it("should report when no milestones are ready for release", async () => {
    const res = await checkAndTriggerMilestoneRelease({ dryRun: true });
    expect(res).toBeDefined();
    expect(typeof res.triggered).toBe("boolean");
    expect(typeof res.reason).toBe("string");
  });

  it("should reject release when open issues remain without force flag", async () => {
    // Milestone 34 currently has open issues
    const res = await checkAndTriggerMilestoneRelease({
      dryRun: true,
      specificMilestoneId: 34,
      force: false,
    });
    expect(res.triggered).toBe(false);
    expect(res.reason).toContain("open issue");
  });
});
