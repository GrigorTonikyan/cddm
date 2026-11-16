#!/usr/bin/env bun
/**
 * Universal Multi-App Package Manager Distribution Dispatcher
 *
 * Atomically synchronizes application release manifests to central, reusable
 * organization package repositories:
 * - Homebrew Tap: Formula/<package>.rb -> <owner>/homebrew-tap
 * - Scoop Bucket: bucket/<package>.json -> <owner>/scoop-bucket
 * - WinGet Manifest: manifests/<p>/<publisher>/<package>/<version>/... -> <owner>/winget-pkgs
 *
 * Enforces pre-flight cryptographic integrity & reachability gates:
 * - Parses SHA256SUMS.txt and rejects missing, malformed, or zeroed (0{64}) digests.
 * - Verifies HTTP 200 OK reachability of every binary archive URL prior to commit/push.
 * - Prefers per-repository SSH deploy keys (HOMEBREW_TAP_DEPLOY_KEY, SCOOP_BUCKET_DEPLOY_KEY,
 *   WINGET_DEPLOY_KEY); falls back to Gitea Contents API with RELEASE_BOT_TOKEN/GITEA_TOKEN.
 * - Rejects passing tokens via --token CLI flags.
 */

import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import {
  classifyTargetsFromChecksums,
  collectTargetAssets,
  findZeroSha256Violations,
  parseSha256SumsValidated,
  renderHomebrewFormula,
  renderScoopManifest,
  renderWingetManifests,
  verifyDistributionUrls,
  type DistributionManifestSpec,
} from "./lib/distribution-manifests";
import {
  commitFilesWithDeployKey,
  commitFileToRepo,
  getExistingFileSha,
  publishRepoPayload,
  type RepoFilePayload,
} from "./lib/gitea-git-transport";
import { GITEA_TOKEN } from "./lib/gitea-client";
import { type PublisherConfig, resolvePublisherConfig } from "./lib/publisher-config";
import { maskSecret } from "./publish-release";
import { getCurrentVersion } from "./version";

export { commitFilesWithDeployKey, commitFileToRepo, getExistingFileSha, type RepoFilePayload };

export interface DeployKeySet {
  homebrew?: string;
  scoop?: string;
  winget?: string;
}

export interface DispatchOptions {
  packageName: string;
  version: string;
  tag: string;
  workspaceRoot: string;
  distDir: string;
  token?: string;
  deployKeys?: DeployKeySet;
  dryRun: boolean;
  verifyUrls?: boolean;
  fetchFn?: typeof fetch;
  config?: PublisherConfig;
}

export interface DispatchResult {
  homebrew: boolean;
  scoop: boolean;
  winget: boolean;
  errors: string[];
}

function readPackageMetadata(workspaceRoot: string): { description: string; license: string } {
  const cargoPath = join(workspaceRoot, "Cargo.toml");
  const content = existsSync(cargoPath) ? readFileSync(cargoPath, "utf-8") : "";
  const descMatch = /description\s*=\s*"([^"]+)"/.exec(content);
  const licenseMatch = /license\s*=\s*"([^"]+)"/.exec(content);
  return {
    description:
      descMatch?.[1] ?? "Polyglot Code De-Duplication Meister & Autonomous Refactoring Engine",
    license: licenseMatch?.[1] ?? "MIT",
  };
}

function resolveSumsPath(workspaceRoot: string, distDir: string): string | undefined {
  const candidates = [
    join(distDir, "SHA256SUMS.txt"),
    join(workspaceRoot, distDir, "SHA256SUMS.txt"),
  ];
  return candidates.find((p) => existsSync(p));
}

