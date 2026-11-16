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

  it("should build standard MCP server highlights using npx -p cddm cddm-mcp", () => {
    const mcp = buildMcpServerSection();
    expect(mcp).toContain("Model Context Protocol (MCP) Server Highlights");
    expect(mcp).toContain("cddm-mcp");
    expect(mcp).toContain("33 Specialized AI Agent Tools");
    expect(mcp).toContain("claude_desktop_config.json");
    expect(mcp).toContain("npx -p cddm cddm-mcp");

    const mcpNoNpm = buildMcpServerSection({ gitea: false, npmjs: false });
    expect(mcpNoNpm).not.toContain("NPM & NPX Ready");
  });

  it("should build 4-pillar interface coverage", () => {
    const parity = buildInterfaceParitySection();
    expect(parity).toContain("4-Pillar Cross-Interface Coverage");
    expect(parity).toContain("cddm");
    expect(parity).toContain("WebUI Studio");
    expect(parity).toContain("MCP Server");
    expect(parity).toContain("TUI Studio");
  });

  it("should build download and installation guides with qualified Homebrew and WinGet manifest URL", () => {
    const install = buildInstallationSection("4.0.1");
    expect(install).toContain("cddm-v4.0.1-x86_64-unknown-linux-gnu.tar.gz");
    expect(install).toContain("cddm-v4.0.1-x86_64-pc-windows-msvc.zip");
    expect(install).toContain("cddm-4.0.1.vsix");
    expect(install).toContain("cargo install cddm-cli cddm-mcp --registry gitea");
    expect(install).toContain("brew install gt-dev/tap/cddm");
    expect(install).toContain("winget install --manifest");
    expect(install).toContain("-p cddm cddm-mcp");
  });

  it("should conditionally render only verified Cargo and NPM channels when specified", () => {
    const cratesAndNpmjs = buildInstallationSection("4.9.4", {
      cargoChannels: { gitea: false, cratesIo: true },
      npmChannels: { gitea: false, npmjs: true },
    });
    expect(cratesAndNpmjs).toContain("cargo install cddm-cli cddm-mcp\n");
    expect(cratesAndNpmjs).not.toContain("--registry gitea");
    expect(cratesAndNpmjs).toContain("npm install -g cddm\nnpx -p cddm cddm-mcp");

    const noneLive = buildInstallationSection("4.9.4", {
      cargoChannels: { gitea: false, cratesIo: false },
      npmChannels: { gitea: false, npmjs: false },
    });
    expect(noneLive).not.toContain("cargo install");
    expect(noneLive).not.toContain("npm install");

    const bothLive = buildInstallationSection("4.9.4", {
      cargoChannels: { gitea: true, cratesIo: true },
      npmChannels: { gitea: true, npmjs: true },
    });
    expect(bothLive).toContain("# Cargo (crates.io)\ncargo install cddm-cli cddm-mcp");
    expect(bothLive).toContain("cargo install cddm-cli cddm-mcp --registry gitea");
    expect(bothLive).toContain("# NPM / NPX (Gitea Private NPM Registry)");
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

  it("should catch invalid release notes containing placeholder stubs, missing MCP, or unverified bare npx/brew commands", () => {
    const badNotes =
      "Automated semantic milestone release for v4.0.1.\n\nSee CHANGELOG.md for detailed component changes.\n\nnpx cddm-mcp\nbrew install cddm\n";
    const validation = validateReleaseNotesContent(badNotes);

    expect(validation.valid).toBe(false);
    expect(validation.errors.some((e) => e.includes("See CHANGELOG.md"))).toBe(true);
    expect(validation.errors.some((e) => e.includes("npx -p cddm cddm-mcp"))).toBe(true);
    expect(validation.errors.some((e) => e.includes("brew install"))).toBe(true);
  });
});
