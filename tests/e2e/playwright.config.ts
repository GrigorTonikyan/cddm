import { defineConfig, devices } from "@playwright/test";

const isCI = Boolean(process.env.CI);

// 1. Screenshots: configurable, default ON ('on' | 'off' | 'only-on-failure')
const screenshot = (process.env.PLAYWRIGHT_SCREENSHOT || process.env.SCREENSHOT || "on") as
  | "on"
  | "off"
  | "only-on-failure";

// 2. HTML reporting: configurable, default OFF
const enableHtmlReport =
  process.env.PLAYWRIGHT_HTML_REPORT === "true" || process.env.HTML_REPORT === "true";
const reporter = enableHtmlReport
  ? [["list"], ["html", { open: "never", outputFolder: "playwright-report" }]]
  : [["list"]];

// 3. Fail fast: configurable, default ON (stop on first failure)
const failFast = process.env.PLAYWRIGHT_FAIL_FAST !== "false" && process.env.FAIL_FAST !== "false";
const maxFailures = failFast ? 1 : undefined;

// 4. Headed mode: configurable, default ON in local/dev, OFF in CI (headless in CI)
const isHeaded =
  process.env.PLAYWRIGHT_HEADED !== undefined
    ? process.env.PLAYWRIGHT_HEADED === "true"
    : process.env.HEADED !== undefined
      ? process.env.HEADED === "true"
      : !isCI;
const headless = !isHeaded;

export default defineConfig({
  testDir: ".",
  timeout: 90000,
  workers: 1,
  maxFailures,
  reporter: reporter as any,
  use: {
    baseURL: "http://localhost:3000",
    headless,
    screenshot,
    trace: "on-first-retry",
  },
  webServer: {
    command: "bun scripts/dev.ts",
    cwd: "../..",
    url: "http://localhost:3000",
    reuseExistingServer: !isCI,
    timeout: 60000,
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
});
