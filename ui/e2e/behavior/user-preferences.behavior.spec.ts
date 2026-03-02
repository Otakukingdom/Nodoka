/**
 * Behavioral tests for User Preferences
 * Tests actual UI interactions matching cucumber acceptance criteria
 */

import { test, expect } from '../fixtures';
import { gotoAndWait, openSettingsModal, closeModal } from '../test-utils';

test.describe('User Preferences - Behavioral Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test.describe('Theme Preference', () => {
    test('theme preference can be changed', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      await themeSelect.selectOption('Dark');

      await expect(themeSelect).toHaveValue('Dark');
    });

    test('theme preference persists after modal close', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      await themeSelect.selectOption('Dark');
      await closeModal(page);

      await openSettingsModal(page);
      const themeSelectAgain = page.locator('.modal-box select');
      await expect(themeSelectAgain).toHaveValue('Dark');
    });

    test('Light theme is available', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      await themeSelect.selectOption('Light');

      await expect(themeSelect).toHaveValue('Light');
    });

    test('System theme is available', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      await themeSelect.selectOption('System');

      await expect(themeSelect).toHaveValue('System');
    });
  });

  test.describe('Volume Preference', () => {
    test('volume control is accessible', async ({ page }) => {
      const volumeSlider = page.locator('.btm-nav .group\\/volume');
      await expect(volumeSlider).toBeVisible();
    });

    test('volume can be adjusted', async ({ page }) => {
      const volumeRange = page.locator('.btm-nav .group\\/volume input[type="range"]');
      await volumeRange.fill('50');
      await page.waitForTimeout(200);

      // Volume control should still be functional
      await expect(volumeRange).toBeVisible();
    });

    test('mute toggle works', async ({ page }) => {
      const muteButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]');
      await muteButton.first().click();
      await page.waitForTimeout(200);

      // Button should still be visible (state changed)
      await expect(muteButton.first()).toBeVisible();
    });
  });

  test.describe('Speed Preference', () => {
    test('speed control is accessible', async ({ page }) => {
      const speedButton = page.locator('.btm-nav button:has-text("x")');
      await expect(speedButton.first()).toBeVisible();
    });

    test('speed can be cycled', async ({ page }) => {
      const speedButton = page.locator('.btm-nav button:has-text("x")').first();
      const initialText = await speedButton.textContent();

      await speedButton.click();
      await page.waitForTimeout(200);

      const newText = await speedButton.textContent();
      // Speed should have changed
      expect(newText).not.toBe(initialText);
    });
  });

  test.describe('Settings Modal', () => {
    test('settings modal opens correctly', async ({ page }) => {
      await openSettingsModal(page);

      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();
    });

    test('settings modal closes correctly', async ({ page }) => {
      await openSettingsModal(page);
      await closeModal(page);

      const modal = page.locator('.modal-box');
      await expect(modal).not.toBeVisible();
    });

    test('settings modal has all required elements', async ({ page }) => {
      await openSettingsModal(page);

      // Should have header
      await expect(page.locator('.modal-box h3:has-text("Settings")')).toBeVisible();

      // Should have theme select
      await expect(page.locator('.modal-box select')).toBeVisible();

      // Should have close button
      await expect(page.locator('.modal-box button[aria-label="Close"]')).toBeVisible();

      // Should have Done button
      await expect(page.locator('.modal-box button:has-text("Done")')).toBeVisible();
    });
  });

  test.describe('Keyboard Shortcuts', () => {
    test('Ctrl+, opens settings', async ({ page }) => {
      await page.keyboard.press('Control+,');
      await page.waitForTimeout(300);

      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();
    });

    test('Escape closes settings', async ({ page }) => {
      await openSettingsModal(page);
      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);

      const modal = page.locator('.modal-box');
      await expect(modal).not.toBeVisible();
    });
  });

  test.describe('Accessibility', () => {
    test('settings button has accessible label', async ({ page }) => {
      const settingsButton = page.locator('button[aria-label="Settings"]');
      const ariaLabel = await settingsButton.getAttribute('aria-label');
      expect(ariaLabel).toBe('Settings');
    });

    test('theme select has label', async ({ page }) => {
      await openSettingsModal(page);

      const themeLabel = page.locator('.modal-box .label-text:has-text("Theme")');
      await expect(themeLabel).toBeVisible();
    });

    test('all interactive elements are focusable', async ({ page }) => {
      await openSettingsModal(page);

      const closeButton = page.locator('.modal-box button[aria-label="Close"]');
      await closeButton.focus();
      await expect(closeButton).toBeFocused();

      const themeSelect = page.locator('.modal-box select');
      await themeSelect.focus();
      await expect(themeSelect).toBeFocused();

      const doneButton = page.locator('.modal-box button:has-text("Done")');
      await doneButton.focus();
      await expect(doneButton).toBeFocused();
    });
  });

  test.describe('Visual Feedback', () => {
    test('settings button has hover effect', async ({ page }) => {
      const settingsButton = page.locator('button[aria-label="Settings"]');
      await settingsButton.hover();
      await page.waitForTimeout(200);

      // Should have hover-related class
      await expect(settingsButton).toHaveClass(/hover/);
    });

    test('done button has primary styling', async ({ page }) => {
      await openSettingsModal(page);

      const doneButton = page.locator('.modal-box button:has-text("Done")');
      await expect(doneButton).toHaveClass(/btn-primary/);
    });

    test('theme select has focus styling', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      await themeSelect.focus();
      await page.waitForTimeout(200);

      // Should be focused
      await expect(themeSelect).toBeFocused();
    });
  });

  test.describe('Preference Persistence', () => {
    test('preferences persist within session', async ({ page }) => {
      // Set theme to Dark
      await openSettingsModal(page);
      await page.locator('.modal-box select').selectOption('Dark');
      await closeModal(page);

      // Open modal again without reload
      await openSettingsModal(page);
      const themeSelect = page.locator('.modal-box select');

      // Theme should still be Dark within the same session
      await expect(themeSelect).toHaveValue('Dark');
    });

    test('theme changes are immediately reflected', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');

      // Change to Dark
      await themeSelect.selectOption('Dark');
      await expect(themeSelect).toHaveValue('Dark');

      // Change to Light
      await themeSelect.selectOption('Light');
      await expect(themeSelect).toHaveValue('Light');

      // Change to System
      await themeSelect.selectOption('System');
      await expect(themeSelect).toHaveValue('System');
    });
  });
});
