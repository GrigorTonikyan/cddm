import { describe, expect, it } from "bun:test";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { parseArgs, runPublishNpmPipeline } from "../publish-npm";

describe("NPM & OIDC Publisher Engine", () => {
  it("parses CLI arguments accurately and rejects --token flags", () => {
    expect(() => parseArgs(["--token", "secret"])).toThrow("prohibited");
    expect(() => parseArgs(["--token=secret"])).toThrow("prohibited");

    const opts = parseArgs(
      ["--registry", "npmjs", "--provenance", "--dry-run", "--dist-dir", "custom-dist"],
      {
        GITHUB_SERVER_URL: "https://git.example.com",
        GITHUB_REPOSITORY: "my-org/cddm",
        GITHUB_REPOSITORY_OWNER: "my-org",
      },
    );
    expect(opts.registry).toBe("npmjs");
    expect(opts.provenance).toBe(true);
    expect(opts.dryRun).toBe(true);
    expect(opts.distDir).toBe("custom-dist");
    expect(opts.host).toBe("git.example.com");
    expect(opts.owner).toBe("my-org");
  });

  it("executes dry-run pipeline for present platform archives and wrapper package", async () => {
    const tempDir = join(tmpdir(), `cddm-npm-pub-test-${Date.now()}`);
    const distDir = join(tempDir, "dist-release");
    mkdirSync(distDir, { recursive: true });
    mkdirSync(join(tempDir, "npm", "cddm"), { recursive: true });

    try {
      writeFileSync(
        join(tempDir, ".env"),
        "GITEA_HOST=git.example.com\nGITEA_REPO=my-org/cddm\nPUBLISHER=my-org\n",
      );
      writeFileSync(join(tempDir, "Cargo.toml"), '[workspace.package]\nversion = "4.10.3"\n');
      writeFileSync(
        join(tempDir, "npm", "cddm", "package.json"),
        '{"name":"cddm","version":"4.10.3"}',
      );
      writeFileSync(join(distDir, "cddm-v4.10.3-x86_64-unknown-linux-gnu.tar.gz"), "archive");

      const opts = parseArgs(["--registry", "gitea", "--dry-run"], {}, tempDir);
      opts.fetchFn = (async () =>
        new Response("Not Found", { status: 404 })) as unknown as typeof fetch;

      const published = await runPublishNpmPipeline(opts);
      expect(published).toEqual(["@cddm/linux-x64", "cddm"]);
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });
});
