#!/usr/bin/env bun
/**
 * CDDM Workspace Cargo Registry & crates.io Publisher
 *
 * Publishes all CDDM workspace crates in strict dependency order
 * (cddm-core -> cddm-lsp -> cddm-mcp -> cddm-cli) to either the authoritative
 * Gitea Cargo sparse registry or public crates.io (including Trusted Publishing OIDC).
 */

import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import {
  buildCargoRegistryIndexUrl,
  formatCargoAuthToken,
  getSparseIndexBaseHttpUrl,
  getSparseIndexCratePath,
  isCrateVersionPublished,
  isCratesIoRegistry,
  normalizeCargoRegistryName,
  parseGiteaCargoIndexUrl,
  parseSparseIndexVersions,
  WORKSPACE_CRATES,
  type WorkspaceCrate,
} from "./lib/cargo-registry";
import { resolvePublisherConfig } from "./lib/publisher-config";
import { maskSecret } from "./publish-release";
import { getCurrentVersion } from "./version";

export {
  buildCargoRegistryIndexUrl,
  formatCargoAuthToken,
  getSparseIndexBaseHttpUrl,
  getSparseIndexCratePath,
  isCrateVersionPublished,
  isCratesIoRegistry,
  normalizeCargoRegistryName,
  parseSparseIndexVersions,
  WORKSPACE_CRATES,
  type WorkspaceCrate,
};

export interface CargoPublishOptions {
  host: string;
  owner: string;
  registry: string;
  version: string;
  indexUrl?: string;
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
  options: Pick<CargoPublishOptions, "host" | "owner" | "registry" | "token" | "indexUrl">,
  baseEnv: Record<string, string | undefined> = process.env,
): Record<string, string> {
  const env: Record<string, string> = {};
  for (const [key, value] of Object.entries(baseEnv)) {
    if (value !== undefined) {
      env[key] = value;
    }
  }

  if (isCratesIoRegistry(options.registry)) {
    if (options.token) {
      env.CARGO_REGISTRY_TOKEN = formatCargoAuthToken(options.token, options.registry);
    }
    return env;
  }

  const envReg = options.registry.toUpperCase().replace(/[^A-Z0-9]/g, "_");
  env[`CARGO_REGISTRIES_${envReg}_INDEX`] =
    options.indexUrl || buildCargoRegistryIndexUrl(options.host, options.owner);
  if (options.token) {
    env[`CARGO_REGISTRIES_${envReg}_TOKEN`] = formatCargoAuthToken(options.token, options.registry);
  }
  return env;
}

export function buildCargoPublishCommand(
  crateName: WorkspaceCrate,
  registry: string,
  noVerify: boolean,
): string[] {
  const cmd = ["cargo", "publish", "-p", crateName, "--allow-dirty"];
  if (!isCratesIoRegistry(registry)) {
    cmd.push("--registry", registry);
  }
  if (noVerify) {
    cmd.push("--no-verify");
  }
  return cmd;
}

async function waitForSparseIndexAvailability(
  options: CargoPublishOptions,
  crateName: WorkspaceCrate,
): Promise<boolean> {
  for (let attempt = 1; attempt <= options.pollAttempts; attempt++) {
    const found = await isCrateVersionPublished({
      registry: options.registry,
      host: options.host,
      owner: options.owner,
      crateName,
      version: options.version,
      token: options.token,
      indexUrl: options.indexUrl,
    });
    if (found) return true;
    if (attempt < options.pollAttempts) {
      await Bun.sleep(options.pollIntervalMs);
    }
  }
  return false;
}

