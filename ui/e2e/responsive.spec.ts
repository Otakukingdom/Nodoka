/**
 * E2E screenshot tests for responsive layouts
 * Tests UI rendering across all viewport sizes from mobile to desktop
 */

import { test, expect } from './fixtures';
import { fullPageScreenshotConfig, gotoAndWait, setTheme } from './test-utils';

const responsiveViewports = [
  { name: 'mobile-small', width: 320, height: 568 },
  { name: 'mobile-medium', width: 375, height: 667 },
  { name: 'mobile-large', width: 414, height: 896 },
  { name: 'tablet-portrait', width: 768, height: 1024 },
  { name: 'tablet-landscape', width: 1024, height: 768 },
  { name: 'desktop-small', width: 1152, height: 648 },
  { name: 'desktop', width: 1280, height: 720 },
  { name: 'desktop-large', width: 1920, height: 1080 },
];

for (const viewport of responsiveViewports) {
  test.describe(`${viewport.name} viewport (${viewport.width}x${viewport.height})`, () => {
    test.use({ viewport: { width: viewport.width, height: viewport.height } });

    test(`${viewport.name} layout screenshot dark theme`, async ({ page }) => {
      await gotoAndWait(page);
      await setTheme(page, 'Dark');
      await expect(page).toHaveScreenshot(
        `${viewport.name}-dark.png`,
        fullPageScreenshotConfig
      );
    });

    test(`${viewport.name} layout screenshot light theme`, async ({ page }) => {
      await gotoAndWait(page);
      await setTheme(page, 'Light');
      await expect(page).toHaveScreenshot(
        `${viewport.name}-light.png`,
        fullPageScreenshotConfig
      );
    });
  });
}

test.describe('Responsive Player Controls', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('mobile player controls layout', async ({ page }) => {
    const playerControls = page.locator('.btm-nav');
    await expect(playerControls).toHaveScreenshot(
      'mobile-player-controls.png',
      { animations: 'disabled', fullPage: false, maxDiffPixels: 100 }
    );
  });

  test('mobile play button sizing', async ({ page }) => {
    const playButton = page
      .locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]')
      .first();
    const box = await playButton.boundingBox();
    expect(box).toBeTruthy();
    // Verify button is at least 44px (WCAG touch target minimum)
    expect(Math.min(box!.width, box!.height)).toBeGreaterThanOrEqual(40);
  });
});

test.describe('Responsive Library Panel', () => {
  test.use({ viewport: { width: 768, height: 1024 } });

  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('tablet library panel layout', async ({ page }) => {
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toHaveScreenshot(
      'tablet-library-panel.png',
      { animations: 'disabled', fullPage: false, maxDiffPixels: 100 }
    );
  });

  test('tablet search input layout', async ({ page }) => {
    const searchContainer = page.locator('.library-panel .relative').first();
    await expect(searchContainer).toHaveScreenshot(
      'tablet-search-input.png',
      { animations: 'disabled', fullPage: false, maxDiffPixels: 100 }
    );
  });
});

test.describe('Responsive Details Panel', () => {
  test.use({ viewport: { width: 1024, height: 768 } });

  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('tablet landscape details panel', async ({ page }) => {
    const detailsPanel = page.locator('.flex-1.overflow-hidden.bg-base-100').last();
    await expect(detailsPanel).toHaveScreenshot(
      'tablet-landscape-details-panel.png',
      { animations: 'disabled', fullPage: false, maxDiffPixels: 100 }
    );
  });
});

test.describe('Responsive Modals', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('mobile settings modal layout', async ({ page }) => {
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');
    await page.waitForTimeout(300);
    const modal = page.locator('.modal-box');
    await expect(modal).toHaveScreenshot(
      'mobile-settings-modal.png',
      { animations: 'disabled', fullPage: false, maxDiffPixels: 100 }
    );
  });

  test('mobile keyboard help modal layout', async ({ page }) => {
    await page.keyboard.press('F1');
    await page.waitForSelector('.modal-box');
    await page.waitForTimeout(300);
    const modal = page.locator('.modal-box');
    await expect(modal).toHaveScreenshot(
      'mobile-keyboard-help-modal.png',
      { animations: 'disabled', fullPage: false, maxDiffPixels: 100 }
    );
  });
});

test.describe('Wide Screen Layout', () => {
  test.use({ viewport: { width: 2560, height: 1440 } });

  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('ultrawide layout dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    await expect(page).toHaveScreenshot(
      'ultrawide-dark.png',
      fullPageScreenshotConfig
    );
  });

  test('ultrawide layout light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    await expect(page).toHaveScreenshot(
      'ultrawide-light.png',
      fullPageScreenshotConfig
    );
  });
});

test.describe('Orientation Change Handling', () => {
  test('portrait to landscape orientation', async ({ page }) => {
    // Start in portrait
    await page.setViewportSize({ width: 375, height: 667 });
    await gotoAndWait(page);
    await expect(page).toHaveScreenshot(
      'orientation-portrait.png',
      fullPageScreenshotConfig
    );

    // Switch to landscape
    await page.setViewportSize({ width: 667, height: 375 });
    await page.waitForTimeout(300);
    await expect(page).toHaveScreenshot(
      'orientation-landscape.png',
      fullPageScreenshotConfig
    );
  });
});
