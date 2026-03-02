/**
 * E2E screenshot tests for app layout
 */

import { test, expect, viewports } from './fixtures';
import { screenshotConfig, fullPageScreenshotConfig, setTheme, gotoAndWait, forceLoadingState } from './test-utils';

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

test.describe('Panel Resize Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('panel divider exists', async ({ page }) => {
    const divider = page.locator('main [role="separator"]');
    await expect(divider).toBeVisible();
    await expect(divider).toHaveAttribute('aria-orientation', 'vertical');
  });

  test('panel divider hover state screenshot', async ({ page }) => {
    const divider = page.locator('main [role="separator"]');
    await divider.hover();
    await page.waitForTimeout(200);
    await expect(divider).toHaveScreenshot('panel-divider-hover.png', screenshotConfig);
  });

  test('panel divider has cursor style', async ({ page }) => {
    const divider = page.locator('main [role="separator"]');
    const cursor = await divider.evaluate((el) => {
      return window.getComputedStyle(el).cursor;
    });
    expect(cursor).toBe('col-resize');
  });

  test('panel divider is keyboard accessible', async ({ page }) => {
    const divider = page.locator('main [role="separator"]');
    await expect(divider).toHaveAttribute('tabindex', '0');
  });

  test('default panel layout screenshot', async ({ page }) => {
    const mainContent = page.locator('main');
    await expect(mainContent).toHaveScreenshot('default-panel-layout.png', screenshotConfig);
  });

  test('double-click resets panel ratio', async ({ page }) => {
    const divider = page.locator('main [role="separator"]');
    const mainContent = page.locator('main');
    const firstChild = mainContent.locator('> div:first-child');

    // Get initial width
    const initialWidth = await firstChild.evaluate((el) => el.getBoundingClientRect().width);

    // Double-click to reset (should stay at 40% default)
    await divider.dblclick();
    await page.waitForTimeout(200);

    // Verify the width is approximately 40% of container
    const containerWidth = await mainContent.evaluate((el) => el.getBoundingClientRect().width);
    const newWidth = await firstChild.evaluate((el) => el.getBoundingClientRect().width);
    const ratio = newWidth / containerWidth;

    // Allow some tolerance for rounding
    expect(ratio).toBeCloseTo(0.4, 1);
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

test.describe('Complete Application Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('Full application - default state', async ({ page }) => {
    await expect(page).toHaveScreenshot('app-full-default.png', fullPageScreenshotConfig);
  });

  test('Full application - loading state', async ({ page }) => {
    // Use forceLoadingState to reliably capture loading state
    await forceLoadingState(page, true);
    await page.waitForTimeout(300);
    await expect(page).toHaveScreenshot('app-full-loading.png', { ...fullPageScreenshotConfig, animations: 'allow' });
    await forceLoadingState(page, false);
  });

  test('Full application - all themes comparison', async ({ page }) => {
    await setTheme(page, 'Light');
    await expect(page).toHaveScreenshot('app-full-light.png', fullPageScreenshotConfig);
    await setTheme(page, 'Dark');
    await expect(page).toHaveScreenshot('app-full-dark.png', fullPageScreenshotConfig);
  });

  test('Full application - system theme', async ({ page }) => {
    await setTheme(page, 'System');
    await expect(page).toHaveScreenshot('app-full-system-theme.png', fullPageScreenshotConfig);
  });
});
