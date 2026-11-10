#!/usr/bin/env bun
/**
 * Unified Lockstep Semantic Versioning, SSoT Manifest Synchronizer & PR Gate for CDDM.
 *
 * Single canonical engine that:
 * 1. Enforces Conventional Commits and calculates the next semantic version from git history.
 * 2. Synchronizes all 10 workspace manifests, distribution URLs, Cargo.lock, and CHANGELOG.md.
 * 3. Verifies 100% SSoT manifest synchronization (--check) and PR version bumps (--check-pr).
 */

import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import {
  checkWorkspaceVersionSync,
  generateChangelogSection,
  updateChangelog,
  updateWorkspaceVersions,
  type VersionSyncCheckResult,
} from "./lib/version-updater";
import { CONVENTIONAL_COMMIT_REGEX as COMMIT_REGEX } from "./validate-commit-msg";

export {
  COMMIT_REGEX,
  checkWorkspaceVersionSync,
  generateChangelogSection,
  updateChangelog,
  updateWorkspaceVersions,
  type VersionSyncCheckResult,
};

export interface ParsedCommit {
  hash: string;
  type: string;
  scope?: string;
  breaking: boolean;
  subject: string;
  raw: string;
}

export type BumpType = "major" | "minor" | "patch" | "none";

export interface SemanticVersion {
  major: number;
  minor: number;
  patch: number;
  raw: string;
}

export interface PrVersionCheckResult {
  passed: boolean;
  currentVersion: string;
  baselineVersion: string;
  requiredBump: BumpType;
  commitCount: number;
  errors: string[];
}

export const CONVENTIONAL_TYPES = [
  "feat",
  "fix",
  "docs",
  "style",
  "refactor",
  "perf",
  "test",
  "build",
  "ci",
  "chore",
  "revert",
] as const;

/**
 * Parse a single commit message into a structured Conventional Commit object.
 */
export function parseCommitLine(hash: string, message: string): ParsedCommit | null {
  const match = COMMIT_REGEX.exec(message.trim());
  if (!match) {
    return null;
  }

  const [, type, scope, breakingExclamation, subject] = match;
  const isBreaking = Boolean(breakingExclamation) || message.includes("BREAKING CHANGE:");

  return {
    hash: hash.trim(),
    type: type ?? "chore",
    scope: scope || undefined,
    breaking: isBreaking,
    subject: subject?.trim() ?? "",
    raw: message.trim(),
  };
}

/**
 * Parse a semver string (e.g. "0.1.2" or "v0.1.2") into numeric components.
 */
export function parseSemver(version: string): SemanticVersion {
  const cleaned = version.replace(/^v/, "").trim();
  const parts = cleaned.split(".").map((p) => Number.parseInt(p, 10));

  const major = parts[0] ?? 0;
  const minor = parts[1] ?? 0;
  const patch = parts[2] ?? 0;

  if (Number.isNaN(major) || Number.isNaN(minor) || Number.isNaN(patch)) {
    throw new Error(`Invalid semver string: "${version}"`);
  }

  return {
    major,
    minor,
    patch,
    raw: `${major}.${minor}.${patch}`,
  };
}

/**
 * Compare two semantic versions: returns 1 if a > b, -1 if a < b, 0 if a === b.
 */
export function compareSemver(a: SemanticVersion | string, b: SemanticVersion | string): number {
  const va = typeof a === "string" ? parseSemver(a) : a;
  const vb = typeof b === "string" ? parseSemver(b) : b;

  if (va.major !== vb.major) return va.major > vb.major ? 1 : -1;
  if (va.minor !== vb.minor) return va.minor > vb.minor ? 1 : -1;
  if (va.patch !== vb.patch) return va.patch > vb.patch ? 1 : -1;
  return 0;
}

/**
 * Increment a semantic version based on bump type.
 */
