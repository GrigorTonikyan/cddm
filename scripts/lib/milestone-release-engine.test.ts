import { describe, expect, it } from "bun:test";
import { giteaFetch } from "./gitea-client";
import { checkAndTriggerMilestoneRelease } from "./milestone-release-engine";
import type { MilestoneMeta } from "./milestone-sync-engine";

describe("milestone-release-engine", () => {
  it("should report when no milestones are ready for release", async () => {
    const res = await checkAndTriggerMilestoneRelease({ dryRun: true });
    expect(res).toBeDefined();
    expect(typeof res.triggered).toBe("boolean");
    expect(typeof res.reason).toBe("string");
  });

  it("should reject release when open issues remain without force flag", async () => {
    const openRes = await giteaFetch<MilestoneMeta[]>("/repos/gt-dev/cddm/milestones?state=open");
    const openMilestoneWithIssues = openRes.data?.find((m) => m.open_issues > 0);
    const targetId = openMilestoneWithIssues?.id ?? 37;

    const res = await checkAndTriggerMilestoneRelease({
      dryRun: true,
      specificMilestoneId: targetId,
      force: false,
    });
    expect(res.triggered).toBe(false);
    expect(res.reason).toContain("open issue");
  });
});
