/// <reference lib="dom" />
import { test, expect, type Page } from "@playwright/test";
import path from "node:path";

const SCREENSHOT_DIR =
  "C:/Users/admin/.gemini/antigravity-cli/brain/53b70aa9-9842-41fe-b6f2-ff97556e2d97/screenshots";

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

test.describe("CDDM WebUI Studio Complete Automated Chrome UI/UX Interactive Validation", () => {
  test.setTimeout(180000);

  test("execute complete interactive UI/UX test across all studio features and dead clone pruning", async ({
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

    // 1. Navigate to http://localhost:3000 & Verify Header
    await page.goto("http://localhost:3000");
    await page.waitForLoadState("networkidle");

    await expect(page.locator("h1")).toContainText("CDDM Studio");
    await expect(page.getByText(/v\d+\.\d+\.\d+/)).toBeVisible();
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

    // 2. Scan Configuration & Codebase Scan Execution
    const dirInput = page.locator('input[placeholder*="e.g. ./src"]');
    await expect(dirInput).toBeVisible();
    await dirInput.fill(".");

    const tokenSlider = page.locator("#scan-min-tokens");
    await expect(tokenSlider).toBeVisible();
    await tokenSlider.fill("40");

    await expect(page.locator("#scan-toggle-type2")).toBeVisible();
    await expect(page.locator("#scan-toggle-type3")).toBeVisible();
    await expect(page.locator("#scan-toggle-type4")).toBeVisible();
    await expect(page.locator("#scan-toggle-crosslanguage")).toBeVisible();

    const ideSelect = page.locator("#scan-preferred-editor");
    await expect(ideSelect).toBeVisible();
    await ideSelect.selectOption("vscode");

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

    await snap(page, "02_scan_completed_metrics.png", true);

    // 3. Dead Code & Dead Clone Cluster Pruning Explorer Modal
    const deadCodeBtn = page.getByRole("button", { name: /Dead Code/i }).first();
    await expect(deadCodeBtn).toBeVisible();
    await deadCodeBtn.click();
    await page.waitForTimeout(600);

    await expect(page.getByText("Polyglot Dead Code Explorer & Safe Pruner")).toBeVisible();
    await expect(page.getByText("Dead Items", { exact: true }).first()).toBeVisible();
    await expect(page.getByText("Unreferenced", { exact: true }).first()).toBeVisible();
    await expect(page.getByText("Unreachable", { exact: true }).first()).toBeVisible();
    await expect(page.getByText("Dead Clones", { exact: true }).first()).toBeVisible();
    await expect(page.getByText("Dead Lines", { exact: true }).first()).toBeVisible();

    const allFilterBtn = page.getByRole("button", { name: /^All \(/i });
    await expect(allFilterBtn).toBeVisible();

    const functionsFilterBtn = page.getByRole("button", { name: /^Functions \(/i });
    await expect(functionsFilterBtn).toBeVisible();
    await functionsFilterBtn.click();
    await page.waitForTimeout(250);

    const unreachableFilterBtn = page.getByRole("button", { name: /^Unreachable \(/i });
    await expect(unreachableFilterBtn).toBeVisible();
    await unreachableFilterBtn.click();
    await page.waitForTimeout(250);

    const deadClonesFilterBtn = page.getByRole("button", { name: /^Dead Clones \(/i });
    await expect(deadClonesFilterBtn).toBeVisible();
    await deadClonesFilterBtn.click();
    await page.waitForTimeout(250);

    const deadCodeSearch = page.getByPlaceholder("Search file, symbol...");
    await expect(deadCodeSearch).toBeVisible();
    await deadCodeSearch.fill("cddm");
    await page.waitForTimeout(250);
    await deadCodeSearch.fill("");
    await page.waitForTimeout(250);

    const dryRunCheckbox = page.getByRole("checkbox", { name: /Dry Run Preview/i });
    await expect(dryRunCheckbox).toBeVisible();
    await expect(dryRunCheckbox).toBeChecked();

    const safeOnlyCheckbox = page.getByRole("checkbox", { name: /Strict Safe-Only/i });
    await expect(safeOnlyCheckbox).toBeVisible();
    await expect(safeOnlyCheckbox).toBeChecked();

    await allFilterBtn.click();
    await page.waitForTimeout(250);

    const selectAllBtn = page.getByRole("button", { name: /Select All|Deselect All/i });
    if (await selectAllBtn.isVisible()) {
      await selectAllBtn.click();
      await page.waitForTimeout(250);
    }

    const pruneBtn = page.getByRole("button", { name: /Preview Pruning|Prune Dead Code/i });
    await expect(pruneBtn).toBeVisible();
    if (await pruneBtn.isEnabled()) {
      await pruneBtn.click();
      await page.waitForTimeout(800);
      await expect(page.getByText(/\[DRY RUN\]/i)).toBeVisible();
    }

    const rescanBtn = page.getByRole("button", { name: /Rescan/i });
    await expect(rescanBtn).toBeVisible();
    await rescanBtn.click();
    await page.waitForTimeout(800);

    await snap(page, "03_dead_code_and_dead_clones_pruning.png");
    await minWin(page);

    // 4. Treemap Visualizer & Language Analytics
    const openTreemapBtn = page.getByRole("button", { name: /Open in Window/i }).first();
    await expect(openTreemapBtn).toBeVisible();
    await openTreemapBtn.click();
    await page.waitForTimeout(500);

    await expect(page.getByText("Duplication Treemap Explorer")).toBeVisible();
    const treemapSvg = page.locator("svg").first();
    await expect(treemapSvg).toBeVisible();

    await snap(page, "04_duplication_treemap_modal.png");
    await minWin(page);

    const langBreakdownBtn = page.getByRole("button", { name: /Language Breakdown/i }).first();
    if (await langBreakdownBtn.isVisible()) {
      await langBreakdownBtn.click();
      await page.waitForTimeout(300);
      const openLangBtn = page.getByRole("button", { name: /Open in Window/i }).first();
      if (await openLangBtn.isVisible()) {
        await openLangBtn.click();
        await page.waitForTimeout(400);
        await expect(page.getByText("Language & Architectural Composition")).toBeVisible();
        await snap(page, "05_language_analytics_modal.png");
        await minWin(page);
      }
    }

    // 5. Clone Pairs Table & Monaco Split Diff Viewer
    await expect(page.getByText("Detected Clone Pairs")).toBeVisible();

    const pairCard1 = page.getByText("#1", { exact: true }).first();
    await pairCard1.click();
    await page.waitForTimeout(300);

    const diffInspectorBtn = page.getByRole("button", { name: /Diff Inspector/i }).first();
    await expect(diffInspectorBtn).toBeVisible();
    await diffInspectorBtn.click();
    await page.waitForTimeout(600);

    await expect(page.getByText(/Diff Inspector/i).first()).toBeVisible();
    await snap(page, "06_clone_pair_diff_viewer.png");
    await minWin(page);

    // 6. Clone Clusters & Multi-site AST Refactoring Sandbox
    const clusterTabBtn = page.getByRole("button", { name: /N-Way Clusters/i });
    await expect(clusterTabBtn).toBeVisible();
    await clusterTabBtn.click();
    await page.waitForTimeout(300);
    await expect(page.getByText("Detected Clone Clusters")).toBeVisible();

    const clusterCard1 = page.getByText("#1", { exact: true }).first();
    await clusterCard1.click();
    await page.waitForTimeout(300);

    const refactorAdvisorBtn = page.getByRole("button", { name: /Refactor Advisor/i }).first();
    if (await refactorAdvisorBtn.isVisible()) {
      await refactorAdvisorBtn.click();
      await page.waitForTimeout(600);
      await expect(page.getByText("Automated Refactoring Advisor")).toBeVisible();
      await snap(page, "07_refactor_advisor_modal.png");
      await minWin(page);
    }

    const sandboxBtn = page.getByRole("button", { name: /Sandbox/i }).first();
    if (await sandboxBtn.isVisible()) {
      await sandboxBtn.click();
      await page.waitForTimeout(600);

      await expect(
        page.getByText("Interactive Auto-Refactor Sandbox & Visual Studio"),
      ).toBeVisible();
      await expect(page.getByRole("button", { name: /Unified Patch Diff/i })).toBeVisible();

      const astTabBtn = page.getByRole("button", { name: /AST-Native Rewrite/i });
      if (await astTabBtn.isVisible()) {
        await astTabBtn.click();
        await page.waitForTimeout(400);
      }

      const healTabBtn = page.getByRole("button", { name: /Auto-Heal/i });
      if (await healTabBtn.isVisible()) {
        await healTabBtn.click();
        await page.waitForTimeout(400);
      }

      const extractTabBtn = page.getByRole("button", { name: /Extract Shared/i });
      if (await extractTabBtn.isVisible()) {
        await extractTabBtn.click();
        await page.waitForTimeout(400);
      }

      await snap(page, "08_refactor_sandbox_multisite.png");
      await minWin(page);
    }

    await page.getByRole("button", { name: /Pairwise/i }).click();
    await page.waitForTimeout(250);

    // 7. Historical Timeline Trends & Multi-Branch Drift Matrix
    const timelineBtn = page.getByRole("button", { name: /Timeline Trends/i });
    await expect(timelineBtn).toBeVisible();
    await timelineBtn.click();
    await page.waitForTimeout(600);

    await expect(page.getByText("Historical Duplication & Git Timeline Evolution")).toBeVisible();

    const matrixTabBtn = page.getByRole("button", { name: /Multi-Branch Drift Matrix/i });
    if (await matrixTabBtn.isVisible()) {
      await matrixTabBtn.click();
      await page.waitForTimeout(300);
      await expect(page.getByRole("button", { name: /Compute Drift Matrix/i })).toBeVisible();
    }

    await snap(page, "09_timeline_and_drift_matrix.png");
    await minWin(page);

    // 8. Architectural Policy Engine Studio
    const policyBtn = page.getByRole("button", { name: /Policy Studio/i });
    await expect(policyBtn).toBeVisible();
    await policyBtn.click();
    await page.waitForTimeout(500);

    await expect(
      page.getByText("Architectural Boundary & Anti-Duplication Policy Studio"),
    ).toBeVisible();
    await expect(page.getByText(/Active Policies/i)).toBeVisible();
    await expect(page.getByText(/Violations Inspector/i)).toBeVisible();
    await expect(page.getByText(/.cddmrules.toml Editor/i)).toBeVisible();

    await snap(page, "10_policy_studio_modal.png");
    await minWin(page);

    // 9. Intelligent AST Suppression & .cddmignore Engine
    const suppressionBtn = page.getByRole("button", { name: /Suppression Rules/i });
    await expect(suppressionBtn).toBeVisible();
    await suppressionBtn.click();
    await page.waitForTimeout(500);

    await expect(page.getByText("Intelligent AST Suppression & .cddmignore Engine")).toBeVisible();
    await snap(page, "11_suppression_rules_modal.png");
    await minWin(page);

    // 10. Coverage Correlation & Dead Code Candidates
    const coverageBtn = page.getByRole("button", { name: /Coverage/i });
    await expect(coverageBtn).toBeVisible();
    await coverageBtn.click();
    await page.waitForTimeout(500);

    await expect(page.getByText("Runtime Execution & Coverage-Aware De-duplication")).toBeVisible();
    await snap(page, "12_coverage_correlation_modal.png");
    await minWin(page);

    // 11. Ecosystem Overlap Detector & Organization Federation Hub
    const overlapBtn = page.getByRole("button", { name: /Overlap Detector/i });
    await expect(overlapBtn).toBeVisible();
    await overlapBtn.click();
    await page.waitForTimeout(500);

    await expect(
      page.getByText("Ecosystem Library Reimplementation & Overlap Detector"),
    ).toBeVisible();
    await snap(page, "13_overlap_detector_modal.png");
    await minWin(page);

    const orgHubBtn = page.getByRole("button", { name: /Org Hub/i });
    await expect(orgHubBtn).toBeVisible();
    await orgHubBtn.click();
    await page.waitForTimeout(500);

    await expect(page.getByText(/Organization Federation Hub/i)).toBeVisible();
    await snap(page, "14_hub_federation_modal.png");
    await minWin(page);

    // 12. Deep Semantic Graph & Cross-Language Matching
    const semanticBtn = page.getByRole("button", { name: /Semantic Graph/i }).first();
    await expect(semanticBtn).toBeVisible();
    await semanticBtn.click();
    await page.waitForTimeout(500);

    await expect(page.getByText("Deep Semantic Graph & Polyglot Isomorphism Engine")).toBeVisible();

    const polyglotTab = page.getByRole("button", { name: /Polyglot Sandbox/i });
    if (await polyglotTab.isVisible()) {
      await polyglotTab.click();
      await page.waitForTimeout(300);
      const compareBtn = page.getByRole("button", { name: /Extract CFGs & Compare Isomorphism/i });
      if (await compareBtn.isVisible()) {
        await compareBtn.click();
        await page.waitForTimeout(800);
      }
    }

    const crossLangTab = page.getByRole("button", { name: /Cross-Language Explorer/i });
    if (await crossLangTab.isVisible()) {
      await crossLangTab.click();
      await page.waitForTimeout(300);
    }

    await snap(page, "15_semantic_graph_and_polyglot.png");
    await minWin(page);

    // 13. Live Event Inspector, Health Score Audit, and Reports
    const eventsBtn = page.getByRole("button", { name: /Events/i }).first();
    if (await eventsBtn.isVisible()) {
      await eventsBtn.click();
      await page.waitForTimeout(400);
      await expect(page.getByText("Live Watch & Real-Time Sync Inspector")).toBeVisible();
      await minWin(page);
    }

    const healthAuditBtn = page.getByRole("button", { name: /Health Audit/i });
    if (await healthAuditBtn.isVisible()) {
      await healthAuditBtn.click();
      await page.waitForTimeout(400);
      await expect(page.getByText("DRY Health Score Audit & Diagnostics")).toBeVisible();
      await minWin(page);
    }

    const reportsBtn = page.getByRole("button", { name: /Reports/i });
    if (await reportsBtn.isVisible()) {
      await reportsBtn.click();
      await page.waitForTimeout(400);
      await expect(page.getByText("Report Center & SARIF Exporter")).toBeVisible();
      await snap(page, "16_reports_and_audit_modals.png");
      await minWin(page);
    }

    // 14. Win2x Desktop Manager (Tile, Cascade, Minimize All)
    await page.evaluate(() => {
      const pills = document.querySelectorAll<HTMLElement>("[data-win2x-minimized-pill]");
      if (pills[0]) pills[0].click();
      if (pills[1]) pills[1].click();
    });
    await page.waitForTimeout(300);

    await page.evaluate(() => {
      const tileBtn = document.querySelector<HTMLElement>('[title="Tile Layout"]');
      if (tileBtn) tileBtn.click();
    });
    await page.waitForTimeout(300);
    await snap(page, "17_win2x_tiled_desktop.png");

    await page.evaluate(() => {
      const cascadeBtn = document.querySelector<HTMLElement>('[title="Cascade Layout"]');
      if (cascadeBtn) cascadeBtn.click();
    });
    await page.waitForTimeout(300);
    await snap(page, "18_win2x_cascaded_desktop.png");

    await page.evaluate(() => {
      const minAllBtn = document.querySelector<HTMLElement>('[title="Minimize All"]');
      if (minAllBtn) minAllBtn.click();
    });
    await page.waitForTimeout(300);

    // 15. Theme Switching (Dark, Light, High-Contrast, Dark)
    await page.evaluate(() => document.documentElement.setAttribute("data-win2x-theme", "light"));
    await page.waitForTimeout(200);
    await snap(page, "19_theme_light.png", true);

    await page.evaluate(() =>
      document.documentElement.setAttribute("data-win2x-theme", "high-contrast"),
    );
    await page.waitForTimeout(200);
    await snap(page, "20_theme_high_contrast.png", true);

    await page.evaluate(() => document.documentElement.setAttribute("data-win2x-theme", "dark"));
    await page.waitForTimeout(200);
    await snap(page, "21_theme_dark_restored.png", true);

    // 16. Keyboard Shortcuts (Esc to close active window)
    await page.getByRole("button", { name: /Config Window/i }).click();
    await page.waitForTimeout(300);
    const configWindow = page.locator("[data-win2x-window]");
    await expect(configWindow).toBeVisible();
    await page.keyboard.press("Escape");
    await page.waitForTimeout(300);

    // 17. Console Log & Runtime Exception Zero-Error Gate
    console.log("Recorded Console Errors:", consoleErrors);
    console.log("Recorded Console Warnings:", consoleWarnings);
    expect(consoleErrors).toEqual([]);
  });
});
