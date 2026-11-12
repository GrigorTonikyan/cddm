import { describe, expect, it } from "bun:test";
import {
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

describe("Gitea Cargo Registry Publisher", () => {
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

  it("should format Cargo Bearer auth headers idempotently", () => {
    expect(formatCargoAuthToken("abc123")).toBe("Bearer abc123");
    expect(formatCargoAuthToken("Bearer abc123")).toBe("Bearer abc123");
    expect(formatCargoAuthToken("token abc123")).toBe("token abc123");
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

  it("should build normalized Cargo registry environment variables", () => {
    const env = buildCargoPublishEnv(
      {
        host: "git.gt-web-dev.com",
        owner: "gt-dev",
        registry: "my-gitea",
        token: "secret-pat",
      },
      {},
    );
    expect(env.CARGO_REGISTRIES_MY_GITEA_INDEX).toBe(
      "sparse+https://git.gt-web-dev.com/api/packages/gt-dev/cargo/",
    );
    expect(env.CARGO_REGISTRIES_MY_GITEA_TOKEN).toBe("Bearer secret-pat");
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
