/**
 * Behavioral tests for Settings Management
 * Tests actual UI interactions matching cucumber acceptance criteria
 */

import { test, expect } from '../fixtures';
import { gotoAndWait, openSettingsModal, closeModal } from '../test-utils';

test.describe('Settings Management - Behavioral Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test.describe('Settings Modal Access', () => {
    test('settings button is visible in top bar', async ({ page }) => {
      const settingsButton = page.locator('button[aria-label="Settings"]');
      await expect(settingsButton).toBeVisible();
    });

    test('settings button has tooltip', async ({ page }) => {
      const settingsButton = page.locator('button[aria-label="Settings"]');
      const title = await settingsButton.getAttribute('title');
      expect(title).toContain('Settings');
    });

    test('clicking settings button opens modal', async ({ page }) => {
      await openSettingsModal(page);

      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();
    });

    test('settings modal shows header', async ({ page }) => {
      await openSettingsModal(page);

      const header = page.locator('.modal-box h3:has-text("Settings")');
      await expect(header).toBeVisible();
    });

    test('settings modal shows description', async ({ page }) => {
      await openSettingsModal(page);

      const description = page.locator('.modal-box').getByText('Customize your experience');
      await expect(description).toBeVisible();
    });
  });

  test.describe('Theme Selection', () => {
    test('theme dropdown is visible', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      await expect(themeSelect).toBeVisible();
    });

    test('theme dropdown has options', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      const options = await themeSelect.locator('option').allTextContents();

      expect(options).toContain('Light');
      expect(options).toContain('Dark');
      expect(options).toContain('System');
    });

    test('theme can be changed to Dark', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      await themeSelect.selectOption('Dark');

      // Value should be Dark
      await expect(themeSelect).toHaveValue('Dark');
    });

    test('theme can be changed to Light', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      await themeSelect.selectOption('Light');

      await expect(themeSelect).toHaveValue('Light');
    });

    test('theme can be changed to System', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      await themeSelect.selectOption('System');

      await expect(themeSelect).toHaveValue('System');
    });

    test('theme label is visible', async ({ page }) => {
      await openSettingsModal(page);

      const themeLabel = page.locator('.modal-box .label-text:has-text("Theme")');
      await expect(themeLabel).toBeVisible();
    });

    test('theme description is visible', async ({ page }) => {
      await openSettingsModal(page);

      const description = page.locator('.modal-box').getByText('Choose your preferred appearance');
      await expect(description).toBeVisible();
    });
  });

  test.describe('Modal Controls', () => {
    test('close button is visible', async ({ page }) => {
      await openSettingsModal(page);

      const closeButton = page.locator('.modal-box button[aria-label="Close"]');
      await expect(closeButton).toBeVisible();
    });

    test('close button closes modal', async ({ page }) => {
      await openSettingsModal(page);

      const closeButton = page.locator('.modal-box button[aria-label="Close"]');
      await closeButton.click();
      await page.waitForTimeout(300);

      const modal = page.locator('.modal-box');
      await expect(modal).not.toBeVisible();
    });

    test('Done button is visible', async ({ page }) => {
      await openSettingsModal(page);

      const doneButton = page.locator('.modal-box button:has-text("Done")');
      await expect(doneButton).toBeVisible();
    });

    test('Done button closes modal', async ({ page }) => {
      await openSettingsModal(page);

      const doneButton = page.locator('.modal-box button:has-text("Done")');
      await doneButton.click();
      await page.waitForTimeout(300);

      const modal = page.locator('.modal-box');
      await expect(modal).not.toBeVisible();
    });

    test('Escape key closes modal', async ({ page }) => {
      await openSettingsModal(page);

      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);

      const modal = page.locator('.modal-box');
      await expect(modal).not.toBeVisible();
    });
  });

  test.describe('Modal Styling', () => {
    test('modal has icon', async ({ page }) => {
      await openSettingsModal(page);

      const icon = page.locator('.modal-box .gradient-premium');
      await expect(icon).toBeVisible();
    });

    test('modal has shadow', async ({ page }) => {
      await openSettingsModal(page);

      const modal = page.locator('.modal-box');
      await expect(modal).toHaveClass(/shadow/);
    });

    test('Done button has primary styling', async ({ page }) => {
      await openSettingsModal(page);

      const doneButton = page.locator('.modal-box button:has-text("Done")');
      await expect(doneButton).toHaveClass(/btn-primary/);
    });
  });

  test.describe('Focus Management', () => {
    test('close button is focusable', async ({ page }) => {
      await openSettingsModal(page);

      const closeButton = page.locator('.modal-box button[aria-label="Close"]');
      await closeButton.focus();

      await expect(closeButton).toBeFocused();
    });

    test('theme select is focusable', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      await themeSelect.focus();

      await expect(themeSelect).toBeFocused();
    });

    test('Done button is focusable', async ({ page }) => {
      await openSettingsModal(page);

      const doneButton = page.locator('.modal-box button:has-text("Done")');
      await doneButton.focus();

      await expect(doneButton).toBeFocused();
    });

    test('Tab key cycles through focusable elements', async ({ page }) => {
      await openSettingsModal(page);

      // Tab through elements
      await page.keyboard.press('Tab');
      await page.keyboard.press('Tab');
      await page.keyboard.press('Tab');

      // Should still be in modal
      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();
    });
  });

  test.describe('Accessibility', () => {
    test('settings button has aria-label', async ({ page }) => {
      const settingsButton = page.locator('button[aria-label="Settings"]');
      const ariaLabel = await settingsButton.getAttribute('aria-label');
      expect(ariaLabel).toBe('Settings');
    });

    test('close button has aria-label', async ({ page }) => {
      await openSettingsModal(page);

      const closeButton = page.locator('.modal-box button[aria-label="Close"]');
      const ariaLabel = await closeButton.getAttribute('aria-label');
      expect(ariaLabel).toBe('Close');
    });

    test('modal has proper heading structure', async ({ page }) => {
      await openSettingsModal(page);

      const heading = page.locator('.modal-box h3');
      const headingText = await heading.textContent();
      expect(headingText).toBe('Settings');
    });
  });

  test.describe('Keyboard Shortcuts', () => {
    test('Ctrl+, opens settings modal', async ({ page }) => {
      await page.keyboard.press('Control+,');
      await page.waitForTimeout(300);

      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();
    });
  });

  test.describe('Modal Behavior', () => {
    test('modal has backdrop', async ({ page }) => {
      await openSettingsModal(page);

      const modal = page.locator('.modal');
      await expect(modal).toBeVisible();
    });

    test('settings persist across modal close', async ({ page }) => {
      await openSettingsModal(page);

      // Change theme to Dark
      const themeSelect = page.locator('.modal-box select');
      await themeSelect.selectOption('Dark');

      // Close modal
      await closeModal(page);

      // Reopen modal
      await openSettingsModal(page);

      // Theme should still be Dark
      await expect(themeSelect).toHaveValue('Dark');
    });
  });

  test.describe('Top Bar Logo', () => {
    test('logo is visible', async ({ page }) => {
      const logo = page.locator('.navbar .gradient-premium');
      await expect(logo).toBeVisible();
    });

    test('app name is visible', async ({ page }) => {
      const appName = page.locator('.navbar').getByText('Nodoka');
      await expect(appName).toBeVisible();
    });

    test('app subtitle is visible', async ({ page }) => {
      const subtitle = page.locator('.navbar').getByText('Audiobook Reader');
      await expect(subtitle).toBeVisible();
    });
  });
});
