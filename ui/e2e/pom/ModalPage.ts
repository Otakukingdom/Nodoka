/**
 * Modal Page Object Model
 * Encapsulates modal dialog interactions for centralized handling
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from './BasePage';

export class ModalPage extends BasePage {
  readonly modal: Locator;
  readonly modalBox: Locator;
  readonly closeButton: Locator;
  readonly doneButton: Locator;
  readonly cancelButton: Locator;
  readonly title: Locator;
  readonly content: Locator;

  constructor(page: Page, modalSelector: string = '.modal') {
    super(page);
    this.modal = page.locator(modalSelector);
    this.modalBox = page.locator('.modal-box');
    this.closeButton = this.modalBox.locator('button[aria-label="Close"], button:has-text("Close")');
    this.doneButton = this.modalBox.locator('button:has-text("Done"), button:has-text("Save")');
    this.cancelButton = this.modalBox.locator('button:has-text("Cancel"), button:has-text("Cancel")');
    this.title = this.modalBox.locator('.modal-title, h2, h3').first();
    this.content = this.modalBox.locator('.modal-content, .modal-body');
  }

  /**
   * Wait for modal to be visible
   */
  async expectVisible(): Promise<void> {
    await this.modalBox.waitFor({ state: 'visible' });
  }

  /**
   * Wait for modal to be hidden
   */
  async expectHidden(): Promise<void> {
    await this.modalBox.waitFor({ state: 'hidden' });
  }

  /**
   * Check if modal is currently visible
   */
  async isVisible(): Promise<boolean> {
    return await this.modalBox.isVisible();
  }

  /**
   * Close modal via close button
   */
  async close(): Promise<void> {
    if (await this.closeButton.isVisible()) {
      await this.closeButton.click();
    } else if (await this.cancelButton.isVisible()) {
      await this.cancelButton.click();
    } else {
      await this.page.keyboard.press('Escape');
    }
    await this.page.waitForTimeout(300);
  }

  /**
   * Close modal via done/save button
   */
  async done(): Promise<void> {
    if (await this.doneButton.isVisible()) {
      await this.doneButton.click();
      await this.page.waitForTimeout(300);
    }
  }

  /**
   * Close modal via Escape key
   */
  async closeViaEscape(): Promise<void> {
    await this.page.keyboard.press('Escape');
    await this.page.waitForTimeout(300);
  }

  /**
   * Get modal title text
   */
  async getTitle(): Promise<string> {
    return (await this.title.textContent()) ?? '';
  }

  /**
   * Get modal content text
   */
  async getContent(): Promise<string> {
    return (await this.content.textContent()) ?? '';
  }

  /**
   * Check if a button with specific text is visible
   */
  async hasButton(text: string): Promise<boolean> {
    const button = this.modalBox.locator(`button:has-text("${text}")`);
    return await button.isVisible();
  }

  /**
   * Click a button by text
   */
  async clickButton(text: string): Promise<void> {
    const button = this.modalBox.locator(`button:has-text("${text}")`).first();
    await button.click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Check if a link with specific text is visible
   */
  async hasLink(text: string): Promise<boolean> {
    const link = this.modalBox.locator(`a:has-text("${text}")`);
    return await link.isVisible();
  }

  /**
   * Click a link by text
   */
  async clickLink(text: string): Promise<void> {
    const link = this.modalBox.locator(`a:has-text("${text}")`).first();
    await link.click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Get all focusable elements in modal
   */
  async getFocusableElements(): Promise<Locator[]> {
    const focusable = this.modalBox.locator('button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])');
    return await focusable.all();
  }

  /**
   * Tab through elements in modal
   */
  async tabThrough(times: number): Promise<void> {
    for (let i = 0; i < times; i++) {
      await this.page.keyboard.press('Tab');
      await this.page.waitForTimeout(100);
    }
  }

  /**
   * Verify focus is trapped within modal
   */
  async verifyFocusTrap(): Promise<boolean> {
    const initialFocus = this.page.locator(':focus');
    const initialInModal = await this.modalBox.evaluate((el, focused) => {
      return el.contains(focused);
    }, await initialFocus.elementHandle()!);

    if (!initialInModal) return false;

    // Tab through multiple times and verify focus stays in modal
    for (let i = 0; i < 20; i++) {
      await this.page.keyboard.press('Tab');
      await this.page.waitForTimeout(50);

      const currentFocus = this.page.locator(':focus');
      if (await currentFocus.isVisible()) {
        const inModal = await this.modalBox.evaluate((el, focused) => {
          return el.contains(focused);
        }, await currentFocus.elementHandle()!);

        if (!inModal) return false;
      }
    }

    return true;
  }

  /**
   * Focus on first focusable element
   */
  async focusFirst(): Promise<void> {
    const focusable = await this.getFocusableElements();
    if (focusable.length > 0) {
      await focusable[0].focus();
    }
  }

  /**
   * Focus on last focusable element
   */
  async focusLast(): Promise<void> {
    const focusable = await this.getFocusableElements();
    if (focusable.length > 0) {
      await focusable[focusable.length - 1].focus();
    }
  }

  /**
   * Get input by label
   */
  getInputByLabel(label: string): Locator {
    return this.modalBox.locator(`.label-text:has-text("${label}") + input, .label-text:has-text("${label}") ~ input, label:has-text("${label}") + input`).first();
  }

  /**
   * Get select by label
   */
  getSelectByLabel(label: string): Locator {
    return this.modalBox.locator(`.label-text:has-text("${label}") + select, .label-text:has-text("${label}") ~ select, label:has-text("${label}") + select`).first();
  }

  /**
   * Fill input by label
   */
  async fillInputByLabel(label: string, value: string): Promise<void> {
    const input = this.getInputByLabel(label);
    await input.fill(value);
    await this.page.waitForTimeout(200);
  }

  /**
   * Select option by label
   */
  async selectOptionByLabel(label: string, value: string): Promise<void> {
    const select = this.getSelectByLabel(label);
    await select.selectOption(value);
    await this.page.waitForTimeout(200);
  }

  /**
   * Check if modal has specific text
   */
  async hasText(text: string): Promise<boolean> {
    const textLocator = this.modalBox.locator(`text=${text}`);
    return await textLocator.isVisible();
  }

  /**
   * Verify modal is visible
   */
  async verifyVisible(): Promise<void> {
    await expect(this.modalBox).toBeVisible();
  }

  /**
   * Verify modal is hidden
   */
  async verifyHidden(): Promise<void> {
    await expect(this.modalBox).not.toBeVisible();
  }

  /**
   * Verify modal title
   */
  async verifyTitle(expected: string): Promise<void> {
    await expect(this.title).toContainText(expected);
  }
}
