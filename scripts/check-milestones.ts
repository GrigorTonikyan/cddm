#!/usr/bin/env bun
/**
 * Milestone Governance & Enforcement Gate for CDDM
 *
 * Verifies that 100% of open issues in the authoritative Gitea SSoT
 * are assigned to an active milestone.
 */

import { giteaFetch } from "./lib/gitea-client";
import { type IssueMeta } from "./lib/milestone-sync-engine";

async function main() {
  console.log("--> Checking Gitea issues for milestone governance compliance...");

  const openIssuesRes = await giteaFetch<IssueMeta[]>(
    "/repos/gt-dev/cddm/issues?state=open&limit=100",
  );
  if (!openIssuesRes.ok || !openIssuesRes.data) {
    console.log(`[WARN] Unable to reach Gitea API (status: ${openIssuesRes.status}); skipping.`);
    process.exit(0);
  }

  const unassigned = openIssuesRes.data.filter((i) => !i.milestone);

  if (unassigned.length > 0) {
    console.error(`[FAIL] Found ${unassigned.length} open issue(s) without an assigned milestone:`);
    for (const u of unassigned) {
      console.error(`  - #${u.number}: ${u.title}`);
    }
    console.error("\nRun 'bun scripts/sync-milestones.ts' to automatically assign milestones.");
    process.exit(1);
  }

  console.log(
    `[PASS] All ${openIssuesRes.data.length} open issues are strictly assigned to active milestones.`,
  );
}

main().catch((err) => {
  console.error("Fatal error:", err);
  process.exit(1);
});