function collectFallbackPayloads(
  options: DispatchOptions,
  config: PublisherConfig,
): {
  brewFiles: RepoFilePayload[];
  scoopFiles: RepoFilePayload[];
  wingetFiles: RepoFilePayload[];
} {
  const brewFiles: RepoFilePayload[] = [];
  const scoopFiles: RepoFilePayload[] = [];
  const wingetFiles: RepoFilePayload[] = [];

  const brewFile = join(
    options.workspaceRoot,
    "packaging",
    "homebrew",
    `${options.packageName}.rb`,
  );
  if (existsSync(brewFile)) {
    brewFiles.push({
      path: `Formula/${options.packageName}.rb`,
      content: readFileSync(brewFile, "utf-8"),
    });
  }

  const scoopFile = join(
    options.workspaceRoot,
    "packaging",
    "scoop",
    `${options.packageName}.json`,
  );
  if (existsSync(scoopFile)) {
    scoopFiles.push({
      path: `bucket/${options.packageName}.json`,
      content: readFileSync(scoopFile, "utf-8"),
    });
  }

  const pkgId = `${config.publisherId}.${options.packageName}`;
  const firstLetter = config.publisherId.charAt(0).toLowerCase();
  const targetDir = `manifests/${firstLetter}/${config.publisherId}/${options.packageName}/${options.version}`;
  const wingetDir = join(options.workspaceRoot, "packaging", "winget");
  for (const fname of [
    `${pkgId}.yaml`,
    `${pkgId}.installer.yaml`,
    `${pkgId}.locale.en-US.yaml`,
    `${options.packageName}.yaml`,
  ]) {
    const fpath = join(wingetDir, fname);
    if (existsSync(fpath)) {
      const destName = fname === `${options.packageName}.yaml` ? `${pkgId}.yaml` : fname;
      wingetFiles.push({ path: `${targetDir}/${destName}`, content: readFileSync(fpath, "utf-8") });
    }
  }

  return { brewFiles, scoopFiles, wingetFiles };
}

/**
 * Execute distribution dispatch to Homebrew, Scoop, and WinGet central repositories.
 */
export async function dispatchDistribution(options: DispatchOptions): Promise<DispatchResult> {
  const config = options.config || resolvePublisherConfig(options.workspaceRoot);
  const deployKeys: DeployKeySet = {
    homebrew: options.deployKeys?.homebrew ?? process.env.HOMEBREW_TAP_DEPLOY_KEY,
    scoop: options.deployKeys?.scoop ?? process.env.SCOOP_BUCKET_DEPLOY_KEY,
    winget: options.deployKeys?.winget ?? process.env.WINGET_DEPLOY_KEY,
  };
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

  const sumsPath = resolveSumsPath(options.workspaceRoot, options.distDir);
  let sumsContent = sumsPath ? readFileSync(sumsPath, "utf-8") : undefined;

  if (!sumsContent && !options.dryRun) {
    const remoteSumsUrl = `https://${config.vcsHost}/${config.vcsRepo}/releases/download/${options.tag}/SHA256SUMS.txt`;
    const fetchFn = options.fetchFn ?? fetch;
    try {
      const res = await fetchFn(remoteSumsUrl, {
        headers: options.token ? { Authorization: `token ${options.token}` } : undefined,
      });
      if (res.ok) {
        sumsContent = await res.text();
        console.log(`\x1b[32m[OK] Fetched remote SHA256SUMS.txt from ${remoteSumsUrl}\x1b[0m`);
      }
    } catch {
      // handled below
    }
  }

  if (!options.dryRun && !sumsContent) {
    throw new Error(
      `Pre-flight security gate failed: SHA256SUMS.txt not found in distDir "${options.distDir}".`,
    );
  }

  let brewFiles: RepoFilePayload[] = [];
  let scoopFiles: RepoFilePayload[] = [];
  let wingetFiles: RepoFilePayload[] = [];

  if (sumsContent) {
    const checksums = parseSha256SumsValidated(sumsContent);
    const releaseBaseUrl = `https://${config.vcsHost}/${config.vcsRepo}/releases/download/${options.tag}`;
    const targets = classifyTargetsFromChecksums(checksums, releaseBaseUrl);
    const targetAssets = collectTargetAssets(targets);

    const shouldVerifyUrls = options.verifyUrls ?? !options.dryRun;
    if (shouldVerifyUrls) {
      await verifyDistributionUrls(targetAssets, {
        token: options.token,
        fetchFn: options.fetchFn,
      });
    }

    const meta = readPackageMetadata(options.workspaceRoot);
    const spec: DistributionManifestSpec = {
      packageName: options.packageName,
      version: options.version,
      publisherId: config.publisherId,
      publisherName: config.publisherName,
      homepage: `https://${config.vcsHost}/${config.vcsRepo}`,
      description: meta.description,
      license: meta.license,
      binaries:
        options.packageName === "cddm" ? ["cddm", "cddm-mcp", "cddm-lsp"] : [options.packageName],
      releaseBaseUrl,
      targets,
    };

    if (targets.macosArm64 || targets.macosX64 || targets.linuxArm64 || targets.linuxX64) {
      brewFiles = [
        { path: `Formula/${options.packageName}.rb`, content: renderHomebrewFormula(spec) },
      ];
    }
    if (targets.windowsX64 || targets.windowsArm64) {
      scoopFiles = [
        { path: `bucket/${options.packageName}.json`, content: renderScoopManifest(spec) },
      ];
      const wg = renderWingetManifests(spec);
      wingetFiles = [
        { path: `${wg.manifestDir}/${wg.versionFile}`, content: wg.versionContent },
        { path: `${wg.manifestDir}/${wg.installerFile}`, content: wg.installerContent },
        { path: `${wg.manifestDir}/${wg.localeFile}`, content: wg.localeContent },
      ];
    }
  } else {
    ({ brewFiles, scoopFiles, wingetFiles } = collectFallbackPayloads(options, config));
  }

  const zeroHashViolations = findZeroSha256Violations(
    [...brewFiles, ...scoopFiles, ...wingetFiles].map((f) => ({
      label: f.path,
      content: f.content,
    })),
  );
  if (zeroHashViolations.length > 0) {
    throw new Error(`Pre-flight zero-hash check failed:\n${zeroHashViolations.join("\n")}`);
  }

  const commitMsg = `release(${options.packageName}): bump to v${options.version}`;

  if (brewFiles.length > 0) {
    result.homebrew = await publishRepoPayload(
      config.vcsHost,
      config.homebrewTapRepo,
      brewFiles,
      commitMsg,
      {
        dryRun: options.dryRun,
        deployKey: deployKeys.homebrew,
        token: options.token,
        fetchFn: options.fetchFn,
      },
    );
  }

  if (scoopFiles.length > 0) {
    result.scoop = await publishRepoPayload(
      config.vcsHost,
      config.scoopBucketRepo,
      scoopFiles,
      commitMsg,
      {
        dryRun: options.dryRun,
        deployKey: deployKeys.scoop,
        token: options.token,
        fetchFn: options.fetchFn,
      },
    );
  }

  if (wingetFiles.length > 0) {
    result.winget = await publishRepoPayload(
      config.vcsHost,
      config.wingetRepo,
      wingetFiles,
      commitMsg,
      {
        dryRun: options.dryRun,
        deployKey: deployKeys.winget,
        token: options.token,
        fetchFn: options.fetchFn,
      },
    );
  }

  return result;
}

