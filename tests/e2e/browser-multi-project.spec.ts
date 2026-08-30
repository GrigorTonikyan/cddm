import { test, expect } from "@playwright/test";

test.describe("CDDM WebUI Browser Real-Time Multi-Project Analysis", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:3000");
    await page.waitForLoadState("networkidle");
  });

  test("should render CDDM Studio interface and header", async ({ page }) => {
    await expect(page.locator("h1")).toContainText("CDDM Studio");
    await expect(page.getByText("Scan Configuration")).toBeVisible();
    await expect(page.getByText("Run Duplicate Analysis")).toBeVisible();
  });

  test("should scan current CDDM repository and render live metrics", async ({ page }) => {
    const dirInput = page.locator('input[placeholder*="e.g. ./src"]');
    await dirInput.fill(".");

    const runBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i });
    await runBtn.click();

    // Verify all 5 metrics cards render with real data
    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 25000 });
    await expect(page.getByText("Duplication Rate")).toBeVisible();
    await expect(page.getByText("Files Scanned")).toBeVisible();
    await expect(page.getByText("Clone Pairs", { exact: true })).toBeVisible();
    await expect(page.getByText("Engine Speed")).toBeVisible();

    await page.screenshot({
      path: "C:/Users/admin/.gemini/antigravity/brain/9f317f72-1dff-4d9f-a2d3-01b03cf11a06/cddm_self_browser_scan.png",
      fullPage: true,
    });
  });

  test("should scan external project x:/projects/cctm in browser", async ({ page }) => {
    const dirInput = page.locator('input[placeholder*="e.g. ./src"]');
    await dirInput.fill("x:/projects/cctm");

    const runBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i });
    await runBtn.click();

    // Verify results for cctm project render
    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 30000 });
    await expect(page.getByText("Duplication Rate")).toBeVisible();
    await expect(page.getByText("Files Scanned")).toBeVisible();
    await expect(page.getByText("Clone Pairs", { exact: true })).toBeVisible();
    await expect(page.getByText("Detected Clone Pairs")).toBeVisible();

    await page.screenshot({
      path: "C:/Users/admin/.gemini/antigravity/brain/9f317f72-1dff-4d9f-a2d3-01b03cf11a06/cddm_cctm_browser_scan.png",
      fullPage: true,
    });
  });

  test("should scan external project x:/projects/consul-website in browser", async ({ page }) => {
    const dirInput = page.locator('input[placeholder*="e.g. ./src"]');
    await dirInput.fill("x:/projects/consul-website");

    const runBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i });
    await runBtn.click();

    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 30000 });
    await expect(page.getByText("Duplication Rate")).toBeVisible();
    await expect(page.getByText("Files Scanned")).toBeVisible();

    await page.screenshot({
      path: "C:/Users/admin/.gemini/antigravity/brain/9f317f72-1dff-4d9f-a2d3-01b03cf11a06/cddm_consul_browser_scan.png",
      fullPage: true,
    });
  });
});
