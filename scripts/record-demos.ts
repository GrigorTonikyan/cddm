#!/usr/bin/env bun
/**
 * Reproducible CLI & WebUI Studio visual demo recorder for CDDM.
 * Executes live cddm CLI scans and ephemeral cddm serve Playwright sessions
 * to generate crisp terminal SVG, Asciinema v2 .cast, and Retina PNG screenshots.
 */

import { mkdir } from "node:fs/promises";
import { join, resolve } from "node:path";
import { parseArgs } from "node:util";
import { chromium } from "@playwright/test";
import { resolveCddmBinary } from "./benchmark-ecosystem";
import { buildAsciinemaV2Cast, renderAnsiTextToSvg, type CastFrame } from "./lib/ansi-svg";
import { printScriptBanner, printScriptHelp } from "./lib/step-runner";

export interface DemoRecorderOptions {
  assetsDir: string;
  castPath: string;
  cddmBin: string;
  port?: number;
  help: boolean;
}

export function parseDemoRecorderArgs(argv: string[]): DemoRecorderOptions {
  const { values } = parseArgs({
    args: argv,
    options: {
      "assets-dir": { type: "string", default: join("docs", "assets") },
      "cast-path": { type: "string", default: join("packaging", "demos", "cddm-cli-demo.cast") },
      "cddm-bin": { type: "string" },
      port: { type: "string" },
      help: { type: "boolean", short: "h", default: false },
    },
  });
  const parsedPort = values.port ? Number.parseInt(values.port, 10) : undefined;
  return {
    assetsDir: resolve(values["assets-dir"] ?? join("docs", "assets")),
    castPath: resolve(values["cast-path"] ?? join("packaging", "demos", "cddm-cli-demo.cast")),
    cddmBin: resolveCddmBinary(values["cddm-bin"]),
    port: parsedPort && Number.isFinite(parsedPort) ? parsedPort : undefined,
    help: Boolean(values.help),
  };
}

export function allocateEphemeralPort(): number {
  const server = Bun.listen({
    hostname: "127.0.0.1",
    port: 0,
    socket: {
      data() {},
    },
  });
  const port = server.port;
  server.stop(true);
  return port;
}

async function waitForHealth(baseUrl: string, timeoutMs = 15000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const res = await fetch(`${baseUrl}/api/health`);
      if (res.ok) return;
    } catch {
      // Server still booting
    }
    await Bun.sleep(150);
  }
  throw new Error(`Timed out waiting for CDDM WebUI server health check at ${baseUrl}`);
}

async function captureCliShowcase(
  cddmBin: string,
  svgOutPath: string,
  castOutPath: string,
): Promise<{ castFrames: number; svgBytes: number }> {
  const frames: CastFrame[] = [];
  let cursorTime = 0.0;
  const promptPrefix = "\x1b[36m$\x1b[0m ";

  const commands: Array<{ label: string; args: string[] }> = [
    { label: "cddm --version", args: ["--version"] },
    {
      label: "cddm scan . --summary --min-tokens 50",
      args: ["scan", ".", "--summary", "--min-tokens", "50"],
    },
  ];

  const transcriptParts: string[] = [];

  for (const item of commands) {
    const cmdBanner = `${promptPrefix}\x1b[1m${item.label}\x1b[0m\n`;
    frames.push({ timeOffsetSec: cursorTime, data: cmdBanner });
    transcriptParts.push(cmdBanner);
    cursorTime += 0.35;

    const proc = Bun.spawn([cddmBin, ...item.args], {
      stdout: "pipe",
      stderr: "pipe",
      env: { ...process.env, CLICOLOR_FORCE: "1", FORCE_COLOR: "1" },
    });
    const start = performance.now();
    const [stdout, exitCode] = await Promise.all([new Response(proc.stdout).text(), proc.exited]);
    const elapsedSec = Math.max(0.2, (performance.now() - start) / 1000);
    if (exitCode !== 0 && exitCode !== 1) {
      throw new Error(`CLI showcase command failed (${exitCode}): ${item.label}`);
    }

    frames.push({ timeOffsetSec: cursorTime, data: `${stdout}\n` });
    transcriptParts.push(stdout);
    cursorTime += elapsedSec + 0.5;
  }

  const castContent = buildAsciinemaV2Cast(frames, {
    width: 112,
    height: 36,
    title: "CDDM CLI Showcase",
  });
  await mkdir(resolve(castOutPath, ".."), { recursive: true });
  await Bun.write(castOutPath, castContent);

  const svgContent = renderAnsiTextToSvg(transcriptParts.join("\n"), {
    title: "CDDM CLI -- Code De-Duplication Meister",
    maxLines: 36,
    widthPx: 960,
  });
  await mkdir(resolve(svgOutPath, ".."), { recursive: true });
  await Bun.write(svgOutPath, svgContent);

  return { castFrames: frames.length, svgBytes: Buffer.byteLength(svgContent, "utf-8") };
}

