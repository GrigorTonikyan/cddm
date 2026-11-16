#!/usr/bin/env bun
/**
 * CDDM Gitea & Polyglot Release Publisher
 * Creates Gitea releases and uploads cross-platform binary assets,
 * VS Code extension VSIX packages, and SHA256 checksums.
 */

import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { dispatchDistribution } from "./dispatch-distribution";
import { getVcsAuthHeader, verifyCargoChannels } from "./lib/cargo-registry";
import { parseSha256SumsValidated } from "./lib/distribution-manifests";
import { ensureSignedReleaseTag } from "./lib/gitea-git-transport";
import { verifyNpmChannels } from "./lib/npm-registry";
import { resolvePublisherConfig } from "./lib/publisher-config";
import { generateReleaseNotes } from "./lib/release-notes-engine";
import { syncManifestsFromChecksums } from "./package-distribution";
import { getCurrentVersion } from "./version";

export { ensureSignedReleaseTag };
export const getAuthHeader = getVcsAuthHeader;

/**
 * Allowed binary & release artifact file extensions for Gitea release downloads.
 * Package manager recipes (.rb, .json, .yaml, .yml) are strictly excluded.
 */
export const FORBIDDEN_RELEASE_EXTENSIONS = [".rb", ".json", ".yaml", ".yml", ".sh", ".ps1"];
export const ALLOWED_RELEASE_EXTENSIONS = [".tar.gz", ".zip", ".vsix", ".txt", ".tar.xz", ".sig"];

export function isReleaseAsset(fileName: string): boolean {
  const lower = fileName.toLowerCase();
  for (const forbidden of FORBIDDEN_RELEASE_EXTENSIONS) {
    if (lower.endsWith(forbidden)) return false;
  }
  for (const allowed of ALLOWED_RELEASE_EXTENSIONS) {
    if (lower.endsWith(allowed)) return true;
  }
  return false;
}

export function maskSecret(secret?: string): string {
  if (!secret || secret.length < 8) return "***";
  return `${secret.slice(0, 3)}...${secret.slice(-3)}`;
}

/**
 * Parse SHA256SUMS.txt content into a map of filename -> sha256 hex digest.
 */
export function parseSha256Sums(content: string): Map<string, string> {
  return parseSha256SumsValidated(content);
}

/**
 * Dynamically renders target-pruned packaging/ manifests from SHA256SUMS.txt
 * (Homebrew, Scoop, and WinGet v1.9.0 multi-file manifests).
 * Manifests are kept strictly in packaging/ and NEVER placed into distDir.
 */
export function injectPackagingChecksums(
  tag: string,
  distDir: string,
  workspaceRoot: string = process.cwd(),
): string[] {
  const sumsPath = join(distDir, "SHA256SUMS.txt");
  if (!existsSync(sumsPath)) return [];

  const cleanVer = tag.replace(/^v/, "");
  return syncManifestsFromChecksums(workspaceRoot, sumsPath, cleanVer);
}

export interface ReleaseOptions {
  host: string;
  repo: string;
  token?: string;
  tag: string;
  target?: string;
  title: string;
  body: string;
  draft: boolean;
  prerelease: boolean;
  distDir: string;
  dryRun: boolean;
  deleteAssetId?: number;
  dispatchPackages?: boolean;
  writeNotesPath?: string;
}

export interface GiteaReleaseResponse {
  id: number;
  tag_name: string;
  name: string;
  body: string;
  draft: boolean;
  prerelease: boolean;
  assets?: Array<{ id: number; name: string; download_url: string }>;
}

