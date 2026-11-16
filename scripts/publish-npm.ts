#!/usr/bin/env bun
/**
 * Universal NPM & OIDC Trusted Publisher for CDDM
 *
 * Packages and publishes @cddm/<platform>-<arch> binary packages and the main `cddm`
 * CLI/MCP/LSP wrapper package to:
 * - Gitea Private NPM Package Registry (`--registry gitea`)
 * - Public registry.npmjs.org via token-less OIDC Trusted Publishing (`--registry npmjs --provenance`)
 *
 * Strictly forbids passing tokens via `--token` CLI flags (process-table leak prevention).
 */

import {
  chmodSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import {
  buildGiteaNpmRegistryUrl,
  buildPlatformNpmManifest,
  isNpmVersionPublished,
  NPMJS_REGISTRY_URL,
  normalizeNpmRegistryName,
  SUPPORTED_NPM_TARGETS,
} from "./lib/npm-registry";
import { resolvePublisherConfig } from "./lib/publisher-config";
import { maskSecret } from "./publish-release";
import { getCurrentVersion } from "./version";

export interface PublishNpmOptions {
  registry: "gitea" | "npmjs";
  host: string;
  owner: string;
  repo: string;
  version: string;
  distDir: string;
  workspaceRoot: string;
  provenance: boolean;
  dryRun: boolean;
  token?: string;
  fetchFn?: typeof fetch;
}

export function parseArgs(
  args: string[],
  customEnv: Record<string, string | undefined> = process.env,
  workspaceRoot: string = process.cwd(),
): PublishNpmOptions {
  const pubConfig = resolvePublisherConfig(workspaceRoot, customEnv);
  const version = getCurrentVersion(workspaceRoot);
  let registry: "gitea" | "npmjs" = "gitea";
  let host = customEnv.GITEA_HOST || pubConfig.vcsHost;
  let owner = customEnv.NPM_REGISTRY_OWNER || pubConfig.publisherId;
  const repo = customEnv.GITEA_REPO || pubConfig.vcsRepo;
  let distDir = "dist-release";
  let provenance = false;
  let dryRun = false;

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (arg === "--token" || arg?.startsWith("--token=")) {
      throw new Error(
        "Passing secrets via --token CLI flags is prohibited; provide NPM_TOKEN, RELEASE_BOT_TOKEN, or GITEA_TOKEN via environment variables or use OIDC Trusted Publishing.",
      );
    }
    if (arg === "--dry-run") dryRun = true;
    else if (arg === "--provenance") provenance = true;
    else if (arg === "--registry" && args[i + 1]) {
      registry = normalizeNpmRegistryName(args[++i]!);
    } else if (arg === "--host" && args[i + 1]) host = args[++i]!;
    else if (arg === "--owner" && args[i + 1]) owner = args[++i]!;
    else if (arg === "--dist-dir" && args[i + 1]) distDir = args[++i]!;
  }

  const token =
    customEnv.NPM_TOKEN ||
    customEnv.NODE_AUTH_TOKEN ||
    customEnv.RELEASE_BOT_TOKEN ||
    customEnv.REGISTRY_TOKEN ||
    customEnv.GITEA_TOKEN ||
    customEnv.GITHUB_TOKEN;

  return {
    registry,
    host,
    owner,
    repo,
    version,
    distDir,
    workspaceRoot,
    provenance,
    dryRun,
    token,
  };
}

function extractArchiveToDir(archivePath: string, destDir: string): void {
  mkdirSync(destDir, { recursive: true });
  const proc = archivePath.endsWith(".zip")
    ? Bun.spawnSync(["unzip", "-qo", archivePath, "-d", destDir], {
        stdout: "pipe",
        stderr: "pipe",
      })
    : Bun.spawnSync(["tar", "-xzf", archivePath, "-C", destDir], {
        stdout: "pipe",
        stderr: "pipe",
      });

  if (proc.exitCode !== 0) {
    throw new Error(`Failed to extract ${archivePath}: ${proc.stderr.toString()}`);
  }
}

function publishDirectoryWithNpm(
  pkgDir: string,
  registryUrl: string,
  options: PublishNpmOptions,
): void {
  const workDir = mkdtempSync(join(tmpdir(), "cddm-npmrc-"));
  const npmrcPath = join(workDir, ".npmrc");
  const parsedUrl = new URL(registryUrl);
  const authPath = `${parsedUrl.host}${parsedUrl.pathname.replace(/\/+$/, "")}/`;

  const lines = [`registry=${registryUrl}`];
  if (options.token) {
    lines.push(`//${authPath}:_authToken=${options.token}`);
  }
  writeFileSync(npmrcPath, `${lines.join("\n")}\n`, { mode: 0o600, encoding: "utf-8" });

  try {
    const cmd = ["npm", "publish", "--userconfig", npmrcPath, "--registry", registryUrl];
    if (options.registry === "npmjs") {
      cmd.push("--access", "public");
      if (options.provenance) {
        cmd.push("--provenance");
      }
    }

    const proc = Bun.spawnSync(cmd, {
      cwd: pkgDir,
      env: {
        ...process.env,
        NODE_AUTH_TOKEN: options.token ?? process.env.NODE_AUTH_TOKEN ?? "",
      },
      stdout: "pipe",
      stderr: "pipe",
    });

    if (proc.exitCode !== 0) {
      const stderr = proc.stderr.toString();
      if (
        stderr.includes("EPUBLISHCONFLICT") ||
        stderr.includes("409") ||
        stderr.includes("already exists")
      ) {
        console.log(`\x1b[32m[SKIP] Package in ${pkgDir} is already published.\x1b[0m`);
        return;
      }
      throw new Error(`npm publish failed for ${pkgDir}:\n${stderr}`);
    }
  } finally {
    rmSync(workDir, { recursive: true, force: true });
  }
}

