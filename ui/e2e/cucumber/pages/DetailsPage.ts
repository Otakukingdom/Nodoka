/**
 * Details Page Object Model
 * Encapsulates details panel interactions (bookmarks, audiobook info, files)
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from '../../pom/BasePage.js';

export class DetailsPage extends BasePage {
  readonly detailsPanel: Locator;
  readonly audiobookTitle: Locator;
  readonly audiobookAuthor: Locator;
  readonly bookmarksSection: Locator;
  readonly bookmarkList: Locator;
  readonly bookmarkItems: Locator;
  readonly noBookmarksMessage: Locator;
  readonly coverArt: Locator;
  readonly coverPlaceholder: Locator;
  readonly progressDisplay: Locator;
  readonly completionBadge: Locator;
  readonly filesList: Locator;
  readonly fileItems: Locator;
  readonly remainingTime: Locator;
  readonly totalDuration: Locator;

  constructor(page: Page) {
    super(page);
    this.detailsPanel = page.locator('.details-panel');
    this.audiobookTitle = page.locator('.details-panel .card-title');
    this.audiobookAuthor = page.locator('.details-panel .author, .details-panel .author-value');
    this.bookmarksSection = page.locator('.details-panel .bookmarks-section');
    this.bookmarkList = page.locator('.details-panel .bookmark-list');
    this.bookmarkItems = page.locator('.details-panel .bookmark-item');
    this.noBookmarksMessage = page.locator('text=No bookmarks');
    this.coverArt = page.locator('.details-panel .cover-art img');
    this.coverPlaceholder = page.locator('.details-panel .cover-art .placeholder');
    this.progressDisplay = page.locator('.details-panel .progress-text, .details-panel .completeness');
    this.completionBadge = page.locator('.details-panel .completion-badge');
    this.filesList = page.locator('.details-panel .files-list, #files-list');
    this.fileItems = page.locator('.details-panel .file-item, #files-list .card');
    this.remainingTime = page.locator('.details-panel .remaining-time, .details-panel .time-remaining');
    this.totalDuration = page.locator('.details-panel .total-duration, .details-panel .duration');
  }

  /**
   * Get bookmark count
   */
  async getBookmarkCount(): Promise<number> {
    return await this.bookmarkItems.count();
  }

  /**
   * Create a bookmark via keyboard shortcut
   */
  async createBookmark(label?: string): Promise<void> {
    await this.page.keyboard.press('Control+b');
    await this.page.waitForTimeout(300);

    if (label) {
      const input = this.page.locator('.bookmark-input input');
      if (await input.isVisible()) {
        await input.fill(label);
        await this.page.keyboard.press('Enter');
      }
    }
  }

  /**
   * Delete first bookmark
   */
  async deleteFirstBookmark(): Promise<void> {
    const deleteButton = this.bookmarkItems.first().locator('button[aria-label="Delete"]');
    if (await deleteButton.isVisible()) {
      await deleteButton.click();
      await this.page.waitForTimeout(300);
    }
  }

  /**
   * Delete bookmark by index
   */
  async deleteBookmark(index: number): Promise<void> {
    const deleteButton = this.bookmarkItems.nth(index).locator('button[aria-label="Delete"]');
    await deleteButton.click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Click bookmark to seek to position
   */
  async clickBookmark(index: number): Promise<void> {
    await this.bookmarkItems.nth(index).click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Get bookmark label
   */
  async getBookmarkLabel(index: number): Promise<string> {
    const label = this.bookmarkItems.nth(index).locator('.bookmark-label');
    return (await label.textContent()) ?? '';
  }

  /**
   * Get bookmark time
   */
  async getBookmarkTime(index: number): Promise<string> {
    const time = this.bookmarkItems.nth(index).locator('.bookmark-time');
    return (await time.textContent()) ?? '';
  }

  /**
   * Check if details panel is visible
   */
  async isVisible(): Promise<boolean> {
    return await this.detailsPanel.isVisible();
  }

  /**
   * Get audiobook title text
   */
  async getTitle(): Promise<string> {
    return (await this.audiobookTitle.textContent()) ?? '';
  }

  /**
   * Get audiobook author text
   */
  async getAuthor(): Promise<string> {
    const authorElement = this.page.locator('.details-panel .author-value, .details-panel .author-text, .details-panel .author');
    if (await authorElement.isVisible()) {
      return (await authorElement.textContent()) ?? '';
    }
    return '';
  }

  /**
   * Check if cover art is displayed
   */
  async hasCoverArt(): Promise<boolean> {
    return await this.coverArt.isVisible();
  }

  /**
   * Check if placeholder is shown
   */
  async hasPlaceholder(): Promise<boolean> {
    return await this.coverPlaceholder.isVisible();
  }

  /**
   * Get progress percentage text
   */
  async getProgressText(): Promise<string> {
    return (await this.progressDisplay.textContent()) ?? '';
  }

  /**
   * Check if completion badge is visible
   */
  async hasCompletionBadge(): Promise<boolean> {
    return await this.completionBadge.isVisible();
  }

  /**
   * Verify details panel is visible
   */
  async expectDetailsVisible(): Promise<void> {
    await expect(this.detailsPanel).toBeVisible();
  }

  /**
   * Verify title is displayed
   */
  async expectTitle(expected: string): Promise<void> {
    await expect(this.audiobookTitle).toContainText(expected);
  }

  /**
   * Verify author is displayed
   */
  async expectAuthor(expected: string): Promise<void> {
    await expect(this.audiobookAuthor).toContainText(expected);
  }

  // ==================== File List Methods ====================

  /**
   * Get file count
   */
  async getFileCount(): Promise<number> {
    return await this.fileItems.count();
  }

  /**
   * Get file name by index
   */
  async getFileName(index: number): Promise<string> {
    const file = this.fileItems.nth(index);
    const nameElement = file.locator('.file-name, .card-title');
    return (await nameElement.textContent()) ?? '';
  }

  /**
   * Get file duration by index
   */
  async getFileDuration(index: number): Promise<string> {
    const file = this.fileItems.nth(index);
    const durationElement = file.locator('.file-duration, .duration');
    return (await durationElement.textContent()) ?? '';
  }

  /**
   * Get file progress by index
   */
  async getFileProgress(index: number): Promise<string> {
    const file = this.fileItems.nth(index);
    const progressElement = file.locator('.file-progress, .progress');
    return (await progressElement.textContent()) ?? '';
  }

  /**
   * Check if file is marked as missing
   */
  async isFileMissing(index: number): Promise<boolean> {
    const file = this.fileItems.nth(index);
    const missingIndicator = file.locator('.file-missing, .missing-indicator');
    return await missingIndicator.isVisible();
  }

  /**
   * Click file to select/play
   */
  async clickFile(index: number): Promise<void> {
    await this.fileItems.nth(index).click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Get remaining time text
   */
  async getRemainingTime(): Promise<string> {
    if (await this.remainingTime.isVisible()) {
      return (await this.remainingTime.textContent()) ?? '';
    }
    return '';
  }

  /**
   * Get total duration text
   */
  async getTotalDuration(): Promise<string> {
    if (await this.totalDuration.isVisible()) {
      return (await this.totalDuration.textContent()) ?? '';
    }
    return '';
  }

  /**
   * Check if bookmarks section is visible
   */
  async isBookmarksSectionVisible(): Promise<boolean> {
    return await this.bookmarksSection.isVisible();
  }

  /**
   * Check if no bookmarks message is shown
   */
  async showsNoBookmarksMessage(): Promise<boolean> {
    return await this.noBookmarksMessage.isVisible();
  }

  /**
   * Get cover art source URL
   */
  async getCoverArtSrc(): Promise<string> {
    if (await this.coverArt.isVisible()) {
      return (await this.coverArt.getAttribute('src')) ?? '';
    }
    return '';
  }

  /**
   * Scroll to bookmarks section
   */
  async scrollToBookmarks(): Promise<void> {
    if (await this.bookmarksSection.isVisible()) {
      await this.bookmarksSection.scrollIntoViewIfNeeded();
    }
  }

  /**
   * Scroll to files section
   */
  async scrollToFiles(): Promise<void> {
    if (await this.filesList.isVisible()) {
      await this.filesList.scrollIntoViewIfNeeded();
    }
  }

  /**
   * Get all file names
   */
  async getAllFileNames(): Promise<string[]> {
    const names: string[] = [];
    const count = await this.getFileCount();
    for (let i = 0; i < count; i++) {
      names.push(await this.getFileName(i));
    }
    return names;
  }

  /**
   * Verify file is selected
   */
  async expectFileSelected(index: number): Promise<void> {
    const file = this.fileItems.nth(index);
    await expect(file).toHaveClass(/ring-primary|selected/);
  }
}
