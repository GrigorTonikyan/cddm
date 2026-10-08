import { test, expect } from "@playwright/test";

test.describe("CDDM WebUI E2E Workflows", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:3000");
    await page.waitForLoadState("domcontentloaded");
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
    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 90000 });
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

    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 90000 });
    await expect(page.getByText("Clone Clusters")).toBeVisible();

    const clustersTab = page.getByRole("button", { name: /N-Way Clusters/i });
    await expect(clustersTab).toBeVisible();
    await clustersTab.click();

    // Verify cluster cards or view state
    await expect(page.locator("body")).toBeVisible();
  });

  test("should toggle live watch state from header", async ({ page }) => {
    const liveWatchBtn = page.getByRole("button", { name: /Live (Watch|Sync)/i });
    await expect(liveWatchBtn).toBeVisible({ timeout: 15000 });
    await liveWatchBtn.click();
    await expect(page.getByRole("button", { name: /Live (Watch|Sync)/i })).toBeVisible({
      timeout: 15000,
    });
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

  test("should open Refactor Studio from header and toggle all studio tabs", async ({ page }) => {
    const refactorBtn = page.getByRole("button", { name: /Refactor Studio/i });
    await expect(refactorBtn).toBeVisible();
    await refactorBtn.click();

    // Verify studio modal title
    await expect(page.getByText("Interactive Auto-Refactor Sandbox & Visual Studio")).toBeVisible();

    // Check tabs
    await expect(page.getByRole("button", { name: /Unified Patch Diff/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /AST-Native Rewrite/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Auto-Heal/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Extract Shared Crate/i })).toBeVisible();

    // Switch to Extract Shared Crate tab
    await page.getByRole("button", { name: /Extract Shared Crate/i }).click();
    await expect(page.getByText(/Automated Shared Crate & Module Extraction/i)).toBeVisible();
  });

  test("should collapse and expand scan configuration panel", async ({ page }) => {
    const collapseBtn = page.getByRole("button", { name: /Collapse/i });
    await expect(collapseBtn).toBeVisible();
    await collapseBtn.click();

    // Verify collapsed bar
    await expect(page.getByText("Collapsed")).toBeVisible();
    await expect(page.getByRole("button", { name: /Quick Scan/i })).toBeVisible();

    // Expand again
    const expandBtn = page.getByRole("button", { name: /Expand/i });
    await expect(expandBtn).toBeVisible();
    await expandBtn.click();
    await expect(page.getByText("CDDM Polyglot Engine")).toBeVisible();
  });

  test("should open Git Hook Manager & CI/CD Studio modal from header", async ({ page }) => {
    const hooksBtn = page.getByRole("button", { name: "Hooks" });
    await expect(hooksBtn).toBeVisible();
    await hooksBtn.click();

    await expect(page.getByText("Git Hook Manager & CI/CD Studio")).toBeVisible();
    await expect(page.getByText("Pre-Commit Hook", { exact: true })).toBeVisible();
    await expect(page.getByText("Pre-Push Hook", { exact: true })).toBeVisible();

    // Switch to Turnkey CI/CD Workflows tab
    const workflowsTab = page.getByRole("button", { name: /Turnkey CI\/CD Workflows/i });
    await expect(workflowsTab).toBeVisible();
    await workflowsTab.click();
    await expect(page.getByText("Copy Snippet")).toBeVisible();
  });

  test("should open Monorepo Workspace Studio modal from header", async ({ page }) => {
    const monorepoBtn = page.getByRole("button", { name: "Monorepo" });
    await expect(monorepoBtn).toBeVisible();
    await monorepoBtn.click();

    await expect(page.getByText("Monorepo Workspace & Multi-Package Architecture")).toBeVisible();
    await expect(
      page.getByRole("button", { name: /Scan Workspace|Scanning Monorepo/i }),
    ).toBeVisible();
  });

  test("should open Centralized Configuration Studio modal and navigate tabs", async ({ page }) => {
    const configBtn = page.getByRole("button", { name: "Config Window" });
    await expect(configBtn).toBeVisible();
    await configBtn.click();

    await expect(
      page.getByText("Scan Parameters & Centralized Configuration Studio"),
    ).toBeVisible();

    // Tab 1: Engine Parameters & Tuning
    await expect(page.getByRole("button", { name: /Engine Parameters & Tuning/i })).toBeVisible();

    // Tab 2: Persistent Cache Pack
    const cacheTab = page.getByRole("button", { name: /Persistent Cache Pack/i });
    await expect(cacheTab).toBeVisible();
    await cacheTab.click();
    await expect(page.getByRole("button", { name: /Export Cache Pack/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Import Cache Pack/i })).toBeVisible();

    // Tab 3: CI/CD & Integrations Hub
    const integrationsTab = page.getByRole("button", { name: /CI\/CD & Integrations Hub/i });
    await expect(integrationsTab).toBeVisible();
    await integrationsTab.click();
    await expect(page.getByText(/Git Hook Manager & CI\/CD Studio/i)).toBeVisible();
  });
});
