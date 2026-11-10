#!/usr/bin/env bun
/**
 * Universal Multi-App Package Manager Distribution Dispatcher
 *
 * Atomically synchronizes application release manifests to central, reusable
 * organization package repositories:
 * - Homebrew Tap: Formula/<package>.rb -> <owner>/homebrew-tap
 * - Scoop Bucket: bucket/<package>.json -> <owner>/scoop-bucket
 * - WinGet Manifest: manifests/<p>/<Publisher>/<Package>/<Version>/... -> <owner>/winget-pkgs
 *
 * Uses VCS variable store (or .env as fallback) for publisher and repository resolution.
 * Authenticates securely via VCS token headers; never exposes tokens in command lines.
 */

import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { type PublisherConfig, resolvePublisherConfig } from "./lib/publisher-config";
import { maskSecret } from "./publish-release";
import { getCurrentVersion } from "./version";

export interface DispatchOptions {
  packageName: string;
  version: string;
  tag: string;
  workspaceRoot: string;
  distDir: string;
  token?: string;
  dryRun: boolean;
  config?: PublisherConfig;
}

export interface DispatchResult {
  homebrew: boolean;
  scoop: boolean;
  winget: boolean;
  errors: string[];
}

/**
 * Fetch existing file blob sha from Gitea contents API if it exists on the target branch.
 */
export async function getExistingFileSha(
  host: string,
  repo: string,
  filePath: string,
  token?: string,
  branch: string = "main",
): Promise<string | null> {
  const url = `https://${host}/api/v1/repos/${repo}/contents/${filePath}?ref=${branch}`;
  try {
    const headers: Record<string, string> = { Accept: "application/json" };
    if (token) {
      headers.Authorization = token.includes(".") ? `Bearer ${token}` : `token ${token}`;
    }
    const res = await fetch(url, { headers });
    if (!res.ok) return null;
    const data = (await res.json()) as { sha?: string };
    return data.sha || null;
  } catch {
    return null;
  }
}

/**
 * Commit a file to a Gitea repository using the contents API.
 */
