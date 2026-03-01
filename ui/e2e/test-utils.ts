/**
 * Shared test utilities for e2e screenshot testing
 */

import { Page } from '@playwright/test';

/**
 * Wait for the application to fully load
 */
export async function waitForAppReady(page: Page): Promise<void> {
  await page.waitForSelector('.navbar', { timeout: 10000 });
  await page.waitForFunction(
    () => {
      const loading = document.querySelector('.loading-spinner');
      return loading === null;
    },
    { timeout: 15000 }
  );
}

/**
 * Navigate to the app and wait for it to be ready
 */
export async function gotoAndWait(page: Page): Promise<void> {
  await page.goto('/');
  await waitForAppReady(page);
}

/**
 * Open the settings modal
 */
export async function openSettingsModal(page: Page): Promise<void> {
  await page.click('button[aria-label="Settings"]');
  await page.waitForSelector('.modal-box');
  await page.waitForTimeout(300);
}

/**
 * Close the current modal
 */
export async function closeModal(page: Page): Promise<void> {
  const closeButton = page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  } else {
    await page.click('.modal-box button:has-text("Done")');
  }
  await page.waitForTimeout(300);
}

/**
 * Set the theme via settings modal
 */
export async function setTheme(page: Page, theme: 'Light' | 'Dark' | 'System'): Promise<void> {
  await openSettingsModal(page);
  await page.selectOption('.modal-box select', theme);
  await page.waitForTimeout(500);
  await closeModal(page);
}

/**
 * Open keyboard help modal via F1
 */
export async function openKeyboardHelp(page: Page): Promise<void> {
  await page.keyboard.press('F1');
  await page.waitForSelector('.modal-box');
  await page.waitForTimeout(300);
}

/**
 * Screenshot configuration for consistent visual regression testing
 */
export const screenshotConfig = {
  animations: 'disabled' as const,
  fullPage: false,
  maxDiffPixels: 100,
};

/**
 * Full page screenshot configuration
 */
export const fullPageScreenshotConfig = {
  ...screenshotConfig,
  fullPage: true as const,
};