export function incrementVersion(current: SemanticVersion, bump: BumpType): SemanticVersion {
  switch (bump) {
    case "major":
      return {
        major: current.major + 1,
        minor: 0,
        patch: 0,
        raw: `${current.major + 1}.0.0`,
      };
    case "minor":
      return {
        major: current.major,
        minor: current.minor + 1,
        patch: 0,
        raw: `${current.major}.${current.minor + 1}.0`,
      };
    case "patch":
      return {
        major: current.major,
        minor: current.minor,
        patch: current.patch + 1,
        raw: `${current.major}.${current.minor}.${current.patch + 1}`,
      };
    case "none":
      return current;
  }
}

/**
 * Determine required version bump from a list of parsed commits.
 */
export function determineBump(commits: ParsedCommit[]): BumpType {
  let hasBreaking = false;
  let hasFeat = false;
  let hasPatch = false;

  for (const commit of commits) {
    if (commit.breaking) {
      hasBreaking = true;
    } else if (commit.type === "feat") {
      hasFeat = true;
    } else if (
      [
        "fix",
        "perf",
        "refactor",
        "style",
        "docs",
        "chore",
        "build",
        "ci",
        "test",
        "revert",
      ].includes(commit.type)
    ) {
      hasPatch = true;
    }
  }

  if (hasBreaking) return "major";
  if (hasFeat) return "minor";
  if (hasPatch) return "patch";
  return "none";
}

/**
 * Retrieve the current workspace version from Cargo.toml.
 */
export function getCurrentVersion(workspaceRoot: string = process.cwd()): string {
  const cargoPath = join(workspaceRoot, "Cargo.toml");
  if (!existsSync(cargoPath)) {
    throw new Error(`Cargo.toml not found at ${cargoPath}`);
  }

  const content = readFileSync(cargoPath, "utf-8");
  const match = /\[workspace\.package\][\s\S]*?version\s*=\s*"([^"]+)"/.exec(content);
  if (!match || !match[1]) {
    throw new Error("Could not find [workspace.package] version in Cargo.toml");
  }

  return match[1];
}

/**
 * Retrieve the latest semantic version git tag (e.g. "4.8.0" from "v4.8.0"), or null if none.
 */
export function getLatestTagVersion(workspaceRoot: string = process.cwd()): string | null {
  try {
    const tagProc = Bun.spawnSync(["git", "describe", "--tags", "--abbrev=0"], {
      cwd: workspaceRoot,
      stderr: "pipe",
    });
    if (tagProc.exitCode === 0) {
      const rawTag = tagProc.stdout.toString().trim();
      if (rawTag) {
        return parseSemver(rawTag).raw;
      }
    }
  } catch {
    // No git tags exist
  }
  return null;
}

/**
 * Retrieve recent git commits since the last tag or initial commit.
 */
export function getCommitsSinceLastTag(workspaceRoot: string = process.cwd()): ParsedCommit[] {
  const latestTagVer = getLatestTagVersion(workspaceRoot);
  const logArgs = latestTagVer
    ? ["git", "log", `v${latestTagVer}..HEAD`, "--pretty=format:%h%x09%s"]
    : ["git", "log", "--pretty=format:%h%x09%s", "-n", "100"];

  const logProc = Bun.spawnSync(logArgs, {
    cwd: workspaceRoot,
    stderr: "pipe",
  });

  if (logProc.exitCode !== 0) {
    return [];
  }

  const lines = logProc.stdout.toString().trim().split("\n").filter(Boolean);
  const parsed: ParsedCommit[] = [];

  for (const line of lines) {
    const [hash, ...rest] = line.split("\t");
    const msg = rest.join("\t");
    if (hash && msg) {
      const commit = parseCommitLine(hash, msg);
      if (commit) {
        parsed.push(commit);
      }
    }
  }

  return parsed;
}

/**
 * Verify that a PR branch has properly bumped the workspace version and updated CHANGELOG.md
 * when release-worthy commits exist since the latest release tag.
 */
