import { describe, expect, it } from "bun:test";
import {
  determineBump,
  incrementVersion,
  parseCommitLine,
  parseSemver,
  generateChangelogSection,
  type ParsedCommit,
} from "../version";
import { validateCommitMessage } from "../validate-commit-msg";

describe("Conventional Commits Validator", () => {
  it("should accept valid conventional commits", () => {
    expect(validateCommitMessage("feat: add M61 rolling hash").valid).toBe(true);
    expect(validateCommitMessage("fix(core): resolve AST parsing edge case").valid).toBe(true);
    expect(validateCommitMessage("docs(readme): update installation guide").valid).toBe(true);
    expect(validateCommitMessage("perf(winnow): optimize sliding window calculation").valid).toBe(
      true,
    );
    expect(validateCommitMessage("refactor: extract constants").valid).toBe(true);
    expect(validateCommitMessage("chore: bump dependencies").valid).toBe(true);
  });

  it("should accept breaking change commits with exclamation mark", () => {
    const res = validateCommitMessage("feat(api)!: redesign scan endpoint payload");
    expect(res.valid).toBe(true);
    expect(res.breaking).toBe(true);
  });

  it("should accept breaking change with footer", () => {
    const msg =
      "fix: change default threshold\n\nBREAKING CHANGE: minimum tokens now defaults to 50";
    const res = validateCommitMessage(msg);
    expect(res.valid).toBe(true);
    expect(res.breaking).toBe(true);
  });

  it("should reject non-conventional commit messages", () => {
    expect(validateCommitMessage("random commit message").valid).toBe(false);
    expect(validateCommitMessage("Fixed bug in detector").valid).toBe(false);
    expect(validateCommitMessage("feat:").valid).toBe(false);
  });
});

describe("Semantic Versioning Engine", () => {
  it("should parse semver strings correctly", () => {
    expect(parseSemver("0.1.2")).toEqual({ major: 0, minor: 1, patch: 2, raw: "0.1.2" });
    expect(parseSemver("v1.4.0")).toEqual({ major: 1, minor: 4, patch: 0, raw: "1.4.0" });
  });

  it("should calculate version increments accurately", () => {
    const v = { major: 0, minor: 1, patch: 2, raw: "0.1.2" };
    expect(incrementVersion(v, "patch").raw).toBe("0.1.3");
    expect(incrementVersion(v, "minor").raw).toBe("0.2.0");
    expect(incrementVersion(v, "major").raw).toBe("1.0.0");
    expect(incrementVersion(v, "none").raw).toBe("0.1.2");
  });

  it("should determine bump type based on conventional commits", () => {
    const commits: ParsedCommit[] = [
      {
        hash: "abc1",
        type: "docs",
        subject: "update readme",
        breaking: false,
        raw: "docs: update readme",
      },
      { hash: "abc2", type: "fix", subject: "fix typo", breaking: false, raw: "fix: fix typo" },
    ];
    expect(determineBump(commits)).toBe("patch");

    commits.push({
      hash: "abc3",
      type: "feat",
      subject: "add AST parser",
      breaking: false,
      raw: "feat: add AST parser",
    });
    expect(determineBump(commits)).toBe("minor");

    commits.push({
      hash: "abc4",
      type: "feat",
      subject: "breaking change",
      breaking: true,
      raw: "feat!: breaking change",
    });
    expect(determineBump(commits)).toBe("major");
  });

  it("should parse commit lines into parsed commit objects", () => {
    const parsed = parseCommitLine("1a2b3c4", "feat(detector): parallelize file tokenization");
    expect(parsed).not.toBeNull();
    expect(parsed?.type).toBe("feat");
    expect(parsed?.scope).toBe("detector");
    expect(parsed?.subject).toBe("parallelize file tokenization");
    expect(parsed?.breaking).toBe(false);
  });

  it("should format categorized changelog markdown", () => {
    const commits: ParsedCommit[] = [
      {
        hash: "abc1234",
        type: "feat",
        scope: "core",
        subject: "add AST hasher",
        breaking: false,
        raw: "",
      },
      { hash: "def5678", type: "fix", subject: "correct clone bounds", breaking: false, raw: "" },
    ];
    const changelog = generateChangelogSection("0.2.0", "2026-08-23", commits);
    expect(changelog).toContain("## [0.2.0] - 2026-08-23");
    expect(changelog).toContain("### 🚀 Features");
    expect(changelog).toContain("**core**: add AST hasher (`abc1234`)");
    expect(changelog).toContain("### 🐛 Bug Fixes");
    expect(changelog).toContain("correct clone bounds (`def5678`)");
  });
});
