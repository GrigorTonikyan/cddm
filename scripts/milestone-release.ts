#!/usr/bin/env bun
/**
 * Automagic Milestone Release Orchestrator
 *
 * Checks for completed milestones and automatically performs semantic versioning,
 * manifest synchronization, tagging, Gitea release creation, and CI build triggers.
 */

import { checkAndTriggerMilestoneRelease } from "./lib/milestone-release-engine";

async function main() {
  const args = process.argv.slice(2);
  const dryRun = args.includes("--dry-run");
  const force = args.includes("--force");
  const checkOnly = args.includes("--check");

  let specificMilestoneId: number | undefined;
  const mIdx = args.indexOf("--milestone");
  const rawArg = mIdx !== -1 ? args[mIdx + 1] : undefined;
  if (rawArg) {
    specificMilestoneId = parseInt(rawArg, 10);
  }

  console.log("--> Checking Gitea milestones for automagic release eligibility...");

  const result = await checkAndTriggerMilestoneRelease({
    dryRun: dryRun || checkOnly,
    specificMilestoneId,
    force,
  });

  if (checkOnly) {
    if (result.triggered) {
      console.log(
        `[READY] Milestone ${result.milestoneTitle} (${result.tag}) is 100% complete and eligible for release.`,
      );
      process.exit(0);
    } else {
      console.log(`[NOT READY] ${result.reason}`);
      process.exit(0);
    }
  }

  if (result.triggered) {
    console.log(`[SUCCESS] ${result.reason}`);
  } else {
    console.log(`[INFO] ${result.reason}`);
  }
}

main().catch((err) => {
  console.error("Fatal error:", err);
  process.exit(1);
});
