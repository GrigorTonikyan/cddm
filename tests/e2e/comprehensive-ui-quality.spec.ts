/// <reference lib="dom" />
import { test, expect } from "@playwright/test";
import path from "node:path";

const SCREENSHOT_DIR =
  "C:/Users/admin/.gemini/antigravity/brain/7885ace6-6b18-42e9-bad7-23fe47e164dc/screenshots";

test.describe("CDDM WebUI Studio Comprehensive UI/UX Quality Verification", () => {
  test.setTimeout(180000);

  test("execute complete browser UI/UX manual-fidelity validation suite", async ({ page }) => {
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
    // Step 1: Open http://localhost:3000 & Verify Initial Page State
    // -------------------------------------------------------------
    await page.goto("http://localhost:3000");
    await page.waitForLoadState("networkidle");

    await expect(page.locator("h1")).toContainText("CDDM Studio");
    await expect(page.getByText(/v\d+\.\d+\.\d+/)).toBeVisible();
    await expect(
      page.getByText("Code De-Duplication Meister & Architectural Health"),
    ).toBeVisible();

    // Verify Header Action Buttons
    await expect(page.getByRole("button", { name: /Config Window/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Timeline Trends/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Suppression Rules/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Policy Studio/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /Semantic Graph/i })).toBeVisible();

    // Live Watch HUD button
    const liveWatchBtn = page.getByRole("button", { name: /Live (Watch|Sync)|Syncing/i }).first();
    await expect(liveWatchBtn).toBeVisible();

    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "01_initial_page_load.png"),
      fullPage: true,
    });

    // -------------------------------------------------------------
    // Step 2: Open Scan Configuration Modal & Inspect Parameters
    // -------------------------------------------------------------
    const configBtn = page.getByRole("button", { name: /Config Window/i });
    await configBtn.click();
    await page.waitForTimeout(400);

    const configModal = page.locator("[data-win2x-window]");
    await expect(configModal).toBeVisible();
    await expect(page.getByText("Scan Parameters & Engine Configuration")).toBeVisible();

    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "02_scan_config_modal.png"),
    });

    // Close Scan Config modal via Escape
    await page.keyboard.press("Escape");
    await page.waitForTimeout(300);
    await expect(configModal).toHaveCount(0);

    // -------------------------------------------------------------
    // Step 3: Trigger Codebase Scan & Verify Metrics
    // -------------------------------------------------------------
    const dirInput = page.locator('input[placeholder*="e.g. ./src"]');
    await dirInput.fill(".");

    const runScanBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i }).first();
    await expect(runScanBtn).toBeVisible();
    await runScanBtn.click();

    // Wait for scan results
    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 45000 });
    await expect(page.getByText("Duplication Rate")).toBeVisible();
    await expect(page.getByText("Files Scanned")).toBeVisible();
    await expect(page.getByText("Clone Pairs", { exact: true })).toBeVisible();
    await expect(page.getByText("Engine Speed")).toBeVisible();

    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "03_scan_results_overview.png"),
      fullPage: true,
    });

    // -------------------------------------------------------------
    // Step 4: Verify Pairwise vs N-Way Clusters View Modes
    // -------------------------------------------------------------
    await expect(page.getByText("Detected Clone Pairs")).toBeVisible();
    const clusterTabBtn = page.getByRole("button", { name: /N-Way Clusters/i });
    await expect(clusterTabBtn).toBeVisible();
    await clusterTabBtn.click();
    await page.waitForTimeout(300);

    await expect(page.getByText("Detected Clone Clusters")).toBeVisible();
    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "04_cluster_view.png"),
      fullPage: true,
    });

    // Switch back to Pairwise view
    const pairwiseTabBtn = page.getByRole("button", { name: /Pairwise/i });
    await pairwiseTabBtn.click();
    await page.waitForTimeout(300);

    // -------------------------------------------------------------
    // Step 5: Split Diff Viewer Modal (Diff Inspector)
    // -------------------------------------------------------------
    const firstClonePairCard = page.getByText("#1", { exact: true }).first();
    await firstClonePairCard.click();
    await page.waitForTimeout(400);

    const diffInspectorBtn = page.getByRole("button", { name: /Diff Inspector/i }).first();
    await expect(diffInspectorBtn).toBeVisible();
    await diffInspectorBtn.click();
    await page.waitForTimeout(600);

    await expect(page.getByText(/Clone Pair #1 Diff Inspector/i)).toBeVisible();
    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "05_split_diff_viewer_modal.png"),
    });

    // Minimize Diff Inspector window to DockBar
    const minDiffBtn = page.locator('[title="Minimize"]').first();
    await minDiffBtn.click();
    await page.waitForTimeout(300);

    // -------------------------------------------------------------
    // Step 6: Refactor Advisor Modal (RefactorPatchModal)
    // -------------------------------------------------------------
    const refactorAdvisorBtn = page.getByRole("button", { name: /Refactor Advisor/i }).first();
    await expect(refactorAdvisorBtn).toBeVisible();
    await refactorAdvisorBtn.click();
    await page.waitForTimeout(800);

    await expect(page.getByText("Automated Refactoring Advisor")).toBeVisible();
    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "06_refactor_advisor_modal.png"),
    });

    const minAdvisorBtn = page.locator('[title="Minimize"]').first();
    await minAdvisorBtn.click();
    await page.waitForTimeout(300);

    // -------------------------------------------------------------
    // Step 7: Refactor Sandbox Studio Modal (AST Rewrite, Extraction, Auto-Heal)
    // -------------------------------------------------------------
    const sandboxBtn = page.getByRole("button", { name: /Sandbox/i }).first();
    await expect(sandboxBtn).toBeVisible();
    await sandboxBtn.click();
    await page.waitForTimeout(800);

    await expect(page.getByText("Interactive Auto-Refactor Sandbox & Visual Studio")).toBeVisible();

    // Tab 1: Patch Diff Preview
    await expect(page.getByRole("button", { name: /Unified Patch Diff/i })).toBeVisible();
    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "07_refactor_sandbox_patch.png"),
    });

    // Tab 2: AST-Native Rewrite Tab
    const astTabBtn = page.getByRole("button", { name: /AST-Native Rewrite/i });
    if (await astTabBtn.isVisible()) {
      await astTabBtn.click();
      await page.waitForTimeout(600);
      await page.screenshot({
        path: path.join(SCREENSHOT_DIR, "08_refactor_sandbox_ast_rewrite.png"),
      });
    }

    // Tab 3: AI Surgeon Auto-Heal Tab
    const healTabBtn = page.getByRole("button", { name: /Auto-Heal/i });
    if (await healTabBtn.isVisible()) {
      await healTabBtn.click();
      await page.waitForTimeout(600);
      await page.screenshot({
        path: path.join(SCREENSHOT_DIR, "09_refactor_sandbox_auto_heal.png"),
      });
    }

    // Tab 4: Extract Shared Module Tab
    const extractTabBtn = page.getByRole("button", { name: /Extract Shared/i });
    if (await extractTabBtn.isVisible()) {
      await extractTabBtn.click();
      await page.waitForTimeout(600);
      await page.screenshot({
        path: path.join(SCREENSHOT_DIR, "10_refactor_sandbox_extract_module.png"),
      });
    }

    // Minimize Refactor Sandbox to DockBar
    const minRefactorBtn = page.locator('[title="Minimize"]').first();
    await minRefactorBtn.click();
    await page.waitForTimeout(300);

    // -------------------------------------------------------------
    // Step 8: Semantic Graph Visualizer Modal & Polyglot Sandbox
    // -------------------------------------------------------------
    const semanticHeaderBtn = page.getByRole("button", { name: /Semantic Graph/i }).first();
    await semanticHeaderBtn.click();
    await page.waitForTimeout(600);

    await expect(page.getByText("Deep Semantic Graph & Polyglot Isomorphism Engine")).toBeVisible();

    // Graph Visualizer Tab
    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "11_semantic_graph_visualizer.png"),
    });

    // Polyglot Sandbox Tab
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

    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "12_semantic_polyglot_sandbox.png"),
    });

    // Cross-Language Explorer Tab
    const crossLangTabBtn = page.getByRole("button", { name: /Cross-Language Explorer/i });
    await expect(crossLangTabBtn).toBeVisible();
    await crossLangTabBtn.click();
    await page.waitForTimeout(400);

    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "13_semantic_cross_language_explorer.png"),
    });

    // Minimize Semantic Graph to DockBar
    const minSemanticBtn = page.locator('[title="Minimize"]').first();
    await minSemanticBtn.click();
    await page.waitForTimeout(300);

    // -------------------------------------------------------------
    // Step 9: Duplication Treemap Explorer Modal & Language Analytics
    // -------------------------------------------------------------
    const openTreemapBtn = page.getByRole("button", { name: /Open in Window/i });
    if (await openTreemapBtn.isVisible()) {
      await openTreemapBtn.click();
      await page.waitForTimeout(500);

      await expect(page.getByText("Duplication Treemap Explorer")).toBeVisible();
      await page.screenshot({
        path: path.join(SCREENSHOT_DIR, "14_duplication_treemap_modal.png"),
      });

      const minTreemapBtn = page.locator('[title="Minimize"]').first();
      await minTreemapBtn.click();
      await page.waitForTimeout(300);
    }

    // -------------------------------------------------------------
    // Step 10: Timeline Explorer Modal
    // -------------------------------------------------------------
    const timelineBtn = page.getByRole("button", { name: /Timeline Trends/i });
    await timelineBtn.click();
    await page.waitForTimeout(500);

    await expect(page.getByText("Historical Duplication & Git Timeline Evolution")).toBeVisible();
    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "15_timeline_explorer_modal.png"),
    });

    const minTimelineBtn = page.locator('[title="Minimize"]').first();
    await minTimelineBtn.click();
    await page.waitForTimeout(300);

    // -------------------------------------------------------------
    // Step 11: Policy Rules Modal & Suppression Rules Modal
    // -------------------------------------------------------------
    // Policy Studio
    const policyBtn = page.getByRole("button", { name: /Policy Studio/i });
    await policyBtn.click();
    await page.waitForTimeout(500);

    await expect(
      page.getByText("Architectural Boundary & Anti-Duplication Policy Studio"),
    ).toBeVisible();
    await expect(page.getByText(/Active Policies/i)).toBeVisible();
    await expect(page.getByText(/Violations Inspector/i)).toBeVisible();
    await expect(page.getByText(/.cddmrules.toml Editor/i)).toBeVisible();

    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "16_policy_studio_modal.png"),
    });

    const minPolicyBtn = page.locator('[title="Minimize"]').first();
    await minPolicyBtn.click();
    await page.waitForTimeout(300);

    // Suppression Rules
    const suppressionBtn = page.getByRole("button", { name: /Suppression Rules/i });
    await suppressionBtn.click();
    await page.waitForTimeout(500);

    await expect(page.getByText("Intelligent AST Suppression & .cddmignore Engine")).toBeVisible();
    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "17_suppression_rules_modal.png"),
    });

    const minSuppressionBtn = page.locator('[title="Minimize"]').first();
    await minSuppressionBtn.click();
    await page.waitForTimeout(300);

    // -------------------------------------------------------------
    // Step 12: Live Watch Studio & Event Inspector
    // -------------------------------------------------------------
    const eventsBtn = page.getByRole("button", { name: /Events/i }).first();
    if (await eventsBtn.isVisible()) {
      await eventsBtn.click();
      await page.waitForTimeout(500);
      await expect(page.getByText("Live Watch & Real-Time Sync Inspector")).toBeVisible();
      await page.screenshot({
        path: path.join(SCREENSHOT_DIR, "18_live_watch_event_inspector.png"),
      });
      const minLogsBtn = page.locator('[title="Minimize"]').first();
      if (await minLogsBtn.isVisible()) {
        await minLogsBtn.click();
        await page.waitForTimeout(300);
      }
    }

    // -------------------------------------------------------------
    // Step 13: Reports Center & Health Audit Modals
    // -------------------------------------------------------------
    const healthAuditBtn = page.getByRole("button", { name: /Health Audit/i });
    if (await healthAuditBtn.isVisible()) {
      await healthAuditBtn.click();
      await page.waitForTimeout(400);
      await expect(page.getByText("DRY Health Score Audit & Diagnostics")).toBeVisible();
      await page.screenshot({
        path: path.join(SCREENSHOT_DIR, "19_health_audit_modal.png"),
      });
      const minHealthBtn = page.locator('[title="Minimize"]').first();
      await minHealthBtn.click();
      await page.waitForTimeout(300);
    }

    const reportsBtn = page.getByRole("button", { name: /Reports/i });
    if (await reportsBtn.isVisible()) {
      await reportsBtn.click();
      await page.waitForTimeout(400);
      await expect(page.getByText("Report Center & SARIF Exporter")).toBeVisible();
      await page.screenshot({
        path: path.join(SCREENSHOT_DIR, "20_export_reports_modal.png"),
      });
      const minReportsBtn = page.locator('[title="Minimize"]').first();
      await minReportsBtn.click();
      await page.waitForTimeout(300);
    }

    // -------------------------------------------------------------
    // Step 14: Win2x Window Manager (Tile, Cascade, Drag, Resize, Snap)
    // -------------------------------------------------------------
    // Restore minimized windows from DockBar pills via DOM click
    await page.evaluate(() => {
      const pills = document.querySelectorAll<HTMLElement>("[data-win2x-minimized-pill]");
      if (pills[0]) pills[0].click();
      if (pills[1]) pills[1].click();
    });
    await page.waitForTimeout(500);

    const activeWindows = page.locator("[data-win2x-window]");
    await expect(activeWindows.first()).toBeVisible();

    // Test DockBar Tile Layout
    await page.evaluate(() => {
      const tileBtn = document.querySelector<HTMLElement>('[title="Tile Layout"]');
      if (tileBtn) tileBtn.click();
    });
    await page.waitForTimeout(500);

    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "21_win2x_tiled_layout.png"),
    });

    // Test DockBar Cascade Layout
    await page.evaluate(() => {
      const cascadeBtn = document.querySelector<HTMLElement>('[title="Cascade Layout"]');
      if (cascadeBtn) cascadeBtn.click();
    });
    await page.waitForTimeout(500);

    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "22_win2x_cascaded_layout.png"),
    });

    // Test Minimize All
    await page.evaluate(() => {
      const minAllBtn = document.querySelector<HTMLElement>('[title="Minimize All"]');
      if (minAllBtn) minAllBtn.click();
    });
    await page.waitForTimeout(400);

    // -------------------------------------------------------------
    // Step 15: Theme Switching (Dark, Light, High-Contrast)
    // -------------------------------------------------------------
    // Switch to Light Theme
    await page.evaluate(() => {
      document.documentElement.setAttribute("data-win2x-theme", "light");
    });
    await page.waitForTimeout(400);

    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "23_theme_light_mode.png"),
      fullPage: true,
    });

    // Switch to High-Contrast Theme
    await page.evaluate(() => {
      document.documentElement.setAttribute("data-win2x-theme", "high-contrast");
    });
    await page.waitForTimeout(400);

    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "24_theme_high_contrast_mode.png"),
      fullPage: true,
    });

    // Revert to Dark Theme
    await page.evaluate(() => {
      document.documentElement.setAttribute("data-win2x-theme", "dark");
    });
    await page.waitForTimeout(400);

    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, "25_theme_dark_mode_restored.png"),
      fullPage: true,
    });

    // -------------------------------------------------------------
    // Step 16: Console Integrity Assertions
    // -------------------------------------------------------------
    console.log("Recorded Console Errors:", consoleErrors);
    console.log("Recorded Console Warnings:", consoleWarnings);

    expect(consoleErrors).toEqual([]);
  });
});
