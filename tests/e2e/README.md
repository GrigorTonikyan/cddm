# CDDM End-to-End (E2E) Test Suite

Automated end-to-end browser tests for the CDDM Studio WebUI using Playwright and Bun.

---

## Running Tests

### 1. Install Dependencies

```bash
bun install
```

### 2. Run Headless Tests

```bash
bun run test
# or:
bunx playwright test
```

### 3. Run Interactive UI Mode

```bash
bunx playwright test --ui
```

Playwright is configured to automatically launch the CDDM Studio WebUI development server on `http://localhost:3000` during test runs.
