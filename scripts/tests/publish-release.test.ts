import { describe, expect, it } from "bun:test";
import {
  createOrGetGiteaRelease,
  getChangelogForVersion,
  parseArgs,
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
});
