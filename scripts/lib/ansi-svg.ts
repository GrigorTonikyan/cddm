/**
 * Pure TypeScript ANSI SGR parser, Asciinema v2 serializer, and vector SVG terminal renderer.
 * Converts real captured CLI ANSI output into clean Asciinema .cast recordings and crisp SVGs.
 */

export interface CastFrame {
  timeOffsetSec: number;
  data: string;
}

export interface AsciinemaHeader {
  version: 2;
  width: number;
  height: number;
  timestamp: number;
  title: string;
  env: {
    TERM: string;
    SHELL: string;
  };
}

interface TextSpan {
  text: string;
  color: string;
  bold: boolean;
  dim: boolean;
}

const DEFAULT_FG = "#e2e8f0";

const SGR_COLOR_MAP: Record<number, string> = {
  30: "#475569",
  31: "#f87171",
  32: "#34d399",
  33: "#fbbf24",
  34: "#60a5fa",
  35: "#c084fc",
  36: "#22d3ee",
  37: "#cbd5e1",
  90: "#64748b",
  91: "#fca5a5",
  92: "#6ee7b7",
  93: "#fde68a",
  94: "#93c5fd",
  95: "#d8b4fe",
  96: "#67e8f9",
  97: "#f8fafc",
};

const ESC_CHAR = String.fromCharCode(0x1b);
const SGR_SEQUENCE_REGEX = new RegExp(`${ESC_CHAR}\\[([0-9;]*)m`, "g");
const ANY_CSI_SEQUENCE_REGEX = new RegExp(`${ESC_CHAR}\\[[0-9;]*[A-Za-z]`, "g");

export function stripExtendedUnicode(input: string): string {
  // Preserve ASCII, common box-drawing (U+2500..U+257F), block elements, and ESC sequences;
  // remove emoji / variation-selector / pictograph codepoints to guarantee zero-emoji compliance.
  let filtered = "";
  for (const ch of input) {
    const cp = ch.codePointAt(0) ?? 0;
    const isEmojiOrSelector =
      (cp >= 0x1f000 && cp <= 0x1faff) ||
      (cp >= 0x2600 && cp <= 0x27bf) ||
      (cp >= 0xfe00 && cp <= 0xfe0f);
    if (!isEmojiOrSelector) {
      filtered += ch;
    }
  }
  return filtered;
}

export function stripAnsiCodes(input: string): string {
  return input.replace(ANY_CSI_SEQUENCE_REGEX, "");
}

