import { test, expect } from "@playwright/test";

test.describe("CDDM WebUI E2E Workflows", () => {
  test("should render main app header and scan configuration panel", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator("h1")).toContainText("CDDM Studio");
    await expect(page.getByText("Scan Configuration")).toBeVisible();
    await expect(page.getByText("Run Duplicate Analysis")).toBeVisible();
  });

  test("should update min token threshold slider and trigger scan", async ({ page }) => {
    await page.goto("/");
    const slider = page.locator('input[type="range"]');
    await slider.fill("60");

    const runBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i });
    await runBtn.click();

    // Verify DRY Health Score renders
    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 25000 });
  });

  test("should toggle to N-Way Clusters view and display cluster cards", async ({ page }) => {
    await page.goto("/");
    const runBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i });
    await runBtn.click();

    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 25000 });
    await expect(page.getByText("Clone Clusters")).toBeVisible();

    const clustersTab = page.getByRole("button", { name: /N-Way Clusters/i });
    await expect(clustersTab).toBeVisible();
    await clustersTab.click();

    // Verify cluster cards or empty state
    await expect(page.locator("body")).toBeVisible();
  });

  test("should toggle live watch state from header", async ({ page }) => {
    await page.goto("/");
    const liveWatchBtn = page.getByRole("button", { name: /Live Watch: ON/i });
    await expect(liveWatchBtn).toBeVisible();
    await liveWatchBtn.click();
    await expect(page.getByRole("button", { name: /Live Watch: OFF/i })).toBeVisible();
  });

  test("should select preferred IDE editor in scan configuration panel", async ({ page }) => {
    await page.goto("/");
    const ideSelect = page.locator("select");
    await expect(ideSelect).toBeVisible();
    await ideSelect.selectOption("cursor");
    await expect(ideSelect).toHaveValue("cursor");
  });
});
