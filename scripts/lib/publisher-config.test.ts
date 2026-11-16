import { describe, expect, it } from "bun:test";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { extractHostFromUrl, parseEnvFile, resolvePublisherConfig } from "./publisher-config";

describe("publisher-config", () => {
  it("parses key-value pairs from .env files correctly", () => {
    const tempDir = join(process.cwd(), "target", "test-env-parse");
    mkdirSync(tempDir, { recursive: true });
    const envFile = join(tempDir, ".env");
    writeFileSync(
      envFile,
      `
# Comment line
PUBLISHER="custom-org"
PUBLISHER_NAME='Custom Org Name'
HOMEBREW_TAP_REPO=custom-org/homebrew-tap
SCOOP_BUCKET_REPO=custom-org/scoop-bucket
WINGET_REPO=custom-org/winget-pkgs
GITEA_HOST=git.example.com
GITEA_REPO=custom-org/app
`,
      "utf-8",
    );

    const parsed = parseEnvFile(envFile);
    expect(parsed.PUBLISHER).toBe("custom-org");
    expect(parsed.PUBLISHER_NAME).toBe("Custom Org Name");
    expect(parsed.HOMEBREW_TAP_REPO).toBe("custom-org/homebrew-tap");
    expect(parsed.SCOOP_BUCKET_REPO).toBe("custom-org/scoop-bucket");
    expect(parsed.WINGET_REPO).toBe("custom-org/winget-pkgs");
    expect(parsed.GITEA_HOST).toBe("git.example.com");
    expect(parsed.GITEA_REPO).toBe("custom-org/app");

    rmSync(tempDir, { recursive: true, force: true });
  });

  it("prioritizes environment variables over .env and fallback defaults", () => {
    const tempDir = join(process.cwd(), "target", "test-env-precedence");
    mkdirSync(tempDir, { recursive: true });
    const envFile = join(tempDir, ".env");
    writeFileSync(envFile, "PUBLISHER=env-file-pub\nPUBLISHER_NAME=Env File Pub\n", "utf-8");

    const customEnv = {
      PUBLISHER: "vcs-actions-pub",
      PUBLISHER_NAME: "VCS Actions Pub",
      HOMEBREW_TAP_REPO: "vcs-actions-pub/homebrew-tap",
    };

    const config = resolvePublisherConfig(tempDir, customEnv);
    expect(config.publisherId).toBe("vcs-actions-pub");
    expect(config.publisherName).toBe("VCS Actions Pub");
    expect(config.homebrewTapRepo).toBe("vcs-actions-pub/homebrew-tap");
    expect(config.scoopBucketRepo).toBe("vcs-actions-pub/scoop-bucket");

    rmSync(tempDir, { recursive: true, force: true });
  });

  it("falls back to .env when environment variables are unset", () => {
    const tempDir = join(process.cwd(), "target", "test-env-fallback");
    mkdirSync(tempDir, { recursive: true });
    const envFile = join(tempDir, ".env");
    writeFileSync(
      envFile,
      "PUBLISHER=fallback-pub\nHOMEBREW_TAP_REPO=fallback-pub/custom-tap\n",
      "utf-8",
    );

    const config = resolvePublisherConfig(tempDir, {});
    expect(config.publisherId).toBe("fallback-pub");
    expect(config.homebrewTapRepo).toBe("fallback-pub/custom-tap");
    expect(config.scoopBucketRepo).toBe("fallback-pub/scoop-bucket");

    rmSync(tempDir, { recursive: true, force: true });
  });

  it("resolves host and owner dynamically from GITHUB_SERVER_URL and GITHUB_REPOSITORY_OWNER", () => {
    const tempDir = join(process.cwd(), "target", "test-env-gha");
    mkdirSync(tempDir, { recursive: true });

    expect(extractHostFromUrl("https://git.gt-web-dev.com")).toBe("git.gt-web-dev.com");
    expect(extractHostFromUrl("")).toBeUndefined();

    const config = resolvePublisherConfig(tempDir, {
      GITHUB_SERVER_URL: "https://gitea.internal.example:8443",
      GITHUB_REPOSITORY_OWNER: "platform-eng",
      GITHUB_REPOSITORY: "platform-eng/cddm",
    });
    expect(config.vcsHost).toBe("gitea.internal.example:8443");
    expect(config.publisherId).toBe("platform-eng");
    expect(config.vcsRepo).toBe("platform-eng/cddm");

    rmSync(tempDir, { recursive: true, force: true });
  });

  it("resolves dynamically from git remote origin when inside repo without env or .env", () => {
    const tempDir = join(process.cwd(), "target", "test-env-empty");
    mkdirSync(tempDir, { recursive: true });

    const config = resolvePublisherConfig(tempDir, {});
    expect(config.vcsHost).toBeDefined();
    expect(config.vcsRepo).toContain("/");
    expect(config.publisherId).toBeDefined();
    expect(config.homebrewTapRepo).toBeDefined();
    expect(config.scoopBucketRepo).toBeDefined();
    expect(config.wingetRepo).toBeDefined();

    rmSync(tempDir, { recursive: true, force: true });
  });

  it("throws explicitly instead of falling back to hardcoded defaults when unresolvable", () => {
    const nonGitDir = join(tmpdir(), `cddm-no-git-config-${Date.now()}`);
    mkdirSync(nonGitDir, { recursive: true });
    try {
      expect(() => resolvePublisherConfig(nonGitDir, {})).toThrow("Unable to resolve VCS host");
    } finally {
      rmSync(nonGitDir, { recursive: true, force: true });
    }
  });
});
