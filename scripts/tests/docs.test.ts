import { describe, expect, it } from "bun:test";
import {
  REQUIRED_DOC_FILES,
  checkMarkdownLinks,
  checkMarkdownTables,
  checkRequiredDocFiles,
  validateDocumentation,
} from "../check-docs";

describe("Documentation Integrity Validator", () => {
  it("should have all required documentation files present in workspace", () => {
    const errors = checkRequiredDocFiles();
    expect(errors).toHaveLength(0);
    expect(REQUIRED_DOC_FILES.length).toBeGreaterThanOrEqual(12);
  });

  it("should detect broken markdown links in sample text", () => {
    const sample = `
# Title
[Valid README](../README.md)
[Broken Link](nonexistent_file_abc123.md)
[Web Link](https://github.com)
`;
    const { linkCount, errors } = checkMarkdownLinks("docs/TEST.md", sample);
    expect(linkCount).toBe(3);
    expect(errors.length).toBe(1);
    expect(errors[0]?.message).toContain("Broken internal markdown link");
  });

  it("should validate all actual repository markdown links without errors", async () => {
    const summary = await validateDocumentation();
    expect(summary.errors).toHaveLength(0);
    expect(summary.filesChecked).toBeGreaterThanOrEqual(12);
    expect(summary.linksChecked).toBeGreaterThan(30);
  });

  it("should detect malformed markdown tables", () => {
    const malformed = `
Some text without table header
| :--- | :--- |
| Cell 1 | Cell 2 |
`;
    const errors = checkMarkdownTables("test.md", malformed);
    expect(errors.length).toBe(1);
    expect(errors[0]?.message).toContain("Malformed markdown table");
  });
});