export function parseArgs(args: string[]): ReleaseOptions {
  const version = getCurrentVersion();
  const pubConfig = resolvePublisherConfig();
  let tag = `v${version}`;
  let host = process.env.GITEA_HOST || pubConfig.vcsHost;
  let repo = process.env.GITEA_REPO || pubConfig.vcsRepo;
  const token =
    process.env.RELEASE_BOT_TOKEN || process.env.GITEA_TOKEN || process.env.GITHUB_TOKEN;
  let distDir = "dist-release";
  let dryRun = false;
  let draft = false;
  let prerelease = false;
  let title = `Release ${tag}`;
  let target = "main";
  let deleteAssetId: number | undefined;
  let dispatchPackages = false;
  let writeNotesPath: string | undefined;

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (arg === "--token" || arg?.startsWith("--token=")) {
      throw new Error(
        "Passing secrets via --token CLI flags is prohibited; provide RELEASE_BOT_TOKEN or GITEA_TOKEN via environment variables.",
      );
    }
    if (arg === "--dry-run") dryRun = true;
    else if (arg === "--draft") draft = true;
    else if (arg === "--prerelease") prerelease = true;
    else if (arg === "--dispatch-packages") dispatchPackages = true;
    else if (arg === "--tag" && args[i + 1]) tag = args[++i]!;
    else if (arg === "--target" && args[i + 1]) target = args[++i]!;
    else if (arg === "--host" && args[i + 1]) host = args[++i]!;
    else if (arg === "--repo" && args[i + 1]) repo = args[++i]!;
    else if (arg === "--dist-dir" && args[i + 1]) distDir = args[++i]!;
    else if (arg === "--title" && args[i + 1]) title = args[++i]!;
    else if (arg === "--write-notes" && args[i + 1]) writeNotesPath = args[++i]!;
    else if (arg === "--delete-asset" && args[i + 1])
      deleteAssetId = Number.parseInt(args[++i]!, 10);
  }

  const releaseBody = generateReleaseNotes(tag.replace(/^v/, ""), {
    milestoneTitle: title,
  });

  return {
    host,
    repo,
    token,
    tag,
    target,
    title,
    body: releaseBody,
    draft,
    prerelease,
    distDir,
    dryRun,
    deleteAssetId,
    dispatchPackages,
    writeNotesPath,
  };
}

export function getChangelogForVersion(version: string, rootDir = process.cwd()): string {
  const changelogPath = join(rootDir, "CHANGELOG.md");
  if (!existsSync(changelogPath)) {
    return `CDDM Automated Release v${version}`;
  }

  const content = readFileSync(changelogPath, "utf-8");
  const targetHeader = `## [${version}]`;
  const startIndex = content.indexOf(targetHeader);

  if (startIndex === -1) {
    return `CDDM Automated Release v${version}\n\nFor full details, see the changelog.`;
  }

  const nextSectionIndex = content.indexOf("\n## [", startIndex + targetHeader.length);
  if (nextSectionIndex === -1) {
    return content.slice(startIndex).trim();
  }

  return content.slice(startIndex, nextSectionIndex).trim();
}

