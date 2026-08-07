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
});
