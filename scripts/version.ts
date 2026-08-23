#!/usr/bin/env bun
/**
 * Automated Semantic Versioning and Changelog Generator for CDDM.
 * Enforces Conventional Commits and calculates the next semantic version
 * by analyzing git commit history since the last release tag.
 *
 * Synchronizes versions across:
 * - Cargo.toml ([workspace.package] version)
 * - package.json (root workspace)
 * - webui/package.json
 * - npm/cddm/package.json
 * - CHANGELOG.md
 */

import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

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

export const COMMIT_REGEX =
  /^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(?:\(([a-zA-Z0-9_-]+)\))?(!)?:\s*(.+)$/;

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
 * Retrieve recent git commits since the last tag or initial commit.
 */
export function getCommitsSinceLastTag(workspaceRoot: string = process.cwd()): ParsedCommit[] {
  let lastTag = "";
  try {
    const tagProc = Bun.spawnSync(["git", "describe", "--tags", "--abbrev=0"], {
      cwd: workspaceRoot,
      stderr: "pipe",
    });
    if (tagProc.exitCode === 0) {
      lastTag = tagProc.stdout.toString().trim();
    }
  } catch {
    // No git tags exist yet
  }

  const logArgs = lastTag
    ? ["git", "log", `${lastTag}..HEAD`, "--pretty=format:%h%x09%s"]
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
 * Generate formatted Markdown changelog section for a release.
 */
export function generateChangelogSection(
  version: string,
  dateStr: string,
  commits: ParsedCommit[],
): string {
  const features = commits.filter((c) => c.type === "feat");
  const fixes = commits.filter((c) => c.type === "fix");
  const perfs = commits.filter((c) => c.type === "perf");
  const refactors = commits.filter((c) => c.type === "refactor");
  const docs = commits.filter((c) => c.type === "docs");
  const chores = commits.filter((c) => ["chore", "build", "ci", "test", "style"].includes(c.type));
  const breakings = commits.filter((c) => c.breaking);

  let section = `## [${version}] - ${dateStr}\n\n`;

  if (breakings.length > 0) {
    section += `### BREAKING CHANGES\n\n`;
    for (const c of breakings) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (features.length > 0) {
    section += `### Features\n\n`;
    for (const c of features) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (fixes.length > 0) {
    section += `### Bug Fixes\n\n`;
    for (const c of fixes) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (perfs.length > 0) {
    section += `### Performance Improvements\n\n`;
    for (const c of perfs) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (refactors.length > 0) {
    section += `### Refactoring\n\n`;
    for (const c of refactors) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (docs.length > 0) {
    section += `### Documentation\n\n`;
    for (const c of docs) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (chores.length > 0) {
    section += `### Tooling & Maintenance\n\n`;
    for (const c of chores) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  return section;
}

/**
 * Update all version manifests across the repository.
 */
export function updateWorkspaceVersions(
  newVersion: string,
  workspaceRoot: string = process.cwd(),
): void {
  // 1. Cargo.toml
  const cargoPath = join(workspaceRoot, "Cargo.toml");
  if (existsSync(cargoPath)) {
    const cargoContent = readFileSync(cargoPath, "utf-8");
    const updatedCargo = cargoContent.replace(
      /(\[workspace\.package\][\s\S]*?version\s*=\s*)"[^"]+"/,
      `$1"${newVersion}"`,
    );
    writeFileSync(cargoPath, updatedCargo, "utf-8");
    console.log(`\x1b[32m[OK] Updated Cargo.toml -> ${newVersion}\x1b[0m`);
  }

  // 2. Root package.json
  const rootPkgPath = join(workspaceRoot, "package.json");
  if (existsSync(rootPkgPath)) {
    const pkg = JSON.parse(readFileSync(rootPkgPath, "utf-8"));
    pkg.version = newVersion;
    writeFileSync(rootPkgPath, JSON.stringify(pkg, null, 2) + "\n", "utf-8");
    console.log(`\x1b[32m[OK] Updated package.json -> ${newVersion}\x1b[0m`);
  }

  // 3. WebUI package.json
  const webuiPkgPath = join(workspaceRoot, "webui", "package.json");
  if (existsSync(webuiPkgPath)) {
    const pkg = JSON.parse(readFileSync(webuiPkgPath, "utf-8"));
    pkg.version = newVersion;
    writeFileSync(webuiPkgPath, JSON.stringify(pkg, null, 2) + "\n", "utf-8");
    console.log(`\x1b[32m[OK] Updated webui/package.json -> ${newVersion}\x1b[0m`);
  }

  // 4. npm/cddm/package.json
  const npmPkgPath = join(workspaceRoot, "npm", "cddm", "package.json");
  if (existsSync(npmPkgPath)) {
    const pkg = JSON.parse(readFileSync(npmPkgPath, "utf-8"));
    pkg.version = newVersion;
    writeFileSync(npmPkgPath, JSON.stringify(pkg, null, 2) + "\n", "utf-8");
    console.log(`\x1b[32m[OK] Updated npm/cddm/package.json -> ${newVersion}\x1b[0m`);
  }

  // 5. README.md badges
  const readmePath = join(workspaceRoot, "README.md");
  if (existsSync(readmePath)) {
    const readmeContent = readFileSync(readmePath, "utf-8");
    const updatedReadme = readmeContent
      .replace(/badge\/npm-[\d.]+-red\.svg/, `badge/npm-${newVersion}-red.svg`)
      .replace(
        /badge\/crates\.io-[\d.]+-brightgreen\.svg/,
        `badge/crates.io-${newVersion}-brightgreen.svg`,
      );
    writeFileSync(readmePath, updatedReadme, "utf-8");
    console.log(`\x1b[32m[OK] Updated README.md -> ${newVersion}\x1b[0m`);
  }

  // 6. Cargo.lock
  Bun.spawnSync(["cargo", "check", "--workspace"], { cwd: workspaceRoot });
}

/**
 * Prepend a new section to CHANGELOG.md.
 */
export function updateChangelog(newSection: string, workspaceRoot: string = process.cwd()): void {
  const changelogPath = join(workspaceRoot, "CHANGELOG.md");
  let header =
    "# Changelog\n\nAll notable changes to **CDDM** will be documented in this file.\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n";

  let existingContent = "";
  if (existsSync(changelogPath)) {
    const full = readFileSync(changelogPath, "utf-8");
    const firstSectionIndex = full.indexOf("\n## [");
    if (firstSectionIndex !== -1) {
      header = full.slice(0, firstSectionIndex + 1);
      existingContent = full.slice(firstSectionIndex + 1);
    } else {
      existingContent = full;
    }
  }

  const updated = `${header}${newSection}${existingContent}`;
  writeFileSync(changelogPath, updated, "utf-8");
  console.log(`\x1b[32m[OK] Updated CHANGELOG.md\x1b[0m`);
}

async function main() {
  const args = process.argv.slice(2);

  if (args.includes("--help") || args.includes("-h")) {
    console.log(`
CDDM Automated Semantic Versioning & Conventional Commits Tool

Usage:
  bun scripts/version.ts [options]

Options:
  --dry-run                 Preview version bump and changelog without writing files
  --get-current             Print current version and exit
  --get-next                Calculate and print next version and exit
  --release-as <version>    Force a specific bump ("major", "minor", "patch", or explicit "1.2.3")
  --git-tag                 Create git release commit and tag (v<version>)
  --help, -h                Show this help message
`);
    process.exit(0);
  }

  const currentVersionStr = getCurrentVersion();
  const currentSemver = parseSemver(currentVersionStr);

  if (args.includes("--get-current")) {
    console.log(currentVersionStr);
    process.exit(0);
  }

  const commits = getCommitsSinceLastTag();
  console.log(`\x1b[36mCurrent Version:\x1b[0m ${currentVersionStr}`);
  console.log(`\x1b[36mFound Commits Since Last Tag:\x1b[0m ${commits.length}`);

  let bumpType: BumpType = determineBump(commits);
  let nextSemver: SemanticVersion;

  const releaseAsIndex = args.indexOf("--release-as");
  if (releaseAsIndex !== -1 && args[releaseAsIndex + 1]) {
    const override = args[releaseAsIndex + 1]!;
    if (["major", "minor", "patch"].includes(override)) {
      bumpType = override as BumpType;
      nextSemver = incrementVersion(currentSemver, bumpType);
    } else {
      nextSemver = parseSemver(override);
      bumpType = "none";
    }
  } else {
    if (bumpType === "none") {
      bumpType = "patch"; // Default to patch bump if invoked explicitly
    }
    nextSemver = incrementVersion(currentSemver, bumpType);
  }

  if (args.includes("--get-next")) {
    console.log(nextSemver.raw);
    process.exit(0);
  }

  console.log(
    `\x1b[35mCalculated Bump:\x1b[0m ${bumpType.toUpperCase()} -> \x1b[32m${nextSemver.raw}\x1b[0m\n`,
  );

  const dateStr = new Date().toISOString().split("T")[0]!;
  const changelogSection = generateChangelogSection(nextSemver.raw, dateStr, commits);

  if (args.includes("--dry-run")) {
    console.log("\x1b[33m--- [DRY RUN PREVIEW] Changelog Section ---\x1b[0m");
    console.log(changelogSection);
    console.log("\x1b[33m--- [DRY RUN] No files modified. ---\x1b[0m");
    process.exit(0);
  }

  // Write updates to all files
  updateWorkspaceVersions(nextSemver.raw);
  updateChangelog(changelogSection);

  // Auto-tag if requested
  if (args.includes("--git-tag")) {
    const tagName = `v${nextSemver.raw}`;
    console.log(`\n\x1b[36mCreating release commit and tag ${tagName}...\x1b[0m`);

    Bun.spawnSync([
      "git",
      "add",
      "Cargo.toml",
      "Cargo.lock",
      "package.json",
      "webui/package.json",
      "npm/cddm/package.json",
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
    `\n\x1b[32m[OK] Successfully updated workspace to version ${nextSemver.raw}!\x1b[0m\n`,
  );
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal versioning error:", err);
    process.exit(1);
  });
}
