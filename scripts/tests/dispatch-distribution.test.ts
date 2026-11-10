import { describe, expect, it } from "bun:test";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { dispatchDistribution, parseArgs } from "../dispatch-distribution";
import type { PublisherConfig } from "../lib/publisher-config";

describe("dispatch-distribution", () => {
  it("parses CLI arguments accurately", () => {
    const opts = parseArgs([
      "--package",
      "forge",
      "--dry-run",
      "--tag",
      "v1.2.3",
      "--dist-dir",
      "custom-dist",
    ]);
    expect(opts.packageName).toBe("forge");
    expect(opts.dryRun).toBe(true);
    expect(opts.tag).toBe("v1.2.3");
    expect(opts.distDir).toBe("custom-dist");
  });

  it("executes dry-run dispatch across all three package managers without errors", async () => {
    const tempDir = join(process.cwd(), "target", "test-dispatch-dry-run");
    mkdirSync(join(tempDir, "packaging", "homebrew"), { recursive: true });
    mkdirSync(join(tempDir, "packaging", "scoop"), { recursive: true });
    mkdirSync(join(tempDir, "packaging", "winget"), { recursive: true });

    writeFileSync(
      join(tempDir, "packaging", "homebrew", "test-app.rb"),
      'class TestApp < Formula\n  version "1.0.0"\nend\n',
      "utf-8",
    );
    writeFileSync(
      join(tempDir, "packaging", "scoop", "test-app.json"),
      '{\n  "version": "1.0.0"\n}\n',
      "utf-8",
    );
    writeFileSync(
      join(tempDir, "packaging", "winget", "test-app.yaml"),
      "PackageIdentifier: my-org.test-app\nPackageVersion: 1.0.0\n",
      "utf-8",
    );

    const mockConfig: PublisherConfig = {
      publisherId: "my-org",
      publisherName: "My Org",
      homebrewTapRepo: "my-org/homebrew-tap",
      scoopBucketRepo: "my-org/scoop-bucket",
      wingetRepo: "my-org/winget-pkgs",
      vcsHost: "git.gt-web-dev.com",
      vcsRepo: "my-org/test-app",
    };

    const res = await dispatchDistribution({
      packageName: "test-app",
      version: "1.0.0",
      tag: "v1.0.0",
      workspaceRoot: tempDir,
      distDir: "dist",
      dryRun: true,
      config: mockConfig,
    });

    expect(res.homebrew).toBe(true);
    expect(res.scoop).toBe(true);
    expect(res.winget).toBe(true);
    expect(res.errors.length).toBe(0);

    rmSync(tempDir, { recursive: true, force: true });
  });

  it("handles missing target files gracefully without throwing fatal errors", async () => {
    const tempDir = join(process.cwd(), "target", "test-dispatch-missing");
    mkdirSync(tempDir, { recursive: true });

    const mockConfig: PublisherConfig = {
      publisherId: "my-org",
      publisherName: "My Org",
      homebrewTapRepo: "my-org/homebrew-tap",
      scoopBucketRepo: "my-org/scoop-bucket",
      wingetRepo: "my-org/winget-pkgs",
      vcsHost: "git.gt-web-dev.com",
      vcsRepo: "my-org/missing-app",
    };

    const res = await dispatchDistribution({
      packageName: "missing-app",
      version: "1.0.0",
      tag: "v1.0.0",
      workspaceRoot: tempDir,
      distDir: "dist",
      dryRun: true,
      config: mockConfig,
    });

    expect(res.homebrew).toBe(false);
    expect(res.scoop).toBe(false);
    expect(res.winget).toBe(false);

    rmSync(tempDir, { recursive: true, force: true });
  });
});
