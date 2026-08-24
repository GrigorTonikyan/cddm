import { test, expect } from "@playwright/test";

test.describe("Windows 11 Desktop-Class Window Management System (win2x-manager)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:3000");
  });

  test("should scan codebase and open multiple simultaneous Refactor Advisor windows", async ({
    page,
  }) => {
    // 1. Run scan
    const dirInput = page.locator('input[placeholder*="e.g. ./src"]');
    await dirInput.fill(".");
    const runBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i });
    await runBtn.click();

    // Wait for clone pairs list to render
    await expect(page.getByText("Detected Clone Pairs")).toBeVisible({ timeout: 25000 });

    // 2. Expand first clone pair card and open Refactor Advisor
    await page.getByText("#1", { exact: true }).click();
    const refactorBtn1 = page.getByRole("button", { name: /Refactor Advisor/i }).first();
    await expect(refactorBtn1).toBeVisible();
    await refactorBtn1.click();

    // Verify first window is open
    const windows = page.locator("[data-win2x-window]");
    await expect(windows).toHaveCount(1);
    await expect(windows.nth(0)).toHaveAttribute("data-active", "true");

    // 3. Minimize first window to DockBar
    const minimizeBtn = page.locator('[title="Minimize"]').first();
    await minimizeBtn.click();
    await expect(windows).toHaveCount(0);

    const pill1 = page.locator("[data-win2x-minimized-pill]").first();
    await expect(pill1).toBeVisible();

    // 4. Open second Refactor Advisor window from card 2
    const card2Header = page.getByText("#2", { exact: true });
    await card2Header.click();

    const cloneCards = page.locator(".group.bg-slate-900\\/70");
    const refactorBtn2 = cloneCards.nth(1).getByRole("button", { name: /Refactor Advisor/i });
    await expect(refactorBtn2).toBeVisible();
    await refactorBtn2.click();
    await expect(windows).toHaveCount(1);

    // 5. Restore first window from DockBar so both are open simultaneously!
    await pill1.click();
    await expect(windows).toHaveCount(2);

    // 6. Click Tile Layout on the DockBar to neatly tile both windows side by side
    const tileBtn = page.locator('[title="Tile Layout"]');
    await expect(tileBtn).toBeVisible();
    await tileBtn.click();

    // 7. Foreground Elevation: Click on window 0 to bring it to foreground
    await windows.nth(0).locator("[data-win2x-titlebar]").click();
    await expect(windows.nth(0)).toHaveAttribute("data-active", "true");
    await expect(windows.nth(1)).toHaveAttribute("data-active", "false");

    // Click on window 1 to bring window 1 to foreground
    await windows.nth(1).locator("[data-win2x-titlebar]").click();
    await expect(windows.nth(1)).toHaveAttribute("data-active", "true");
    await expect(windows.nth(0)).toHaveAttribute("data-active", "false");

    // 8. Context Menu: Right click titlebar of active window
    await windows.nth(1).locator("[data-win2x-titlebar]").click({ button: "right" });

    // Verify context menu renders with Windows 11 options
    const contextMenu = page.locator("[data-win2x-context-menu]");
    await expect(contextMenu).toBeVisible();
    await expect(contextMenu.getByText("Cascade All")).toBeVisible();
    await expect(contextMenu.getByText("Tile All")).toBeVisible();
    await expect(contextMenu.getByText("Minimize")).toBeVisible();

    // Click Cascade All from context menu
    await contextMenu.getByText("Cascade All").click();
    await expect(contextMenu).not.toBeVisible();

    // 9. Windows 11 Snap Layouts Flyout: Hover over the Maximize button of active window
    const maxBtn = windows.nth(1).locator('[title="Maximize"]');
    await maxBtn.hover();
    // Wait for 300ms hover delay
    await page.waitForTimeout(350);

    const snapMenu = page.locator("[data-win2x-snap-layouts-menu]");
    await expect(snapMenu).toBeVisible();

    // Select the first slot of 50/50 Split
    const firstSlot = snapMenu.locator("button").first();
    await firstSlot.click();
    await expect(snapMenu).not.toBeVisible();

    // 10. Close active window using Escape key
    await page.keyboard.press("Escape");
    await expect(windows).toHaveCount(1);
  });

  test("should open all companion modal windows (Diff Inspector, Treemap, Language Analytics, Health Audit, Reports, Scan Config)", async ({
    page,
  }) => {
    // 1. Open Scan Config Modal from header
    const configBtn = page.getByRole("button", { name: /Config Window/i });
    await configBtn.click();
    const windows = page.locator("[data-win2x-window]");
    await expect(windows).toHaveCount(1);
    await expect(page.getByText("Scan Parameters & Engine Configuration")).toBeVisible();

    // 2. Run scan
    const runBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i }).first();
    await runBtn.click();
    await expect(page.getByText("Detected Clone Pairs")).toBeVisible({ timeout: 25000 });

    // 3. Open Health Audit Modal from DRY score card
    const healthCard = page.getByText("DRY Health Score").first();
    await healthCard.click();
    await expect(page.getByText("DRY Health Score Audit & Diagnostics")).toBeVisible();

    // 4. Open Export & Reports Modal from header
    const reportsBtn = page.getByRole("button", { name: /Reports/i }).first();
    await reportsBtn.click();
    await expect(page.getByText("Report Center & SARIF Exporter")).toBeVisible();

    // 5. Open Treemap Explorer Modal from Visual Analytics
    const openInWinBtn = page.getByRole("button", { name: /Open in Window/i });
    await openInWinBtn.click();
    await expect(page.getByText("Duplication Treemap Explorer")).toBeVisible();

    // 6. Open Diff Inspector Modal from first clone pair
    await page.getByText("#1", { exact: true }).click();
    const diffBtn = page.getByRole("button", { name: /Diff Inspector/i }).first();
    await diffBtn.click();
    await expect(page.getByText(/Clone Pair #1 Diff Inspector/i)).toBeVisible();

    // Verify all 5 modal windows exist in the win2x window manager simultaneously
    await expect(windows).toHaveCount(5);

    // 7. Click Tile Layout on DockBar to neatly tile all 5 windows
    const tileBtn = page.locator('[title="Tile Layout"]');
    await tileBtn.click();

    // 8. Minimize all windows to DockBar
    const minAllBtn = page.locator('[title="Minimize All"]');
    await minAllBtn.click();
    await expect(windows).toHaveCount(0);

    const pills = page.locator("[data-win2x-minimized-pill]");
    await expect(pills).toHaveCount(5);
  });
});
