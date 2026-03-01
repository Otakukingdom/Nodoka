/**
 * E2E screenshot tests for settings modal
 */

import { test, expect } from './fixtures';
import { screenshotConfig, gotoAndWait, openSettingsModal, setTheme } from './test-utils';

test.describe('Settings Modal Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('settings modal screenshot', async ({ page }) => {
    await openSettingsModal(page);
    const modal = page.locator('.modal-box');
    await expect(modal).toHaveScreenshot('settings-modal.png', screenshotConfig);
  });

  test('settings modal dark theme screenshot', async ({ page }) => {
    await setTheme(page, 'Dark');
    await openSettingsModal(page);
    const modal = page.locator('.modal-box');
    await expect(modal).toHaveScreenshot('settings-modal-dark.png', screenshotConfig);
  });

  test('settings button hover state screenshot', async ({ page }) => {
    const settingsButton = page.locator('button[aria-label="Settings"]');
    await settingsButton.hover();
    await page.waitForTimeout(200);
    await expect(settingsButton).toHaveScreenshot('settings-button-hover.png', screenshotConfig);
  });

  test('settings button focus ring screenshot', async ({ page }) => {
    const settingsButton = page.locator('button[aria-label="Settings"]');
    await settingsButton.focus();
    await page.waitForTimeout(200);
    await expect(settingsButton).toHaveScreenshot('settings-button-focus.png', screenshotConfig);
  });

  test('modal close button focus ring screenshot', async ({ page }) => {
    await openSettingsModal(page);
    const closeButton = page.locator('.modal-box button[aria-label="Close"]');
    await closeButton.focus();
    await page.waitForTimeout(200);
    await expect(closeButton).toHaveScreenshot('modal-close-focus.png', screenshotConfig);
  });
});

test.describe('Settings Modal Functionality Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('settings modal opens and closes', async ({ page }) => {
    await openSettingsModal(page);
    await expect(page.locator('.modal-box')).toBeVisible();

    await page.click('.modal-box button:has-text("Done")');
    await page.waitForTimeout(300);
    await expect(page.locator('.modal-box')).not.toBeVisible();
  });

  test('escape key closes modal', async ({ page }) => {
    await openSettingsModal(page);
    await expect(page.locator('.modal-box')).toBeVisible();

    await page.keyboard.press('Escape');
    await page.waitForTimeout(300);
    await expect(page.locator('.modal-box')).not.toBeVisible();
  });

  test('theme toggle changes theme', async ({ page }) => {
    await openSettingsModal(page);

    const html = page.locator('html');
    const initialTheme = await html.getAttribute('data-theme');

    await page.selectOption('.modal-box select', 'Dark');
    await page.waitForTimeout(300);

    const newTheme = await html.getAttribute('data-theme');
    expect(newTheme).not.toBe(initialTheme);
  });

  test('settings modal has close button in header', async ({ page }) => {
    await openSettingsModal(page);
    const closeButton = page.locator('.modal-box button[aria-label="Close"]');
    await expect(closeButton).toBeVisible();
  });

  test('settings button has tooltip', async ({ page }) => {
    const settingsButton = page.locator('button[aria-label="Settings"]');
    await expect(settingsButton).toHaveAttribute('title', 'Settings (Ctrl+,)');
  });
});

test.describe('System Theme Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('system theme option is available', async ({ page }) => {
    await openSettingsModal(page);
    const systemOption = page.locator('.modal-box select option[value="System"]');
    await expect(systemOption).toHaveCount(1);
  });

  test('system theme can be selected', async ({ page }) => {
    await openSettingsModal(page);
    await page.selectOption('.modal-box select', 'System');
    await page.waitForTimeout(300);
    await page.click('.modal-box button:has-text("Done")');
    await page.waitForTimeout(300);

    const html = page.locator('html');
    const theme = await html.getAttribute('data-theme');
    expect(['light', 'dark']).toContain(theme);
  });
});
