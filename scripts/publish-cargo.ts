#!/usr/bin/env bun
/**
 * CDDM Private Gitea Cargo Registry Publisher
 *
 * Publishes all CDDM workspace crates in strict dependency order
 * (cddm-core -> cddm-lsp -> cddm-mcp -> cddm-cli) to the authoritative
 * Gitea Cargo sparse registry (or custom Cargo registry).
 */

import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { resolvePublisherConfig } from "./lib/publisher-config";
import { getAuthHeader, maskSecret } from "./publish-release";
import { getCurrentVersion } from "./version";

export const WORKSPACE_CRATES = ["cddm-core", "cddm-lsp", "cddm-mcp", "cddm-cli"] as const;
export type WorkspaceCrate = (typeof WORKSPACE_CRATES)[number];

export interface CargoPublishOptions {
  host: string;
  owner: string;
  registry: string;
  version: string;
  token?: string;
  workspaceRoot: string;
  crates: WorkspaceCrate[];
  dryRun: boolean;
  noVerify: boolean;
  pollAttempts: number;
  pollIntervalMs: number;
}

export interface CargoPublishResult {
  crateName: WorkspaceCrate;
  status: "published" | "skipped" | "dry-run";
}

export function buildCargoRegistryIndexUrl(host: string, owner: string): string {
  return `sparse+https://${host}/api/packages/${owner}/cargo/`;
}

/**
 * Compute Cargo sparse index relative path for a crate name per Cargo's index layout.
 */
export function getSparseIndexCratePath(crateName: string): string {
  const lower = crateName.toLowerCase();
  if (lower.length === 1) return `1/${lower}`;
  if (lower.length === 2) return `2/${lower}`;
  if (lower.length === 3) return `3/${lower.slice(0, 1)}/${lower}`;
  return `${lower.slice(0, 2)}/${lower.slice(2, 4)}/${lower}`;
}

export function parseSparseIndexVersions(ndjson: string): Set<string> {
  const versions = new Set<string>();
  for (const line of ndjson.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    try {
      const parsed = JSON.parse(trimmed) as { vers?: unknown };
      if (typeof parsed.vers === "string") {
        versions.add(parsed.vers);
      }
    } catch {
      // Ignore malformed index lines
    }
  }
  return versions;
}

export function formatCargoAuthToken(token: string): string {
  const trimmed = token.trim();
  if (trimmed.startsWith("Bearer ") || trimmed.startsWith("token ")) {
    return trimmed;
  }
  return `Bearer ${trimmed}`;
}

/**
 * Temporarily attach `registry = "<name>"` to internal workspace dependencies
 * in root Cargo.toml so alternative-registry publishing and `cargo install --registry <name>`
 * resolve internal crates from the same private registry rather than crates.io.
 */
export function injectWorkspaceDependencyRegistry(
  cargoTomlContent: string,
  registryName: string,
): string {
  return cargoTomlContent.replace(
    /^((?:cddm-(?:core|lsp|mcp))\s*=\s*\{[^}\r\n]+)\}/gm,
    (fullMatch, inner: string) => {
      if (/\bregistry\s*=/.test(inner)) {
        return fullMatch;
      }
      return `${inner.trimEnd()}, registry = "${registryName}" }`;
    },
  );
}

export function buildCargoPublishEnv(
  options: Pick<CargoPublishOptions, "host" | "owner" | "registry" | "token">,
  baseEnv: Record<string, string | undefined> = process.env,
): Record<string, string> {
  const env: Record<string, string> = {};
  for (const [key, value] of Object.entries(baseEnv)) {
    if (value !== undefined) {
      env[key] = value;
    }
  }
  const envReg = options.registry.toUpperCase().replace(/[^A-Z0-9]/g, "_");
  env[`CARGO_REGISTRIES_${envReg}_INDEX`] = buildCargoRegistryIndexUrl(options.host, options.owner);
  if (options.token) {
    env[`CARGO_REGISTRIES_${envReg}_TOKEN`] = formatCargoAuthToken(options.token);
  }
  return env;
}

export async function isCrateVersionPublished(
  host: string,
  owner: string,
  crateName: string,
  version: string,
  token?: string,
): Promise<boolean> {
  const indexPath = getSparseIndexCratePath(crateName);
  const url = `https://${host}/api/packages/${owner}/cargo/${indexPath}`;
  const headers: Record<string, string> = { Accept: "text/plain, application/json" };
  if (token) {
    headers.Authorization = getAuthHeader(token);
  }

  try {
    const res = await fetch(url, { headers });
    if (!res.ok) return false;
    const body = await res.text();
    return parseSparseIndexVersions(body).has(version);
  } catch {
    return false;
  }
}

async function waitForSparseIndexAvailability(
  options: CargoPublishOptions,
  crateName: WorkspaceCrate,
): Promise<void> {
  for (let attempt = 1; attempt <= options.pollAttempts; attempt++) {
    const found = await isCrateVersionPublished(
      options.host,
      options.owner,
      crateName,
      options.version,
      options.token,
    );
    if (found) return;
    if (attempt < options.pollAttempts) {
      await Bun.sleep(options.pollIntervalMs);
    }
  }
}

