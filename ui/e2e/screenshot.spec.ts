/**
 * E2E screenshot tests for visual regression testing
 */

import { test, expect } from '@playwright/test';

test.describe('UI Screenshot Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for app to load (loading spinner should disappear)
    await page.waitForSelector('.navbar', { timeout: 10000 });
    // Wait for the loading state to complete
    await page.waitForFunction(() => {
      const loading = document.querySelector('.loading-spinner');
      return loading === null;
    }, { timeout: 15000 });
  });

  test('main layout screenshot', async ({ page }) => {
    await expect(page).toHaveScreenshot('main-layout.png', {
      fullPage: true,
      animations: 'disabled',
    });
  });

  test('library panel screenshot', async ({ page }) => {
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toHaveScreenshot('library-panel.png', {
      animations: 'disabled',
    });
  });

  test('player controls screenshot', async ({ page }) => {
    const playerControls = page.locator('.btm-nav');
    await expect(playerControls).toHaveScreenshot('player-controls.png', {
      animations: 'disabled',
    });
  });

  test('top bar screenshot', async ({ page }) => {
    const topBar = page.locator('.navbar');
    await expect(topBar).toHaveScreenshot('top-bar.png', {
      animations: 'disabled',
    });
  });

  test('settings modal screenshot', async ({ page }) => {
    // Click settings button
    await page.click('button[aria-label="Settings"]');
    // Wait for modal to appear
    await page.waitForSelector('.modal-box');
    const modal = page.locator('.modal-box');
    await expect(modal).toHaveScreenshot('settings-modal.png', {
      animations: 'disabled',
    });
  });

  test('keyboard help modal screenshot', async ({ page }) => {
    // Press F1 to open keyboard help
    await page.keyboard.press('F1');
    // Wait for modal to appear
    await page.waitForSelector('.modal-box');
    const modal = page.locator('.modal-box');
    await expect(modal).toHaveScreenshot('keyboard-help-modal.png', {
      animations: 'disabled',
    });
  });

  test('dark theme screenshot', async ({ page }) => {
    // Open settings
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');
    // Switch to dark theme
    await page.selectOption('.modal-box select', 'Dark');
    await page.waitForTimeout(500);
    // Close modal
    await page.click('.modal-box button:has-text("Close")');
    await page.waitForTimeout(300);
    await expect(page).toHaveScreenshot('dark-theme.png', {
      fullPage: true,
      animations: 'disabled',
    });
  });
});

test.describe('UI Functionality Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.navbar', { timeout: 10000 });
  });

  test('settings modal opens and closes', async ({ page }) => {
    // Open settings
    await page.click('button[aria-label="Settings"]');
    await expect(page.locator('.modal-box')).toBeVisible();

    // Close with button
    await page.click('.modal-box button:has-text("Close")');
    await expect(page.locator('.modal-box')).not.toBeVisible();
  });

  test('keyboard help modal opens with F1', async ({ page }) => {
    await page.keyboard.press('F1');
    await expect(page.locator('.modal-box')).toBeVisible();
    await expect(page.locator('.modal-box h3')).toContainText('Keyboard Shortcuts');
  });

  test('escape key closes modal', async ({ page }) => {
    await page.click('button[aria-label="Settings"]');
    await expect(page.locator('.modal-box')).toBeVisible();

    await page.keyboard.press('Escape');
    await expect(page.locator('.modal-box')).not.toBeVisible();
  });

  test('theme toggle changes theme', async ({ page }) => {
    // Open settings
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');

    // Check initial theme
    const html = page.locator('html');
    const initialTheme = await html.getAttribute('data-theme');

    // Switch theme
    await page.selectOption('.modal-box select', 'Dark');
    await page.waitForTimeout(300);

    // Verify theme changed (should have dark class or data-theme)
    const newTheme = await html.getAttribute('data-theme');
    expect(newTheme).not.toBe(initialTheme);
  });
});
