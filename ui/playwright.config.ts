/**
 * Playwright e2e test configuration
 */

import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [['html'], ['list']],
  outputDir: './test-results',
  snapshotDir: './e2e-screenshots',
  expect: {
    // Allow small pixel differences for cross-platform consistency
    toHaveScreenshot: {
      maxDiffPixels: 100,
      maxDiffPixelRatio: 0.01,
      animations: 'disabled',
      // Use consistent naming for cross-platform compatibility
      snapshotPathTemplate: '{snapshotDir}/{testFilePath}/{arg}-{platform}{ext}',
    },
  },
  use: {
    baseURL: 'http://localhost:5173',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
    // Consistent viewport for screenshot tests
    viewport: { width: 1280, height: 720 },
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: {
    command: 'PLAYWRIGHT_TEST=true npm run dev',
    url: 'http://localhost:5173',
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
  },
});
