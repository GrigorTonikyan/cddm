#!/usr/bin/env bun
/**
 * Milestone Visibility Report for CDDM (informational, never fails)\r\n *\r\n * Reports open issues in the authoritative Gitea SSoT that have no milestone.\r\n * Milestones are optional; issues themselves are the mandatory tracking unit.
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
    console.log(
      `[INFO] ${unassigned.length} open issue(s) have no milestone (milestones are optional):`,
    );
    for (const u of unassigned) {
      console.log(`  - #${u.number}: ${u.title}`);
    }
    console.log("Run 'bun scripts/sync-milestones.ts' to assign milestones if desired.");
  }

  console.log(
    `[PASS] Milestone report complete: ${openIssuesRes.data.length - unassigned.length} of ${openIssuesRes.data.length} open issues have milestones (not required).`,
  );
}

main().catch((err) => {
  console.error("Fatal error:", err);
  process.exit(1);
});
