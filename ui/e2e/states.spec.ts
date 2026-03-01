/**
 * E2E screenshot tests for various UI states
 * Captures empty states, loading states, and other special UI conditions
 */

import { test, expect } from './fixtures';
import { screenshotConfig, fullPageScreenshotConfig, gotoAndWait, setTheme } from './test-utils';

test.describe('Empty State Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('empty library state light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    const emptyState = page.locator('#audiobook-list');
    await expect(emptyState).toHaveScreenshot('empty-library-light.png', screenshotConfig);
  });

  test('empty library state dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const emptyState = page.locator('#audiobook-list');
    await expect(emptyState).toHaveScreenshot('empty-library-dark.png', screenshotConfig);
  });

  test('empty bookmarks state light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    const bookmarksSection = page.locator('#bookmarks-list');
    await expect(bookmarksSection).toHaveScreenshot('empty-bookmarks-light.png', screenshotConfig);
  });

  test('empty bookmarks state dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const bookmarksSection = page.locator('#bookmarks-list');
    await expect(bookmarksSection).toHaveScreenshot('empty-bookmarks-dark.png', screenshotConfig);
  });

  test('empty files state light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    const filesSection = page.locator('#files-list');
    await expect(filesSection).toHaveScreenshot('empty-files-light.png', screenshotConfig);
  });

  test('empty files state dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const filesSection = page.locator('#files-list');
    await expect(filesSection).toHaveScreenshot('empty-files-dark.png', screenshotConfig);
  });
});

test.describe('Loading State Screenshots', () => {
  test('skeleton loading animation frame', async ({ page }) => {
    await page.goto('/');
    // Try to catch skeleton state before app fully loads
    const skeleton = page.locator('.skeleton').first();
    const isVisible = await skeleton.isVisible({ timeout: 3000 }).catch(() => false);
    if (isVisible) {
      await expect(skeleton).toHaveScreenshot('skeleton-frame.png', screenshotConfig);
    }
  });

  test('skeleton with shimmer effect', async ({ page }) => {
    await page.goto('/');
    const skeletonShimmer = page.locator('.skeleton-shimmer, .skeleton').first();
    const isVisible = await skeletonShimmer.isVisible({ timeout: 3000 }).catch(() => false);
    if (isVisible) {
      await expect(skeletonShimmer).toHaveScreenshot('skeleton-shimmer.png', {
        ...screenshotConfig,
        animations: 'allow',
      });
    }
  });
});

test.describe('Initial App State Screenshots', () => {
  test('initial app load state', async ({ page }) => {
    await page.goto('/');
    // Capture state immediately after navigation
    await page.waitForLoadState('domcontentloaded');
    await expect(page).toHaveScreenshot('initial-load-state.png', fullPageScreenshotConfig);
  });

  test('app fully loaded state', async ({ page }) => {
    await gotoAndWait(page);
    await expect(page).toHaveScreenshot('fully-loaded-state.png', fullPageScreenshotConfig);
  });
});

test.describe('Player State Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('player idle state', async ({ page }) => {
    const playerControls = page.locator('.btm-nav');
    await expect(playerControls).toHaveScreenshot('player-idle-state.png', screenshotConfig);
  });

  test('player with disabled controls', async ({ page }) => {
    // When no audiobook is loaded, some controls may be disabled
    const playerControls = page.locator('.btm-nav');
    await expect(playerControls).toHaveScreenshot('player-disabled-state.png', screenshotConfig);
  });
});

test.describe('Theme Transition States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('light theme applied state', async ({ page }) => {
    await setTheme(page, 'Light');
    const html = page.locator('html');
    await expect(html).toHaveAttribute('data-theme', 'light');
    await expect(page).toHaveScreenshot('light-theme-applied.png', fullPageScreenshotConfig);
  });

  test('dark theme applied state', async ({ page }) => {
    await setTheme(page, 'Dark');
    const html = page.locator('html');
    await expect(html).toHaveAttribute('data-theme', 'dark');
    await expect(page).toHaveScreenshot('dark-theme-applied.png', fullPageScreenshotConfig);
  });

  test('system theme state', async ({ page }) => {
    await gotoAndWait(page);
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');
    await page.selectOption('.modal-box select', 'System');
    await page.waitForTimeout(500);
    await page.click('.modal-box button:has-text("Done")');
    await page.waitForTimeout(300);
    // System theme should match prefers-color-scheme
    await expect(page).toHaveScreenshot('system-theme-applied.png', fullPageScreenshotConfig);
  });
});

test.describe('Details Panel State Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('details panel empty state light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    const detailsPanel = page.locator('.flex-1.overflow-hidden.bg-base-100').last();
    await expect(detailsPanel).toHaveScreenshot('details-panel-empty-light.png', screenshotConfig);
  });

  test('details panel empty state dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const detailsPanel = page.locator('.flex-1.overflow-hidden.bg-base-100').last();
    await expect(detailsPanel).toHaveScreenshot('details-panel-empty-dark.png', screenshotConfig);
  });
});

test.describe('Library Panel State Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('library panel empty state light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toHaveScreenshot('library-panel-empty-light.png', screenshotConfig);
  });

  test('library panel empty state dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toHaveScreenshot('library-panel-empty-dark.png', screenshotConfig);
  });
});

test.describe('Search State Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('search input empty state', async ({ page }) => {
    const searchInput = page.locator('input[placeholder*="Search"]');
    await expect(searchInput).toHaveScreenshot('search-empty.png', screenshotConfig);
  });

  test('search input with query', async ({ page }) => {
    const searchInput = page.locator('input[placeholder*="Search"]');
    await searchInput.fill('fantasy audiobook');
    await page.waitForTimeout(200);
    await expect(searchInput).toHaveScreenshot('search-with-query.png', screenshotConfig);
  });

  test('search no results state', async ({ page }) => {
    const searchInput = page.locator('input[placeholder*="Search"]');
    await searchInput.fill('zzzzzznonexistentbook');
    await page.waitForTimeout(500);
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toHaveScreenshot('search-no-results.png', screenshotConfig);
  });
});

test.describe('Focus State Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('body focus state', async ({ page }) => {
    await page.keyboard.press('Tab');
    await page.waitForTimeout(200);
    await expect(page).toHaveScreenshot('initial-focus-state.png', fullPageScreenshotConfig);
  });

  test('settings button focus state', async ({ page }) => {
    const settingsButton = page.locator('button[aria-label="Settings"]');
    await settingsButton.focus();
    await page.waitForTimeout(200);
    await expect(settingsButton).toHaveScreenshot('settings-focus-state.png', screenshotConfig);
  });
});
