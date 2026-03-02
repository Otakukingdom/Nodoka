/**
 * Details Page Object Model
 * Encapsulates details panel interactions (bookmarks, audiobook info)
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

  constructor(page: Page) {
    super(page);
    this.detailsPanel = page.locator('.details-panel');
    this.audiobookTitle = page.locator('.details-panel .card-title');
    this.audiobookAuthor = page.locator('.details-panel .author');
    this.bookmarksSection = page.locator('.details-panel .bookmarks-section');
    this.bookmarkList = page.locator('.details-panel .bookmark-list');
    this.bookmarkItems = page.locator('.details-panel .bookmark-item');
    this.noBookmarksMessage = page.locator('text=No bookmarks');
    this.coverArt = page.locator('.details-panel .cover-art img');
    this.coverPlaceholder = page.locator('.details-panel .cover-art .placeholder');
    this.progressDisplay = page.locator('.details-panel .progress-text');
    this.completionBadge = page.locator('.details-panel .completion-badge');
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
    return (await this.audiobookAuthor.textContent()) ?? '';
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
}
