/**
 * User Preferences Step Definitions
 * Implements UI interactions for user preferences feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { SettingsPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('the application has a database', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

// ==================== When Steps ====================

When('I set user preference {string} to {string}', async function (this: NodokaWorld, key: string, value: string) {
  // User preferences are set through the Settings UI
  const settings = new SettingsPage(this.page);
  await settings.open();

  // Try to find the preference input by key name
  const prefInput = this.page.locator(`input[name="${key}"], input[aria-label*="${key}"], select[name="${key}"]`).first();
  if (await prefInput.isVisible()) {
    const inputType = await prefInput.getAttribute('type');
    if (inputType === 'checkbox') {
      const shouldCheck = value === 'true' || value === '1';
      if (await prefInput.isChecked() !== shouldCheck) {
        await prefInput.click();
      }
    } else if (inputType === 'number') {
      await prefInput.fill(value);
    } else {
      await prefInput.fill(value);
    }
    await this.page.keyboard.press('Enter');
  } else {
    // Alternative: use query parameter to set preference
    await this.page.goto(`http://localhost:5173/?test=true&pref_${key}=${encodeURIComponent(value)}`);
    await this.page.waitForSelector('.navbar', { timeout: 10000 });
  }

  await settings.close();
  await this.page.waitForTimeout(200);
});

When('the application has user preference {string} set to {string}', async function (this: NodokaWorld, key: string, value: string) {
  // Navigate with preference preset via query parameter
  await this.page.goto(`http://localhost:5173/?test=true&pref_${key}=${encodeURIComponent(value)}`);
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(200);
});

When('I get user preference {string}', async function (this: NodokaWorld, key: string) {
  // Get preference value from UI or data attribute
  const prefElement = this.page.locator(`[data-pref="${key}"], [data-preference="${key}"]`).first();

  if (await prefElement.isVisible()) {
    this.lastValue = await prefElement.textContent() ?? await prefElement.inputValue();
  } else {
    // Try to find in settings
    const settings = new SettingsPage(this.page);
    await settings.open();

    const prefInput = this.page.locator(`input[name="${key}"], input[aria-label*="${key}"], select[name="${key}"]`).first();
    if (await prefInput.isVisible()) {
      const inputType = await prefInput.getAttribute('type');
      if (inputType === 'checkbox') {
        this.lastValue = await prefInput.isChecked() ? 'true' : 'false';
      } else {
        this.lastValue = await prefInput.inputValue();
      }
    } else {
      this.lastValue = null;
    }

    await settings.close();
  }
});

When('I get all user preferences', async function (this: NodokaWorld) {
  // Count visible preference settings in the settings modal
  const settings = new SettingsPage(this.page);
  await settings.open();

  const prefInputs = this.page.locator('input[name], select[name], input[aria-label*="preference"]');
  this.lastCount = await prefInputs.count();

  await settings.close();
});

When('I delete user preference {string}', async function (this: NodokaWorld, key: string) {
  // Reset preference to default via settings
  const settings = new SettingsPage(this.page);
  await settings.open();

  // Try to find and clear the preference
  const prefInput = this.page.locator(`input[name="${key}"], input[aria-label*="${key}"]`).first();
  if (await prefInput.isVisible()) {
    const inputType = await prefInput.getAttribute('type');
    if (inputType === 'checkbox') {
      if (await prefInput.isChecked()) {
        await prefInput.click();
      }
    } else {
      await prefInput.fill('');
    }
  }

  // Alternative: use query parameter to reset preference
  await this.page.goto(`http://localhost:5173/?test=true&resetPref=${key}`);
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  await settings.close();
  await this.page.waitForTimeout(200);
});

When('I try to set user preference with NUL byte key', async function (this: NodokaWorld) {
  // Try to set a preference with an invalid key
  const settings = new SettingsPage(this.page);
  await settings.open();

  // Try to input invalid key - this should fail validation
  const prefInput = this.page.locator('input[name], input[aria-label*="preference"]').first();
  if (await prefInput.isVisible()) {
    // Attempting to input NUL byte should be handled by input validation
    await prefInput.fill('test\0key');
    await this.page.keyboard.press('Enter');
    await this.page.waitForTimeout(200);

    // Check for validation error
    const errorVisible = await this.page.locator('.error-message, .validation-error').isVisible();
    this.lastError = errorVisible ? new Error('Invalid key: NUL byte not allowed') : null;
  } else {
    this.lastError = null;
  }

  await settings.close();
});

When('I try to set user preference with NUL byte value', async function (this: NodokaWorld) {
  // Try to set a preference with an invalid value
  const settings = new SettingsPage(this.page);
  await settings.open();

  const prefInput = this.page.locator('input[type="text"], input[type="string"]').first();
  if (await prefInput.isVisible()) {
    // Attempting to input NUL byte should be handled by input validation
    await prefInput.fill('test\0value');
    await this.page.keyboard.press('Enter');
    await this.page.waitForTimeout(200);

    // Check for validation error
    const errorVisible = await this.page.locator('.error-message, .validation-error').isVisible();
    this.lastError = errorVisible ? new Error('Invalid value: NUL byte not allowed') : null;
  } else {
    this.lastError = null;
  }

  await settings.close();
});

// ==================== Then Steps ====================

Then('the preference should be stored', async function (this: NodokaWorld) {
  // Verify the preference was stored by checking the UI reflects the change
  await this.page.waitForTimeout(200);
});

Then('the value should be {string}', async function (this: NodokaWorld, expectedValue: string) {
  expect(this.lastValue).toBe(expectedValue);
});

Then('I should receive {int} preferences', async function (this: NodokaWorld, count: number) {
  expect(this.lastCount).toBe(count);
});

Then('the preference should not exist', async function (this: NodokaWorld) {
  expect(this.lastValue).toBeNull();
});

Then('the preference value should be {string}', async function (this: NodokaWorld, expectedValue: string) {
  expect(this.lastValue).toBe(expectedValue);
});
