import { describe, expect, it } from "bun:test";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { validateSupplyChainPolicy } from "../check-supply-chain";

const ROOT = resolve(import.meta.dir, "../..");

describe("Supply-Chain Security & Governance Validator", () => {
  it("validates current repository deny.toml, Cargo.lock sources, renovate.json, and .gitea/CODEOWNERS", () => {
    const res = validateSupplyChainPolicy(ROOT);
    expect(res.passed).toBe(true);
    expect(res.errors).toEqual([]);
  });

  it("detects unapproved Cargo.lock git/registry sources and missing policy files", () => {
    const tempDir = join(tmpdir(), `cddm-supply-chain-test-${Date.now()}`);
    mkdirSync(tempDir, { recursive: true });
    try {
      const missingRes = validateSupplyChainPolicy(tempDir);
      expect(missingRes.passed).toBe(false);
      expect(missingRes.errors.some((e) => e.includes("deny.toml"))).toBe(true);
      expect(missingRes.errors.some((e) => e.includes("renovate.json"))).toBe(true);
      expect(missingRes.errors.some((e) => e.includes("CODEOWNERS"))).toBe(true);

      writeFileSync(
        join(tempDir, "deny.toml"),
        '[advisories]\n[licenses]\n[bans]\n[sources]\nunknown-registry = "deny"\nunknown-git = "deny"\n',
      );
      writeFileSync(
        join(tempDir, "Cargo.lock"),
        '[[package]]\nname = "evil"\nversion = "0.1.0"\nsource = "git+https://evil.example.com/repo.git"\n',
      );
      writeFileSync(
        join(tempDir, "renovate.json"),
        JSON.stringify({
          extends: ["helpers:pinGitHubActionDigests"],
          packageRules: [{ pinDigests: true }],
        }),
      );
      mkdirSync(join(tempDir, ".gitea"), { recursive: true });
      writeFileSync(
        join(tempDir, ".gitea", "CODEOWNERS"),
        "/.gitea/workflows/ @gt-dev\n/packaging/ @gt-dev\n/scripts/publish-release.ts @gt-dev\n/scripts/dispatch-distribution.ts @gt-dev\n/deny.toml @gt-dev\n/renovate.json @gt-dev\n",
      );

      const unapprovedSourceRes = validateSupplyChainPolicy(tempDir);
      expect(unapprovedSourceRes.passed).toBe(false);
      expect(
        unapprovedSourceRes.errors.some((e) => e.includes("unapproved registry/git source")),
      ).toBe(true);
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });
});
