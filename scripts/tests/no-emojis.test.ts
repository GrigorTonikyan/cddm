import { describe, expect, it } from "bun:test";
import { scanDirectoryForEmojis, scanFileForEmojis, EMOJI_REGEX } from "../check-no-emojis";
import { validateCommitMessage } from "../validate-commit-msg";

describe("No-Emoji Policy Scanner", () => {
  it("should detect common emojis and pictographic symbols", () => {
    const testCases = [
      "Feature description with rocket \u{1F680}",
      "Bug fix with caterpillar \u{1F41B}",
      "Docs with books \u{1F4DA}",
      "Tooling with hammer/wrench \u{1F6E0}",
      "Active tasks with target \u{1F3AF}",
      "Lightning fast \u{26A1}",
      "Palette design \u{1F3A8}",
      "Checkmark \u{2714} or cross \u{2716}",
      "Warning sign \u{26A0}",
      "Recycle sign \u{267B}",
    ];

    for (const text of testCases) {
      EMOJI_REGEX.lastIndex = 0;
      expect(EMOJI_REGEX.test(text)).toBe(true);
    }
  });

  it("should not trigger on clean standard code, markdown, and punctuation", () => {
    const cleanSamples = [
      "const foo = bar => ({ id: 123, name: 'cddm' });",
      "# Header 1\n## Header 2\n- List item\n| Col 1 | Col 2 |",
      "[PASS] All checks passed cleanly!",
      "[ERROR] Command failed with exit code 1",
      "Duplication percentage: 12.5% <= 15.0%",
      "a + b * c / d - e == f != g <= h >= i",
    ];

    for (const text of cleanSamples) {
      EMOJI_REGEX.lastIndex = 0;
      expect(EMOJI_REGEX.test(text)).toBe(false);
    }
  });

  it("should reject commit messages containing emojis", () => {
    expect(validateCommitMessage("feat: \u{1F680} add AST indexer").valid).toBe(false);
    expect(validateCommitMessage("fix: \u{1F41B} resolve memory leak").valid).toBe(false);
    expect(validateCommitMessage("docs: update README \u{1F4DA}").valid).toBe(false);
    expect(validateCommitMessage("feat(core): add AST indexer").valid).toBe(true);
    expect(validateCommitMessage("fix(webui): resolve memory leak").valid).toBe(true);
  });

  it("should scan single files correctly and report 0 violations on clean files", () => {
    const readmeViolations = scanFileForEmojis("README.md");
    expect(readmeViolations).toEqual([]);
    expect(readmeViolations.length).toBe(0);

    const changelogViolations = scanFileForEmojis("CHANGELOG.md");
    expect(changelogViolations).toEqual([]);
    expect(changelogViolations.length).toBe(0);
  });

  it("should verify that the entire repository has 0 emoji violations", () => {
    const violations = scanDirectoryForEmojis(".");
    expect(violations).toEqual([]);
    expect(violations.length).toBe(0);
  });
});
