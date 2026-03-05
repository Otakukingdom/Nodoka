/**
 * Settings Page Object Model
 * Encapsulates settings modal interactions (theme, preferences)
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from './BasePage';

export class SettingsPage extends BasePage {
  // Locators
  readonly settingsButton: Locator;
  readonly modal: Locator;
  readonly modalBox: Locator;
  readonly closeButton: Locator;
  readonly doneButton: Locator;
  readonly themeSelect: Locator;
  readonly themeLabel: Locator;
  readonly resetButton: Locator;

  constructor(page: Page) {
    super(page);
    this.settingsButton = page.locator('button[aria-label="Settings"]');
    this.modal = page.locator('.modal');
    this.modalBox = page.locator('.modal-box');
    this.closeButton = page.locator('.modal-box button[aria-label="Close"]');
    this.doneButton = page.locator('.modal-box button:has-text("Done")');
    this.themeSelect = page.getByLabel('Select theme');
    this.themeLabel = page.locator('.modal-box .label-text:has-text("Theme")');
    this.resetButton = page.locator('.modal-box button').filter({ hasText: /reset/i });
  }

  /**
   * Open settings modal
   */
  async open(): Promise<void> {
    await this.settingsButton.click();
    await this.page.waitForSelector('.modal-box');
    await this.page.waitForTimeout(300);
  }

  /**
   * Close settings modal via close button
   */
  async close(): Promise<void> {
    if (await this.closeButton.isVisible()) {
      await this.closeButton.click();
    } else {
      await this.doneButton.click();
    }
    await this.page.waitForTimeout(300);
  }

  /**
   * Close settings modal via Done button
   */
  async done(): Promise<void> {
    await this.doneButton.click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Close settings modal via Escape key
   */
  async closeViaEscape(): Promise<void> {
    await this.page.keyboard.press('Escape');
    await this.page.waitForTimeout(300);
  }

  /**
   * Check if modal is open
   */
  async isOpen(): Promise<boolean> {
    return await this.modalBox.isVisible();
  }

  /**
   * Set theme
   */
  async setTheme(theme: 'Light' | 'Dark' | 'System'): Promise<void> {
    await this.themeSelect.selectOption(theme);
    await this.page.waitForTimeout(200);
  }

  /**
   * Get current theme
   */
  async getTheme(): Promise<string> {
    return await this.themeSelect.inputValue();
  }

  /**
   * Click reset button
   */
  async reset(): Promise<void> {
    if (await this.resetButton.count() > 0) {
      await this.resetButton.first().click();
      await this.page.waitForTimeout(300);
    }
  }

  /**
   * Open settings via keyboard shortcut
   */
  async openViaShortcut(): Promise<void> {
    await this.page.keyboard.press('Control+,');
    await this.page.waitForSelector('.modal-box');
    await this.page.waitForTimeout(300);
  }

  /**
   * Get all form elements in settings
   */
  async getFormElements(): Promise<Locator[]> {
    const inputs = await this.modalBox.locator('input').all();
    const selects = await this.modalBox.locator('select').all();
    return [...inputs, ...selects];
  }

  /**
   * Focus on close button
   */
  async focusCloseButton(): Promise<void> {
    await this.closeButton.focus();
  }

  /**
   * Focus on theme select
   */
  async focusThemeSelect(): Promise<void> {
    await this.themeSelect.focus();
  }

  /**
   * Focus on done button
   */
  async focusDoneButton(): Promise<void> {
    await this.doneButton.focus();
  }

  /**
   * Tab through elements
   */
  async tabThrough(times: number): Promise<void> {
    for (let i = 0; i < times; i++) {
      await this.page.keyboard.press('Tab');
      await this.page.waitForTimeout(100);
    }
  }

  /**
   * Verify modal is visible
   */
  async expectModalVisible(): Promise<void> {
    await expect(this.modalBox).toBeVisible();
  }

  /**
   * Verify modal is hidden
   */
  async expectModalHidden(): Promise<void> {
    await expect(this.modalBox).not.toBeVisible();
  }

  /**
   * Verify theme is set
   */
  async expectTheme(expected: string): Promise<void> {
    await expect(this.themeSelect).toHaveValue(expected);
  }

  /**
   * Look for preference input by label
   */
  async getPreferenceInput(label: string): Promise<Locator | null> {
    const input = this.modalBox.locator(`.label-text:has-text("${label}") + input, .label-text:has-text("${label}") ~ input`);
    const count = await input.count();
    return count > 0 ? input.first() : null;
  }

  /**
   * Look for preference select by label
   */
  async getPreferenceSelect(label: string): Promise<Locator | null> {
    const select = this.modalBox.locator(`.label-text:has-text("${label}") + select, .label-text:has-text("${label}") ~ select`);
    const count = await select.count();
    return count > 0 ? select.first() : null;
  }

  /**
   * Add directory via file picker button click
   */
  async clickAddDirectory(): Promise<void> {
    const addButton = this.modalBox.locator('button:has-text("Add Directory")');
    await addButton.click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Remove directory by index
   */
  async removeDirectory(index: number): Promise<void> {
    const removeButton = this.modalBox.locator('.directory-item button[aria-label="Remove"]').nth(index);
    if (await removeButton.isVisible()) {
      await removeButton.click();
      await this.page.waitForTimeout(300);
    }
  }

  /**
   * Rescan directory by index
   */
  async rescanDirectory(index: number): Promise<void> {
    const rescanButton = this.modalBox.locator('.directory-item button[aria-label="Rescan"]').nth(index);
    if (await rescanButton.isVisible()) {
      await rescanButton.click();
      await this.page.waitForTimeout(500);
    }
  }

  /**
   * Set skip forward duration
   */
  async setSkipForwardDuration(seconds: number): Promise<void> {
    const select = await this.getPreferenceSelect('Skip forward');
    if (select) {
      await select.selectOption(seconds.toString());
      await this.page.waitForTimeout(200);
    }
  }

  /**
   * Set skip backward duration
   */
  async setSkipBackwardDuration(seconds: number): Promise<void> {
    const select = await this.getPreferenceSelect('Skip backward');
    if (select) {
      await select.selectOption(seconds.toString());
      await this.page.waitForTimeout(200);
    }
  }

  /**
   * Check if theme option is available
   */
  async isThemeOptionAvailable(theme: string): Promise<boolean> {
    const options = await this.themeSelect.locator('option').allTextContents();
    return options.some(opt => opt.includes(theme));
  }

  /**
   * Get directory count
   */
  async getDirectoryCount(): Promise<number> {
    const directories = this.modalBox.locator('.directory-item');
    return await directories.count();
  }

  /**
   * Get all directories
   */
  getDirectories(): Locator {
    return this.modalBox.locator('.directory-item');
  }
}