export async function commitFileToRepo(
  host: string,
  repo: string,
  filePath: string,
  content: string,
  message: string,
  token?: string,
  branch: string = "main",
): Promise<boolean> {
  if (!token) {
    console.log(`\x1b[33m[SKIP] No token provided for repository ${repo}\x1b[0m`);
    return false;
  }

  const existingSha = await getExistingFileSha(host, repo, filePath, token, branch);
  const url = `https://${host}/api/v1/repos/${repo}/contents/${filePath}`;
  const base64Content = Buffer.from(content, "utf-8").toString("base64");

  const method = existingSha ? "PUT" : "POST";
  const body: Record<string, unknown> = {
    content: base64Content,
    message,
    branch,
  };
  if (existingSha) {
    body.sha = existingSha;
  }

  try {
    const authHeader = token.includes(".") ? `Bearer ${token}` : `token ${token}`;
    const res = await fetch(url, {
      method,
      headers: {
        Authorization: authHeader,
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      const errText = await res.text();
      console.error(
        `\x1b[31m[FAIL] Could not commit ${filePath} to ${repo} (${res.status}): ${errText}\x1b[0m`,
      );
      return false;
    }
    console.log(`\x1b[32m[OK] Successfully synchronized ${filePath} to ${repo}\x1b[0m`);
    return true;
  } catch (err) {
    console.error(`\x1b[31m[ERROR] Network error committing to ${repo}: ${String(err)}\x1b[0m`);
    return false;
  }
}

/**
 * Execute distribution dispatch to Homebrew, Scoop, and WinGet central repositories.
 */
export async function dispatchDistribution(options: DispatchOptions): Promise<DispatchResult> {
  const config = options.config || resolvePublisherConfig(options.workspaceRoot);
  const result: DispatchResult = {
    homebrew: false,
    scoop: false,
    winget: false,
    errors: [],
  };

  console.log("\x1b[36m--> Starting Universal Package Distribution Dispatch...\x1b[0m");
  console.log(`Package:      ${options.packageName} v${options.version}`);
  console.log(`Publisher:    ${config.publisherId} (${config.publisherName})`);
  console.log(`VCS Host:     ${config.vcsHost}`);
  console.log(`Homebrew Tap: ${config.homebrewTapRepo}`);
  console.log(`Scoop Bucket: ${config.scoopBucketRepo}`);
  console.log(`WinGet Repo:  ${config.wingetRepo}`);
  if (options.token) {
    console.log(`Auth Token:   ${maskSecret(options.token)}`);
  }

  // 1. Homebrew Tap Dispatch
  const brewFile = join(
    options.workspaceRoot,
    "packaging",
    "homebrew",
    `${options.packageName}.rb`,
  );
  if (existsSync(brewFile)) {
    const brewContent = readFileSync(brewFile, "utf-8");
    const targetPath = `Formula/${options.packageName}.rb`;
    const commitMsg = `release(${options.packageName}): bump to v${options.version}`;
    if (options.dryRun) {
      console.log(
        `\x1b[33m[DRY-RUN] Would commit ${targetPath} to ${config.homebrewTapRepo}\x1b[0m`,
      );
      result.homebrew = true;
    } else {
      result.homebrew = await commitFileToRepo(
        config.vcsHost,
        config.homebrewTapRepo,
        targetPath,
        brewContent,
        commitMsg,
        options.token,
      );
    }
  }

  // 2. Scoop Bucket Dispatch
  const scoopFile = join(
    options.workspaceRoot,
    "packaging",
    "scoop",
    `${options.packageName}.json`,
  );
  if (existsSync(scoopFile)) {
    const scoopContent = readFileSync(scoopFile, "utf-8");
    const targetPath = `bucket/${options.packageName}.json`;
    const commitMsg = `release(${options.packageName}): bump to v${options.version}`;
    if (options.dryRun) {
      console.log(
        `\x1b[33m[DRY-RUN] Would commit ${targetPath} to ${config.scoopBucketRepo}\x1b[0m`,
      );
      result.scoop = true;
    } else {
      result.scoop = await commitFileToRepo(
        config.vcsHost,
        config.scoopBucketRepo,
        targetPath,
        scoopContent,
        commitMsg,
        options.token,
      );
    }
  }

  // 3. WinGet Repository Dispatch
  // WinGet hierarchical layout: manifests/<p>/<Publisher>/<Package>/<Version>/<Publisher>.<Package>.yaml
  const wingetCandidates = [
    join(options.workspaceRoot, "packaging", "winget", `${options.packageName}.yaml`),
    join(
      options.workspaceRoot,
      "packaging",
      "winget",
      `${config.publisherId}.${options.packageName}.yaml`,
    ),
  ];
  const wingetFile = wingetCandidates.find((f) => existsSync(f));
  if (wingetFile) {
    const wingetContent = readFileSync(wingetFile, "utf-8");
    const firstLetter = config.publisherId.charAt(0).toLowerCase();
    const pkgCap = options.packageName.toUpperCase();
    const targetPath = `manifests/${firstLetter}/${config.publisherId}/${pkgCap}/${options.version}/${config.publisherId}.${pkgCap}.yaml`;
    const commitMsg = `release(${options.packageName}): manifest for v${options.version}`;
    if (options.dryRun) {
      console.log(`\x1b[33m[DRY-RUN] Would commit ${targetPath} to ${config.wingetRepo}\x1b[0m`);
      result.winget = true;
    } else {
      result.winget = await commitFileToRepo(
        config.vcsHost,
        config.wingetRepo,
        targetPath,
        wingetContent,
        commitMsg,
        options.token,
      );
    }
  }

  return result;
}

export function parseArgs(args: string[]): DispatchOptions {
  let version = getCurrentVersion();
  let packageName = "cddm";
  let tag = `v${version}`;
  let workspaceRoot = process.cwd();
  let distDir = "dist-release";
  let token =
    process.env.DISTRIBUTION_TOKEN ||
    process.env.GITEA_TOKEN ||
    process.env.REGISTRY_TOKEN ||
    process.env.GITHUB_TOKEN;
  let dryRun = false;

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (arg === "--dry-run") dryRun = true;
    else if (arg === "--package" && args[i + 1]) packageName = args[++i]!;
    else if (arg === "--version" && args[i + 1]) version = args[++i]!;
    else if (arg === "--tag" && args[i + 1]) tag = args[++i]!;
    else if (arg === "--token" && args[i + 1]) token = args[++i]!;
    else if (arg === "--dist-dir" && args[i + 1]) distDir = args[++i]!;
    else if (arg === "--workspace" && args[i + 1]) workspaceRoot = resolve(args[++i]!);
  }

  return {
    packageName,
    version,
    tag,
    workspaceRoot,
    distDir,
    token,
    dryRun,
  };
}

async function main() {
  const options = parseArgs(process.argv.slice(2));
  const res = await dispatchDistribution(options);
  console.log("\nDistribution Dispatch Summary:");
  console.log(`- Homebrew: ${res.homebrew ? "OK" : "Skipped/Failed"}`);
  console.log(`- Scoop:    ${res.scoop ? "OK" : "Skipped/Failed"}`);
  console.log(`- WinGet:   ${res.winget ? "OK" : "Skipped/Failed"}`);
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal dispatch error:", err);
    process.exit(1);
  });
}
