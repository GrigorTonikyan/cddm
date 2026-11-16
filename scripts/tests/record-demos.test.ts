import { describe, expect, test } from "bun:test";
import {
  buildAsciinemaV2Cast,
  renderAnsiTextToSvg,
  stripAnsiCodes,
  stripExtendedUnicode,
} from "../lib/ansi-svg";
import { allocateEphemeralPort, parseDemoRecorderArgs } from "../record-demos";

describe("visual demo recorder", () => {
  test("parses CLI options and allocates valid ephemeral ports", () => {
    const opts = parseDemoRecorderArgs(["--assets-dir", "tmp/assets", "--port", "4123"]);
    expect(opts.assetsDir.endsWith("assets")).toBe(true);
    expect(opts.port).toBe(4123);

    const ephemeral = allocateEphemeralPort();
    expect(ephemeral).toBeGreaterThan(1024);
    expect(ephemeral).toBeLessThanOrEqual(65535);
  });

  test("serializes Asciinema v2 cast files cleanly without emoji artifacts", () => {
    const cast = buildAsciinemaV2Cast(
      [
        { timeOffsetSec: 0, data: "\x1b[32mPASS\x1b[0m\nClean code" },
        { timeOffsetSec: 1.25, data: "Next line \u{1F680}" },
      ],
      { width: 100, height: 30, title: "Demo", timestamp: 1700000000 },
    );

    const lines = cast.trim().split("\n");
    expect(lines).toHaveLength(3);
    const header = JSON.parse(lines[0] ?? "{}") as {
      version: number;
      width: number;
      title: string;
    };
    expect(header.version).toBe(2);
    expect(header.width).toBe(100);
    expect(header.title).toBe("Demo");

    const frame2 = JSON.parse(lines[2] ?? "[]") as [number, string, string];
    expect(frame2[0]).toBe(1.25);
    expect(frame2[1]).toBe("o");
    expect(frame2[2]).toBe("Next line ");
  });

  test("renders ANSI SGR codes safely to XML-escaped dark terminal SVG", () => {
    const svg = renderAnsiTextToSvg('\x1b[1;36mcddm\x1b[0m <scan> & "verify"\n\x1b[32mOK\x1b[0m', {
      title: "Terminal Demo",
    });

    expect(svg).toContain('<svg xmlns="http://www.w3.org/2000/svg"');
    expect(svg).toContain('fill="#22d3ee"');
    expect(svg).toContain('font-weight="700"');
    expect(svg).toContain("&lt;scan&gt; &amp; &quot;verify&quot;");
    expect(stripAnsiCodes("\x1b[32mOK\x1b[0m")).toBe("OK");
    expect(stripExtendedUnicode("abc \u{1F525} def")).toBe("abc  def");
  });
});