export function checkPrVersionBump(workspaceRoot: string = process.cwd()): PrVersionCheckResult {
  const errors: string[] = [];
  const syncRes = checkWorkspaceVersionSync(workspaceRoot);
  if (!syncRes.synchronized) {
    for (const m of syncRes.mismatches) {
      errors.push(`${m.file} (${m.field}): expected "${m.expected}", found "${m.actual}"`);
    }
  }

  const currentVersion = syncRes.canonicalVersion;
  const baselineVersion = getLatestTagVersion(workspaceRoot) ?? currentVersion;
  const commits = getCommitsSinceLastTag(workspaceRoot);
  const releaseWorthyCommits = commits.filter(
    (c) => c.breaking || ["feat", "fix", "perf", "refactor", "revert"].includes(c.type),
  );
  const requiredBump = determineBump(releaseWorthyCommits);

  if (requiredBump !== "none") {
    if (compareSemver(currentVersion, baselineVersion) <= 0) {
      const expectedNext = incrementVersion(parseSemver(baselineVersion), requiredBump).raw;
      errors.push(
        `Workspace version (${currentVersion}) has not been bumped above latest release tag (v${baselineVersion}) despite ${releaseWorthyCommits.length} release-worthy commit(s) requiring a ${requiredBump.toUpperCase()} bump (expected >= ${expectedNext}). Run 'bun scripts/version.ts' to bump and sync manifests.`,
      );
    }

    const changelogPath = join(workspaceRoot, "CHANGELOG.md");
    if (existsSync(changelogPath)) {
      const changelogContent = readFileSync(changelogPath, "utf-8");
      if (!changelogContent.includes(`## [${currentVersion}]`)) {
        errors.push(
          `CHANGELOG.md is missing a release entry for current version "## [${currentVersion}]". Run 'bun scripts/version.ts' to generate it.`,
        );
      }
    }
  }

  return {
    passed: errors.length === 0,
    currentVersion,
    baselineVersion,
    requiredBump,
    commitCount: commits.length,
    errors,
  };
}

