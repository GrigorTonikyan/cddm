import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { describe, expect, it } from "bun:test";

const ROOT = resolve(import.meta.dir, "../..");

function readRepoFile(relPath: string): string {
  return readFileSync(join(ROOT, relPath), "utf-8");
}

describe("CI/CD & Container Governance Invariants", () => {
  it("keeps ci.yaml PR-gated without redundant main push triggers", () => {
    const ciYaml = readRepoFile(".gitea/workflows/ci.yaml");
    expect(ciYaml).toContain("pull_request:");
    expect(ciYaml).not.toMatch(/^on:\r?\n\s+push:/m);
    expect(ciYaml).toContain("self:gt-dev/actions/gitleaks-scan@v1");
    expect(ciYaml).toContain("self:gt-dev/actions/setup-toolchains@v1");
    expect(ciYaml).toContain("playwright-runner:v");
    expect(ciYaml).toContain("rust-build:");
    expect(ciYaml).toContain("rust-ci:");
    expect(ciYaml).toContain("scripts/check-supply-chain.ts");
  });

  it("isolates main auto-tagging into auto-tag-release.yaml and configures Renovate", () => {
    expect(existsSync(join(ROOT, ".gitea/workflows/auto-tag-release.yaml"))).toBeTrue();
    const autoTag = readRepoFile(".gitea/workflows/auto-tag-release.yaml");
    expect(autoTag).toContain("push:");
    expect(autoTag).toContain("branches: [main]");
    expect(autoTag).toContain("TARGET_SHA: ${{ github.sha }}");
    expect(autoTag).toContain("RELEASE_SIGNING_KEY: ${{ secrets.RELEASE_SIGNING_KEY");
    expect(autoTag).toContain("scripts/publish-release.ts");

    expect(existsSync(join(ROOT, ".gitea/workflows/renovate.yaml"))).toBeTrue();
    const renovateYaml = readRepoFile(".gitea/workflows/renovate.yaml");
    expect(renovateYaml).toContain("schedule:");
    expect(renovateYaml).toContain("renovate");
  });

  it("hardens release.yaml without setup-buildx-action or hardcoded host fallbacks", () => {
    const releaseYaml = readRepoFile(".gitea/workflows/release.yaml");
    expect(releaseYaml).not.toContain("setup-buildx-action");
    expect(releaseYaml).not.toContain("docker:dind");
    expect(releaseYaml).not.toContain("|| 'git.gt-web-dev.com'");
    expect(releaseYaml).toContain("self:gt-dev/actions/docker-build-push@v1");
    expect(releaseYaml).toContain("gt-dev/actions#47");
    expect(releaseYaml).toContain("publish-npm:");
    expect(releaseYaml).toContain("scripts/publish-npm.ts --registry gitea");
    expect(releaseYaml).toContain("HOMEBREW_TAP_DEPLOY_KEY");
    expect(releaseYaml).toContain("SCOOP_BUCKET_DEPLOY_KEY");
    expect(releaseYaml).toContain("WINGET_DEPLOY_KEY");
    expect(releaseYaml).toContain(
      "GITEA_TOKEN: ${{ secrets.RELEASE_BOT_TOKEN || secrets.REGISTRY_TOKEN || secrets.GITEA_TOKEN || secrets.GITHUB_TOKEN }}",
    );
    expect(releaseYaml).not.toContain("cache-mode: registry");
    expect(releaseYaml).toContain("cache-mode: inline");
    expect(releaseYaml).toContain("build-webui:");
    expect(releaseYaml).toContain("force-unsafe-io");
    expect(releaseYaml).toContain("~/.cache/cargo-xwin");

    const ghReleaseYml = readRepoFile(".github/workflows/release.yml");
    expect(ghReleaseYml).toContain("id-token: write");
    expect(ghReleaseYml).toContain("scripts/publish-npm.ts --registry npmjs --provenance");
  });

  it("pins Dockerfile base images to sha256 digests and includes .dockerignore", () => {
    expect(existsSync(join(ROOT, ".dockerignore"))).toBeTrue();
    const dockerignore = readRepoFile(".dockerignore");
    expect(dockerignore).toContain(".git");
    expect(dockerignore).toContain("target");
    expect(dockerignore).toContain("node_modules");

    const dockerfile = readRepoFile("Dockerfile");
    expect(dockerfile.startsWith("# syntax=docker/dockerfile:1")).toBeTrue();
    expect(dockerfile).not.toMatch(/^FROM\s+[^\s]+:latest/m);
    expect(dockerfile).toContain("@sha256:");
    expect(dockerfile).toContain("--mount=type=cache");
    expect(dockerfile).toContain("force-unsafe-io");
    expect(dockerfile).toContain("HEALTHCHECK");
  });
});
