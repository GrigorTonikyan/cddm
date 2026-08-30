import { describe, it, expect } from "bun:test";
import { generateChangelogSection } from "./version-updater";
import type { ParsedCommit } from "../version";

describe("version-updater utility", () => {
  it("generates markdown changelog section properly for categorized commits", () => {
    const mockCommits: ParsedCommit[] = [
      {
        hash: "abc1234",
        type: "feat",
        scope: "engine",
        subject: "add parallel parser recycling",
        breaking: false,
        raw: "feat(engine): add parallel parser recycling",
      },
      {
        hash: "def5678",
        type: "fix",
        scope: "mcp",
        subject: "handle missing parameter error codes",
        breaking: false,
        raw: "fix(mcp): handle missing parameter error codes",
      },
      {
        hash: "789ghij",
        type: "feat",
        scope: "api",
        subject: "breaking API v2 changes",
        breaking: true,
        raw: "feat(api)!: breaking API v2 changes",
      },
    ];

    const changelog = generateChangelogSection("1.8.0", "2026-08-30", mockCommits);
    expect(changelog).toContain("## [1.8.0] - 2026-08-30");
    expect(changelog).toContain("### BREAKING CHANGES");
    expect(changelog).toContain("breaking API v2 changes (`789ghij`)");
    expect(changelog).toContain("### Features");
    expect(changelog).toContain("**engine**: add parallel parser recycling (`abc1234`)");
    expect(changelog).toContain("### Bug Fixes");
    expect(changelog).toContain("**mcp**: handle missing parameter error codes (`def5678`)");
  });

  it("handles empty commit list gracefully", () => {
    const changelog = generateChangelogSection("1.0.0", "2026-08-30", []);
    expect(changelog).toBe("## [1.0.0] - 2026-08-30\n\n");
  });
});