export async function createOrGetGiteaRelease(
  options: ReleaseOptions,
): Promise<GiteaReleaseResponse | null> {
  const apiUrl = `https://${options.host}/api/v1/repos/${options.repo}/releases`;

  if (options.dryRun || !options.token) {
    console.log(`\x1b[33m[DRY-RUN] Would create release at: ${apiUrl}\x1b[0m`);
    console.log(`  Tag: ${options.tag}`);
    console.log(`  Title: ${options.title}`);
    console.log(`  Draft: ${options.draft}`);
    return {
      id: 99999,
      tag_name: options.tag,
      name: options.title,
      body: options.body,
      draft: options.draft,
      prerelease: options.prerelease,
    };
  }

  const authHeader = getVcsAuthHeader(options.token);
  const checkRes = await fetch(`${apiUrl}/tags/${options.tag}`, {
    headers: {
      Authorization: authHeader,
      Accept: "application/json",
    },
  });

  if (checkRes.ok) {
    const existing = (await checkRes.json()) as GiteaReleaseResponse;
    console.log(`\x1b[36mFound existing release ID ${existing.id} for tag ${options.tag}\x1b[0m`);

    // Ensure release body meets the rich release notes standard and reflects live channels
    if (
      !existing.body ||
      existing.body !== options.body ||
      existing.body.includes("See CHANGELOG.md") ||
      !existing.body.includes("Model Context Protocol")
    ) {
      console.log(
        `--> Updating release #${existing.id} notes on Gitea with rich release notes and verified channels...`,
      );
      const updateRes = await fetch(`${apiUrl}/${existing.id}`, {
        method: "PATCH",
        headers: {
          Authorization: authHeader,
          "Content-Type": "application/json",
          Accept: "application/json",
        },
        body: JSON.stringify({
          name: existing.name || options.title,
          body: options.body,
        }),
      });
      if (updateRes.ok) {
        existing.body = options.body;
      }
    }
    return existing;
  }

  const createRes = await fetch(apiUrl, {
    method: "POST",
    headers: {
      Authorization: authHeader,
      "Content-Type": "application/json",
      Accept: "application/json",
    },
    body: JSON.stringify({
      tag_name: options.tag,
      target_commitish: options.target || "main",
      name: options.title,
      body: options.body,
      draft: options.draft,
      prerelease: options.prerelease,
    }),
  });

  if (!createRes.ok) {
    const errText = await createRes.text();
    throw new Error(`Failed to create release on Gitea (${createRes.status}): ${errText}`);
  }

  return (await createRes.json()) as GiteaReleaseResponse;
}

export async function uploadAssetToGitea(
  options: ReleaseOptions,
  releaseId: number,
  filePath: string,
): Promise<boolean> {
  const fileName = basename(filePath);
  const uploadUrl = `https://${options.host}/api/v1/repos/${options.repo}/releases/${releaseId}/assets?name=${encodeURIComponent(fileName)}`;

  if (options.dryRun || !options.token) {
    console.log(`\x1b[33m[DRY-RUN] Would upload asset: ${fileName} -> ${uploadUrl}\x1b[0m`);
    return true;
  }

  const fileData = readFileSync(filePath);
  const formData = new FormData();
  const blob = new Blob([fileData], { type: "application/octet-stream" });
  formData.append("attachment", blob, fileName);

  const res = await fetch(uploadUrl, {
    method: "POST",
    headers: {
      Authorization: getAuthHeader(options.token),
    },
    body: formData,
  });

  if (!res.ok) {
    const errText = await res.text();
    console.error(`\x1b[31mFailed to upload asset ${fileName} (${res.status}): ${errText}\x1b[0m`);
    return false;
  }

  console.log(`\x1b[32m[OK] Uploaded release asset: ${fileName}\x1b[0m`);
  return true;
}

export async function deleteReleaseAsset(
  options: ReleaseOptions,
  releaseId: number,
  assetId: number,
): Promise<boolean> {
  const deleteUrl = `https://${options.host}/api/v1/repos/${options.repo}/releases/${releaseId}/assets/${assetId}`;

  if (options.dryRun || !options.token) {
    console.log(`\x1b[33m[DRY-RUN] Would delete asset ${assetId} from release ${releaseId}\x1b[0m`);
    return true;
  }

  const res = await fetch(deleteUrl, {
    method: "DELETE",
    headers: {
      Authorization: getAuthHeader(options.token),
      Accept: "application/json",
    },
  });

  if (!res.ok) {
    const errText = await res.text();
    console.error(`\x1b[31mFailed to delete asset ${assetId} (${res.status}): ${errText}\x1b[0m`);
    return false;
  }

  console.log(`\x1b[32m[OK] Deleted release asset ${assetId}\x1b[0m`);
  return true;
}

