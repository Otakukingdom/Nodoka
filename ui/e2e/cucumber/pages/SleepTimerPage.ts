/**
 * Sleep Timer Page Object Model
 * Encapsulates sleep timer modal interactions
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from '../../pom/BasePage.js';

export class SleepTimerPage extends BasePage {
  readonly sleepTimerButton: Locator;
  readonly modal: Locator;
  readonly modalBox: Locator;
  readonly closeButton: Locator;
  readonly durationOptions: Locator;
  readonly endOfChapterOption: Locator;
  readonly fadeOptions: Locator;
  readonly timerDisplay: Locator;
  readonly timerMode: Locator;
  readonly cancelButton: Locator;

  constructor(page: Page) {
    super(page);
    this.sleepTimerButton = page.locator('button[aria-label="Sleep timer"]');
    this.modal = page.locator('.modal');
    this.modalBox = page.locator('.modal-box:has-text("Sleep")');
    this.closeButton = this.modalBox.locator('button[aria-label="Close"]');
    this.durationOptions = this.modalBox.locator('button:has-text("min")');
    this.endOfChapterOption = this.modalBox.locator('button:has-text("End of chapter")');
    this.fadeOptions = this.modalBox.locator('select').filter({ hasText: 'Fade' });
    this.timerDisplay = page.locator('.sleep-timer-display');
    this.timerMode = page.locator('.sleep-timer-mode');
    this.cancelButton = page.locator('button[aria-label="Cancel timer"]');
  }

  /**
   * Open sleep timer modal
   */
  async open(): Promise<void> {
    await this.sleepTimerButton.click();
    await this.page.waitForSelector('.modal-box:has-text("Sleep")');
    await this.page.waitForTimeout(300);
  }

  /**
   * Open sleep timer via keyboard shortcut
   */
  async openViaShortcut(): Promise<void> {
    await this.page.keyboard.press('Control+t');
    await this.page.waitForSelector('.modal-box:has-text("Sleep")');
    await this.page.waitForTimeout(300);
  }

  /**
   * Close sleep timer modal
   */
  async close(): Promise<void> {
    await this.closeButton.click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Set duration in minutes
   */
  async setDuration(minutes: number): Promise<void> {
    await this.open();
    await this.modalBox.locator(`button:has-text("${minutes} min")`).click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Set end of chapter mode
   */
  async setEndOfChapter(): Promise<void> {
    await this.open();
    await this.endOfChapterOption.click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Set fade duration
   */
  async setFadeDuration(seconds: number): Promise<void> {
    await this.open();
    const fadeSelect = this.modalBox.locator('select').first();
    await fadeSelect.selectOption(seconds.toString());
    await this.page.waitForTimeout(200);
  }

  /**
   * Cancel active timer
   */
  async cancelTimer(): Promise<void> {
    if (await this.cancelButton.isVisible()) {
      await this.cancelButton.click();
      await this.page.waitForTimeout(300);
    }
  }

  /**
   * Check if modal is open
   */
  async isOpen(): Promise<boolean> {
    return await this.modalBox.isVisible();
  }

  /**
   * Check if timer is active
   */
  async isTimerActive(): Promise<boolean> {
    return await this.timerDisplay.isVisible();
  }

  /**
   * Get remaining time display
   */
  async getRemainingTime(): Promise<string> {
    return (await this.timerDisplay.textContent()) ?? '';
  }

  /**
   * Get timer mode text
   */
  async getTimerMode(): Promise<string> {
    return (await this.timerMode.textContent()) ?? '';
  }

  /**
   * Verify modal is visible
   */
  async expectModalVisible(): Promise<void> {
    await expect(this.modalBox).toBeVisible();
  }

  /**
   * Verify timer is active
   */
  async expectTimerActive(): Promise<void> {
    await expect(this.timerDisplay).toBeVisible();
  }

  // ==================== Enhanced Methods for Comprehensive UI Testing ====================

  /**
   * Get available duration options text
   */
  async getDurationOptions(): Promise<string[]> {
    const options = this.modalBox.locator('button:has-text("min")');
    const count = await options.count();
    const texts: string[] = [];
    for (let i = 0; i < count; i++) {
      texts.push((await options.nth(i).textContent()) ?? '');
    }
    return texts;
  }

  /**
   * Check if end of chapter option is available
   */
  async hasEndOfChapterOption(): Promise<boolean> {
    return await this.endOfChapterOption.isVisible();
  }

  /**
   * Get available fade duration options
   */
  async getFadeOptions(): Promise<string[]> {
    const fadeSelect = this.modalBox.locator('select').first();
    const options = await fadeSelect.locator('option').allInnerTexts();
    return options;
  }

  /**
   * Select fade duration by value
   */
  async selectFadeOption(seconds: number): Promise<void> {
    const fadeSelect = this.modalBox.locator('select').first();
    await fadeSelect.selectOption(seconds.toString());
    await this.page.waitForTimeout(200);
  }

  /**
   * Verify modal has title "Sleep Timer"
   */
  async hasTitle(): Promise<boolean> {
    const title = this.modalBox.locator('h3, .modal-title');
    const text = await title.textContent();
    return text?.includes('Sleep') ?? false;
  }

  /**
   * Check if modal has close button
   */
  async hasCloseButton(): Promise<boolean> {
    return await this.closeButton.isVisible();
  }

  /**
   * Close modal via Escape key
   */
  async closeViaEscape(): Promise<void> {
    await this.page.keyboard.press('Escape');
    await this.page.waitForTimeout(300);
  }

  /**
   * Get timer mode (Duration or EndOfChapter)
   */
  async getTimerModeText(): Promise<string> {
    const modeElement = this.modalBox.locator('.timer-mode, .sleep-timer-mode');
    if (await modeElement.isVisible()) {
      return (await modeElement.textContent()) ?? '';
    }
    return '';
  }

  /**
   * Check if cancel timer button is visible
   */
  async hasCancelButton(): Promise<boolean> {
    return await this.cancelButton.isVisible();
  }

  /**
   * Click duration button by minutes value
   */
  async clickDurationButton(minutes: number): Promise<void> {
    const button = this.modalBox.locator(`button:has-text("${minutes} min")`);
    await button.click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Verify all expected duration options are present
   */
  async verifyDurationOptions(expected: number[]): Promise<boolean> {
    for (const minutes of expected) {
      const button = this.modalBox.locator(`button:has-text("${minutes} min")`);
      if (!(await button.isVisible())) {
        return false;
      }
    }
    return true;
  }
}
