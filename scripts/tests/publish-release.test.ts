import { describe, expect, it } from "bun:test";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  createOrGetGiteaRelease,
  getChangelogForVersion,
  injectPackagingChecksums,
  parseArgs,
  parseSha256Sums,
  uploadAssetToGitea,
} from "../publish-release";

describe("Gitea Release Publisher Engine", () => {
  it("should parse default and custom CLI arguments", () => {
    const defaultOpts = parseArgs([]);
    expect(defaultOpts.host).toBe("git.gt-web-dev.com");
    expect(defaultOpts.repo).toBe("gt-dev/cddm");
    expect(defaultOpts.tag).toMatch(/^v\d+\.\d+\.\d+/);
    expect(defaultOpts.dryRun).toBe(false);

    const customOpts = parseArgs([
      "--host",
      "git.example.com",
      "--repo",
      "myorg/myrepo",
      "--tag",
      "v2.0.0",
      "--dry-run",
      "--draft",
    ]);
    expect(customOpts.host).toBe("git.example.com");
    expect(customOpts.repo).toBe("myorg/myrepo");
    expect(customOpts.tag).toBe("v2.0.0");
    expect(customOpts.dryRun).toBe(true);
    expect(customOpts.draft).toBe(true);
  });

  it("should extract release changelog section for existing version", () => {
    const changelog = getChangelogForVersion("1.9.0");
    expect(typeof changelog).toBe("string");
    expect(changelog.length).toBeGreaterThan(0);
  });

  it("should simulate release creation in dry-run mode", async () => {
    const opts = parseArgs(["--dry-run", "--tag", "v1.9.0", "--title", "Release v1.9.0"]);
    const release = await createOrGetGiteaRelease(opts);
    expect(release).toBeDefined();
    expect(release?.tag_name).toBe("v1.9.0");
    expect(release?.name).toBe("Release v1.9.0");
  });

  it("should simulate asset upload in dry-run mode", async () => {
    const opts = parseArgs(["--dry-run"]);
    const success = await uploadAssetToGitea(opts, 12345, "package.json");
    expect(success).toBe(true);
  });

  it("should parse SHA256SUMS.txt and inject real checksums into packaging manifests", () => {
    const tempDir = join(tmpdir(), `cddm-publish-checksum-test-${Date.now()}`);
    const distDir = join(tempDir, "dist-release");
    mkdirSync(distDir, { recursive: true });
    mkdirSync(join(tempDir, "packaging", "homebrew"), { recursive: true });
    mkdirSync(join(tempDir, "packaging", "scoop"), { recursive: true });
    mkdirSync(join(tempDir, "packaging", "winget"), { recursive: true });

    const linuxHash = "1111111111111111111111111111111111111111111111111111111111111111";
    const winHash = "2222222222222222222222222222222222222222222222222222222222222222";

    try {
      writeFileSync(
        join(distDir, "SHA256SUMS.txt"),
        `${linuxHash}  cddm-v4.9.0-x86_64-unknown-linux-gnu.tar.gz\n${winHash}  cddm-v4.9.0-x86_64-pc-windows-gnu.zip\n`,
      );
      writeFileSync(
        join(tempDir, "packaging", "homebrew", "cddm.rb"),
        `class Cddm < Formula\n  version "4.9.0"\n  url "https://git.gt-web-dev.com/gt-dev/cddm/releases/download/v#{version}/cddm-v#{version}-x86_64-unknown-linux-gnu.tar.gz"\n  sha256 "0000000000000000000000000000000000000000000000000000000000000000"\nend\n`,
      );
      writeFileSync(
        join(tempDir, "packaging", "scoop", "cddm.json"),
        JSON.stringify({
          version: "4.9.0",
          architecture: {
            "64bit": {
              url: "https://git.gt-web-dev.com/gt-dev/cddm/releases/download/v4.9.0/cddm-v4.9.0-x86_64-pc-windows-gnu.zip",
              hash: "0000000000000000000000000000000000000000000000000000000000000000",
            },
          },
        }),
      );
      writeFileSync(
        join(tempDir, "packaging", "winget", "GrigorTonikyan.cddm.yaml"),
        `PackageVersion: 4.9.0\nInstallers:\n  - Architecture: x64\n    InstallerUrl: https://git.gt-web-dev.com/gt-dev/cddm/releases/download/v4.9.0/cddm-v4.9.0-x86_64-pc-windows-gnu.zip\n    InstallerSha256: 0000000000000000000000000000000000000000000000000000000000000000\n`,
      );

      const sumsMap = parseSha256Sums(readFileSync(join(distDir, "SHA256SUMS.txt"), "utf-8"));
      expect(sumsMap.get("cddm-v4.9.0-x86_64-unknown-linux-gnu.tar.gz")).toBe(linuxHash);
      expect(sumsMap.get("cddm-v4.9.0-x86_64-pc-windows-gnu.zip")).toBe(winHash);

      const injected = injectPackagingChecksums("v4.9.0", distDir, tempDir);
      expect(injected.length).toBe(3);

      expect(readFileSync(join(distDir, "cddm.rb"), "utf-8")).toContain(linuxHash);
      expect(readFileSync(join(distDir, "cddm.json"), "utf-8")).toContain(winHash);
      expect(readFileSync(join(distDir, "GrigorTonikyan.cddm.yaml"), "utf-8")).toContain(winHash);
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });
});
