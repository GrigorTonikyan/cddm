/**
 * Automagic Milestone Completion & Release Engine for CDDM
 */

import { giteaFetch } from "./gitea-client";
import { extractVersionFromMilestoneTitle, type MilestoneMeta } from "./milestone-sync-engine";
import { generateReleaseNotes } from "./release-notes-engine";

export interface MilestoneReleaseResult {
  triggered: boolean;
  milestoneId?: number;
  milestoneTitle?: string;
  version?: string;
  tag?: string;
  reason: string;
}

export interface MilestoneReleaseOptions {
  dryRun?: boolean;
  specificMilestoneId?: number;
  force?: boolean;
}

/**
 * Checks open Gitea milestones and automatically triggers the versioning,
 * manifest synchronization, tagging, and release publication workflow
 * whenever a milestone reaches 100% completion (0 open issues).
 */
export async function checkAndTriggerMilestoneRelease(
  options: MilestoneReleaseOptions = {},
): Promise<MilestoneReleaseResult> {
  const { dryRun = false, specificMilestoneId, force = false } = options;

  // 1. Fetch all open milestones from Gitea
  const openRes = await giteaFetch<MilestoneMeta[]>("/repos/gt-dev/cddm/milestones?state=open");
  if (!openRes.ok || !openRes.data) {
    return {
      triggered: false,
      reason: `Failed to query open milestones from Gitea (status: ${openRes.status})`,
    };
  }

  const openMilestones = openRes.data;

  // 2. Identify candidate milestone
  let candidate: MilestoneMeta | undefined;
  if (specificMilestoneId) {
    candidate = openMilestones.find((m) => m.id === specificMilestoneId);
    if (!candidate) {
      return {
        triggered: false,
        reason: `Milestone ID ${specificMilestoneId} is not an open milestone on Gitea.`,
      };
    }
  } else {
    // Find first open milestone that has 0 open issues and > 0 closed issues
    candidate = openMilestones.find(
      (m) => (m.open_issues === 0 && m.closed_issues > 0) || (force && m.open_issues === 0),
    );
  }

  if (!candidate) {
    return {
      triggered: false,
      reason: "No open milestones have reached 100% completion (all issues closed).",
    };
  }

  if (candidate.open_issues > 0 && !force) {
    return {
      triggered: false,
      milestoneId: candidate.id,
      milestoneTitle: candidate.title,
      reason: `Milestone "${candidate.title}" still has ${candidate.open_issues} open issue(s).`,
    };
  }

  // 3. Extract semantic version from milestone title
  const version = extractVersionFromMilestoneTitle(candidate.title);
  if (!version) {
    return {
      triggered: false,
      milestoneId: candidate.id,
      milestoneTitle: candidate.title,
      reason: `Could not parse semantic version from milestone title: "${candidate.title}"`,
    };
  }

  const tag = `v${version}`;

  // 4. Check if tag already exists in git
  const tagCheck = Bun.spawnSync(["git", "tag", "-l", tag]);
  const tagExists = tagCheck.stdout.toString().trim() === tag;

  if (tagExists) {
    // Tag already exists; close milestone if open
    await giteaFetch(`/repos/gt-dev/cddm/milestones/${candidate.id}`, {
      method: "PATCH",
      body: JSON.stringify({ state: "closed" }),
    });
    return {
      triggered: false,
      milestoneId: candidate.id,
      milestoneTitle: candidate.title,
      version,
      tag,
      reason: `Tag ${tag} already exists. Milestone closed.`,
    };
  }

  if (dryRun) {
    return {
      triggered: true,
      milestoneId: candidate.id,
      milestoneTitle: candidate.title,
      version,
      tag,
      reason: `[DRY RUN] Milestone "${candidate.title}" is 100% complete. Ready for release ${tag}.`,
    };
  }

  console.log(`\n\x1b[35m=== Automagic Milestone Release: ${candidate.title} (${tag}) ===\x1b[0m`);

  // 5. Run scripts/version.ts --release-as <version> --git-tag
  const versionProc = Bun.spawnSync([
    "bun",
    "scripts/version.ts",
    "--release-as",
    version,
    "--git-tag",
  ]);

  if (versionProc.exitCode !== 0) {
    return {
      triggered: false,
      milestoneId: candidate.id,
      milestoneTitle: candidate.title,
      version,
      tag,
      reason: `version.ts failed: ${versionProc.stderr.toString()}`,
    };
  }

  // 6. Push release commit and tag to origin (Gitea)
  console.log(`--> Pushing release commit to origin main...`);
  const pushMain = Bun.spawnSync(["git", "push", "origin", "main"]);
  if (pushMain.exitCode !== 0) {
    console.warn(`    [WARN] Push main failed: ${pushMain.stderr.toString()}`);
  }

  console.log(`--> Pushing release tag ${tag} to origin...`);
  const pushTag = Bun.spawnSync(["git", "push", "origin", tag]);
  if (pushTag.exitCode !== 0) {
    console.warn(`    [WARN] Push tag failed: ${pushTag.stderr.toString()}`);
  }

  // 7. Close milestone on Gitea
  console.log(`--> Closing milestone #${candidate.id} on Gitea...`);
  await giteaFetch(`/repos/gt-dev/cddm/milestones/${candidate.id}`, {
    method: "PATCH",
    body: JSON.stringify({ state: "closed" }),
  });

  // 8. Create official Gitea release
  console.log(`--> Publishing Gitea release ${tag}...`);
  const richReleaseNotes = generateReleaseNotes(version, {
    milestoneTitle: candidate.title,
  });
  const releaseRes = await giteaFetch("/repos/gt-dev/cddm/releases", {
    method: "POST",
    body: JSON.stringify({
      tag_name: tag,
      target_commitish: "main",
      name: `CDDM ${tag} - ${candidate.title}`,
      body: richReleaseNotes,
      draft: false,
      prerelease: false,
    }),
  });

  return {
    triggered: true,
    milestoneId: candidate.id,
    milestoneTitle: candidate.title,
    version,
    tag,
    reason: `Release ${tag} published successfully (Gitea release status: ${releaseRes.status}).`,
  };
}