async function captureWebUiScreenshots(
  cddmBin: string,
  assetsDir: string,
  portOverride?: number,
): Promise<{ dashboardPath: string; studioPath: string }> {
  await mkdir(assetsDir, { recursive: true });
  const port = portOverride ?? allocateEphemeralPort();
  const baseUrl = `http://127.0.0.1:${port}`;
  const serverProc = Bun.spawn([cddmBin, "serve", "--port", String(port)], {
    stdout: "pipe",
    stderr: "pipe",
  });

  const dashboardPath = join(assetsDir, "cddm-webui-studio.png");
  const studioPath = join(assetsDir, "cddm-webui-refactor.png");
  const browser = await chromium.launch({ headless: true });

  try {
    await waitForHealth(baseUrl);
    const context = await browser.newContext({
      viewport: { width: 1440, height: 900 },
      deviceScaleFactor: 2,
    });
    const page = await context.newPage();
    await page.goto(baseUrl, { waitUntil: "networkidle" });

    // Wait for initial mount scan results to render DRY Health Score card cleanly
    await page
      .getByText(/DRY Health Score/i)
      .first()
      .waitFor({ state: "visible", timeout: 30000 });
    const collapseBtn = page.getByRole("button", { name: /Collapse/i }).first();
    if (await collapseBtn.isVisible().catch(() => false)) {
      await collapseBtn.click();
    }
    await page.waitForTimeout(500);
    await page.screenshot({ path: dashboardPath, fullPage: false });

    // Open Refactor Studio / interactive modal for second visual demo
    const refactorButton = page.getByRole("button", { name: /Refactor Studio/i }).first();
    await refactorButton.click();
    await page
      .getByRole("heading", { name: /Refactor Studio|Semantic|Refactor/i })
      .first()
      .waitFor({
        state: "visible",
        timeout: 15000,
      });
    await page.waitForTimeout(600);
    await page.screenshot({ path: studioPath, fullPage: false });
    await context.close();
  } finally {
    await browser.close();
    serverProc.kill();
    await serverProc.exited.catch(() => {});
  }

  return { dashboardPath, studioPath };
}

async function main(): Promise<void> {
  const options = parseDemoRecorderArgs(process.argv.slice(2));
  if (options.help) {
    printScriptHelp("Visual Demo Recorder", "bun scripts/record-demos.ts [options]", [
      ["--assets-dir <dir>", "Output directory for PNG/SVG visual demos (default: docs/assets)"],
      [
        "--cast-path <file>",
        "Output Asciinema v2 .cast path (default: packaging/demos/cddm-cli-demo.cast)",
      ],
      ["--cddm-bin <path>", "Explicit path to cddm binary (defaults to target/release/cddm)"],
      ["--port <port>", "Optional HTTP port for ephemeral WebUI server"],
      ["-h, --help", "Show help message"],
    ]);
    return;
  }

  printScriptBanner("CDDM Visual Demo Recorder");
  const svgOutPath = join(options.assetsDir, "cddm-cli-demo.svg");
  const cliResult = await captureCliShowcase(options.cddmBin, svgOutPath, options.castPath);
  console.log(
    `[OK] Recorded Asciinema cast (${cliResult.castFrames} frames) -> ${options.castPath}`,
  );
  console.log(`[OK] Rendered CLI SVG (${cliResult.svgBytes} bytes) -> ${svgOutPath}`);

  const webResult = await captureWebUiScreenshots(options.cddmBin, options.assetsDir, options.port);
  console.log(`[OK] Captured WebUI Dashboard -> ${webResult.dashboardPath}`);
  console.log(`[OK] Captured WebUI Refactor Studio -> ${webResult.studioPath}`);
}

if (import.meta.main) {
  await main();
}
