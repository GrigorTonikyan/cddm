/// <reference lib="dom" />
import { test, expect, type Page } from "@playwright/test";
import path from "node:path";

const SCREENSHOT_DIR =
  "C:/Users/admin/.gemini/antigravity-cli/brain/78875dd8-63dc-4975-9be3-1338e457d7f2/screenshots";

async function snap(page: Page, filename: string, fullPage = false) {
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, filename), fullPage });
}

async function minWin(page: Page) {
  await page.locator('[title="Minimize"]').first().click();
  await page.waitForTimeout(300);
}

test.describe("CDDM WebUI Studio Comprehensive UI/UX Quality Verification", () => {
  test.setTimeout(240000);

  test("execute complete browser UI/UX manual-fidelity validation suite", async ({ page }) => {
    const consoleErrors: string[] = [];
    const consoleWarnings: string[] = [];

    page.on("console", (msg) => {
      if (msg.type() === "error") consoleErrors.push(msg.text());
      else if (msg.type() === "warning") consoleWarnings.push(msg.text());
    });
    page.on("pageerror", (err) => consoleErrors.push(`[PageError] ${err.message}`));

    // 1. Initial Page Load & Header Controls
    await page.goto("http://localhost:3000");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("h1")).toContainText("CDDM Studio");
    await expect(page.getByText("v1.7.0")).toBeVisible();
    await expect(
      page.getByText("Code De-Duplication Meister & Architectural Health"),
    ).toBeVisible();
    await expect(page.getByRole("button", { name: /Config Window/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Timeline Trends/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Suppression Rules/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Policy Studio/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Semantic Graph/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Overlap Detector/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Org Hub/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Coverage/i })).toBeVisible();
    await expect(
      page.getByRole("button", { name: /Live (Watch|Sync)|Syncing/i }).first(),
    ).toBeVisible();
    await snap(page, "01_initial_page_load.png", true);

    // 2. Scan Configuration Modal
    await page.getByRole("button", { name: /Config Window/i }).click();
    await page.waitForTimeout(400);
    const configModal = page.locator("[data-win2x-window]");
    await expect(configModal).toBeVisible();
    await expect(page.getByText("Scan Parameters & Engine Configuration")).toBeVisible();
    const tokenSlider = configModal.locator('input[type="range"]');
    await expect(tokenSlider).toBeVisible();
    await tokenSlider.fill("40");
    await snap(page, "02_scan_config_modal.png");
    const closeBtn = configModal.locator('[title="Close"]').first();
    if (await closeBtn.isVisible()) {
      await closeBtn.click();
    } else {
      await page.keyboard.press("Escape");
    }
    await page.waitForTimeout(400);

    // 3. Duplicate Analysis Scan Execution
    await page.locator('input[placeholder*="e.g. ./src"]').fill(".");
    const runScanBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i }).first();
    await expect(runScanBtn).toBeVisible();
    await runScanBtn.click();
    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 45000 });
    await expect(page.getByText("Duplication Rate")).toBeVisible();
    await expect(page.getByText("Files Scanned")).toBeVisible();
    await expect(page.getByText("Clone Pairs", { exact: true })).toBeVisible();
    await expect(page.getByText("Engine Speed")).toBeVisible();
    await expect(page.getByText("/ 100")).toBeVisible();
    await snap(page, "03_scan_results_overview.png", true);

    // 4. Pairwise vs N-Way Clusters View Modes & Card Expansion
    await expect(page.getByText("Detected Clone Pairs")).toBeVisible();
    const clusterTabBtn = page.getByRole("button", { name: /N-Way Clusters/i });
    await expect(clusterTabBtn).toBeVisible();
    await clusterTabBtn.click();
    await page.waitForTimeout(300);
    await expect(page.getByText("Detected Clone Clusters")).toBeVisible();
    await page.getByText("#1", { exact: true }).first().click();
    await page.waitForTimeout(400);
    await snap(page, "04_cluster_view_expanded.png", true);

    // Switch back to Pairwise and expand clone pair card #1
    await page.getByRole("button", { name: /Pairwise/i }).click();
    await page.waitForTimeout(300);
    await page.getByText("#1", { exact: true }).first().click();
    await page.waitForTimeout(400);

    // 5. Split Diff Viewer / Diff Inspector Modal
    const diffInspectorBtn = page.getByRole("button", { name: /Diff Inspector/i }).first();
    await expect(diffInspectorBtn).toBeVisible();
    await diffInspectorBtn.click();
    await page.waitForTimeout(600);
    await expect(page.getByText(/Diff Inspector/i).first()).toBeVisible();
    await snap(page, "05_split_diff_viewer_modal.png");
    await minWin(page);

    // 6. Refactor Advisor Modal
    const refactorAdvisorBtn = page.getByRole("button", { name: /Refactor Advisor/i }).first();
    await expect(refactorAdvisorBtn).toBeVisible();
    await refactorAdvisorBtn.click();
    await page.waitForTimeout(800);
    await expect(page.getByText("Automated Refactoring Advisor")).toBeVisible();
    await snap(page, "06_refactor_advisor_modal.png");
    await minWin(page);

    // 7. Refactor Sandbox Studio Modal (All 4 Tabs)
    const sandboxBtn = page.getByRole("button", { name: /Sandbox/i }).first();
    await expect(sandboxBtn).toBeVisible();
    await sandboxBtn.click();
    await page.waitForTimeout(800);
    await expect(page.getByText("Interactive Auto-Refactor Sandbox & Visual Studio")).toBeVisible();
    await expect(page.getByRole("button", { name: /Unified Patch Diff/i })).toBeVisible();
    await snap(page, "07_refactor_sandbox_patch.png");

    const astTabBtn = page.getByRole("button", { name: /AST-Native Rewrite/i });
    if (await astTabBtn.isVisible()) {
      await astTabBtn.click();
      await page.waitForTimeout(600);
      await snap(page, "08_refactor_sandbox_ast_rewrite.png");
    }
    const healTabBtn = page.getByRole("button", { name: /Auto-Heal/i });
    if (await healTabBtn.isVisible()) {
      await healTabBtn.click();
      await page.waitForTimeout(600);
      await snap(page, "09_refactor_sandbox_auto_heal.png");
    }
    const extractTabBtn = page.getByRole("button", { name: /Extract Shared/i });
    if (await extractTabBtn.isVisible()) {
      await extractTabBtn.click();
      await page.waitForTimeout(600);
      await snap(page, "10_refactor_sandbox_extract_module.png");
    }
    await minWin(page);

    // 8. Semantic Graph Visualizer Modal & Polyglot Sandbox
    await page
      .getByRole("button", { name: /Semantic Graph/i })
      .first()
      .click();
    await page.waitForTimeout(600);
    await expect(page.getByText("Deep Semantic Graph & Polyglot Isomorphism Engine")).toBeVisible();
    await snap(page, "11_semantic_graph_visualizer.png");

    const polyglotTabBtn = page.getByRole("button", { name: /Polyglot Sandbox/i });
    await expect(polyglotTabBtn).toBeVisible();
    await polyglotTabBtn.click();
    await page.waitForTimeout(400);

    const textareas = page.locator("textarea");
    if ((await textareas.count()) >= 2) {
      await textareas
        .nth(0)
        .fill(
          `pub fn calculate_sum(items: &[i32]) -> i32 {\n    let mut total = 0;\n    for x in items { if *x > 0 { total += *x; } }\n    return total;\n}`,
        );
      await textareas
        .nth(1)
        .fill(
          `pub fn compute_total(values: &[i32]) -> i32 {\n    let mut sum = 0;\n    for v in values { if *v > 0 { sum += *v; } }\n    return sum;\n}`,
        );
      const compareBtn = page.getByRole("button", { name: /Extract CFGs & Compare Isomorphism/i });
      if (await compareBtn.isVisible()) {
        await compareBtn.click();
        await page.waitForTimeout(1000);
      }
    }
    await snap(page, "12_semantic_polyglot_sandbox.png");

    const crossLangTabBtn = page.getByRole("button", { name: /Cross-Language Explorer/i });
    await expect(crossLangTabBtn).toBeVisible();
    await crossLangTabBtn.click();
    await page.waitForTimeout(400);
    await snap(page, "13_semantic_cross_language_explorer.png");
    await minWin(page);

    // 9. Duplication Treemap Explorer Modal & Language Analytics Modal
    const openTreemapBtn = page.getByRole("button", { name: /Open in Window/i }).first();
    if (await openTreemapBtn.isVisible()) {
      await openTreemapBtn.click();
      await page.waitForTimeout(500);
      await expect(page.getByText("Duplication Treemap Explorer")).toBeVisible();
      await snap(page, "14_duplication_treemap_modal.png");
      await minWin(page);
    }

    const langBreakdownBtn = page.getByRole("button", { name: /Language Breakdown/i }).first();
    if (await langBreakdownBtn.isVisible()) {
      await langBreakdownBtn.click();
      await page.waitForTimeout(400);
      const openLangBtn = page.getByRole("button", { name: /Open in Window/i }).first();
      if (await openLangBtn.isVisible()) {
        await openLangBtn.click();
        await page.waitForTimeout(500);
        await expect(page.getByText("Language & Architectural Composition")).toBeVisible();
        await snap(page, "14b_language_analytics_modal.png");
        await minWin(page);
      }
    }

    // 10. Timeline Trends Explorer Modal
    await page.getByRole("button", { name: /Timeline Trends/i }).click();
    await page.waitForTimeout(500);
    await expect(page.getByText("Historical Duplication & Git Timeline Evolution")).toBeVisible();
    await snap(page, "15_timeline_evolution_tab.png");
    const matrixTabBtn = page.getByRole("button", { name: /Multi-Branch Drift Matrix/i });
    if (await matrixTabBtn.isVisible()) {
      await matrixTabBtn.click();
      await page.waitForTimeout(400);
      await snap(page, "16_timeline_branch_drift_matrix.png");
    }
    await minWin(page);

    // 11. Policy Studio Modal
    await page.getByRole("button", { name: /Policy Studio/i }).click();
    await page.waitForTimeout(500);
    await expect(
      page.getByText("Architectural Boundary & Anti-Duplication Policy Studio"),
    ).toBeVisible();
    await expect(page.getByText(/Active Policies/i)).toBeVisible();
    await expect(page.getByText(/Violations Inspector/i)).toBeVisible();
    await expect(page.getByText(/.cddmrules.toml Editor/i)).toBeVisible();
    await snap(page, "17_policy_studio_modal.png");
    await minWin(page);

    // 12. Suppression Rules Modal
    await page.getByRole("button", { name: /Suppression Rules/i }).click();
    await page.waitForTimeout(500);
    await expect(page.getByText("Intelligent AST Suppression & .cddmignore Engine")).toBeVisible();
    await snap(page, "18_suppression_rules_modal.png");
    await minWin(page);

    // 13. Coverage Correlation Modal & Overlap Detector Modal
    await page.getByRole("button", { name: /Coverage/i }).click();
    await page.waitForTimeout(500);
    await expect(page.getByText("Runtime Execution & Coverage-Aware De-duplication")).toBeVisible();
    await snap(page, "19_coverage_correlation_modal.png");
    await minWin(page);

    await page.getByRole("button", { name: /Overlap Detector/i }).click();
    await page.waitForTimeout(500);
    await expect(
      page.getByText("Ecosystem Library Reimplementation & Overlap Detector"),
    ).toBeVisible();
    await snap(page, "20_overlap_detector_modal.png");
    await minWin(page);

    // 14. Organization Federation Hub Modal
    await page.getByRole("button", { name: /Org Hub/i }).click();
    await page.waitForTimeout(500);
    await expect(page.getByText(/Organization Federation Hub/i)).toBeVisible();
    await snap(page, "21_hub_federation_modal.png");
    await minWin(page);

    // 15. Live Watch HUD & Event Inspector Modal
    const eventsBtn = page.getByRole("button", { name: /Events/i }).first();
    if (await eventsBtn.isVisible()) {
      await eventsBtn.click();
      await page.waitForTimeout(500);
      await expect(page.getByText("Live Watch & Real-Time Sync Inspector")).toBeVisible();
      await snap(page, "22_live_event_inspector_modal.png");
      const minEventsBtn = page.locator('[title="Minimize"]').first();
      if (await minEventsBtn.isVisible()) await minEventsBtn.click();
    }

    // 16. Health Score Audit & Diagnostics Modal
    const healthAuditBtn = page.getByRole("button", { name: /Health Audit/i });
    if (await healthAuditBtn.isVisible()) {
      await healthAuditBtn.click();
      await page.waitForTimeout(400);
      await expect(page.getByText("DRY Health Score Audit & Diagnostics")).toBeVisible();
      await snap(page, "23_health_audit_modal.png");
      await minWin(page);
    }

    // 17. Reports Center & SARIF Exporter Modal
    const reportsBtn = page.getByRole("button", { name: /Reports/i });
    if (await reportsBtn.isVisible()) {
      await reportsBtn.click();
      await page.waitForTimeout(400);
      await expect(page.getByText("Report Center & SARIF Exporter")).toBeVisible();
      await snap(page, "24_export_reports_modal.png");
      await minWin(page);
    }

    // 18. Win2x Window Manager (Restore, Tile, Cascade, Minimize All)
    await page.evaluate(() => {
      const pills = document.querySelectorAll<HTMLElement>("[data-win2x-minimized-pill]");
      if (pills[0]) pills[0].click();
      if (pills[1]) pills[1].click();
    });
    await page.waitForTimeout(500);
    await expect(page.locator("[data-win2x-window]").first()).toBeVisible();

    await page.evaluate(() => {
      const tileBtn = document.querySelector<HTMLElement>('[title="Tile Layout"]');
      if (tileBtn) tileBtn.click();
    });
    await page.waitForTimeout(500);
    await snap(page, "25_win2x_tiled_layout.png");

    await page.evaluate(() => {
      const cascadeBtn = document.querySelector<HTMLElement>('[title="Cascade Layout"]');
      if (cascadeBtn) cascadeBtn.click();
    });
    await page.waitForTimeout(500);
    await snap(page, "26_win2x_cascaded_layout.png");

    await page.evaluate(() => {
      const minAllBtn = document.querySelector<HTMLElement>('[title="Minimize All"]');
      if (minAllBtn) minAllBtn.click();
    });
    await page.waitForTimeout(400);

    // 19. Theme Switching (Light, High-Contrast, Dark)
    await page.evaluate(() => document.documentElement.setAttribute("data-win2x-theme", "light"));
    await page.waitForTimeout(400);
    await snap(page, "27_theme_light_mode.png", true);

    await page.evaluate(() =>
      document.documentElement.setAttribute("data-win2x-theme", "high-contrast"),
    );
    await page.waitForTimeout(400);
    await snap(page, "28_theme_high_contrast_mode.png", true);

    await page.evaluate(() => document.documentElement.setAttribute("data-win2x-theme", "dark"));
    await page.waitForTimeout(400);
    await snap(page, "29_theme_dark_mode_restored.png", true);

    // 20. Console Integrity Assertions (0 Console Errors)
    console.log("Recorded Console Errors:", consoleErrors);
    console.log("Recorded Console Warnings:", consoleWarnings);
    expect(consoleErrors).toEqual([]);
  });
});
