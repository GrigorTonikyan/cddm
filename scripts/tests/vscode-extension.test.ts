import { describe, expect, it } from "bun:test";
import {
  DEFAULT_MIN_TOKENS,
  DEFAULT_STUDIO_URL,
  SUPPORTED_LANGUAGES,
} from "../../editors/vscode/src/constants";

describe("VS Code Extension Polyglot Language Matrix", () => {
  it("should contain all 24 supported programming languages", () => {
    expect(SUPPORTED_LANGUAGES).toContain("rust");
    expect(SUPPORTED_LANGUAGES).toContain("typescript");
    expect(SUPPORTED_LANGUAGES).toContain("typescriptreact");
    expect(SUPPORTED_LANGUAGES).toContain("javascript");
    expect(SUPPORTED_LANGUAGES).toContain("javascriptreact");
    expect(SUPPORTED_LANGUAGES).toContain("python");
    expect(SUPPORTED_LANGUAGES).toContain("go");
    expect(SUPPORTED_LANGUAGES).toContain("c");
    expect(SUPPORTED_LANGUAGES).toContain("cpp");
    expect(SUPPORTED_LANGUAGES).toContain("java");
    expect(SUPPORTED_LANGUAGES).toContain("csharp");
    expect(SUPPORTED_LANGUAGES).toContain("ruby");
    expect(SUPPORTED_LANGUAGES).toContain("php");
    expect(SUPPORTED_LANGUAGES).toContain("swift");
    expect(SUPPORTED_LANGUAGES).toContain("shellscript");
    expect(SUPPORTED_LANGUAGES).toContain("lua");
    expect(SUPPORTED_LANGUAGES).toContain("json");
    expect(SUPPORTED_LANGUAGES).toContain("html");
    expect(SUPPORTED_LANGUAGES).toContain("kotlin");
    expect(SUPPORTED_LANGUAGES).toContain("zig");
    expect(SUPPORTED_LANGUAGES).toContain("scala");
    expect(SUPPORTED_LANGUAGES).toContain("elixir");
    expect(SUPPORTED_LANGUAGES).toContain("sql");
    expect(SUPPORTED_LANGUAGES).toContain("dockerfile");
    expect(SUPPORTED_LANGUAGES.length).toBeGreaterThanOrEqual(24);
  });

  it("should have valid default configuration constants", () => {
    expect(DEFAULT_STUDIO_URL).toBe("http://127.0.0.1:3000");
    expect(DEFAULT_MIN_TOKENS).toBe(50);
  });
});
