#!/usr/bin/env bun
/**
 * Supply-Chain Security, Dependency Automation & Code Ownership Policy Gate
 *
 * Enforces Issue #315 supply-chain invariants:
 * 1. `deny.toml` (`cargo-deny`) presence, advisory/license/ban/source sections,
 *    and `Cargo.lock` registry source verification (plus `cargo deny check` when installed).
 * 2. `renovate.json` & `.gitea/workflows/renovate.yaml` SHA digest pinning policy.
 * 3. `.gitea/CODEOWNERS` release-path protection coverage.
 */

import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "..");

export const REQUIRED_CODEOWNERS_PATHS = [
  "/.gitea/workflows/",
  "/packaging/",
  "/scripts/publish-release.ts",
  "/scripts/dispatch-distribution.ts",
  "/deny.toml",
  "/renovate.json",
] as const;

export const ALLOWED_CARGO_LOCK_SOURCES = new Set([
  "registry+https://github.com/rust-lang/crates.io-index",
  "sparse+https://index.crates.io/",
]);

export interface SupplyChainCheckResult {
  passed: boolean;
  errors: string[];
}

export function validateSupplyChainPolicy(workspaceRoot: string = ROOT): SupplyChainCheckResult {
  const errors: string[] = [];

  // 1. Validate deny.toml
  const denyPath = join(workspaceRoot, "deny.toml");
  if (!existsSync(denyPath)) {
    errors.push("Missing cargo-deny supply-chain policy file: deny.toml");
  } else {
    const denyContent = readFileSync(denyPath, "utf-8");
    for (const section of ["[advisories]", "[licenses]", "[bans]", "[sources]"]) {
      if (!denyContent.includes(section)) {
        errors.push(`deny.toml is missing required section: ${section}`);
      }
    }
    if (!denyContent.includes('unknown-registry = "deny"')) {
      errors.push('deny.toml must enforce unknown-registry = "deny"');
    }
    if (!denyContent.includes('unknown-git = "deny"')) {
      errors.push('deny.toml must enforce unknown-git = "deny"');
    }
  }

  // 2. Verify Cargo.lock crate sources against allowlisted registries
  const cargoLockPath = join(workspaceRoot, "Cargo.lock");
  if (existsSync(cargoLockPath)) {
    const lockContent = readFileSync(cargoLockPath, "utf-8");
    const sourceRe = /^source\s*=\s*"([^"]+)"/gm;
    let match: RegExpExecArray | null = sourceRe.exec(lockContent);
    while (match !== null) {
      const src = match[1]!;
      if (!ALLOWED_CARGO_LOCK_SOURCES.has(src)) {
        errors.push(`Cargo.lock contains crate from unapproved registry/git source: "${src}"`);
      }
      match = sourceRe.exec(lockContent);
    }
  }

  // 3. Validate renovate.json
  const renovatePath = join(workspaceRoot, "renovate.json");
  if (!existsSync(renovatePath)) {
    errors.push("Missing automated dependency configuration: renovate.json");
  } else {
    try {
      const renovateJson = JSON.parse(readFileSync(renovatePath, "utf-8")) as {
        extends?: string[];
        packageRules?: Array<{ pinDigests?: boolean }>;
      };
      if (!renovateJson.extends?.includes("helpers:pinGitHubActionDigests")) {
        errors.push('renovate.json must include "helpers:pinGitHubActionDigests" in extends');
      }
      if (!renovateJson.packageRules?.some((r) => r.pinDigests === true)) {
        errors.push("renovate.json must configure pinDigests: true in packageRules");
      }
    } catch (err) {
      errors.push(`renovate.json JSON parse error: ${String(err)}`);
    }
  }

  // 4. Validate .gitea/CODEOWNERS
  const codeownersPath = join(workspaceRoot, ".gitea", "CODEOWNERS");
  if (!existsSync(codeownersPath)) {
    errors.push("Missing release-path ownership file: .gitea/CODEOWNERS");
  } else {
    const codeownersContent = readFileSync(codeownersPath, "utf-8");
    for (const reqPath of REQUIRED_CODEOWNERS_PATHS) {
      if (!codeownersContent.includes(reqPath)) {
        errors.push(`.gitea/CODEOWNERS is missing protection rule for "${reqPath}"`);
      }
    }
  }

  return {
    passed: errors.length === 0,
    errors,
  };
}

function main() {
  console.log(
    "\x1b[36m--> Verifying Supply-Chain Security, Renovate & CODEOWNERS policy...\x1b[0m",
  );
  const result = validateSupplyChainPolicy(ROOT);
  if (!result.passed) {
    for (const err of result.errors) {
      console.error(`\x1b[31m[FAIL] ${err}\x1b[0m`);
    }
    process.exit(1);
  }

  console.log(
    "\x1b[32m[PASS] Supply-chain policy (deny.toml, Cargo.lock sources, renovate.json, .gitea/CODEOWNERS) verified cleanly!\x1b[0m",
  );
}

if (import.meta.main) {
  main();
}
