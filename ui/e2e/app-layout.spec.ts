/**
 * E2E screenshot tests for app layout
 */

import { test, expect, viewports } from './fixtures';
import { screenshotConfig, fullPageScreenshotConfig, setTheme, gotoAndWait } from './test-utils';

test.describe('App Layout Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('main layout screenshot', async ({ page }) => {
    await expect(page).toHaveScreenshot('main-layout.png', fullPageScreenshotConfig);
  });

  test('top bar screenshot', async ({ page }) => {
    const topBar = page.locator('.navbar');
    await expect(topBar).toHaveScreenshot('top-bar.png', screenshotConfig);
  });

  test('dark theme screenshot', async ({ page }) => {
    await setTheme(page, 'Dark');
    await expect(page).toHaveScreenshot('dark-theme.png', fullPageScreenshotConfig);
  });

  test('light theme screenshot', async ({ page }) => {
    await setTheme(page, 'Light');
    await expect(page).toHaveScreenshot('light-theme.png', fullPageScreenshotConfig);
  });

  test('modal backdrop screenshot', async ({ page }) => {
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');
    await page.waitForTimeout(300);
    await expect(page).toHaveScreenshot('modal-backdrop.png', fullPageScreenshotConfig);
  });
});

test.describe('App Layout Component Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('top bar has gradient logo', async ({ page }) => {
    const logo = page.locator('.navbar .gradient-premium');
    await expect(logo).toBeVisible();
  });

  test('logo hover state screenshot', async ({ page }) => {
    const logo = page.locator('.navbar .gradient-premium');
    await logo.hover();
    await page.waitForTimeout(300);
    await expect(logo).toHaveScreenshot('logo-hover.png', screenshotConfig);
  });
});

test.describe('Responsive Layout Screenshots', () => {
  test.use({ viewport: viewports.tablet });

  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('tablet viewport main layout screenshot', async ({ page }) => {
    await expect(page).toHaveScreenshot('tablet-layout.png', fullPageScreenshotConfig);
  });
});

test.describe('Small Viewport Layout Screenshots', () => {
  test.use({ viewport: viewports.small });

  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('small viewport main layout screenshot', async ({ page }) => {
    await expect(page).toHaveScreenshot('small-viewport-layout.png', fullPageScreenshotConfig);
  });
});

test.describe('Mobile Viewport Layout Screenshots', () => {
  test.use({ viewport: viewports.mobile });

  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('mobile portrait layout screenshot', async ({ page }) => {
    await expect(page).toHaveScreenshot('mobile-portrait-layout.png', fullPageScreenshotConfig);
  });
});