function escapeXml(raw: string): string {
  return raw
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function parseAnsiLine(line: string): TextSpan[] {
  const spans: TextSpan[] = [];
  let lastIdx = 0;
  let color = DEFAULT_FG;
  let bold = false;
  let dim = false;

  for (const match of line.matchAll(SGR_SEQUENCE_REGEX)) {
    const matchIdx = match.index ?? 0;
    if (matchIdx > lastIdx) {
      spans.push({ text: line.slice(lastIdx, matchIdx), color, bold, dim });
    }
    const rawParams = match[1] ? match[1].split(";").map((n) => Number.parseInt(n, 10) || 0) : [0];
    for (let i = 0; i < rawParams.length; i++) {
      const code = rawParams[i] ?? 0;
      const mappedColor = SGR_COLOR_MAP[code];
      if (code === 0) {
        color = DEFAULT_FG;
        bold = false;
        dim = false;
      } else if (code === 1) {
        bold = true;
      } else if (code === 2) {
        dim = true;
      } else if (code === 22) {
        bold = false;
        dim = false;
      } else if (code === 39) {
        color = DEFAULT_FG;
      } else if (mappedColor) {
        color = mappedColor;
      } else if (code === 38 && rawParams[i + 1] === 2 && i + 4 < rawParams.length) {
        const r = rawParams[i + 2] ?? 0;
        const g = rawParams[i + 3] ?? 0;
        const b = rawParams[i + 4] ?? 0;
        color = `rgb(${r},${g},${b})`;
        i += 4;
      }
    }
    lastIdx = matchIdx + match[0].length;
  }

  if (lastIdx < line.length) {
    spans.push({ text: line.slice(lastIdx), color, bold, dim });
  }
  return spans;
}

export function buildAsciinemaV2Cast(
  frames: CastFrame[],
  options: { width?: number; height?: number; title?: string; timestamp?: number } = {},
): string {
  const header: AsciinemaHeader = {
    version: 2,
    width: options.width ?? 112,
    height: options.height ?? 34,
    timestamp: options.timestamp ?? Math.floor(Date.now() / 1000),
    title: options.title ?? "CDDM CLI Showcase",
    env: { TERM: "xterm-256color", SHELL: "/bin/bash" },
  };
  const lines = [JSON.stringify(header)];
  for (const frame of frames) {
    const cleanData = stripExtendedUnicode(frame.data)
      .replaceAll("\r\n", "\n")
      .replaceAll("\n", "\r\n");
    lines.push(JSON.stringify([Number(frame.timeOffsetSec.toFixed(3)), "o", cleanData]));
  }
  return `${lines.join("\n")}\n`;
}

export function renderAnsiTextToSvg(
  rawAnsi: string,
  options: { title?: string; maxLines?: number; widthPx?: number } = {},
): string {
  const title = options.title ?? "cddm scan . --summary";
  const maxLines = options.maxLines ?? 36;
  const widthPx = options.widthPx ?? 960;
  const lineHeight = 20;
  const paddingX = 24;
  const headerHeight = 44;

  const cleaned = stripExtendedUnicode(rawAnsi)
    .replace(/\r\n/g, "\n")
    .replace(/\r/g, "")
    .replace(/\n{3,}/g, "\n\n")
    .trim();
  const allLines = cleaned.split("\n");
  const visibleLines = allLines.slice(0, maxLines);
  const heightPx = headerHeight + 20 + visibleLines.length * lineHeight + 20;

  const lineSvgElements = visibleLines.map((rawLine, idx) => {
    const y = headerHeight + 28 + idx * lineHeight;
    const spans = parseAnsiLine(rawLine);
    if (spans.length === 0) {
      return `<text x="${paddingX}" y="${y}" xml:space="preserve"> </text>`;
    }
    const tspans = spans
      .map((span) => {
        const attrs = [`fill="${span.color}"`];
        if (span.bold) attrs.push(`font-weight="700"`);
        if (span.dim) attrs.push(`opacity="0.7"`);
        return `<tspan ${attrs.join(" ")}>${escapeXml(span.text)}</tspan>`;
      })
      .join("");
    return `<text x="${paddingX}" y="${y}" xml:space="preserve">${tspans}</text>`;
  });

  return `<svg xmlns="http://www.w3.org/2000/svg" width="${widthPx}" height="${heightPx}" viewBox="0 0 ${widthPx} ${heightPx}" role="img" aria-label="${escapeXml(title)}">
  <defs>
    <linearGradient id="termBg" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0%" stop-color="#0b1120"/>
      <stop offset="100%" stop-color="#0f172a"/>
    </linearGradient>
  </defs>
  <rect width="${widthPx}" height="${heightPx}" rx="12" fill="url(#termBg)" stroke="#1e293b" stroke-width="1.5"/>
  <rect x="0.75" y="0.75" width="${widthPx - 1.5}" height="${headerHeight}" rx="11" fill="#1e293b"/>
  <rect x="0.75" y="${headerHeight - 10}" width="${widthPx - 1.5}" height="10" fill="#1e293b"/>
  <circle cx="24" cy="22" r="6" fill="#ef4444"/>
  <circle cx="44" cy="22" r="6" fill="#f59e0b"/>
  <circle cx="64" cy="22" r="6" fill="#10b981"/>
  <text x="${Math.round(widthPx / 2)}" y="27" text-anchor="middle" fill="#94a3b8" font-family="ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace" font-size="13" font-weight="600">${escapeXml(title)}</text>
  <g font-family="ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace" font-size="13.5">
    ${lineSvgElements.join("\n    ")}
  </g>
</svg>
`;
}