export function parseArgs(
  args: string[],
  workspaceRoot: string = process.cwd(),
  customEnv: Record<string, string | undefined> = process.env,
): CargoPublishOptions {
  let registry = normalizeCargoRegistryName(customEnv.CARGO_REGISTRY_NAME || "gitea");
  let explicitHost: string | undefined;
  let explicitOwner: string | undefined;
  let explicitVersion: string | undefined;
  let dryRun = false;
  let noVerify = false;
  const selectedCrates: WorkspaceCrate[] = [];

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (arg === "--token" || arg?.startsWith("--token=")) {
      throw new Error(
        "Passing secrets via --token CLI flags is prohibited; provide CARGO_REGISTRY_TOKEN or RELEASE_BOT_TOKEN via environment variables.",
      );
    } else if (arg === "--dry-run") dryRun = true;
    else if (arg === "--no-verify") noVerify = true;
    else if (arg === "--host" && args[i + 1]) explicitHost = args[++i]!;
    else if (arg === "--owner" && args[i + 1]) explicitOwner = args[++i]!;
    else if (arg === "--registry" && args[i + 1]) registry = normalizeCargoRegistryName(args[++i]!);
    else if (arg === "--version" && args[i + 1]) explicitVersion = args[++i]!;
    else if (arg === "--workspace" && args[i + 1]) workspaceRoot = resolve(args[++i]!);
    else if (arg === "--crate" && args[i + 1]) {
      const candidate = args[++i]! as WorkspaceCrate;
      if (!WORKSPACE_CRATES.includes(candidate)) {
        throw new Error(`Unknown workspace crate: ${candidate}`);
      }
      selectedCrates.push(candidate);
    }
  }

  const pubConfig = resolvePublisherConfig(workspaceRoot, customEnv);
  const envReg = registry.toUpperCase().replace(/[^A-Z0-9]/g, "_");
  const preConfiguredIndex = customEnv[`CARGO_REGISTRIES_${envReg}_INDEX`];
  const parsedIndex = parseGiteaCargoIndexUrl(preConfiguredIndex);

  const host = isCratesIoRegistry(registry)
    ? "crates.io"
    : explicitHost || customEnv.GITEA_HOST || parsedIndex.host || pubConfig.vcsHost;
  const owner = isCratesIoRegistry(registry)
    ? "crates.io"
    : explicitOwner || customEnv.CARGO_REGISTRY_OWNER || parsedIndex.owner || pubConfig.publisherId;
  const indexUrl =
    !isCratesIoRegistry(registry) && !explicitHost && !explicitOwner
      ? preConfiguredIndex
      : undefined;
  const version = explicitVersion || getCurrentVersion(workspaceRoot);
  const token = isCratesIoRegistry(registry)
    ? customEnv.CARGO_REGISTRY_TOKEN || customEnv.CRATES_IO_TOKEN
    : customEnv[`CARGO_REGISTRIES_${envReg}_TOKEN`] ||
      customEnv.CARGO_REGISTRY_TOKEN ||
      customEnv.RELEASE_BOT_TOKEN ||
      customEnv.REGISTRY_TOKEN ||
      customEnv.GITEA_TOKEN ||
      customEnv.GITHUB_TOKEN;

  const cratesIo = isCratesIoRegistry(registry);

  return {
    host,
    owner,
    registry,
    version,
    indexUrl,
    token,
    workspaceRoot,
    crates: selectedCrates.length > 0 ? selectedCrates : [...WORKSPACE_CRATES],
    dryRun,
    noVerify,
    pollAttempts: cratesIo ? 30 : 30,
    pollIntervalMs: cratesIo ? 2000 : 1000,
  };
}

export async function runCargoPublishPipeline(
  options: CargoPublishOptions,
): Promise<CargoPublishResult[]> {
  const cratesIo = isCratesIoRegistry(options.registry);
  const indexDisplay = cratesIo
    ? "https://index.crates.io/"
    : options.indexUrl || buildCargoRegistryIndexUrl(options.host, options.owner);

  console.log(`\x1b[36m--> Initializing CDDM Cargo Registry Publisher...\x1b[0m`);
  console.log(`Registry Name:  ${options.registry}`);
  console.log(`Sparse Index:   ${indexDisplay}`);
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
        `\x1b[33m[DRY-RUN] Would publish ${crateName} v${options.version} -> ${indexDisplay}\x1b[0m`,
      );
      results.push({ crateName, status: "dry-run" });
    }
    return results;
  }

  if (!options.token) {
    throw new Error(
      cratesIo
        ? "Missing crates.io token. Set CARGO_REGISTRY_TOKEN (or CRATES_IO_TOKEN) via environment."
        : "Missing Cargo registry token. Set CARGO_REGISTRY_TOKEN, RELEASE_BOT_TOKEN, REGISTRY_TOKEN, or GITEA_TOKEN.",
    );
  }

  const originalCargoToml = readFileSync(cargoTomlPath, "utf-8");
  const patchedCargoToml = cratesIo
    ? originalCargoToml
    : injectWorkspaceDependencyRegistry(originalCargoToml, options.registry);
  const env = buildCargoPublishEnv(options);

  try {
    if (!cratesIo && patchedCargoToml !== originalCargoToml) {
      writeFileSync(cargoTomlPath, patchedCargoToml, "utf-8");
    }

    for (let idx = 0; idx < options.crates.length; idx++) {
      const crateName = options.crates[idx]!;
      const alreadyPublished = await isCrateVersionPublished({
        registry: options.registry,
        host: options.host,
        owner: options.owner,
        crateName,
        version: options.version,
        token: options.token,
        indexUrl: options.indexUrl,
      });
      if (alreadyPublished) {
        console.log(
          `\x1b[33m[SKIP] ${crateName} v${options.version} already exists in ${options.registry}\x1b[0m`,
        );
        results.push({ crateName, status: "skipped" });
        continue;
      }

      const cmd = buildCargoPublishCommand(crateName, options.registry, options.noVerify);
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
        const indexed = await waitForSparseIndexAvailability(options, crateName);
        if (!indexed) {
          console.warn(
            `\x1b[33m[WARN] Timed out waiting for ${crateName} v${options.version} in sparse index; proceeding to next crate.\x1b[0m`,
          );
        }
      }
    }
  } finally {
    if (!cratesIo) {
      writeFileSync(cargoTomlPath, originalCargoToml, "utf-8");
    }
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
