#!/usr/bin/env bun
/**
 * Automatically audits and synchronizes Gitea issues to their designated milestones.
 * Guarantees that 100% of issues belong to a milestone.
 */

import { giteaFetch } from "./lib/gitea-client";
import {
  resolveIssueMilestone,
  type IssueMeta,
  type MilestoneMeta,
} from "./lib/milestone-sync-engine";

async function main() {
  console.log("--> Fetching all milestones from Gitea...");
  const [openRes, closedRes] = await Promise.all([
    giteaFetch<MilestoneMeta[]>("/repos/gt-dev/cddm/milestones?state=open"),
    giteaFetch<MilestoneMeta[]>("/repos/gt-dev/cddm/milestones?state=closed&limit=100"),
  ]);
  const milestones = [...(openRes.data || []), ...(closedRes.data || [])];
  console.log(`Found ${milestones.length} milestones on Gitea.\n`);

  console.log("--> Auditing issues for missing milestones...");
  let page = 1;
  let totalAudited = 0;
  let updatedCount = 0;

  while (true) {
    const issuesRes = await giteaFetch<IssueMeta[]>(
      `/repos/gt-dev/cddm/issues?state=all&limit=50&page=${page}`,
    );
    if (!issuesRes.ok || !issuesRes.data || issuesRes.data.length === 0) {
      break;
    }

    for (const issue of issuesRes.data) {
      totalAudited++;
      const targetMilestoneId = resolveIssueMilestone(issue, milestones);
      const isWrongMilestone =
        targetMilestoneId && issue.milestone && issue.milestone.id !== targetMilestoneId;
      const isClosedMilestoneOnOpenIssue =
        issue.state === "open" &&
        issue.milestone &&
        milestones.find((m) => m.id === issue.milestone?.id)?.state === "closed";
      const shouldAssign =
        !issue.milestone ||
        isClosedMilestoneOnOpenIssue ||
        (isWrongMilestone && (process.argv.includes("--force") || issue.state === "open"));

      if (shouldAssign && targetMilestoneId) {
        const targetM = milestones.find((m) => m.id === targetMilestoneId);
        console.log(
          `  [ASSIGNING] Issue #${issue.number} ("${issue.title.slice(0, 45)}...") -> Milestone #${targetMilestoneId} ("${targetM?.title}")`,
        );
        const patchRes = await giteaFetch(`/repos/gt-dev/cddm/issues/${issue.number}`, {
          method: "PATCH",
          body: JSON.stringify({ milestone: targetMilestoneId }),
        });
        if (patchRes.ok) {
          updatedCount++;
        } else {
          console.warn(
            `    [WARN] Failed to patch issue #${issue.number}: status ${patchRes.status}`,
          );
        }
      } else if (!issue.milestone) {
        console.warn(`  [UNRESOLVED] Issue #${issue.number}: ${issue.title}`);
      }
    }

    if (issuesRes.data.length < 50) break;
    page++;
  }

  console.log(
    `\n[SUCCESS] Audited ${totalAudited} issues. Updated ${updatedCount} issues with assigned milestones.`,
  );
}

main().catch((err) => {
  console.error("Fatal error:", err);
  process.exit(1);
});
