import { describe, expect, it } from "bun:test";
import {
  buildCargoPublishCommand,
  buildCargoPublishEnv,
  buildCargoRegistryIndexUrl,
  formatCargoAuthToken,
  getSparseIndexCratePath,
  injectWorkspaceDependencyRegistry,
  parseArgs,
  parseSparseIndexVersions,
  runCargoPublishPipeline,
  WORKSPACE_CRATES,
} from "../publish-cargo";

describe("Gitea Cargo Registry & crates.io Publisher", () => {
  it("should define workspace crates in strict dependency order", () => {
    expect([...WORKSPACE_CRATES]).toEqual(["cddm-core", "cddm-lsp", "cddm-mcp", "cddm-cli"]);
  });

  it("should compute Cargo sparse index URLs and crate paths", () => {
    expect(buildCargoRegistryIndexUrl("git.gt-web-dev.com", "gt-dev")).toBe(
      "sparse+https://git.gt-web-dev.com/api/packages/gt-dev/cargo/",
    );
    expect(getSparseIndexCratePath("a")).toBe("1/a");
    expect(getSparseIndexCratePath("ab")).toBe("2/ab");
    expect(getSparseIndexCratePath("abc")).toBe("3/a/abc");
    expect(getSparseIndexCratePath("cddm-core")).toBe("cd/dm/cddm-core");
    expect(getSparseIndexCratePath("cddm-cli")).toBe("cd/dm/cddm-cli");
  });

  it("should parse sparse index NDJSON versions safely", () => {
    const ndjson = [
      JSON.stringify({ name: "cddm-core", vers: "4.9.1" }),
      "not-valid-json",
      JSON.stringify({ name: "cddm-core", vers: "4.9.2" }),
      "",
    ].join("\n");
    const versions = parseSparseIndexVersions(ndjson);
    expect(versions.has("4.9.1")).toBe(true);
    expect(versions.has("4.9.2")).toBe(true);
    expect(versions.has("4.9.3")).toBe(false);
  });

  it("should format Cargo Bearer auth headers idempotently for Gitea", () => {
    expect(formatCargoAuthToken("abc123")).toBe("Bearer abc123");
    expect(formatCargoAuthToken("Bearer abc123")).toBe("Bearer abc123");
    expect(formatCargoAuthToken("token abc123")).toBe("token abc123");
    expect(formatCargoAuthToken("cio-raw-token", "crates-io")).toBe("cio-raw-token");
  });

  it("should inject registry into internal workspace dependencies without duplicating", () => {
    const input = [
      "[workspace.dependencies]",
      'anyhow = "^1"',
      'cddm-core = { path = "crates/cddm-core", version = "4.9.2" }',
      'cddm-lsp = { path = "crates/cddm-lsp", version = "4.9.2" }',
      'cddm-mcp = { path = "crates/cddm-mcp", version = "4.9.2", registry = "gitea" }',
    ].join("\n");

    const output = injectWorkspaceDependencyRegistry(input, "gitea");
    expect(output).toContain(
      'cddm-core = { path = "crates/cddm-core", version = "4.9.2", registry = "gitea" }',
    );
    expect(output).toContain(
      'cddm-lsp = { path = "crates/cddm-lsp", version = "4.9.2", registry = "gitea" }',
    );
    expect(output.match(/registry = "gitea"/g)?.length).toBe(3);
  });

  it("should build normalized Cargo registry environment variables for Gitea and crates.io", () => {
    const giteaEnv = buildCargoPublishEnv(
      {
        host: "git.gt-web-dev.com",
        owner: "gt-dev",
        registry: "my-gitea",
        token: "secret-pat",
      },
      {},
    );
    expect(giteaEnv.CARGO_REGISTRIES_MY_GITEA_INDEX).toBe(
      "sparse+https://git.gt-web-dev.com/api/packages/gt-dev/cargo/",
    );
    expect(giteaEnv.CARGO_REGISTRIES_MY_GITEA_TOKEN).toBe("Bearer secret-pat");

    const cratesIoEnv = buildCargoPublishEnv(
      {
        host: "crates.io",
        owner: "crates.io",
        registry: "crates-io",
        token: "oidc-temp-token",
      },
      {},
    );
    expect(cratesIoEnv.CARGO_REGISTRY_TOKEN).toBe("oidc-temp-token");
    expect(cratesIoEnv.CARGO_REGISTRIES_CRATES_IO_INDEX).toBeUndefined();
    expect(cratesIoEnv.CARGO_REGISTRIES_CRATES_IO_TOKEN).toBeUndefined();
  });

  it("should build distinct cargo publish commands for Gitea vs crates.io", () => {
    expect(buildCargoPublishCommand("cddm-core", "gitea", false)).toEqual([
      "cargo",
      "publish",
      "-p",
      "cddm-core",
      "--allow-dirty",
      "--registry",
      "gitea",
    ]);
    expect(buildCargoPublishCommand("cddm-core", "crates.io", true)).toEqual([
      "cargo",
      "publish",
      "-p",
      "cddm-core",
      "--allow-dirty",
      "--no-verify",
    ]);
  });

  it("should reject --token CLI flags and honor RELEASE_BOT_TOKEN + GITHUB_SERVER_URL env", () => {
    expect(() => parseArgs(["--token", "leaked"])).toThrow("prohibited");
    expect(() => parseArgs(["--token=leaked"])).toThrow("prohibited");

    const opts = parseArgs(["--registry", "gitea"], process.cwd(), {
      GITHUB_SERVER_URL: "https://gitea.example.org",
      GITHUB_REPOSITORY_OWNER: "custom-org",
      RELEASE_BOT_TOKEN: "bot-secret-token",
    });
    expect(opts.host).toBe("gitea.example.org");
    expect(opts.owner).toBe("custom-org");
    expect(opts.token).toBe("bot-secret-token");

    const cratesIoOpts = parseArgs(["--registry", "crates.io"], process.cwd(), {
      CARGO_REGISTRY_TOKEN: "crates-oidc-token",
      GITEA_TOKEN: "should-not-use",
    });
    expect(cratesIoOpts.registry).toBe("crates-io");
    expect(cratesIoOpts.token).toBe("crates-oidc-token");
  });

  it("should parse CLI args and execute dry-run pipeline", async () => {
    const opts = parseArgs([
      "--host",
      "git.example.com",
      "--owner",
      "myorg",
      "--registry",
      "gitea",
      "--version",
      "4.9.3",
      "--crate",
      "cddm-core",
      "--crate",
      "cddm-cli",
      "--dry-run",
    ]);

    expect(opts.host).toBe("git.example.com");
    expect(opts.owner).toBe("myorg");
    expect(opts.registry).toBe("gitea");
    expect(opts.version).toBe("4.9.3");
    expect(opts.crates).toEqual(["cddm-core", "cddm-cli"]);
    expect(opts.dryRun).toBe(true);

    const results = await runCargoPublishPipeline(opts);
    expect(results).toEqual([
      { crateName: "cddm-core", status: "dry-run" },
      { crateName: "cddm-cli", status: "dry-run" },
    ]);
  });
});
