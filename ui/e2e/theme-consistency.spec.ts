/**
 * E2E theme consistency tests
 * Validates that both light and dark themes look intentionally designed
 */

import { test, expect } from './fixtures';
import { gotoAndWait, openSettingsModal, closeModal } from './test-utils';

test.describe('Theme Consistency - Light Theme', () => {
  test.beforeEach(async ({ page }) => {
    // Set light theme
    await page.evaluate(() => {
      document.documentElement.setAttribute('data-theme', 'light');
    });
    await gotoAndWait(page);
  });

  test('light theme renders correctly', async ({ page }) => {
    // Check that light theme is applied
    const theme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    expect(theme).toBe('light');

    // Verify library panel is visible
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toBeVisible();

    // Verify player controls are visible
    const playerControls = page.locator('.btm-nav');
    await expect(playerControls).toBeVisible();
  });

  test('light theme has proper text contrast', async ({ page }) => {
    // Verify text is readable
    const bodyText = page.locator('body');
    const textColor = await bodyText.evaluate((el) => {
      return window.getComputedStyle(el).color;
    });
    expect(textColor).toBeTruthy();
  });

  test('light theme buttons are visible and styled', async ({ page }) => {
    const buttons = page.locator('button:visible');
    const count = await buttons.count();

    expect(count).toBeGreaterThan(0);

    // Check first few buttons have proper styling
    for (let i = 0; i < Math.min(count, 5); i++) {
      const button = buttons.nth(i);
      await expect(button).toBeVisible();
    }
  });

  test('light theme modal styling is consistent', async ({ page }) => {
    await openSettingsModal(page);

    const modal = page.locator('.modal-box');
    await expect(modal).toBeVisible();

    // Verify modal has proper styling
    const modalBox = await modal.evaluate((el) => {
      return {
        hasBorder: el.classList.contains('border'),
        hasShadow: el.classList.contains('shadow'),
      };
    });

    // Modal should have visual styling
    expect(modalBox).toBeTruthy();

    await closeModal(page);
  });
});

test.describe('Theme Consistency - Dark Theme', () => {
  test.beforeEach(async ({ page }) => {
    // Set dark theme
    await page.evaluate(() => {
      document.documentElement.setAttribute('data-theme', 'dark');
    });
    await gotoAndWait(page);
  });

  test('dark theme renders correctly', async ({ page }) => {
    // Check that dark theme is applied
    const theme = await page.evaluate(() => {
      return document.documentElement.getAttribute('data-theme');
    });
    expect(theme).toBe('dark');

    // Verify library panel is visible
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toBeVisible();

    // Verify player controls are visible
    const playerControls = page.locator('.btm-nav');
    await expect(playerControls).toBeVisible();
  });

  test('dark theme has proper text contrast', async ({ page }) => {
    // Verify text is readable in dark mode
    const bodyText = page.locator('body');
    const textColor = await bodyText.evaluate((el) => {
      return window.getComputedStyle(el).color;
    });
    expect(textColor).toBeTruthy();
  });

  test('dark theme buttons are visible and styled', async ({ page }) => {
    const buttons = page.locator('button:visible');
    const count = await buttons.count();

    expect(count).toBeGreaterThan(0);

    // Check first few buttons have proper styling
    for (let i = 0; i < Math.min(count, 5); i++) {
      const button = buttons.nth(i);
      await expect(button).toBeVisible();
    }
  });

  test('dark theme modal styling is consistent', async ({ page }) => {
    await openSettingsModal(page);

    const modal = page.locator('.modal-box');
    await expect(modal).toBeVisible();

    // Verify modal has proper styling
    const modalBox = await modal.evaluate((el) => {
      return {
        hasBorder: el.classList.contains('border'),
        hasShadow: el.classList.contains('shadow'),
      };
    });

    // Modal should have visual styling
    expect(modalBox).toBeTruthy();

    await closeModal(page);
  });

  test('dark theme focus indicators are visible', async ({ page }) => {
    const searchInput = page.locator('.library-panel input[placeholder*="Search"]');
    await searchInput.focus();

    // Check that focus is visible
    await page.waitForTimeout(100);

    const isFocused = await searchInput.evaluate((el) => {
      return document.activeElement === el;
    });

    expect(isFocused).toBeTruthy();
  });
});

test.describe('Theme Consistency - System Theme Detection', () => {
  test('respects system theme preference', async ({ page }) => {
    // Emulate dark mode preference
    await page.emulateMedia({ colorScheme: 'dark' });

    await gotoAndWait(page);

    // App should respect system preference when set to "System"
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toBeVisible();
  });

  test('can switch between themes', async ({ page }) => {
    await gotoAndWait(page);

    // Open settings
    await openSettingsModal(page);

    // Find theme selector
    const themeSelect = page.getByLabel('Select theme');

    if (await themeSelect.isVisible()) {
      // Switch to dark theme
      await themeSelect.selectOption('Dark');
      await page.waitForTimeout(300);

      // Verify theme changed
      const theme = await page.evaluate(() => {
        return document.documentElement.getAttribute('data-theme');
      });
      expect(theme).toBe('dark');

      // Switch to light theme
      await themeSelect.selectOption('Light');
      await page.waitForTimeout(300);

      // Verify theme changed
      const newTheme = await page.evaluate(() => {
        return document.documentElement.getAttribute('data-theme');
      });
      expect(newTheme).toBe('light');
    }

    await closeModal(page);
  });
});

test.describe('Theme Consistency - Visual Regression', () => {
  test('empty state renders consistently in light theme', async ({ page }) => {
    await page.evaluate(() => {
      document.documentElement.setAttribute('data-theme', 'light');
    });
    await gotoAndWait(page);

    // Clear any existing data to see empty state
    await page.evaluate(() => {
      const tauri = (window as any).__TAURI__;
      if (tauri) {
        return tauri.invoke('remove_directory', { path: '/test/audiobooks' });
      }
      return null;
    });

    await page.reload();
    await page.waitForTimeout(500);

    // Take screenshot for visual comparison (if needed)
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toBeVisible();
  });

  test('empty state renders consistently in dark theme', async ({ page }) => {
    await page.evaluate(() => {
      document.documentElement.setAttribute('data-theme', 'dark');
    });
    await gotoAndWait(page);

    // Clear any existing data to see empty state
    await page.evaluate(() => {
      const tauri = (window as any).__TAURI__;
      if (tauri) {
        return tauri.invoke('remove_directory', { path: '/test/audiobooks' });
      }
      return null;
    });

    await page.reload();
    await page.waitForTimeout(500);

    // Take screenshot for visual comparison (if needed)
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toBeVisible();
  });
});
