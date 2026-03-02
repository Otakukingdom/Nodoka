/**
 * Base Page Object Model
 * Provides common utilities for navigation, waiting, and error handling
 */

import { Page, Locator } from '@playwright/test';

export abstract class BasePage {
  protected readonly page: Page;

  constructor(page: Page) {
    this.page = page;
  }

  /**
   * Navigate to the app and wait for it to be ready
   */
  async goto(): Promise<void> {
    await this.page.goto('/');
    await this.waitForReady();
  }

  /**
   * Wait for the application to be fully loaded
   */
  async waitForReady(): Promise<void> {
    await this.page.waitForSelector('.navbar', { timeout: 10000 });
    await this.page.waitForFunction(
      () => {
        const loading = document.querySelector('.loading-spinner');
        return loading === null;
      },
      { timeout: 15000 }
    );
  }

  /**
   * Wait for a locator to be visible
   */
  async waitForVisible(locator: Locator, timeout = 5000): Promise<void> {
    await locator.waitFor({ state: 'visible', timeout });
  }

  /**
   * Wait for a locator to be hidden
   */
  async waitForHidden(locator: Locator, timeout = 5000): Promise<void> {
    await locator.waitFor({ state: 'hidden', timeout });
  }

  /**
   * Press a keyboard shortcut
   */
  async pressShortcut(shortcut: string): Promise<void> {
    await this.page.keyboard.press(shortcut);
    await this.page.waitForTimeout(200);
  }

  /**
   * Wait for a specific timeout
   */
  async wait(ms: number): Promise<void> {
    await this.page.waitForTimeout(ms);
  }

  /**
   * Take a screenshot for debugging
   */
  async screenshot(path: string): Promise<Buffer> {
    return await this.page.screenshot({ path, fullPage: false });
  }

  /**
   * Check if element is visible
   */
  async isVisible(locator: Locator): Promise<boolean> {
    try {
      return await locator.isVisible();
    } catch {
      return false;
    }
  }

  /**
   * Click and wait for response
   */
  async clickAndWait(locator: Locator, waitMs = 300): Promise<void> {
    await locator.click();
    await this.page.waitForTimeout(waitMs);
  }
}