export function parseArgs(args: string[]): DispatchOptions {
  let workspaceRoot = process.cwd();
  let packageName = "cddm";
  let version = getCurrentVersion();
  let tag = `v${version}`;
  let distDir = "dist-release";
  const token =
    process.env.RELEASE_BOT_TOKEN || process.env.GITEA_TOKEN || process.env.GITHUB_TOKEN;
  let dryRun = false;

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (arg === "--token" || arg?.startsWith("--token=")) {
      throw new Error(
        "Passing secrets via --token CLI flags is prohibited; provide HOMEBREW_TAP_DEPLOY_KEY, SCOOP_BUCKET_DEPLOY_KEY, WINGET_DEPLOY_KEY, RELEASE_BOT_TOKEN, or GITEA_TOKEN via environment variables.",
      );
    }
    if (arg === "--dry-run") dryRun = true;
    else if (arg === "--package" && args[i + 1]) packageName = args[++i]!;
    else if (arg === "--version" && args[i + 1]) version = args[++i]!;
    else if (arg === "--tag" && args[i + 1]) tag = args[++i]!;
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
  if (!options.token && !options.dryRun && GITEA_TOKEN && GITEA_TOKEN !== "token-placeholder") {
    options.token = GITEA_TOKEN;
  }
  const res = await dispatchDistribution(options);
  console.log("\nDistribution Dispatch Summary:");
  console.log(`- Homebrew: ${res.homebrew ? "OK" : "Skipped/Failed"}`);
  console.log(`- Scoop:    ${res.scoop ? "OK" : "Skipped/Failed"}`);
  console.log(`- WinGet:   ${res.winget ? "OK" : "Skipped/Failed"}`);
  if (!options.dryRun && (!res.homebrew || !res.scoop || !res.winget)) {
    throw new Error("One or more package manager distribution channels failed to update.");
  }
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal dispatch error:", err);
    process.exit(1);
  });
}
