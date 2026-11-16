import { describe, expect, it } from "bun:test";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { resolvePublisherConfig } from "../lib/publisher-config";
import {
  createOrGetGiteaRelease,
  deleteReleaseAsset,
  getAuthHeader,
  getChangelogForVersion,
  injectPackagingChecksums,
  isReleaseAsset,
  maskSecret,
  parseArgs,
  parseSha256Sums,
  uploadAssetToGitea,
} from "../publish-release";

describe("Gitea Release Publisher Engine", () => {
  it("should parse default and custom CLI arguments and reject --token flags", () => {
    const expectedPub = resolvePublisherConfig(process.cwd());
    const defaultOpts = parseArgs([]);
    expect(defaultOpts.host).toBe(process.env.GITEA_HOST || expectedPub.vcsHost);
    expect(defaultOpts.repo).toBe(process.env.GITEA_REPO || expectedPub.vcsRepo);
    expect(defaultOpts.tag).toMatch(/^v\d+\.\d+\.\d+/);
    expect(defaultOpts.dryRun).toBe(false);
    expect(defaultOpts.target).toBe("main");

    expect(() => parseArgs(["--token", "leaked"])).toThrow("prohibited");
    expect(() => parseArgs(["--token=leaked"])).toThrow("prohibited");

    const customOpts = parseArgs([
      "--host",
      "git.example.com",
      "--repo",
      "myorg/myrepo",
      "--tag",
      "v2.0.0",
      "--target",
      "feat/my-branch",
      "--write-notes",
      "release-notes.md",
      "--dry-run",
      "--draft",
    ]);
    expect(customOpts.host).toBe("git.example.com");
    expect(customOpts.repo).toBe("myorg/myrepo");
    expect(customOpts.tag).toBe("v2.0.0");
    expect(customOpts.target).toBe("feat/my-branch");
    expect(customOpts.writeNotesPath).toBe("release-notes.md");
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
    expect(getAuthHeader("abc123")).toBe("token abc123");
    expect(getAuthHeader("jwt.token.val")).toBe("Bearer jwt.token.val");
  });

  it("should PATCH existing Gitea release body when verified channels update release notes", async () => {
    const originalFetch = globalThis.fetch;
    let patchedBody = "";
    globalThis.fetch = (async (_input: string | URL | Request, init?: RequestInit) => {
      if (!init?.method || init.method === "GET") {
        return new Response(
          JSON.stringify({
            id: 77,
            tag_name: "v4.9.4",
            name: "Release v4.9.4",
            body: "Old notes mentioning Model Context Protocol without crates.io",
            draft: false,
            prerelease: false,
          }),
          { status: 200, headers: { "Content-Type": "application/json" } },
        );
      }
      if (init.method === "PATCH") {
        patchedBody = typeof init.body === "string" ? init.body : "";
        return new Response(JSON.stringify({ id: 77 }), { status: 200 });
      }
      return new Response("unexpected", { status: 500 });
    }) as typeof fetch;

    try {
      const opts = {
        ...parseArgs(["--tag", "v4.9.4"]),
        token: "mock", // gitleaks:allow — unit test fixture
        body: "Updated notes with Model Context Protocol and verified crates.io",
      };
      const updated = await createOrGetGiteaRelease(opts);
      expect(updated?.body).toBe(opts.body);
      expect(patchedBody).toContain("verified crates.io");
    } finally {
      globalThis.fetch = originalFetch;
    }
  });

  it("should simulate asset upload in dry-run mode", async () => {
    const opts = parseArgs(["--dry-run"]);
    const success = await uploadAssetToGitea(opts, 12345, "package.json");
    expect(success).toBe(true);
  });

  it("should validate allowed and forbidden release download asset extensions", () => {
    expect(isReleaseAsset("cddm-v4.9.0-x86_64-pc-windows-msvc.zip")).toBe(true);
    expect(isReleaseAsset("cddm-v4.9.0-x86_64-unknown-linux-gnu.tar.gz")).toBe(true);
    expect(isReleaseAsset("cddm-4.9.0.vsix")).toBe(true);
    expect(isReleaseAsset("SHA256SUMS.txt")).toBe(true);

    expect(isReleaseAsset("cddm.json")).toBe(false);
    expect(isReleaseAsset("cddm.rb")).toBe(false);
    expect(isReleaseAsset("gt-dev.cddm.yaml")).toBe(false);
    expect(isReleaseAsset("manifest.yml")).toBe(false);
    expect(isReleaseAsset("install.sh")).toBe(false);
  });

  it("should mask sensitive tokens in logs", () => {
    expect(maskSecret("")).toBe("***");
    expect(maskSecret("short")).toBe("***");
    expect(maskSecret("3a5b58eb053ec28f0c99a94b56f372c3698027b3")).toBe("3a5...7b3");
  });

  it("should simulate asset deletion in dry-run mode", async () => {
    const opts = parseArgs(["--dry-run"]);
    const success = await deleteReleaseAsset(opts, 434, 255);
    expect(success).toBe(true);
  });

  it("should parse SHA256SUMS.txt and inject real target-pruned checksums into packaging manifests", () => {
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
        join(tempDir, ".env"),
        "GITEA_HOST=vcs.example.com\nGITEA_REPO=test-org/cddm\nPUBLISHER=test-org\nPUBLISHER_NAME=Test Org\n",
      );
      writeFileSync(
        join(tempDir, "Cargo.toml"),
        '[workspace.package]\nversion = "4.9.0"\ndescription = "CDDM"\nlicense = "MIT"\n',
      );
      writeFileSync(
        join(distDir, "SHA256SUMS.txt"),
        `${linuxHash}  cddm-v4.9.0-x86_64-unknown-linux-gnu.tar.gz\n${winHash}  cddm-v4.9.0-x86_64-pc-windows-msvc.zip\n`,
      );

      const sumsMap = parseSha256Sums(readFileSync(join(distDir, "SHA256SUMS.txt"), "utf-8"));
      expect(sumsMap.get("cddm-v4.9.0-x86_64-unknown-linux-gnu.tar.gz")).toBe(linuxHash);
      expect(sumsMap.get("cddm-v4.9.0-x86_64-pc-windows-msvc.zip")).toBe(winHash);

      const injected = injectPackagingChecksums("v4.9.0", distDir, tempDir);
      expect(injected.length).toBe(5);

      const brewRb = readFileSync(join(tempDir, "packaging", "homebrew", "cddm.rb"), "utf-8");
      expect(brewRb).toContain(linuxHash);
      expect(brewRb).toContain('generate_completions_from_executable(bin/"cddm", "completions")');
      expect(brewRb).not.toContain("on_macos");

      const scoopJson = readFileSync(join(tempDir, "packaging", "scoop", "cddm.json"), "utf-8");
      expect(scoopJson).toContain(winHash);

      const wingetInst = readFileSync(
        join(tempDir, "packaging", "winget", "test-org.cddm.installer.yaml"),
        "utf-8",
      );
      expect(wingetInst).toContain(winHash.toUpperCase());
      expect(wingetInst).toContain("RelativeFilePath: cddm-lsp.exe");

      expect(existsSync(join(distDir, "cddm.rb"))).toBe(false);
      expect(existsSync(join(distDir, "cddm.json"))).toBe(false);
      expect(existsSync(join(distDir, "cddm.yaml"))).toBe(false);
      expect(existsSync(join(distDir, "test-org.cddm.yaml"))).toBe(false);
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });
});
