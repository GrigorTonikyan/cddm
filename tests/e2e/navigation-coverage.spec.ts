import { test, expect } from "@playwright/test";

test.describe("CDDM Top-Level Navigation & Discoverability Matrix", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:3000");
    await page.waitForLoadState("domcontentloaded");
  });

  test("should open all top-level header modes and verify window titles", async ({ page }) => {
    test.setTimeout(90000);

    // Helper to open modal, verify title, and close
    const assertModalFlow = async (buttonName: string | RegExp, expectedTitle: string | RegExp) => {
      const btn = page.getByRole("button", { name: buttonName });
      await expect(btn).toBeVisible({ timeout: 15000 });
      await btn.click();

      // Assert Win2x window title
      await expect(page.getByText(expectedTitle).first()).toBeVisible({ timeout: 15000 });

      // Close the modal via Escape key
      await page.keyboard.press("Escape");
      await page.waitForTimeout(300);
    };

    // 1. Semantic Graph
    await assertModalFlow("Semantic Graph", "Deep Semantic Graph & Polyglot Isomorphism Engine");

    // 2. Policy Studio
    await assertModalFlow(
      "Policy Studio",
      "Architectural Boundary & Anti-Duplication Policy Studio",
    );

    // 3. Timeline Trends
    await assertModalFlow("Timeline Trends", "Temporal Code Health & Evolution Timeline");

    // 4. Overlap Detector
    await assertModalFlow("Overlap Detector", "Third-Party Library & Dependency Overlap Detector");

    // 5. Org Hub
    await assertModalFlow("Org Hub", "Cross-Repository Knowledge Hub & Organizational Federation");

    // 6. Coverage
    await assertModalFlow("Coverage", "Runtime Execution & Coverage-Aware De-duplication");

    // 7. Dead Code
    await assertModalFlow("Dead Code", "Polyglot Dead Code Explorer & Safe Pruner");

    // 8. Refactor Studio (and check Extract Shared Crate tab)
    const refactorBtn = page.getByRole("button", { name: "Refactor Studio" });
    await expect(refactorBtn).toBeVisible();
    await refactorBtn.click();
    await expect(page.getByText("Interactive Auto-Refactor Sandbox & Visual Studio")).toBeVisible();

    // Verify Extract tab
    const extractTab = page.getByRole("button", { name: /Extract Shared Crate/i });
    await expect(extractTab).toBeVisible();
    await extractTab.click();
    await expect(page.getByText(/Automated Shared Crate & Module Extraction/i)).toBeVisible();

    await page.keyboard.press("Escape");
    await page.waitForTimeout(300);

    // 9. Monorepo
    await assertModalFlow("Monorepo", "Monorepo Workspace & Multi-Package Architecture");

    // 10. MCP Apps
    await assertModalFlow("MCP Apps", "MCP Apps Generative UI Studio");

    // 11. Config Window
    await assertModalFlow("Config Window", "Scan Parameters & Centralized Configuration Studio");

    // 12. Diff Scan
    await assertModalFlow("Diff Scan", "Git Working Tree & PR Branch Diff Analyzer");

    // 13. Suppression Rules
    await assertModalFlow(
      "Suppression Rules",
      "Granular Suppression Rules & De-Duplication Exceptions",
    );

    // 14. Hooks via keyboard shortcut '8'
    await page.keyboard.press("8");
    await expect(page.getByText("Git Hook Manager & CI/CD Studio")).toBeVisible({ timeout: 15000 });
    await page.keyboard.press("Escape");
    await page.waitForTimeout(300);
  });

  test("should launch refactor workflows directly from cluster cards and standalone header", async ({
    page,
  }) => {
    test.setTimeout(90000);

    // 1. Ensure scan results are loaded
    const scanBtn = page
      .getByRole("button", { name: /Run Duplicate Analysis|Scanning Codebase/i })
      .first();
    await expect(scanBtn).toBeVisible({ timeout: 15000 });
    if (await page.getByRole("button", { name: /Run Duplicate Analysis/i }).isVisible()) {
      await page.getByRole("button", { name: /Run Duplicate Analysis/i }).click();
    }
    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 90000 });

    // 2. Switch to N-Way Clusters view
    const clustersTab = page.getByRole("button", { name: /N-Way Clusters/i });
    await expect(clustersTab).toBeVisible();
    await clustersTab.click();

    // 3. Locate first cluster card
    const firstClusterCard = page.locator(".group.bg-slate-900\\/70").first();
    await expect(firstClusterCard).toBeVisible({ timeout: 15000 });

    // 4. Test "Refactor" patch launch on cluster card (without expanding clone pairs)
    const clusterRefactorBtn = firstClusterCard.getByRole("button", { name: "Refactor" });
    await expect(clusterRefactorBtn).toBeVisible();
    await clusterRefactorBtn.click();

    await expect(page.getByText("Multi-Site Refactoring Patch Synthesizer")).toBeVisible({
      timeout: 15000,
    });
    await page.keyboard.press("Escape");
    await page.waitForTimeout(400);

    // 5. Test "Studio" sandbox launch on cluster card (without expanding clone pairs)
    const clusterStudioBtn = firstClusterCard.getByRole("button", { name: "Studio" });
    await expect(clusterStudioBtn).toBeVisible();
    await clusterStudioBtn.click();

    await expect(page.getByText("Interactive Auto-Refactor Sandbox & Visual Studio")).toBeVisible({
      timeout: 15000,
    });
    await page.keyboard.press("Escape");
    await page.waitForTimeout(400);
  });

  test("should verify Code Editor interactive surface, file picker, and split diff mode", async ({
    page,
  }) => {
    test.setTimeout(60000);

    // 1. Open Code Editor from top-level header
    const editorBtn = page.getByRole("button", { name: "Code Editor" });
    await expect(editorBtn).toBeVisible();
    await editorBtn.click();

    await expect(page.getByText("Integrated Code Editor & Split Diff Studio")).toBeVisible({
      timeout: 15000,
    });

    // 2. Verify file dropdown selector exists
    const fileSelect = page.getByLabel("Select file to edit");
    await expect(fileSelect).toBeVisible();

    // 3. Verify language dropdown selector exists
    const langSelect = page.getByLabel("Select syntax language");
    await expect(langSelect).toBeVisible();

    // 4. Verify Side-by-Side split diff mode toggle
    const splitDiffBtn = page.getByRole("button", { name: /Side-by-Side/i });
    if (await splitDiffBtn.isVisible()) {
      await splitDiffBtn.click();
      await page.waitForTimeout(300);
    }

    // 5. Verify Close via Escape key
    await page.keyboard.press("Escape");
    await page.waitForTimeout(300);
    await expect(page.getByText("Integrated Code Editor & Split Diff Studio")).not.toBeVisible();
  });

  test("should verify dead code metrics in summary banner and view mode correlation", async ({
    page,
  }) => {
    test.setTimeout(90000);

    // 1. Wait for scan results
    await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 90000 });

    // 2. Verify Dead Code card exists in SummaryBanner
    const deadCodeBannerCard = page.locator(
      '[title="Click to switch to Polyglot Dead Code Studio"]',
    );
    await expect(deadCodeBannerCard).toBeVisible();
    await expect(deadCodeBannerCard.getByText(/removable LOC/i)).toBeVisible();

    // 3. Click Dead Code banner card to switch view mode
    await deadCodeBannerCard.click();
    await expect(page.getByText("Polyglot Dead Code Studio").first()).toBeVisible();

    // 4. Switch back to Pairwise view
    const pairwiseTab = page.getByRole("button", { name: /Pairwise/i });
    await expect(pairwiseTab).toBeVisible();
    await pairwiseTab.click();
    await expect(page.getByText("Detected Clone Pairs")).toBeVisible();

    // 5. Switch to N-Way Clusters view
    const clustersTab = page.getByRole("button", { name: /N-Way Clusters/i });
    await expect(clustersTab).toBeVisible();
    await clustersTab.click();
    await expect(page.getByText("Detected Clone Clusters")).toBeVisible();
  });
});
