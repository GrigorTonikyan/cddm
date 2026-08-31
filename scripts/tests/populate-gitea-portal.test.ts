import { describe, expect, it } from "bun:test";
import { getApiHeaders, GITEA_BASE, GITEA_OWNER, GITEA_REPO } from "../lib/gitea-client";
import { SEED_ISSUES } from "../lib/gitea-issue-pr-data";
import { SEED_PRS } from "../lib/gitea-pr-sync";
import { WIKI_PAGES } from "../lib/gitea-wiki-data";

describe("Gitea Portal Population Suite", () => {
  it("should define standard API headers with token authentication", () => {
    const headers = getApiHeaders(true);
    expect(headers.Authorization).toBeString();
    expect(headers.Authorization?.startsWith("token ")).toBeTrue();
    expect(headers["Content-Type"]).toBe("application/json");
    expect(headers.Accept).toBe("application/json");
  });

  it("should configure valid repository constants", () => {
    expect(GITEA_BASE).toBe("https://git.gt-web-dev.com");
    expect(GITEA_OWNER).toBe("gt-dev");
    expect(GITEA_REPO).toBe("gt-dev/cddm");
  });

  it("should include complete seed issues covering all key pillars", () => {
    expect(SEED_ISSUES.length).toBeGreaterThanOrEqual(8);
    for (const issue of SEED_ISSUES) {
      expect(issue.title).toBeString();
      expect(issue.body.length).toBeGreaterThan(20);
      expect(issue.milestoneTitle).toBeString();
      expect(issue.labels.length).toBeGreaterThan(0);
    }
  });

  it("should include valid pull request specifications", () => {
    expect(SEED_PRS.length).toBeGreaterThanOrEqual(6);
    const branches = new Set<string>();
    for (const pr of SEED_PRS) {
      expect(pr.title).toBeString();
      expect(pr.branch).toBeString();
      expect(pr.body).toBeString();
      expect(branches.has(pr.branch)).toBeFalse();
      branches.add(pr.branch);
    }
  });

  it("should define comprehensive wiki documentation pages", () => {
    expect(WIKI_PAGES.length).toBe(9);
    const titles = WIKI_PAGES.map((p) => p.title);
    expect(titles).toContain("Home");
    expect(titles).toContain("Getting-Started");
    expect(titles).toContain("CLI-Reference");
    expect(titles).toContain("WebUI-Studio");
    expect(titles).toContain("MCP-Server-Protocol");
    expect(titles).toContain("TUI-Studio");
    expect(titles).toContain("AST-Engine-and-Deduplication");
    expect(titles).toContain("4-Pillar-Feature-Parity");
    expect(titles).toContain("CI-CD-and-Releases");

    for (const page of WIKI_PAGES) {
      expect(page.content.startsWith("#")).toBeTrue();
      expect(page.content.length).toBeGreaterThan(100);
    }
  });
});
