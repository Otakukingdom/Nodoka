/**
 * Dedicated Playwright configuration for screenshot generation
 * Optimized for comprehensive UI state capture
 */

import { defineConfig, devices } from '@playwright/test';

// Output directory from environment or default
const outputDir = process.env.SCREENSHOT_OUTPUT_DIR || 'screenshots';

export default defineConfig({
  testDir: './e2e',
  testMatch: 'screenshot-generation.spec.ts',
  fullyParallel: true, // Enable parallel execution for faster screenshots
  forbidOnly: true,
  retries: 0, // No retries for faster execution
  workers: process.env.SCREENSHOT_WORKERS ? parseInt(process.env.SCREENSHOT_WORKERS, 10) : 5, // Pool of 5 browser instances
  reporter: [['list'], ['html', { outputFolder: 'playwright-report-screenshots' }]],
  outputDir: './test-results-screenshots',
  snapshotDir: './e2e-screenshots',
  timeout: 60000, // 1 minute per test
  expect: {
    toHaveScreenshot: {
      maxDiffPixels: 200,
      maxDiffPixelRatio: 0.02,
      animations: 'disabled',
      // Use custom path template for organized output
      snapshotPathTemplate: `{snapshotDir}/${outputDir}/{testFilePath}/{arg}{ext}`,
    },
  },
  use: {
    baseURL: 'http://localhost:5173',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
    viewport: { width: 1280, height: 720 },
  },
  projects: [
    // Single project - tests handle theme/viewport changes internally via helpers
    {
      name: 'screenshot-chromium',
      use: {
        ...devices['Desktop Chrome'],
        viewport: { width: 1280, height: 720 },
      },
    },
  ],
  webServer: {
    command: 'VITE_PLAYWRIGHT_TEST=true npm run dev',
    url: 'http://localhost:5173',
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
  },
});
