/// <reference lib="dom" />
import { test, expect, type Page } from "@playwright/test";
import path from "node:path";

const SCREENSHOT_DIR =
  "C:/Users/admin/.gemini/antigravity-cli/brain/daf91369-b2f6-4f6c-8ead-87408eaae753/screenshots";

async function snap(page: Page, filename: string, fullPage = false) {
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, filename), fullPage });
}

async function minWin(page: Page) {
  const minBtn = page.locator('[title="Minimize"]').first();
  if (await minBtn.isVisible()) {
    await minBtn.click();
    await page.waitForTimeout(300);
  }
}

test.describe("CDDM WebUI Studio Complete Browser QA Specialist Suite", () => {
  test.setTimeout(240000);

  test("execute comprehensive interactive UI/UX test across all studio features", async ({
    page,
  }) => {
    const consoleErrors: string[] = [];
    const consoleWarnings: string[] = [];

    page.on("console", (msg) => {
      if (msg.type() === "error") {
        consoleErrors.push(msg.text());
      } else if (msg.type() === "warning") {
        consoleWarnings.push(msg.text());
      }
    });

    page.on("pageerror", (err) => {
      consoleErrors.push(`[PageError] ${err.message}`);
    });

    // -------------------------------------------------------------
    // STEP 1: Navigate to http://localhost:3000 and Verify Header
    // -------------------------------------------------------------
    await page.goto("http://localhost:3000");
    await page.waitForLoadState("networkidle");

    await expect(page.locator("h1")).toContainText("CDDM Studio");
    await expect(page.getByText(/v1\.\d+\.\d+/)).toBeVisible();
    await expect(
      page.getByText("Code De-Duplication Meister & Architectural Health"),
    ).toBeVisible();

    // Verify Action Buttons in Header
    await expect(page.getByRole("button", { name: /Config Window/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Timeline Trends/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Suppression Rules/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Policy Studio/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Semantic Graph/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Overlap Detector/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Org Hub/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Coverage/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Dead Code/i }).first()).toBeVisible();
    await expect(
      page.getByRole("button", { name: /Live (Watch|Sync)|Syncing/i }).first(),
    ).toBeVisible();

    await snap(page, "01_header_and_overview.png", true);

    // -------------------------------------------------------------
    // STEP 2: Test Scan Configuration Panel & Toggles
    // -------------------------------------------------------------
    const dirInput = page.locator('input[placeholder*="e.g. ./src"]');
    await expect(dirInput).toBeVisible();
    await dirInput.fill(".");

    const tokenSlider = page.locator('input[type="range"]').first();
    await expect(tokenSlider).toBeVisible();
    await tokenSlider.fill("45");
    await expect(page.getByText("45 tokens", { exact: true })).toBeVisible();

    // Checkbox toggles
    const type2Checkbox = page.getByRole("checkbox", { name: /Type-2/i });
    await expect(type2Checkbox).toBeVisible();

    const type3Checkbox = page.getByRole("checkbox", { name: /Type-3/i });
    await expect(type3Checkbox).toBeVisible();

    const type4Checkbox = page.getByRole("checkbox", { name: /Type-4 \(Semantic Clones\)/i });
    await expect(type4Checkbox).toBeVisible();
    const type4Initial = await type4Checkbox.isChecked();
    await type4Checkbox.click();
    await expect(type4Checkbox).toBeChecked({ checked: !type4Initial });

    const polyglotCheckbox = page.getByRole("checkbox", {
      name: /Cross-Language \(Polyglot\)/i,
    });
    await expect(polyglotCheckbox).toBeVisible();
    const polyglotInitial = await polyglotCheckbox.isChecked();
    await polyglotCheckbox.click();
    await expect(polyglotCheckbox).toBeChecked({ checked: !polyglotInitial });

    const gitBlameCheckbox = page.getByRole("checkbox", { name: /Git Blame/i });
    await expect(gitBlameCheckbox).toBeVisible();

    // IDE selector
    const ideSelect = page.locator("select").first();
    await expect(ideSelect).toBeVisible();
    await ideSelect.selectOption("vscode");

    await snap(page, "02_scan_config_toggled.png");

    // -------------------------------------------------------------
    // STEP 3: Execute Scan & Verify Progress & Telemetry Cards
    // -------------------------------------------------------------
    const scanBtn = page
      .getByRole("button", { name: /Run Duplicate Analysis|Scanning Codebase/i })
      .first();
    await expect(scanBtn).toBeVisible();
    if (await page.getByRole("button", { name: /Run Duplicate Analysis/i }).isVisible()) {
      await page.getByRole("button", { name: /Run Duplicate Analysis/i }).click();
    }

    // Wait for Scan Results and Summary Cards
    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 60000 });
    await expect(page.getByText("Duplication Rate")).toBeVisible();
    await expect(page.getByText("Files Scanned")).toBeVisible();
    await expect(page.getByText("Clone Pairs", { exact: true })).toBeVisible();
    await expect(page.getByText("Clone Clusters", { exact: true })).toBeVisible();
    await expect(page.getByText("Engine Speed")).toBeVisible();
    await expect(page.getByText("/ 100")).toBeVisible();

    await snap(page, "03_scan_completed_metrics.png", true);

    // -------------------------------------------------------------
    // STEP 4: Test DRY Health Score Card Click -> HealthAuditModal
    // -------------------------------------------------------------
    const dryScoreCard = page.getByText("DRY Health Score").first();
    await dryScoreCard.click();
    await page.waitForTimeout(500);

    await expect(page.getByText("DRY Health Score Audit & Diagnostics")).toBeVisible();
    await expect(page.getByText("Architectural DRY Health")).toBeVisible();
    await expect(page.getByText("CI Quality Gate Status")).toBeVisible();
    await expect(page.getByText("Audit Metrics & Remediation Priorities")).toBeVisible();
    await expect(page.getByText("Recommended Action Items")).toBeVisible();
    await snap(page, "04_health_audit_modal.png");
    await minWin(page);

    // -------------------------------------------------------------
    // STEP 5: Test Visual Analytics & Treemap Explorer Modal
    // -------------------------------------------------------------
    const openTreemapBtn = page.getByRole("button", { name: /Open in Window/i }).first();
    await expect(openTreemapBtn).toBeVisible();
    await openTreemapBtn.click();
    await page.waitForTimeout(500);

    await expect(page.getByText("Duplication Treemap Explorer")).toBeVisible();
    const treemapSvg = page.locator("svg").first();
    await expect(treemapSvg).toBeVisible();

    // Verify SVG rect elements exist
    const svgRects = page.locator("svg rect");
    const rectCount = await svgRects.count();
    expect(rectCount).toBeGreaterThan(0);

    await snap(page, "05_treemap_explorer_modal.png");
    await minWin(page);

    // -------------------------------------------------------------
    // STEP 6: Test Clone Pairs & Clusters, Diff Viewer & Refactor Modals
    // -------------------------------------------------------------
    await expect(page.getByText("Detected Clone Pairs")).toBeVisible();

    // Expand clone pair card #1
    const pairCard1 = page.getByText("#1", { exact: true }).first();
    await pairCard1.click();
    await page.waitForTimeout(400);

    // Diff Inspector Modal
    const diffInspectorBtn = page.getByRole("button", { name: /Diff Inspector/i }).first();
    await expect(diffInspectorBtn).toBeVisible();
    await diffInspectorBtn.click();
    await page.waitForTimeout(600);
    await expect(page.getByText(/Diff Inspector/i).first()).toBeVisible();
    await snap(page, "06_split_diff_modal.png");
    await minWin(page);

    // Refactor Advisor Modal
    const refactorAdvisorBtn = page.getByRole("button", { name: /Refactor Advisor/i }).first();
    await expect(refactorAdvisorBtn).toBeVisible();
    await refactorAdvisorBtn.click();
    await page.waitForTimeout(800);
    await expect(page.getByText("Automated Refactoring Advisor")).toBeVisible();
    await snap(page, "07_refactor_advisor_modal.png");
    await minWin(page);

    // Refactor Sandbox Modal (4 tabs)
    const sandboxBtn = page.getByRole("button", { name: /Sandbox/i }).first();
    await expect(sandboxBtn).toBeVisible();
    await sandboxBtn.click();
    await page.waitForTimeout(800);

    await expect(page.getByText("Interactive Auto-Refactor Sandbox & Visual Studio")).toBeVisible();

    // Tab 1: Patch Diff
    await expect(page.getByRole("button", { name: /Unified Patch Diff/i })).toBeVisible();

    // Tab 2: AST-Native Rewrite
    const astTabBtn = page.getByRole("button", { name: /AST-Native Rewrite/i });
    if (await astTabBtn.isVisible()) {
      await astTabBtn.click();
      await page.waitForTimeout(500);
      await expect(
        page
          .getByText(/Extracted Shared Helper|Synthesized Function Implementation|Tree-sitter AST/i)
          .first(),
      ).toBeVisible();
    }

    // Tab 3: Auto-Heal & Verification
    const healTabBtn = page.getByRole("button", { name: /Auto-Heal/i });
    if (await healTabBtn.isVisible()) {
      await healTabBtn.click();
      await page.waitForTimeout(500);
      await expect(page.getByText(/AI Code Surgeon/i).first()).toBeVisible();
    }

    // Tab 4: Extract Shared Module
    const extractTabBtn = page.getByRole("button", { name: /Extract Shared/i });
    if (await extractTabBtn.isVisible()) {
      await extractTabBtn.click();
      await page.waitForTimeout(500);
      await expect(page.getByText(/Automated Shared Crate/i).first()).toBeVisible();
    }

    await snap(page, "08_refactor_sandbox_tabs.png");
    await minWin(page);

    // Switch to N-Way Clusters tab
    const clusterTabBtn = page.getByRole("button", { name: /N-Way Clusters/i });
    await expect(clusterTabBtn).toBeVisible();
    await clusterTabBtn.click();
    await page.waitForTimeout(400);
    await expect(page.getByText("Detected Clone Clusters")).toBeVisible();
    await snap(page, "09_clone_clusters_view.png", true);

    // -------------------------------------------------------------
    // STEP 7: Test Dead Code Explorer Modal
    // -------------------------------------------------------------
    const deadCodeBtn = page.getByRole("button", { name: /Dead Code/i }).first();
    await expect(deadCodeBtn).toBeVisible();
    await deadCodeBtn.click();
    await page.waitForTimeout(700);

    await expect(page.getByText("Polyglot Dead Code Explorer")).toBeVisible();
    await expect(page.getByText("Dead Items", { exact: true }).first()).toBeVisible();
    await expect(page.getByText("Unreferenced", { exact: true }).first()).toBeVisible();
    await expect(page.getByText("Unreachable", { exact: true }).first()).toBeVisible();
    await expect(page.getByText("Dead Clones", { exact: true }).first()).toBeVisible();
    await expect(page.getByText("Dead Lines", { exact: true }).first()).toBeVisible();
    await snap(page, "10_dead_code_explorer_modal.png");
    await minWin(page);

    // -------------------------------------------------------------
    // STEP 8: Test Coverage Correlation Modal
    // -------------------------------------------------------------
    const coverageBtn = page.getByRole("button", { name: /Coverage/i });
    await expect(coverageBtn).toBeVisible();
    await coverageBtn.click();
    await page.waitForTimeout(600);

    await expect(page.getByText("Runtime Execution & Coverage-Aware De-duplication")).toBeVisible();
    await snap(page, "11_coverage_correlation_modal.png");
    await minWin(page);

    // -------------------------------------------------------------
    // STEP 9: Test Hub Federation Modal
    // -------------------------------------------------------------
    const orgHubBtn = page.getByRole("button", { name: /Org Hub/i });
    await expect(orgHubBtn).toBeVisible();
    await orgHubBtn.click();
    await page.waitForTimeout(600);

    await expect(page.getByText(/Organization Federation Hub/i)).toBeVisible();
    await snap(page, "12_hub_federation_modal.png");
    await minWin(page);

    // -------------------------------------------------------------
    // STEP 10: Test Policy Studio & Suppression Rules Modals
    // -------------------------------------------------------------
    const policyBtn = page.getByRole("button", { name: /Policy Studio/i });
    await expect(policyBtn).toBeVisible();
    await policyBtn.click();
    await page.waitForTimeout(600);

    await expect(
      page.getByText("Architectural Boundary & Anti-Duplication Policy Studio"),
    ).toBeVisible();
    await expect(page.getByText(/Active Policies/i)).toBeVisible();
    await expect(page.getByText(/Violations Inspector/i)).toBeVisible();
    await expect(page.getByText(/.cddmrules.toml Editor/i)).toBeVisible();
    await snap(page, "13_policy_studio_modal.png");
    await minWin(page);

    const suppressionBtn = page.getByRole("button", { name: /Suppression Rules/i });
    await expect(suppressionBtn).toBeVisible();
    await suppressionBtn.click();
    await page.waitForTimeout(600);

    await expect(page.getByText("Intelligent AST Suppression & .cddmignore Engine")).toBeVisible();
    await snap(page, "14_suppression_rules_modal.png");
    await minWin(page);

    // -------------------------------------------------------------
    // STEP 11: Test Timeline Explorer & Multi-Branch Drift Matrix
    // -------------------------------------------------------------
    const timelineBtn = page.getByRole("button", { name: /Timeline Trends/i });
    await expect(timelineBtn).toBeVisible();
    await timelineBtn.click();
    await page.waitForTimeout(600);

    await expect(page.getByText("Historical Duplication & Git Timeline Evolution")).toBeVisible();

    const matrixTabBtn = page.getByRole("button", { name: /Multi-Branch Drift Matrix/i });
    if (await matrixTabBtn.isVisible()) {
      await matrixTabBtn.click();
      await page.waitForTimeout(400);
      await expect(page.getByRole("button", { name: /Compute Drift Matrix/i })).toBeVisible();
    }

    await snap(page, "15_timeline_explorer_matrix.png");
    await minWin(page);

    // -------------------------------------------------------------
    // STEP 12: Test Semantic Graph Modal & Polyglot Sandbox
    // -------------------------------------------------------------
    const semanticBtn = page.getByRole("button", { name: /Semantic Graph/i }).first();
    await expect(semanticBtn).toBeVisible();
    await semanticBtn.click();
    await page.waitForTimeout(600);

    await expect(page.getByText("Deep Semantic Graph & Polyglot Isomorphism Engine")).toBeVisible();

    const polyglotTab = page.getByRole("button", { name: /Polyglot Sandbox/i });
    if (await polyglotTab.isVisible()) {
      await polyglotTab.click();
      await page.waitForTimeout(400);
      await expect(page.getByText(/Implementation A:/i)).toBeVisible();
      await expect(page.getByText(/Implementation B:/i)).toBeVisible();
      const compareBtn = page.getByRole("button", {
        name: /Extract CFGs & Compare Isomorphism/i,
      });
      if (await compareBtn.isVisible()) {
        await compareBtn.click();
        await page.waitForTimeout(1000);
      }
    }

    await snap(page, "16_semantic_graph_modal.png");
    await minWin(page);

    // -------------------------------------------------------------
    // STEP 13: Test Ecosystem Overlap Detector Modal
    // -------------------------------------------------------------
    const overlapBtn = page.getByRole("button", { name: /Overlap Detector/i });
    await expect(overlapBtn).toBeVisible();
    await overlapBtn.click();
    await page.waitForTimeout(600);

    await expect(
      page.getByText("Ecosystem Library Reimplementation & Overlap Detector"),
    ).toBeVisible();
    await snap(page, "17_overlap_detector_modal.png");
    await minWin(page);

    // -------------------------------------------------------------
    // STEP 14: Test Live Watch HUD & Event Inspector Modal
    // -------------------------------------------------------------
    const eventsBtn = page.getByRole("button", { name: /Events/i }).first();
    await expect(eventsBtn).toBeVisible();
    await eventsBtn.click();
    await page.waitForTimeout(600);

    await expect(page.getByText("Live Watch & Real-Time Sync Inspector")).toBeVisible();
    await snap(page, "18_live_event_inspector_modal.png");
    await minWin(page);

    // -------------------------------------------------------------
    // STEP 15: Test Win2x Window Manager (Restore, Tile, Cascade)
    // -------------------------------------------------------------
    await page.evaluate(() => {
      const pills = document.querySelectorAll<HTMLElement>("[data-win2x-minimized-pill]");
      if (pills[0]) pills[0].click();
      if (pills[1]) pills[1].click();
    });
    await page.waitForTimeout(400);

    await page.evaluate(() => {
      const tileBtn = document.querySelector<HTMLElement>('[title="Tile Layout"]');
      if (tileBtn) tileBtn.click();
    });
    await page.waitForTimeout(400);
    await snap(page, "19_win2x_desktop_tiled.png");

    await page.evaluate(() => {
      const cascadeBtn = document.querySelector<HTMLElement>('[title="Cascade Layout"]');
      if (cascadeBtn) cascadeBtn.click();
    });
    await page.waitForTimeout(400);

    await page.evaluate(() => {
      const minAllBtn = document.querySelector<HTMLElement>('[title="Minimize All"]');
      if (minAllBtn) minAllBtn.click();
    });
    await page.waitForTimeout(400);

    // -------------------------------------------------------------
    // STEP 16: Test Theme Switching (Light, High-Contrast, Dark)
    // -------------------------------------------------------------
    await page.evaluate(() => document.documentElement.setAttribute("data-win2x-theme", "light"));
    await page.waitForTimeout(300);

    await page.evaluate(() =>
      document.documentElement.setAttribute("data-win2x-theme", "high-contrast"),
    );
    await page.waitForTimeout(300);

    await page.evaluate(() => document.documentElement.setAttribute("data-win2x-theme", "dark"));
    await page.waitForTimeout(300);
    await snap(page, "20_theme_switching.png", true);

    // -------------------------------------------------------------
    // STEP 17: Console Log & Error Assertions
    // -------------------------------------------------------------
    console.log("Console Errors recorded during test:", consoleErrors);
    console.log("Console Warnings recorded during test:", consoleWarnings);
    expect(consoleErrors).toEqual([]);
  });
});
