/**
 * E2E screenshot tests for library panel and related components
 */

import { test, expect } from './fixtures';
import { screenshotConfig, gotoAndWait, openKeyboardHelp, setTheme } from './test-utils';

test.describe('Library Panel Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('library panel screenshot', async ({ page }) => {
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toHaveScreenshot('library-panel.png', screenshotConfig);
  });

  test('empty library state screenshot', async ({ page }) => {
    const emptyState = page.locator('#audiobook-list');
    await expect(emptyState).toHaveScreenshot('empty-library-state.png', screenshotConfig);
  });

  test('search input focused screenshot', async ({ page }) => {
    const searchInput = page.locator('input[placeholder*="Search"]');
    await searchInput.focus();
    await page.waitForTimeout(200);
    const toolbar = page.locator('.library-panel > div:first-child');
    await expect(toolbar).toHaveScreenshot('search-focused.png', screenshotConfig);
  });

  test('search input focus ring screenshot', async ({ page }) => {
    const searchInput = page.locator('input[placeholder*="Search"]');
    await searchInput.focus();
    await page.waitForTimeout(200);
    await expect(searchInput).toHaveScreenshot('search-input-focus.png', screenshotConfig);
  });

  test('search input with icon screenshot', async ({ page }) => {
    const searchContainer = page.locator('.library-panel .relative');
    await expect(searchContainer).toHaveScreenshot('search-input-with-icon.png', screenshotConfig);
  });

  test('audiobook card hover state screenshot', async ({ page }) => {
    const firstCard = page.locator('.library-panel .card').first();
    if (await firstCard.isVisible()) {
      await firstCard.hover();
      await page.waitForTimeout(200);
      await expect(firstCard).toHaveScreenshot('audiobook-card-hover.png', screenshotConfig);
    }
  });
});

test.describe('Details Panel Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('details panel screenshot', async ({ page }) => {
    const detailsPanel = page.locator('.flex-1.overflow-hidden.bg-base-100').last();
    await expect(detailsPanel).toHaveScreenshot('details-panel.png', screenshotConfig);
  });

  test('details panel files section screenshot', async ({ page }) => {
    const filesSection = page.locator('#files-list');
    await expect(filesSection).toHaveScreenshot('files-section.png', screenshotConfig);
  });

  test('details panel bookmarks section screenshot', async ({ page }) => {
    const bookmarksSection = page.locator('#bookmarks-list');
    await expect(bookmarksSection).toHaveScreenshot('bookmarks-section.png', screenshotConfig);
  });
});

test.describe('Keyboard Help Modal Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('keyboard help modal screenshot', async ({ page }) => {
    await openKeyboardHelp(page);
    const modal = page.locator('.modal-box');
    await expect(modal).toHaveScreenshot('keyboard-help-modal.png', screenshotConfig);
  });

  test('dark theme keyboard help modal screenshot', async ({ page }) => {
    await setTheme(page, 'Dark');
    await openKeyboardHelp(page);
    const modal = page.locator('.modal-box');
    await expect(modal).toHaveScreenshot('keyboard-help-modal-dark.png', screenshotConfig);
  });
});

test.describe('Keyboard Help Modal Functionality Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('keyboard help modal opens with F1', async ({ page }) => {
    await openKeyboardHelp(page);
    await expect(page.locator('.modal-box')).toBeVisible();
    await expect(page.locator('.modal-box')).toContainText('Keyboard Shortcuts');
  });

  test('keyboard shortcuts modal has kbd elements', async ({ page }) => {
    await openKeyboardHelp(page);
    const kbdElements = page.locator('.modal-box kbd');
    await expect(kbdElements.first()).toBeVisible();
  });
});

test.describe('Accessibility Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('all buttons have accessible labels', async ({ page }) => {
    const buttons = await page.locator('button').all();
    for (const button of buttons) {
      const ariaLabel = await button.getAttribute('aria-label');
      const textContent = await button.textContent();
      expect(ariaLabel || textContent?.trim()).toBeTruthy();
    }
  });

  test('modal has proper focus management', async ({ page }) => {
    await openKeyboardHelp(page);
    await expect(page.locator('.modal-box')).toBeVisible();
    const closeButton = page.locator('.modal-box button[aria-label="Close"]');
    await expect(closeButton).toBeVisible();
  });
});

test.describe('Loading State Screenshots', () => {
  test('loading skeleton screenshot', async ({ page }) => {
    await page.goto('/');
    await page
      .waitForSelector('.skeleton', { timeout: 5000 })
      .catch(() => {})
    ;
    const skeleton = page.locator('.skeleton').first();
    if (await skeleton.isVisible()) {
      await expect(skeleton).toHaveScreenshot('loading-skeleton.png', screenshotConfig);
    }
  });
});
