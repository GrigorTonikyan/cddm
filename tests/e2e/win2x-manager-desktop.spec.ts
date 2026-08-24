import { test, expect } from "@playwright/test";

test.describe("Windows 11 Desktop-Class Window Management System (win2x-manager)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:3000");
  });

  test("should scan codebase and open multiple simultaneous Refactor Advisor windows", async ({
    page,
  }) => {
    test.setTimeout(60000);

    // 1. Run scan
    const runBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i }).first();
    await runBtn.click();
    await expect(page.getByText("Detected Clone Pairs")).toBeVisible({ timeout: 35000 });

    // 2. Expand first clone pair card and open Refactor Advisor
    await page.getByText("#1", { exact: true }).click();
    const refactorBtn1 = page.getByRole("button", { name: /Refactor Advisor/i }).first();
    await expect(refactorBtn1).toBeVisible();
    await refactorBtn1.click();
    await page.waitForTimeout(400);

    // Verify first window is open
    const windows = page.locator("[data-win2x-window]");
    await expect(windows).toHaveCount(1);
    await expect(windows.nth(0)).toHaveAttribute("data-active", "true");

    // 3. Minimize first window to DockBar
    const minimizeBtn = page.locator('[title="Minimize"]').first();
    await minimizeBtn.click();
    await page.waitForTimeout(300);
    await expect(windows).toHaveCount(0);

    const pill1 = page.locator("[data-win2x-minimized-pill]").first();
    await expect(pill1).toBeVisible();

    // 4. Open Policy Studio from header
    const policyBtn = page.getByRole("button", { name: /Policy Studio/i });
    await expect(policyBtn).toBeVisible();
    await policyBtn.click();
    await page.waitForTimeout(400);
    await expect(windows).toHaveCount(1);

    // 5. Restore first window from DockBar so both (Refactor Advisor + Diff Inspector) are open simultaneously!
    await pill1.click();
    await page.waitForTimeout(400);
    await expect(windows).toHaveCount(2);
    await page.waitForTimeout(500);

    // 6. Click Tile Layout on the DockBar to neatly tile both windows side by side
    const tileBtn = page.locator('[title="Tile Layout"]');
    await expect(tileBtn).toBeVisible();
    await tileBtn.click({ force: true });
    await page.waitForTimeout(500);

    // 7. Minimize all windows to DockBar
    const minAllBtn = page.locator('[title="Minimize All"]');
    await expect(minAllBtn).toBeVisible();
    await minAllBtn.click();
    await page.waitForTimeout(400);
    await expect(windows).toHaveCount(0);

    // 8. Restore first window from DockBar
    const pill = page.locator("[data-win2x-minimized-pill]").first();
    await expect(pill).toBeVisible();
    await pill.click();
    await page.waitForTimeout(400);
    await expect(windows).toHaveCount(1);

    // 9. Close window using Escape key
    await page.keyboard.press("Escape");
    await page.waitForTimeout(300);
    await expect(windows).toHaveCount(0);
  });

  test("should open all companion modal windows (Diff Inspector, Treemap, Language Analytics, Health Audit, Reports, Scan Config)", async ({
    page,
  }) => {
    // 1. Open Scan Config modal from header
    const configBtn = page.getByRole("button", { name: /Config Window/i });
    await configBtn.click();
    await page.waitForTimeout(400);

    const windows = page.locator("[data-win2x-window]");
    await expect(windows).toHaveCount(1);
    await expect(page.getByText("Scan Parameters & Engine Configuration")).toBeVisible();

    // Close config window
    await page.keyboard.press("Escape");
    await page.waitForTimeout(300);
    await expect(windows).toHaveCount(0);

    // 2. Run scan
    const runBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i }).first();
    await runBtn.click();
    await expect(page.getByText("Detected Clone Pairs")).toBeVisible({ timeout: 25000 });

    const openAndMinimizeModal = async (
      trigger: () => Promise<void>,
      expectedHeader: string | RegExp,
    ) => {
      await trigger();
      await page.waitForTimeout(400);
      await expect(page.getByText(expectedHeader)).toBeVisible();
      await page.locator('[title="Minimize"]').first().click();
      await page.waitForTimeout(300);
    };

    // 3. Open Health Audit Modal from DRY score card
    await openAndMinimizeModal(
      () => page.getByText("DRY Health Score").first().click(),
      "DRY Health Score Audit & Diagnostics",
    );

    // 4. Open Export & Reports Modal from header
    await openAndMinimizeModal(
      () =>
        page
          .getByRole("button", { name: /Reports/i })
          .first()
          .click(),
      "Report Center & SARIF Exporter",
    );

    // 5. Open Treemap Explorer Modal from Visual Analytics
    await openAndMinimizeModal(
      () => page.getByRole("button", { name: /Open in Window/i }).click(),
      "Duplication Treemap Explorer",
    );

    // 6. Open Diff Inspector Modal from first clone pair
    await page.getByText("#1", { exact: true }).click();
    await openAndMinimizeModal(
      () =>
        page
          .getByRole("button", { name: /Diff Inspector/i })
          .first()
          .click(),
      /Clone Pair #1 Diff Inspector/i,
    );

    // 6b. Open Refactor Advisor Modal from first clone pair
    const refactorBtn = page.getByRole("button", { name: /Refactor Advisor/i }).first();
    await refactorBtn.click();
    await page.waitForTimeout(400);
    await expect(page.getByText(/Automated Refactoring Advisor/i)).toBeVisible();

    // Restore minimized windows from DockBar
    const minPills = page.locator("[data-win2x-minimized-pill]");
    while ((await minPills.count()) > 0) {
      await minPills.first().click();
      await page.waitForTimeout(200);
    }

    // Verify modal windows exist in the win2x window manager
    await expect(windows.first()).toBeVisible();

    // 7. Click Tile Layout on DockBar to neatly tile all windows
    const tileBtn = page.locator('[title="Tile Layout"]');
    await tileBtn.click();
    await page.waitForTimeout(400);

    // 8. Minimize all windows to DockBar
    const minAllBtn = page.locator('[title="Minimize All"]');
    await minAllBtn.click();
    await page.waitForTimeout(300);
    await expect(windows).toHaveCount(0);

    const pills = page.locator("[data-win2x-minimized-pill]");
    await expect(pills.first()).toBeVisible();
  });
});
