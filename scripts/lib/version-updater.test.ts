import { describe, expect, it } from "bun:test";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import type { ParsedCommit } from "../version";
import {
  checkWorkspaceVersionSync,
  generateChangelogSection,
  updateChangelog,
  updateWorkspaceVersions,
} from "./version-updater";

describe("version-updater utility", () => {
  it("generates markdown changelog section properly for categorized commits", () => {
    const mockCommits: ParsedCommit[] = [
      {
        hash: "abc1234",
        type: "feat",
        scope: "engine",
        subject: "add parallel parser recycling",
        breaking: false,
        raw: "feat(engine): add parallel parser recycling",
      },
      {
        hash: "def5678",
        type: "fix",
        scope: "mcp",
        subject: "handle missing parameter error codes",
        breaking: false,
        raw: "fix(mcp): handle missing parameter error codes",
      },
      {
        hash: "789ghij",
        type: "feat",
        scope: "api",
        subject: "breaking API v2 changes",
        breaking: true,
        raw: "feat(api)!: breaking API v2 changes",
      },
    ];

    const changelog = generateChangelogSection("1.8.0", "2026-08-30", mockCommits);
    expect(changelog).toContain("## [1.8.0] - 2026-08-30");
    expect(changelog).toContain("### BREAKING CHANGES");
    expect(changelog).toContain("breaking API v2 changes (`789ghij`)");
    expect(changelog).toContain("### Features");
    expect(changelog).toContain("**engine**: add parallel parser recycling (`abc1234`)");
    expect(changelog).toContain("### Bug Fixes");
    expect(changelog).toContain("**mcp**: handle missing parameter error codes (`def5678`)");
  });

  it("handles empty commit list gracefully", () => {
    const changelog = generateChangelogSection("1.0.0", "2026-08-30", []);
    expect(changelog).toBe("## [1.0.0] - 2026-08-30\n\n");
  });

  it("synchronizes and validates all 10 workspace manifests in lockstep", () => {
    const tempRoot = join(tmpdir(), `cddm-version-sync-test-${Date.now()}`);
    mkdirSync(join(tempRoot, "webui"), { recursive: true });
    mkdirSync(join(tempRoot, "npm", "cddm"), { recursive: true });
    mkdirSync(join(tempRoot, "editors", "vscode"), { recursive: true });
    mkdirSync(join(tempRoot, "packaging", "homebrew"), { recursive: true });
    mkdirSync(join(tempRoot, "packaging", "scoop"), { recursive: true });
    mkdirSync(join(tempRoot, "packaging", "winget"), { recursive: true });

    try {
      writeFileSync(
        join(tempRoot, "Cargo.toml"),
        `[workspace]\n[workspace.package]\nversion = "4.8.0"\nedition = "2024"\n`,
      );
      writeFileSync(
        join(tempRoot, "Cargo.lock"),
        `[[package]]\nname = "cddm-core"\nversion = "4.8.0"\n\n[[package]]\nname = "cddm-cli"\nversion = "4.8.0"\n`,
      );
      writeFileSync(join(tempRoot, "package.json"), JSON.stringify({ version: "4.7.0" }));
      writeFileSync(join(tempRoot, "webui", "package.json"), JSON.stringify({ version: "4.7.0" }));
      writeFileSync(
        join(tempRoot, "npm", "cddm", "package.json"),
        JSON.stringify({
          version: "4.7.0",
          optionalDependencies: {
            "@cddm/linux-x64": "0.1.0",
            "@cddm/win32-x64": "0.1.0",
          },
        }),
      );
      writeFileSync(
        join(tempRoot, "editors", "vscode", "package.json"),
        JSON.stringify({ version: "4.7.0" }),
      );
      writeFileSync(
        join(tempRoot, "packaging", "homebrew", "cddm.rb"),
        `class Cddm < Formula\n  version "4.7.0"\nend\n`,
      );
      writeFileSync(
        join(tempRoot, "packaging", "scoop", "cddm.json"),
        JSON.stringify({
          version: "4.7.0",
          architecture: {
            "64bit": {
              url: "https://git.gt-web-dev.com/gt-dev/cddm/releases/download/v1.9.0/cddm-v1.9.0-x86_64-pc-windows-msvc.zip",
            },
          },
        }),
      );
      writeFileSync(
        join(tempRoot, "packaging", "winget", "gt-dev.cddm.yaml"),
        `PackageVersion: 4.7.0\nInstallers:\n  - Architecture: x64\n    InstallerUrl: https://git.gt-web-dev.com/gt-dev/cddm/releases/download/v1.9.0/cddm-v1.9.0-x86_64-pc-windows-msvc.zip\n`,
      );
      writeFileSync(
        join(tempRoot, "README.md"),
        `![npm](https://img.shields.io/badge/npm-4.7.0-red.svg) ![crates](https://img.shields.io/badge/crates.io-4.7.0-brightgreen.svg)`,
      );

      // Initially out of sync
      const initialCheck = checkWorkspaceVersionSync(tempRoot);
      expect(initialCheck.synchronized).toBe(false);
      expect(initialCheck.mismatches.length).toBeGreaterThan(5);

      // Synchronize to 4.9.0
      updateWorkspaceVersions("4.9.0", tempRoot);

      const postCheck = checkWorkspaceVersionSync(tempRoot);
      expect(postCheck.synchronized).toBe(true);
      expect(postCheck.canonicalVersion).toBe("4.9.0");
      expect(postCheck.mismatches).toHaveLength(0);

      const lockText = readFileSync(join(tempRoot, "Cargo.lock"), "utf-8");
      expect(lockText).toContain('name = "cddm-core"\nversion = "4.9.0"');
      expect(lockText).toContain('name = "cddm-cli"\nversion = "4.9.0"');
    } finally {
      rmSync(tempRoot, { recursive: true, force: true });
    }
  });

  it("updates CHANGELOG.md idempotently without duplicating the same version header", () => {
    const tempRoot = join(tmpdir(), `cddm-changelog-test-${Date.now()}`);
    mkdirSync(tempRoot, { recursive: true });
    try {
      const section1 = "## [4.9.0] - 2026-11-10\n\n### Features\n\n- first feature (`aaa1111`)\n\n";
      const section2 =
        "## [4.9.0] - 2026-11-10\n\n### Features\n\n- first feature (`aaa1111`)\n- second feature (`bbb2222`)\n\n";

      updateChangelog(section1, tempRoot);
      updateChangelog(section2, tempRoot);

      const content = readFileSync(join(tempRoot, "CHANGELOG.md"), "utf-8");
      const matches = content.match(/## \[4\.9\.0\]/g);
      expect(matches?.length).toBe(1);
      expect(content).toContain("second feature (`bbb2222`)");
    } finally {
      rmSync(tempRoot, { recursive: true, force: true });
    }
  });
});
