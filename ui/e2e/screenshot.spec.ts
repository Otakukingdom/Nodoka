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
    // Wait for modal animation to complete
    await page.waitForTimeout(300);
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
    // Wait for modal animation to complete
    await page.waitForTimeout(300);
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
    // Close modal - updated to use new button text
    await page.click('.modal-box button:has-text("Done")');
    await page.waitForTimeout(300);
    await expect(page).toHaveScreenshot('dark-theme.png', {
      fullPage: true,
      animations: 'disabled',
    });
  });

  test('light theme screenshot', async ({ page }) => {
    // Ensure light theme is active
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');
    await page.selectOption('.modal-box select', 'Light');
    await page.waitForTimeout(500);
    await page.click('.modal-box button:has-text("Done")');
    await page.waitForTimeout(300);
    await expect(page).toHaveScreenshot('light-theme.png', {
      fullPage: true,
      animations: 'disabled',
    });
  });

  test('empty library state screenshot', async ({ page }) => {
    // Check if empty state is visible
    const emptyState = page.locator('#audiobook-list');
    await expect(emptyState).toHaveScreenshot('empty-library-state.png', {
      animations: 'disabled',
    });
  });

  test('details panel files section screenshot', async ({ page }) => {
    const filesSection = page.locator('#files-list');
    await expect(filesSection).toHaveScreenshot('files-section.png', {
      animations: 'disabled',
    });
  });

  test('details panel bookmarks section screenshot', async ({ page }) => {
    const bookmarksSection = page.locator('#bookmarks-list');
    await expect(bookmarksSection).toHaveScreenshot('bookmarks-section.png', {
      animations: 'disabled',
    });
  });

  test('search input focused screenshot', async ({ page }) => {
    const searchInput = page.locator('input[placeholder*="Search"]');
    await searchInput.focus();
    await page.waitForTimeout(200);
    const toolbar = page.locator('.library-panel > div:first-child');
    await expect(toolbar).toHaveScreenshot('search-focused.png', {
      animations: 'disabled',
    });
  });

  test('modal backdrop screenshot', async ({ page }) => {
    // Open settings modal
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');
    await page.waitForTimeout(300);
    // Take full page screenshot to capture backdrop
    await expect(page).toHaveScreenshot('modal-backdrop.png', {
      animations: 'disabled',
    });
  });
});

test.describe('UI Component Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.navbar', { timeout: 10000 });
  });

  test('top bar has gradient logo', async ({ page }) => {
    const logo = page.locator('.navbar .gradient-premium');
    await expect(logo).toBeVisible();
  });

  test('player controls has play button', async ({ page }) => {
    const playButton = page.locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]');
    await expect(playButton).toBeVisible();
  });

  test('settings button has tooltip', async ({ page }) => {
    const settingsButton = page.locator('button[aria-label="Settings"]');
    await expect(settingsButton).toHaveAttribute('title', 'Settings (Ctrl+,)');
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

    // Close with button - updated to use new button text
    await page.click('.modal-box button:has-text("Done")');
    await page.waitForTimeout(300);
    await expect(page.locator('.modal-box')).not.toBeVisible();
  });

  test('keyboard help modal opens with F1', async ({ page }) => {
    await page.keyboard.press('F1');
    await expect(page.locator('.modal-box')).toBeVisible();
    await expect(page.locator('.modal-box')).toContainText('Keyboard Shortcuts');
  });

  test('escape key closes modal', async ({ page }) => {
    await page.click('button[aria-label="Settings"]');
    await expect(page.locator('.modal-box')).toBeVisible();

    await page.keyboard.press('Escape');
    await page.waitForTimeout(300);
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

  test('settings modal has close button in header', async ({ page }) => {
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');

    // Check for X button in header
    const closeButton = page.locator('.modal-box button[aria-label="Close"]');
    await expect(closeButton).toBeVisible();
  });

  test('keyboard shortcuts modal has kbd elements', async ({ page }) => {
    await page.keyboard.press('F1');
    await page.waitForSelector('.modal-box');

    // Check for kbd elements
    const kbdElements = page.locator('.modal-box kbd');
    await expect(kbdElements.first()).toBeVisible();
  });
});

test.describe('Accessibility Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.navbar', { timeout: 10000 });
  });

  test('all buttons have accessible labels', async ({ page }) => {
    const buttons = await page.locator('button').all();
    for (const button of buttons) {
      const ariaLabel = await button.getAttribute('aria-label');
      const textContent = await button.textContent();
      // Button should have either aria-label or text content
      expect(ariaLabel || textContent?.trim()).toBeTruthy();
    }
  });

  test('modal has proper focus management', async ({ page }) => {
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');

    // Modal should be visible
    await expect(page.locator('.modal-box')).toBeVisible();

    // Close button should be focusable
    const closeButton = page.locator('.modal-box button[aria-label="Close"]');
    await expect(closeButton).toBeVisible();
  });
});