async function main() {
  const args = process.argv.slice(2);

  if (args.includes("--help") || args.includes("-h")) {
    console.log(`
CDDM Unified Lockstep Semantic Versioning & SSoT Manifest Synchronizer

Usage:
  bun scripts/version.ts [options]

Options:
  --check                   Verify all 10 workspace manifests & packaging URLs match Cargo.toml
  --check-pr                Verify PR branch bumped version & CHANGELOG.md over latest release tag
  --auto                    Idempotently calculate and apply SemVer bump from latest release tag
  --dry-run                 Preview version bump and changelog without writing files
  --get-current             Print current Cargo.toml workspace version and exit
  --get-latest-tag          Print latest Git tag version and exit
  --get-next                Calculate and print next version and exit
  --release-as <version>    Force a specific bump ("major", "minor", "patch", or explicit "1.2.3")
  --git-tag                 Create git release commit and tag (v<version>)
  --help, -h                Show this help message
`);
    process.exit(0);
  }

  const currentVersionStr = getCurrentVersion();

  if (args.includes("--get-current")) {
    console.log(currentVersionStr);
    process.exit(0);
  }

  if (args.includes("--get-latest-tag")) {
    console.log(getLatestTagVersion() ?? currentVersionStr);
    process.exit(0);
  }

  if (args.includes("--check")) {
    console.log("--> Verifying SSoT workspace version synchronization across all manifests...");
    const res = checkWorkspaceVersionSync();
    if (!res.synchronized) {
      console.error(
        `\x1b[31m[FAIL] Found ${res.mismatches.length} version synchronization drift error(s) against Cargo.toml (${res.canonicalVersion}):\x1b[0m`,
      );
      for (const m of res.mismatches) {
        console.error(`  - ${m.file} [${m.field}]: expected "${m.expected}", found "${m.actual}"`);
      }
      console.error("\nRun 'bun scripts/version.ts' to synchronize all workspace manifests.");
      process.exit(1);
    }
    console.log(
      `\x1b[32m[PASS] All ${res.checkedFiles} workspace manifests and packaging URLs are 100% synchronized at v${res.canonicalVersion}.\x1b[0m`,
    );
    process.exit(0);
  }

  if (args.includes("--check-pr")) {
    console.log("--> Verifying PR branch version bump and manifest synchronization...");
    const prRes = checkPrVersionBump();
    if (!prRes.passed) {
      console.error(`\x1b[31m[FAIL] PR version gate check failed:\x1b[0m`);
      for (const err of prRes.errors) {
        console.error(`  - ${err}`);
      }
      process.exit(1);
    }
    console.log(
      `\x1b[32m[PASS] PR version gate passed (baseline: v${prRes.baselineVersion} -> current: v${prRes.currentVersion}, commits: ${prRes.commitCount}).\x1b[0m`,
    );
    process.exit(0);
  }

  // Use latest git tag as baseline for idempotent branch bumping
  const baselineVersionStr = getLatestTagVersion() ?? currentVersionStr;
  const baselineSemver = parseSemver(baselineVersionStr);
  const commits = getCommitsSinceLastTag();

  let bumpType: BumpType = determineBump(commits);
  let nextSemver: SemanticVersion;

  const releaseAsIndex = args.indexOf("--release-as");
  if (releaseAsIndex !== -1 && args[releaseAsIndex + 1]) {
    const override = args[releaseAsIndex + 1]!;
    if (["major", "minor", "patch"].includes(override)) {
      bumpType = override as BumpType;
      nextSemver = incrementVersion(baselineSemver, bumpType);
    } else {
      nextSemver = parseSemver(override);
      bumpType = "none";
    }
  } else {
    if (bumpType === "none") {
      bumpType = "patch";
    }
    nextSemver = incrementVersion(baselineSemver, bumpType);
  }

  if (args.includes("--get-next")) {
    console.log(nextSemver.raw);
    process.exit(0);
  }

  console.log(`\x1b[36mCurrent Workspace Version:\x1b[0m ${currentVersionStr}`);
  console.log(`\x1b[36mBaseline Release Tag:\x1b[0m      v${baselineVersionStr}`);
  console.log(`\x1b[36mCommits Since Last Tag:\x1b[0m    ${commits.length}`);
  console.log(
    `\x1b[35mCalculated Target Version:\x1b[0m ${bumpType.toUpperCase()} -> \x1b[32m${nextSemver.raw}\x1b[0m\n`,
  );

  const dateStr = new Date().toISOString().split("T")[0]!;
  const changelogSection = generateChangelogSection(nextSemver.raw, dateStr, commits);

  if (args.includes("--dry-run")) {
    console.log("\x1b[33m--- [DRY RUN PREVIEW] Changelog Section ---\x1b[0m");
    console.log(changelogSection);
    console.log("\x1b[33m--- [DRY RUN] No files modified. ---\x1b[0m");
    process.exit(0);
  }

  updateWorkspaceVersions(nextSemver.raw);
  updateChangelog(changelogSection);

  if (args.includes("--git-tag")) {
    const tagName = `v${nextSemver.raw}`;
    console.log(`\n\x1b[36mCreating release commit and tag ${tagName}...\x1b[0m`);

    Bun.spawnSync(["vp", "fmt"]);
    Bun.spawnSync([
      "git",
      "add",
      "Cargo.toml",
      "Cargo.lock",
      "package.json",
      "webui/package.json",
      "npm/cddm/package.json",
      "editors/vscode/package.json",
      "packaging/homebrew/cddm.rb",
      "packaging/scoop/cddm.json",
      "packaging/winget/cddm.yaml",
      "CHANGELOG.md",
      "README.md",
    ]);
    const commitProc = Bun.spawnSync(["git", "commit", "-m", `chore(release): ${tagName}`]);
    if (commitProc.exitCode === 0) {
      Bun.spawnSync(["git", "tag", "-a", tagName, "-m", `Release ${tagName}`]);
      console.log(`\x1b[32m[OK] Release commit and tag ${tagName} created successfully!\x1b[0m`);
    } else {
      console.error(`\x1b[31mFailed to create git commit: ${commitProc.stderr.toString()}\x1b[0m`);
    }
  }

  console.log(
    `\n\x1b[32m[OK] Successfully synchronized workspace to version ${nextSemver.raw}!\x1b[0m\n`,
  );
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal versioning error:", err);
    process.exit(1);
  });
}
