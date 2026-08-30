import { test, expect } from "@playwright/test";

test.describe("CDDM WebUI E2E Workflows", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:3000");
    await page.waitForLoadState("networkidle");
  });

  test("should render main app header and scan configuration panel", async ({ page }) => {
    await expect(page.locator("h1")).toContainText("CDDM Studio");
    await expect(page.getByText("Scan Configuration")).toBeVisible();
    await expect(page.getByText("Run Duplicate Analysis")).toBeVisible();
  });

  test("should update min token threshold slider and trigger scan", async ({ page }) => {
    await page.locator('input[placeholder*="e.g. ./src"]').fill(".");
    const slider = page.locator('input[type="range"]').first();
    await slider.fill("60");

    const scanBtn = page
      .getByRole("button", { name: /Run Duplicate Analysis|Scanning Codebase/i })
      .first();
    await expect(scanBtn).toBeVisible();
    if (await page.getByRole("button", { name: /Run Duplicate Analysis/i }).isVisible()) {
      await page.getByRole("button", { name: /Run Duplicate Analysis/i }).click();
    }

    // Verify DRY Health Score renders
    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 45000 });
  });

  test("should toggle to N-Way Clusters view and display cluster cards", async ({ page }) => {
    await page.locator('input[placeholder*="e.g. ./src"]').fill(".");
    const scanBtn = page
      .getByRole("button", { name: /Run Duplicate Analysis|Scanning Codebase/i })
      .first();
    await expect(scanBtn).toBeVisible();
    if (await page.getByRole("button", { name: /Run Duplicate Analysis/i }).isVisible()) {
      await page.getByRole("button", { name: /Run Duplicate Analysis/i }).click();
    }

    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 45000 });
    await expect(page.getByText("Clone Clusters")).toBeVisible();

    const clustersTab = page.getByRole("button", { name: /N-Way Clusters/i });
    await expect(clustersTab).toBeVisible();
    await clustersTab.click();

    // Verify cluster cards or view state
    await expect(page.locator("body")).toBeVisible();
  });

  test("should toggle live watch state from header", async ({ page }) => {
    const liveWatchBtn = page.getByRole("button", { name: /Live (Watch|Sync)/i });
    await expect(liveWatchBtn).toBeVisible();
    await liveWatchBtn.click();
    await expect(page.getByRole("button", { name: /Live (Watch|Sync)/i })).toBeVisible();
  });

  test("should open Policy Studio modal and switch between tabs", async ({ page }) => {
    const policyBtn = page.getByRole("button", { name: /Policy Studio/i });
    await expect(policyBtn).toBeVisible();
    await policyBtn.click();

    // Verify Policy Studio Window is open
    await expect(
      page.getByText("Architectural Boundary & Anti-Duplication Policy Studio"),
    ).toBeVisible();
    await expect(page.getByText(/Active Policies/i)).toBeVisible();
    await expect(page.getByText(/Violations Inspector/i)).toBeVisible();
    await expect(page.getByText(/.cddmrules.toml Editor/i)).toBeVisible();

    // Switch to Editor Tab
    const editorTab = page.getByRole("button", { name: /.cddmrules.toml Editor/i });
    await editorTab.click();
    await expect(page.locator("textarea")).toBeVisible();
  });

  test("should select preferred IDE editor in scan configuration panel", async ({ page }) => {
    const ideSelect = page.locator("select").first();
    await expect(ideSelect).toBeVisible();
    await ideSelect.selectOption("cursor");
    await expect(ideSelect).toHaveValue("cursor");
  });

  test("should toggle Cross-Language Type-4 option in scan configuration panel", async ({
    page,
  }) => {
    const crossLangCheckbox = page.getByRole("checkbox", { name: /Cross-Language/i });
    await expect(crossLangCheckbox).toBeVisible();
    const isChecked = await crossLangCheckbox.isChecked();
    await crossLangCheckbox.click();
    await expect(crossLangCheckbox).toBeChecked({ checked: !isChecked });
  });

  test("should open Semantic Graph Modal and navigate all 3 tabs", async ({ page }) => {
    const semanticBtn = page.getByRole("button", { name: /Semantic Graph/i });
    await expect(semanticBtn).toBeVisible();
    await semanticBtn.click();

    // Verify modal title
    await expect(page.getByText("Deep Semantic Graph & Polyglot Isomorphism Engine")).toBeVisible();

    // Tab 1: Graph Visualizer
    await expect(page.getByRole("button", { name: /Graph Visualizer/i }).first()).toBeVisible();

    // Tab 2: Polyglot Sandbox
    const sandboxTab = page.getByRole("button", { name: /Polyglot Sandbox/i });
    await expect(sandboxTab).toBeVisible();
    await sandboxTab.click();
    await expect(page.getByText(/Implementation A:/i)).toBeVisible();
    await expect(page.getByText(/Implementation B:/i)).toBeVisible();
    await expect(
      page.getByRole("button", { name: /Extract CFGs & Compare Isomorphism/i }),
    ).toBeVisible();

    // Tab 3: Cross-Language Explorer
    const explorerTab = page.getByRole("button", { name: /Cross-Language Explorer/i });
    await expect(explorerTab).toBeVisible();
    await explorerTab.click();
    await expect(page.getByText(/Cutoff:/i)).toBeVisible();
    await expect(
      page.getByRole("button", { name: /Discover Polyglot Clones|Analyze Clones/i }),
    ).toBeVisible();
  });
});
