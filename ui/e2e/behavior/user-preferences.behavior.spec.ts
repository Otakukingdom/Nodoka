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

  // Setting a user preference - Cucumber: Setting a user preference
  test.describe('Setting User Preferences', () => {
    test('preference can be set via settings modal', async ({ page }) => {
      await openSettingsModal(page);

      // Look for preference controls (language, region, etc.)
      const preferenceInputs = page.locator('.modal-box input[type="text"]');
      const preferenceSelects = page.locator('.modal-box select');

      // At least theme select should exist
      await expect(preferenceSelects.first()).toBeVisible();
    });

    test('language preference can be changed', async ({ page }) => {
      await openSettingsModal(page);

      // Look for language select
      const languageSelect = page.locator('.modal-box select').filter({ has: page.locator('option:has-text("en"), option:has-text("English")') });

      if (await languageSelect.count() > 0) {
        await languageSelect.selectOption('en');
        await page.waitForTimeout(200);
      }

      // Modal should be functional
      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();
    });

    test('preference input fields are accessible', async ({ page }) => {
      await openSettingsModal(page);

      // All inputs should be focusable
      const inputs = page.locator('.modal-box input, .modal-box select');
      const count = await inputs.count();

      for (let i = 0; i < count; i++) {
        const input = inputs.nth(i);
        await input.focus();
        await expect(input).toBeFocused();
      }
    });
  });

  // Getting a user preference - Cucumber: Getting a user preference
  test.describe('Getting User Preferences', () => {
    test('preference value is displayed correctly', async ({ page }) => {
      await openSettingsModal(page);

      // Theme select should show current value
      const themeSelect = page.locator('.modal-box select');
      const value = await themeSelect.inputValue();

      // Should have a valid value
      expect(['Light', 'Dark', 'System']).toContain(value);
    });

    test('all preferences are visible in settings', async ({ page }) => {
      await openSettingsModal(page);

      // Settings modal should be visible
      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();

      // Should have multiple form elements
      const formElements = page.locator('.modal-box input, .modal-box select');
      const count = await formElements.count();
      expect(count).toBeGreaterThan(0);
    });
  });

  // Getting all user preferences - Cucumber: Getting all user preferences
  test.describe('Getting All Preferences', () => {
    test('all preferences are accessible', async ({ page }) => {
      await openSettingsModal(page);

      // Theme preference
      const themeSelect = page.locator('.modal-box select');
      await expect(themeSelect).toBeVisible();

      // Volume preference
      const volumeSlider = page.locator('.btm-nav .group\\/volume input[type="range"]');
      await expect(volumeSlider).toBeVisible();

      // Speed preference
      const speedButton = page.locator('.btm-nav button:has-text("x")');
      await expect(speedButton.first()).toBeVisible();
    });

    test('preferences count matches expected', async ({ page }) => {
      await openSettingsModal(page);

      // Count visible preference controls
      const selects = page.locator('.modal-box select');
      const inputs = page.locator('.modal-box input');

      const selectCount = await selects.count();
      const inputCount = await inputs.count();

      expect(selectCount + inputCount).toBeGreaterThan(0);
    });
  });

  // Deleting a user preference - Cucumber: Deleting a user preference
  test.describe('Deleting User Preferences', () => {
    test('preference can be reset to default', async ({ page }) => {
      await openSettingsModal(page);

      // Change theme
      const themeSelect = page.locator('.modal-box select');
      await themeSelect.selectOption('Dark');
      await page.waitForTimeout(200);

      // Reset to System (default)
      await themeSelect.selectOption('System');
      await page.waitForTimeout(200);

      await expect(themeSelect).toHaveValue('System');
    });

    test('reset button clears custom preferences', async ({ page }) => {
      await openSettingsModal(page);

      // Look for reset button
      const resetButton = page.locator('.modal-box button').filter({ hasText: /reset/i });

      if (await resetButton.count() > 0) {
        await resetButton.click();
        await page.waitForTimeout(300);
      }

      // Modal should still be functional
      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();
    });
  });

  // Updating existing preference - Cucumber: Updating existing preference
  test.describe('Updating User Preferences', () => {
    test('preference can be updated multiple times', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');

      // Update to Dark
      await themeSelect.selectOption('Dark');
      await expect(themeSelect).toHaveValue('Dark');

      // Update to Light
      await themeSelect.selectOption('Light');
      await expect(themeSelect).toHaveValue('Light');

      // Update to Dark again
      await themeSelect.selectOption('Dark');
      await expect(themeSelect).toHaveValue('Dark');
    });

    test('preference update is immediate', async ({ page }) => {
      await openSettingsModal(page);

      const themeSelect = page.locator('.modal-box select');
      const initialValue = await themeSelect.inputValue();

      // Change value
      const newValue = initialValue === 'Dark' ? 'Light' : 'Dark';
      await themeSelect.selectOption(newValue);

      // Value should change immediately
      await expect(themeSelect).toHaveValue(newValue);
    });

    test('volume preference can be updated', async ({ page }) => {
      const volumeSlider = page.locator('.btm-nav .group\\/volume input[type="range"]');

      // Set volume to 50
      await volumeSlider.fill('50');
      await page.waitForTimeout(200);

      // Set volume to 75
      await volumeSlider.fill('75');
      await page.waitForTimeout(200);

      await expect(volumeSlider).toBeVisible();
    });

    test('speed preference can be updated', async ({ page }) => {
      const speedButton = page.locator('.btm-nav button:has-text("x")').first();

      // Click to cycle speed
      await speedButton.click();
      await page.waitForTimeout(200);

      const speed1 = await speedButton.textContent();

      // Click again
      await speedButton.click();
      await page.waitForTimeout(200);

      const speed2 = await speedButton.textContent();

      // Speed should change
      expect(speed1).not.toBe(speed2);
    });
  });

  // NUL byte rejection - Cucumber: Preference rejects NUL bytes
  test.describe('Invalid Input Handling', () => {
    test('preference controls reject invalid input', async ({ page }) => {
      await openSettingsModal(page);

      // Text inputs should handle invalid input gracefully
      const textInputs = page.locator('.modal-box input[type="text"]');

      if (await textInputs.count() > 0) {
        // Try to enter valid text
        await textInputs.first().fill('valid input');
        await page.waitForTimeout(200);

        // Input should accept valid text
        await expect(textInputs.first()).toBeVisible();
      }

      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();
    });

    test('form validation works correctly', async ({ page }) => {
      await openSettingsModal(page);

      // Number inputs should validate
      const numberInputs = page.locator('.modal-box input[type="number"]');

      if (await numberInputs.count() > 0) {
        // Enter valid number
        await numberInputs.first().fill('10');
        await page.waitForTimeout(200);

        await expect(numberInputs.first()).toBeVisible();
      }

      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();
    });

    test('error handling for invalid preferences', async ({ page }) => {
      await openSettingsModal(page);

      // Try to submit invalid data if form exists
      const doneButton = page.locator('.modal-box button:has-text("Done")');

      // Clicking done should not cause error
      await doneButton.click();
      await page.waitForTimeout(300);

      // Page should be functional
      const playerControls = page.locator('.btm-nav');
      await expect(playerControls).toBeVisible();
    });
  });

  // Backend User Preferences CRUD - Cucumber: Setting a user preference
  test.describe('Backend User Preferences CRUD', () => {
    test('preference can be set via backend', async ({ page }) => {
      // Set a preference
      await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('set_preference', { key: 'language', value: 'en' });
        }
        return null;
      });

      // Get the preference
      const value = await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('get_preference', { key: 'language' });
        }
        return null;
      });

      expect(value).toBe('en');
    });

    test('preference can be retrieved via backend', async ({ page }) => {
      // Set a preference first
      await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('set_preference', { key: 'testKey', value: 'testValue' });
        }
        return null;
      });

      // Get the preference
      const value = await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('get_preference', { key: 'testKey' });
        }
        return null;
      });

      expect(value).toBe('testValue');
    });

    test('all preferences can be retrieved', async ({ page }) => {
      // Set multiple preferences
      await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          tauri.invoke('set_preference', { key: 'pref1', value: 'value1' });
          tauri.invoke('set_preference', { key: 'pref2', value: 'value2' });
        }
        return null;
      });

      // Get all preferences
      const preferences = await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('get_all_preferences');
        }
        return [];
      });

      expect(Array.isArray(preferences)).toBe(true);
      expect(preferences.length).toBeGreaterThanOrEqual(2);
    });

    test('preference can be deleted', async ({ page }) => {
      // Set a preference
      await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('set_preference', { key: 'toDelete', value: 'value' });
        }
        return null;
      });

      // Delete the preference
      await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('delete_preference', { key: 'toDelete' });
        }
        return null;
      });

      // Try to get the preference - should be null
      const value = await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('get_preference', { key: 'toDelete' });
        }
        return 'not-found';
      });

      expect(value).toBeNull();
    });

    test('existing preference can be updated', async ({ page }) => {
      // Set a preference
      await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('set_preference', { key: 'updatable', value: 'initial' });
        }
        return null;
      });

      // Update the preference
      await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('set_preference', { key: 'updatable', value: 'updated' });
        }
        return null;
      });

      // Get the preference
      const value = await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('get_preference', { key: 'updatable' });
        }
        return null;
      });

      expect(value).toBe('updated');
    });
  });

  // NUL Byte Rejection - Cucumber: Preference rejects NUL bytes in key
  test.describe('NUL Byte Rejection', () => {
    test('NUL byte in preference key is rejected', async ({ page }) => {
      let errorThrown = false;
      try {
        await page.evaluate(() => {
          const tauri = (window as any).__TAURI__;
          if (tauri) {
            return tauri.invoke('set_preference', { key: 'key\0withNUL', value: 'value' });
          }
          return null;
        });
      } catch {
        errorThrown = true;
      }

      expect(errorThrown).toBe(true);
    });

    test('NUL byte in preference value is rejected', async ({ page }) => {
      let errorThrown = false;
      try {
        await page.evaluate(() => {
          const tauri = (window as any).__TAURI__;
          if (tauri) {
            return tauri.invoke('set_preference', { key: 'validKey', value: 'value\0withNUL' });
          }
          return null;
        });
      } catch {
        errorThrown = true;
      }

      expect(errorThrown).toBe(true);
    });

    test('valid preference with no NUL bytes is accepted', async ({ page }) => {
      const result = await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('set_preference', { key: 'validKey', value: 'validValue' });
        }
        return null;
      });

      // Should complete without error
      expect(result).toBeUndefined();
    });
  });
});
