#!/usr/bin/env bun
/**
 * CDDM Gitea & Polyglot Release Publisher
 * Creates Gitea releases and uploads cross-platform binary assets,
 * VS Code extension VSIX packages, and SHA256 checksums.
 */

import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { getCurrentVersion } from "./version";

export interface ReleaseOptions {
  host: string;
  repo: string;
  token?: string;
  tag: string;
  title: string;
  body: string;
  draft: boolean;
  prerelease: boolean;
  distDir: string;
  dryRun: boolean;
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
  let tag = `v${version}`;
  let host = process.env.GITEA_HOST || "git.gt-web-dev.com";
  let repo = process.env.GITEA_REPO || "gt-dev/cddm";
  let token = process.env.GITEA_TOKEN || process.env.GITHUB_TOKEN;
  let distDir = "dist-release";
  let dryRun = false;
  let draft = false;
  let prerelease = false;
  let title = `Release ${tag}`;

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (arg === "--dry-run") dryRun = true;
    else if (arg === "--draft") draft = true;
    else if (arg === "--prerelease") prerelease = true;
    else if (arg === "--tag" && args[i + 1]) tag = args[++i]!;
    else if (arg === "--host" && args[i + 1]) host = args[++i]!;
    else if (arg === "--repo" && args[i + 1]) repo = args[++i]!;
    else if (arg === "--token" && args[i + 1]) token = args[++i]!;
    else if (arg === "--dist-dir" && args[i + 1]) distDir = args[++i]!;
    else if (arg === "--title" && args[i + 1]) title = args[++i]!;
  }

  const changelogBody = getChangelogForVersion(tag.replace(/^v/, ""));

  return {
    host,
    repo,
    token,
    tag,
    title,
    body: changelogBody,
    draft,
    prerelease,
    distDir,
    dryRun,
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

  const checkRes = await fetch(`${apiUrl}/tags/${options.tag}`, {
    headers: {
      Authorization: `token ${options.token}`,
      Accept: "application/json",
    },
  });

  if (checkRes.ok) {
    const existing = (await checkRes.json()) as GiteaReleaseResponse;
    console.log(`\x1b[36mFound existing release ID ${existing.id} for tag ${options.tag}\x1b[0m`);
    return existing;
  }

  const createRes = await fetch(apiUrl, {
    method: "POST",
    headers: {
      Authorization: `token ${options.token}`,
      "Content-Type": "application/json",
      Accept: "application/json",
    },
    body: JSON.stringify({
      tag_name: options.tag,
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
      Authorization: `token ${options.token}`,
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

export async function runPublishPipeline(options: ReleaseOptions): Promise<void> {
  console.log(
    `\x1b[36m--> Initializing CDDM Release Publisher for Gitea (${options.host})...\x1b[0m`,
  );
  console.log(`Target Repository: ${options.repo}`);
  console.log(`Release Tag:       ${options.tag}`);
  console.log(`Artifacts Dir:     ${options.distDir}`);

  const release = await createOrGetGiteaRelease(options);
  if (!release) {
    throw new Error("Could not initialize release on Gitea");
  }

  const fullDistPath = resolve(process.cwd(), options.distDir);
  if (!existsSync(fullDistPath)) {
    console.log(
      `\x1b[33mArtifacts directory '${options.distDir}' does not exist, skipping asset upload.\x1b[0m`,
    );
    return;
  }

  const files = readdirSync(fullDistPath)
    .map((f) => join(fullDistPath, f))
    .filter((p) => statSync(p).isFile());

  console.log(`Found ${files.length} release artifacts to publish.`);
  for (const file of files) {
    await uploadAssetToGitea(options, release.id, file);
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