export async function runPublishNpmPipeline(options: PublishNpmOptions): Promise<string[]> {
  const registryUrl =
    options.registry === "npmjs"
      ? NPMJS_REGISTRY_URL
      : buildGiteaNpmRegistryUrl(options.host, options.owner);

  console.log(`\x1b[36m--> Initializing CDDM NPM Publisher (${options.registry})...\x1b[0m`);
  console.log(`Registry URL:  ${registryUrl}`);
  console.log(`Version:       ${options.version}`);
  console.log(`Provenance:    ${options.provenance ? "enabled (OIDC)" : "disabled"}`);
  if (options.token) {
    console.log(`Auth Token:    ${maskSecret(options.token)}`);
  }

  const published: string[] = [];
  const fullDistDir = resolve(options.workspaceRoot, options.distDir);
  const homepage = `https://${options.host}/${options.repo}`;
  const repoGitUrl = `https://${options.host}/${options.repo}.git`;

  for (const target of SUPPORTED_NPM_TARGETS) {
    const archiveName = `cddm-v${options.version}-${target.rustTarget}${target.archiveExt}`;
    const archivePath = join(fullDistDir, archiveName);
    if (!existsSync(archivePath)) {
      continue;
    }

    const alreadyLive = await isNpmVersionPublished({
      registry: options.registry,
      host: options.host,
      owner: options.owner,
      packageName: target.packageName,
      version: options.version,
      token: options.token,
      fetchFn: options.fetchFn,
    });
    if (alreadyLive) {
      console.log(
        `\x1b[32m[SKIP] ${target.packageName}@${options.version} already published\x1b[0m`,
      );
      published.push(target.packageName);
      continue;
    }

    if (options.dryRun) {
      console.log(
        `\x1b[33m[DRY-RUN] Would package & publish ${target.packageName}@${options.version} -> ${registryUrl}\x1b[0m`,
      );
      published.push(target.packageName);
      continue;
    }

    const stagingDir = mkdtempSync(join(tmpdir(), "cddm-npm-pkg-"));
    try {
      extractArchiveToDir(archivePath, stagingDir);
      for (const bin of target.binaries) {
        const binPath = join(stagingDir, bin);
        if (existsSync(binPath) && !bin.endsWith(".exe")) {
          chmodSync(binPath, 0o755);
        }
      }
      writeFileSync(
        join(stagingDir, "package.json"),
        buildPlatformNpmManifest({
          packageName: target.packageName,
          version: options.version,
          os: target.os,
          cpu: target.cpu,
          binaries: target.binaries,
          homepage,
          repositoryUrl: repoGitUrl,
        }),
        "utf-8",
      );
      publishDirectoryWithNpm(stagingDir, registryUrl, options);
      console.log(`\x1b[32m[OK] Published ${target.packageName}@${options.version}\x1b[0m`);
      published.push(target.packageName);
    } finally {
      rmSync(stagingDir, { recursive: true, force: true });
    }
  }

  const wrapperDir = join(options.workspaceRoot, "npm", "cddm");
  if (existsSync(join(wrapperDir, "package.json"))) {
    const mainLive = await isNpmVersionPublished({
      registry: options.registry,
      host: options.host,
      owner: options.owner,
      packageName: "cddm",
      version: options.version,
      token: options.token,
      fetchFn: options.fetchFn,
    });

    if (mainLive) {
      console.log(`\x1b[32m[SKIP] cddm@${options.version} already published\x1b[0m`);
      published.push("cddm");
    } else if (options.dryRun) {
      console.log(
        `\x1b[33m[DRY-RUN] Would publish wrapper package cddm@${options.version} -> ${registryUrl}\x1b[0m`,
      );
      published.push("cddm");
    } else {
      const pkgRaw = readFileSync(join(wrapperDir, "package.json"), "utf-8");
      const stagingWrapper = mkdtempSync(join(tmpdir(), "cddm-npm-main-"));
      try {
        const pkgJson = JSON.parse(pkgRaw) as Record<string, unknown>;
        if (options.registry === "gitea") {
          pkgJson.publishConfig = { registry: registryUrl };
        }
        mkdirSync(join(stagingWrapper, "bin"), { recursive: true });
        mkdirSync(join(stagingWrapper, "lib"), { recursive: true });
        for (const f of ["cddm.js", "cddm-mcp.js", "cddm-lsp.js"]) {
          const src = join(wrapperDir, "bin", f);
          if (existsSync(src)) {
            writeFileSync(join(stagingWrapper, "bin", f), readFileSync(src));
          }
        }
        const libSrc = join(wrapperDir, "lib", "run-binary.js");
        if (existsSync(libSrc)) {
          writeFileSync(join(stagingWrapper, "lib", "run-binary.js"), readFileSync(libSrc));
        }
        writeFileSync(
          join(stagingWrapper, "package.json"),
          `${JSON.stringify(pkgJson, null, 2)}\n`,
          "utf-8",
        );
        publishDirectoryWithNpm(stagingWrapper, registryUrl, options);
        console.log(`\x1b[32m[OK] Published cddm@${options.version}\x1b[0m`);
        published.push("cddm");
      } finally {
        rmSync(stagingWrapper, { recursive: true, force: true });
      }
    }
  }

  return published;
}

async function main() {
  const options = parseArgs(process.argv.slice(2));
  await runPublishNpmPipeline(options);
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("\x1b[31mFatal NPM publishing error:\x1b[0m", err);
    process.exit(1);
  });
}