export async function runPublishPipeline(options: ReleaseOptions): Promise<void> {
  console.log(
    `\x1b[36m--> Initializing CDDM Release Publisher for Gitea (${options.host})...\x1b[0m`,
  );
  console.log(`Target Repository: ${options.repo}`);
  console.log(`Release Tag:       ${options.tag}`);
  console.log(`Artifacts Dir:     ${options.distDir}`);

  if (!options.writeNotesPath && !options.deleteAssetId) {
    ensureSignedReleaseTag(options.tag, options.target || "HEAD", {
      token: options.token,
      host: options.host,
      repo: options.repo,
      dryRun: options.dryRun,
    });
  }

  let effectiveOptions = options;
  if (!options.dryRun && !options.deleteAssetId) {
    const owner = options.repo.includes("/")
      ? options.repo.split("/")[0]!
      : resolvePublisherConfig().publisherId;
    const cleanVersion = options.tag.replace(/^v/, "");
    const [cargoChannels, npmChannels] = await Promise.all([
      verifyCargoChannels({
        host: options.host,
        owner,
        version: cleanVersion,
        token: options.token,
      }),
      verifyNpmChannels({
        host: options.host,
        owner,
        version: cleanVersion,
        token: options.token,
      }),
    ]);
    console.log(
      `Cargo Channels:    gitea=${cargoChannels.gitea ? "live" : "unavailable"}, crates.io=${cargoChannels.cratesIo ? "live" : "unavailable"}`,
    );
    console.log(
      `NPM Channels:      gitea=${npmChannels.gitea ? "live" : "unavailable"}, npmjs=${npmChannels.npmjs ? "live" : "unavailable"}`,
    );
    effectiveOptions = {
      ...options,
      body: generateReleaseNotes(cleanVersion, {
        milestoneTitle: options.title,
        cargoChannels,
        npmChannels,
      }),
    };
  }

  if (options.writeNotesPath) {
    const resolvedNotesPath = resolve(process.cwd(), options.writeNotesPath);
    mkdirSync(dirname(resolvedNotesPath), { recursive: true });
    writeFileSync(resolvedNotesPath, effectiveOptions.body, "utf-8");
    console.log(`\x1b[32m[OK] Wrote release notes -> ${resolvedNotesPath}\x1b[0m`);
    return;
  }

  const release = await createOrGetGiteaRelease(effectiveOptions);
  if (!release) {
    throw new Error("Could not initialize release on Gitea");
  }

  if (options.deleteAssetId) {
    await deleteReleaseAsset(options, release.id, options.deleteAssetId);
    return;
  }

  const fullDistPath = resolve(process.cwd(), options.distDir);
  if (!existsSync(fullDistPath)) {
    console.log(
      `\x1b[33mArtifacts directory '${options.distDir}' does not exist, skipping asset upload.\x1b[0m`,
    );
    return;
  }

  const injected = injectPackagingChecksums(options.tag, fullDistPath);
  if (injected.length > 0) {
    console.log(
      `\x1b[32m[OK] Injected SHA-256 checksums into ${injected.length} packaging manifest(s).\x1b[0m`,
    );
  }

  const files = readdirSync(fullDistPath)
    .map((f) => join(fullDistPath, f))
    .filter((p) => statSync(p).isFile() && isReleaseAsset(basename(p)));

  console.log(`Found ${files.length} release artifacts to publish.`);
  for (const file of files) {
    await uploadAssetToGitea(options, release.id, file);
  }

  if (options.dispatchPackages) {
    console.log("\n--> Dispatching package manifests to central ecosystem repositories...");
    await dispatchDistribution({
      packageName: "cddm",
      version: options.tag.replace(/^v/, ""),
      tag: options.tag,
      workspaceRoot: process.cwd(),
      distDir: options.distDir,
      token: options.token,
      dryRun: options.dryRun,
    });
  }

  console.log(`\x1b[32m[SUCCESS] Release ${options.tag} published successfully!\x1b[0m\n`);
}

async function main() {
  const options = parseArgs(process.argv.slice(2));
  await runPublishPipeline(options);
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("\x1b[31mFatal release publishing error:\x1b[0m", err);
    process.exit(1);
  });
}
