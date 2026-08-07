# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: scan-workflow.spec.ts >> CDDM WebUI E2E Workflows >> should render main app header and scan configuration panel
- Location: scan-workflow.spec.ts:4:7

# Error details

```
Error: page.goto: net::ERR_CONNECTION_REFUSED at http://localhost:3000/
Call log:
  - navigating to "http://localhost:3000/", waiting until "load"

```

# Test source

```ts
  1  | import { test, expect } from "@playwright/test";
  2  | 
  3  | test.describe("CDDM WebUI E2E Workflows", () => {
  4  |   test("should render main app header and scan configuration panel", async ({ page }) => {
> 5  |     await page.goto("/");
     |                ^ Error: page.goto: net::ERR_CONNECTION_REFUSED at http://localhost:3000/
  6  |     await expect(page.locator("h1")).toContainText("CDDM Studio");
  7  |     await expect(page.getByText("Scan Configuration")).toBeVisible();
  8  |     await expect(page.getByText("Run Duplicate Analysis")).toBeVisible();
  9  |   });
  10 | 
  11 |   test("should update min token threshold slider and trigger scan", async ({ page }) => {
  12 |     await page.goto("/");
  13 |     const slider = page.locator('input[type="range"]');
  14 |     await slider.fill("60");
  15 | 
  16 |     const runBtn = page.getByRole("button", { name: /Run Duplicate Analysis/i });
  17 |     await runBtn.click();
  18 | 
  19 |     // Verify DRY Health Score renders
  20 |     await expect(page.getByText("DRY Health Score")).toBeVisible({ timeout: 25000 });
  21 |   });
  22 | });
  23 | 
```