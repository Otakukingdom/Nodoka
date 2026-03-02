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
      // Note: {platform} in snapshot names creates separate screenshots per OS (darwin, linux, win32).
      // This prevents false failures due to font rendering and UI differences across platforms.
      // Team members on different OSes will need to generate their own baseline screenshots.
      // If sharing baselines across platforms is required, remove {platform} and increase maxDiffPixels.
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
    command: 'VITE_PLAYWRIGHT_TEST=true npm run dev',
    url: 'http://localhost:5173',
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
  },
});
