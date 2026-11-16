import { describe, expect, it } from "bun:test";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { ZERO_SHA256 } from "../lib/distribution-manifests";
import { dispatchDistribution, parseArgs } from "../dispatch-distribution";
import type { PublisherConfig } from "../lib/publisher-config";

const VALID_LINUX_SHA = "1".repeat(64);
const VALID_WIN_SHA = "2".repeat(64);

describe("dispatch-distribution", () => {
  it("parses CLI arguments accurately and rejects --token flags", () => {
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

    expect(() => parseArgs(["--token", "leaked"])).toThrow("prohibited");
    expect(() => parseArgs(["--token=leaked"])).toThrow("prohibited");
  });

  it("executes dry-run dispatch with SHA256SUMS.txt, pruning unbuilt targets and emitting WinGet v1.9.0 files", async () => {
    const tempDir = join(process.cwd(), "target", "test-dispatch-dry-run");
    const distDir = join(tempDir, "dist-release");
    mkdirSync(distDir, { recursive: true });
    writeFileSync(
      join(tempDir, "Cargo.toml"),
      '[workspace.package]\nversion = "1.0.0"\ndescription = "Test App"\nlicense = "MIT"\n',
    );
    writeFileSync(
      join(distDir, "SHA256SUMS.txt"),
      `${VALID_LINUX_SHA}  cddm-v1.0.0-x86_64-unknown-linux-gnu.tar.gz\n${VALID_WIN_SHA}  cddm-v1.0.0-x86_64-pc-windows-msvc.zip\n`,
      "utf-8",
    );

    const mockConfig: PublisherConfig = {
      publisherId: "my-org",
      publisherName: "My Org",
      homebrewTapRepo: "my-org/homebrew-tap",
      scoopBucketRepo: "my-org/scoop-bucket",
      wingetRepo: "my-org/winget-pkgs",
      vcsHost: "vcs.example.com",
      vcsRepo: "my-org/cddm",
    };

    const res = await dispatchDistribution({
      packageName: "cddm",
      version: "1.0.0",
      tag: "v1.0.0",
      workspaceRoot: tempDir,
      distDir,
      dryRun: true,
      config: mockConfig,
    });

    expect(res.homebrew).toBe(true);
    expect(res.scoop).toBe(true);
    expect(res.winget).toBe(true);
    expect(res.errors.length).toBe(0);

    rmSync(tempDir, { recursive: true, force: true });
  });

  it("enforces WinGet lowercase case-parity paths and verifies HTTP 200 reachability before committing", async () => {
    const tempDir = join(process.cwd(), "target", "test-dispatch-live-mock");
    const distDir = join(tempDir, "dist-release");
    mkdirSync(distDir, { recursive: true });
    writeFileSync(
      join(distDir, "SHA256SUMS.txt"),
      `${VALID_WIN_SHA}  cddm-v1.0.0-x86_64-pc-windows-msvc.zip\n`,
      "utf-8",
    );

    const mockConfig: PublisherConfig = {
      publisherId: "my-org",
      publisherName: "My Org",
      homebrewTapRepo: "my-org/homebrew-tap",
      scoopBucketRepo: "my-org/scoop-bucket",
      wingetRepo: "my-org/winget-pkgs",
      vcsHost: "vcs.example.com",
      vcsRepo: "my-org/cddm",
    };

    const committedPaths: string[] = [];
    const mockFetch = (async (input: string | URL | Request, init?: RequestInit) => {
      const url = typeof input === "string" ? input : input.toString();
      if (url.includes("/releases/download/")) {
        return new Response(null, { status: 200 });
      }
      if (url.includes("/contents/") && init?.method === "GET") {
        return new Response("not found", { status: 404 });
      }
      if (url.includes("/contents/") && init?.method === "POST") {
        committedPaths.push(url);
        return new Response(JSON.stringify({ content: { sha: "abc" } }), { status: 201 });
      }
      return new Response("unexpected", { status: 500 });
    }) as unknown as typeof fetch;

    try {
      const res = await dispatchDistribution({
        packageName: "cddm",
        version: "1.0.0",
        tag: "v1.0.0",
        workspaceRoot: tempDir,
        distDir,
        token: "mock-token", // gitleaks:allow — unit test fixture
        dryRun: false,
        verifyUrls: true,
        fetchFn: mockFetch,
        config: mockConfig,
      });

      expect(res.homebrew).toBe(false);
      expect(res.scoop).toBe(true);
      expect(res.winget).toBe(true);
      expect(
        committedPaths.some((p) => p.includes("/manifests/m/my-org/cddm/1.0.0/my-org.cddm.yaml")),
      ).toBe(true);
      expect(
        committedPaths.some((p) =>
          p.includes("/manifests/m/my-org/cddm/1.0.0/my-org.cddm.installer.yaml"),
        ),
      ).toBe(true);
      expect(
        committedPaths.some((p) =>
          p.includes("/manifests/m/my-org/cddm/1.0.0/my-org.cddm.locale.en-US.yaml"),
        ),
      ).toBe(true);
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });

  it("hard-fails on 0{64} hashes or non-200 HTTP responses before dispatch", async () => {
    const tempDir = join(process.cwd(), "target", "test-dispatch-security-gates");
    const distDir = join(tempDir, "dist-release");
    mkdirSync(distDir, { recursive: true });
    writeFileSync(
      join(distDir, "SHA256SUMS.txt"),
      `${ZERO_SHA256}  cddm-v1.0.0-x86_64-pc-windows-msvc.zip\n`,
      "utf-8",
    );

    const mockConfig: PublisherConfig = {
      publisherId: "my-org",
      publisherName: "My Org",
      homebrewTapRepo: "my-org/homebrew-tap",
      scoopBucketRepo: "my-org/scoop-bucket",
      wingetRepo: "my-org/winget-pkgs",
      vcsHost: "vcs.example.com",
      vcsRepo: "my-org/cddm",
    };

    try {
      await expect(
        dispatchDistribution({
          packageName: "cddm",
          version: "1.0.0",
          tag: "v1.0.0",
          workspaceRoot: tempDir,
          distDir,
          dryRun: true,
          config: mockConfig,
        }),
      ).rejects.toThrow("zeroed SHA-256 digest");

      writeFileSync(
        join(distDir, "SHA256SUMS.txt"),
        `${VALID_WIN_SHA}  cddm-v1.0.0-x86_64-pc-windows-msvc.zip\n`,
        "utf-8",
      );

      await expect(
        dispatchDistribution({
          packageName: "cddm",
          version: "1.0.0",
          tag: "v1.0.0",
          workspaceRoot: tempDir,
          distDir,
          token: "mock-token", // gitleaks:allow — unit test fixture
          dryRun: false,
          verifyUrls: true,
          fetchFn: (async () => new Response(null, { status: 404 })) as unknown as typeof fetch,
          config: mockConfig,
        }),
      ).rejects.toThrow("expected HTTP 200 OK, got HTTP 404");
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });

  it("handles missing target files gracefully in dry-run without SHA256SUMS.txt", async () => {
    const tempDir = join(process.cwd(), "target", "test-dispatch-missing");
    mkdirSync(tempDir, { recursive: true });

    const mockConfig: PublisherConfig = {
      publisherId: "my-org",
      publisherName: "My Org",
      homebrewTapRepo: "my-org/homebrew-tap",
      scoopBucketRepo: "my-org/scoop-bucket",
      wingetRepo: "my-org/winget-pkgs",
      vcsHost: "vcs.example.com",
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