export function parseArgs(
  args: string[],
  workspaceRoot: string = process.cwd(),
): CargoPublishOptions {
  const pubConfig = resolvePublisherConfig(workspaceRoot);
  let host = process.env.GITEA_HOST || pubConfig.vcsHost;
  let owner = process.env.CARGO_REGISTRY_OWNER || pubConfig.publisherId;
  let registry = process.env.CARGO_REGISTRY_NAME || "gitea";
  let version = getCurrentVersion(workspaceRoot);
  let token =
    process.env.CARGO_REGISTRY_TOKEN ||
    process.env.REGISTRY_TOKEN ||
    process.env.GITEA_TOKEN ||
    process.env.GITHUB_TOKEN;
  let dryRun = false;
  let noVerify = false;
  const selectedCrates: WorkspaceCrate[] = [];

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (arg === "--dry-run") dryRun = true;
    else if (arg === "--no-verify") noVerify = true;
    else if (arg === "--host" && args[i + 1]) host = args[++i]!;
    else if (arg === "--owner" && args[i + 1]) owner = args[++i]!;
    else if (arg === "--registry" && args[i + 1]) registry = args[++i]!;
    else if (arg === "--version" && args[i + 1]) version = args[++i]!;
    else if (arg === "--token" && args[i + 1]) token = args[++i]!;
    else if (arg === "--workspace" && args[i + 1]) workspaceRoot = resolve(args[++i]!);
    else if (arg === "--crate" && args[i + 1]) {
      const candidate = args[++i]! as WorkspaceCrate;
      if (!WORKSPACE_CRATES.includes(candidate)) {
        throw new Error(`Unknown workspace crate: ${candidate}`);
      }
      selectedCrates.push(candidate);
    }
  }

  return {
    host,
    owner,
    registry,
    version,
    token,
    workspaceRoot,
    crates: selectedCrates.length > 0 ? selectedCrates : [...WORKSPACE_CRATES],
    dryRun,
    noVerify,
    pollAttempts: 15,
    pollIntervalMs: 1000,
  };
}

export async function runCargoPublishPipeline(
  options: CargoPublishOptions,
): Promise<CargoPublishResult[]> {
  const indexUrl = buildCargoRegistryIndexUrl(options.host, options.owner);
  console.log(`\x1b[36m--> Initializing CDDM Cargo Registry Publisher...\x1b[0m`);
  console.log(`Registry Name:  ${options.registry}`);
  console.log(`Sparse Index:   ${indexUrl}`);
  console.log(`Crate Version:  ${options.version}`);
  console.log(`Crates Queue:   ${options.crates.join(" -> ")}`);
  if (options.token) {
    console.log(`Auth Token:     ${maskSecret(options.token)}`);
  }

  const cargoTomlPath = join(options.workspaceRoot, "Cargo.toml");
  if (!existsSync(cargoTomlPath)) {
    throw new Error(`Cargo.toml not found at ${cargoTomlPath}`);
  }

  const results: CargoPublishResult[] = [];
  if (options.dryRun) {
    for (const crateName of options.crates) {
      console.log(
        `\x1b[33m[DRY-RUN] Would publish ${crateName} v${options.version} -> ${indexUrl}\x1b[0m`,
      );
      results.push({ crateName, status: "dry-run" });
    }
    return results;
  }

  if (!options.token) {
    throw new Error(
      "Missing Cargo registry token. Set CARGO_REGISTRY_TOKEN, REGISTRY_TOKEN, or GITEA_TOKEN.",
    );
  }

  const originalCargoToml = readFileSync(cargoTomlPath, "utf-8");
  const patchedCargoToml = injectWorkspaceDependencyRegistry(originalCargoToml, options.registry);
  const env = buildCargoPublishEnv(options);

  try {
    writeFileSync(cargoTomlPath, patchedCargoToml, "utf-8");

    for (let idx = 0; idx < options.crates.length; idx++) {
      const crateName = options.crates[idx]!;
      const alreadyPublished = await isCrateVersionPublished(
        options.host,
        options.owner,
        crateName,
        options.version,
        options.token,
      );
      if (alreadyPublished) {
        console.log(
          `\x1b[33m[SKIP] ${crateName} v${options.version} already exists in ${options.registry}\x1b[0m`,
        );
        results.push({ crateName, status: "skipped" });
        continue;
      }

      const cmd = [
        "cargo",
        "publish",
        "-p",
        crateName,
        "--registry",
        options.registry,
        "--allow-dirty",
      ];
      if (options.noVerify) {
        cmd.push("--no-verify");
      }

      console.log(`\x1b[36m--> Publishing ${crateName} v${options.version}...\x1b[0m`);
      const proc = Bun.spawnSync(cmd, {
        cwd: options.workspaceRoot,
        env,
        stdout: "inherit",
        stderr: "inherit",
      });

      if (proc.exitCode !== 0) {
        throw new Error(
          `cargo publish failed for ${crateName} v${options.version} (exit code ${proc.exitCode})`,
        );
      }

      console.log(`\x1b[32m[OK] Published ${crateName} v${options.version}\x1b[0m`);
      results.push({ crateName, status: "published" });

      if (idx < options.crates.length - 1) {
        await waitForSparseIndexAvailability(options, crateName);
      }
    }
  } finally {
    writeFileSync(cargoTomlPath, originalCargoToml, "utf-8");
  }

  console.log(`\x1b[32m[SUCCESS] Cargo registry publishing completed.\x1b[0m`);
  return results;
}

async function main(): Promise<void> {
  const options = parseArgs(process.argv.slice(2));
  await runCargoPublishPipeline(options);
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("\x1b[31mFatal Cargo publishing error:\x1b[0m", err);
    process.exit(1);
  });
}
