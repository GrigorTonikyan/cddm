import { test, expect } from "@playwright/test";

test.describe("CDDM Semantic Graph Visualizer & Live Watch SSE Sync", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:3000");
  });

  test("should display live watch sync status and open semantic graph modal from header", async ({
    page,
  }) => {
    // 1. Verify header Live Watch status
    const liveWatchIndicator = page.getByText(/Live Watch/i).first();
    await expect(liveWatchIndicator).toBeVisible();

    // 2. Open Semantic Graph Modal from header
    const semanticGraphBtn = page.getByRole("button", { name: /Semantic Graph/i }).first();
    await expect(semanticGraphBtn).toBeVisible();
    await semanticGraphBtn.click();
    await page.waitForTimeout(400);

    // 3. Verify Semantic Graph Window header & subtitle
    await expect(page.getByText("Deep Semantic Graph & Polyglot Isomorphism Engine")).toBeVisible();
    await expect(
      page.getByText("Control Flow Graph extraction, Program Dependence def-use chains"),
    ).toBeVisible();

    // 4. Verify tabs exist
    const visualizerTab = page.getByRole("button", { name: /Graph Visualizer/i }).first();
    const sandboxTab = page.getByRole("button", { name: /Polyglot Sandbox/i });
    const crossLangTab = page.getByRole("button", { name: /Cross-Language Explorer/i });
    await expect(visualizerTab).toBeVisible();
    await expect(sandboxTab).toBeVisible();
    await expect(crossLangTab).toBeVisible();

    // 5. Switch to Polyglot Sandbox tab
    await sandboxTab.click();
    await page.waitForTimeout(300);

    // 6. Enter sample code snippets into Fragment A and Fragment B
    const textareas = page.locator("textarea");
    await expect(textareas).toHaveCount(2);

    const codeA = `pub fn calculate_sum(items: &[i32]) -> i32 {
    let mut total = 0;
    for x in items {
        if *x > 0 {
            total += *x;
        }
    }
    return total;
}`;

    const codeB = `pub fn compute_total(values: &[i32]) -> i32 {
    let mut sum = 0;
    for v in values {
        if *v > 0 {
            sum += *v;
        }
    }
    return sum;
}`;

    await textareas.nth(0).fill(codeA);
    await textareas.nth(1).fill(codeB);

    // 7. Click Extract CFGs & Compare Isomorphism
    const compareBtn = page.getByRole("button", { name: /Extract CFGs & Compare Isomorphism/i });
    await expect(compareBtn).toBeVisible();
    await compareBtn.click();
    await page.waitForTimeout(1200);

    // 8. Verify Similarity badge and graph display
    await expect(
      page
        .locator("[data-win2x-window]")
        .getByText(/Similarity|Isomorphic/i)
        .first(),
    ).toBeVisible();
    await expect(page.locator("[data-win2x-window] svg").first()).toBeVisible();
    await page.waitForTimeout(400);

    // 10. Verify SVG nodes
    const svgRects = page.locator("svg rect");
    expect(await svgRects.count()).toBeGreaterThanOrEqual(4);

    // 11. Toggle PDG Data Dependencies checkbox
    const pdgCheckbox = page.locator('input[type="checkbox"]').last();
    await expect(pdgCheckbox).toBeVisible();
    await pdgCheckbox.click();
    await page.waitForTimeout(300);

    // 12. Close window using Escape key
    await page.keyboard.press("Escape");
    await page.waitForTimeout(300);
    await expect(
      page.getByText("Deep Semantic Graph & Polyglot Isomorphism Engine"),
    ).not.toBeVisible();
  });

  test("should open Semantic Graph inspection directly from a clone pair card", async ({
    page,
  }) => {
    test.setTimeout(60000);

    // 1. Run duplicate analysis scan
    const runBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i }).first();
    await runBtn.click();
    await expect(page.getByText("Detected Clone Pairs")).toBeVisible({ timeout: 35000 });

    // 2. Expand first clone pair
    await page.getByText("#1", { exact: true }).click();
    await page.waitForTimeout(400);

    // 3. Click Semantic Graph action button on the card
    const semanticGraphCardBtn = page.getByRole("button", { name: /Semantic Graph/i }).last();
    await expect(semanticGraphCardBtn).toBeVisible();
    await semanticGraphCardBtn.click();
    await page.waitForTimeout(600);

    // 4. Verify Semantic Graph Modal opens
    await expect(page.getByText("Deep Semantic Graph & Polyglot Isomorphism Engine")).toBeVisible();
  });
});
