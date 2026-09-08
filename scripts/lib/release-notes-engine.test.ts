import { describe, expect, it } from "bun:test";
import {
  buildInstallationSection,
  buildInterfaceParitySection,
  buildMcpServerSection,
  extractChangelogSection,
  generateReleaseNotes,
  validateReleaseNotesContent,
} from "./release-notes-engine";

describe("release-notes-engine", () => {
  it("should extract changelog section for an existing version", () => {
    const section = extractChangelogSection("4.0.1");
    expect(section).toContain("## [4.0.1]");
    expect(section).toContain("Bug Fixes");
    expect(section).toContain("Tooling & Maintenance");
  });

  it("should fallback gracefully for non-existent version", () => {
    const section = extractChangelogSection("99.99.99");
    expect(section).toContain("### Release v99.99.99");
  });

  it("should build standard MCP server highlights", () => {
    const mcp = buildMcpServerSection();
    expect(mcp).toContain("Model Context Protocol (MCP) Server Highlights");
    expect(mcp).toContain("cddm-mcp");
    expect(mcp).toContain("33 Specialized AI Agent Tools");
    expect(mcp).toContain("claude_desktop_config.json");
    expect(mcp).toContain("npx cddm-mcp");
  });

  it("should build 4-pillar interface coverage", () => {
    const parity = buildInterfaceParitySection();
    expect(parity).toContain("4-Pillar Cross-Interface Coverage");
    expect(parity).toContain("cddm");
    expect(parity).toContain("WebUI Studio");
    expect(parity).toContain("MCP Server");
    expect(parity).toContain("TUI Studio");
  });

  it("should build download and installation guides", () => {
    const install = buildInstallationSection("4.0.1");
    expect(install).toContain("cddm-v4.0.1-x86_64-unknown-linux-gnu.tar.gz");
    expect(install).toContain("cddm-v4.0.1-x86_64-pc-windows-gnu.zip");
    expect(install).toContain("cddm-4.0.1.vsix");
    expect(install).toContain("cargo install cddm-cli cddm-mcp");
    expect(install).toContain("npm install -g cddm");
    expect(install).toContain("npx cddm-mcp");
  });

  it("should generate comprehensive valid release notes", () => {
    const notes = generateReleaseNotes("4.0.1", {
      milestoneTitle: "Rust Nightly Toolchain & Dependency Modernization",
    });

    expect(notes).toContain("## CDDM v4.0.1 — Rust Nightly Toolchain & Dependency Modernization");
    expect(notes).toContain("## [4.0.1]");
    expect(notes).toContain("Model Context Protocol (MCP)");
    expect(notes).toContain("cddm-mcp");
    expect(notes).toContain("Installation & Distribution");

    const validation = validateReleaseNotesContent(notes);
    expect(validation.valid).toBe(true);
    expect(validation.errors).toHaveLength(0);
  });

  it("should catch invalid release notes containing placeholder stubs or missing MCP", () => {
    const badNotes =
      "Automated semantic milestone release for v4.0.1.\n\nSee CHANGELOG.md for detailed component changes.";
    const validation = validateReleaseNotesContent(badNotes);

    expect(validation.valid).toBe(false);
    expect(validation.errors.some((e) => e.includes("See CHANGELOG.md"))).toBe(true);
    expect(validation.errors.some((e) => e.includes("MCP"))).toBe(true);
  });
});
