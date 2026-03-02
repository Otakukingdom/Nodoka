/**
 * Library Page Object Model
 * Encapsulates library panel interactions (select, search, sort)
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from './BasePage';

export class LibraryPage extends BasePage {
  // Locators
  readonly libraryPanel: Locator;
  readonly audiobookList: Locator;
  readonly audiobookCards: Locator;
  readonly searchInput: Locator;
  readonly sortSelect: Locator;
  readonly filesList: Locator;
  readonly fileCards: Locator;

  constructor(page: Page) {
    super(page);
    this.libraryPanel = page.locator('.library-panel');
    this.audiobookList = page.locator('#audiobook-list');
    this.audiobookCards = page.locator('#audiobook-list .card');
    this.searchInput = page.locator('.library-panel input[placeholder*="Search"]');
    this.sortSelect = page.locator('.library-panel select');
    this.filesList = page.locator('#files-list');
    this.fileCards = page.locator('#files-list .card');
  }

  /**
   * Get audiobook count
   */
  async getAudiobookCount(): Promise<number> {
    return await this.audiobookCards.count();
  }

  /**
   * Select an audiobook by index
   */
  async selectAudiobook(index: number): Promise<void> {
    const card = this.audiobookCards.nth(index);
    await card.click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Select an audiobook by name
   */
  async selectAudiobookByName(name: string): Promise<void> {
    const card = this.audiobookCards.filter({ hasText: name }).first();
    await card.click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Get selected audiobook
   */
  async getSelectedAudiobook(): Promise<Locator | null> {
    const selected = this.audiobookCards.filter({ has: this.page.locator('.ring-primary') });
    const count = await selected.count();
    return count > 0 ? selected.first() : null;
  }

  /**
   * Search for audiobooks
   */
  async search(query: string): Promise<void> {
    await this.searchInput.fill(query);
    await this.page.waitForTimeout(500);
  }

  /**
   * Clear search
   */
  async clearSearch(): Promise<void> {
    await this.searchInput.clear();
    await this.page.waitForTimeout(500);
  }

  /**
   * Sort audiobooks by criteria
   */
  async sortBy(criteria: 'RecentlyPlayed' | 'Title' | 'Progress'): Promise<void> {
    await this.sortSelect.selectOption(criteria);
    await this.page.waitForTimeout(500);
  }

  /**
   * Get file count for selected audiobook
   */
  async getFileCount(): Promise<number> {
    return await this.fileCards.count();
  }

  /**
   * Select a file by index
   */
  async selectFile(index: number): Promise<void> {
    const file = this.fileCards.nth(index);
    await file.click();
    await this.page.waitForTimeout(200);
  }

  /**
   * Get selected file
   */
  async getSelectedFile(): Promise<Locator | null> {
    const selected = this.fileCards.filter({ has: this.page.locator('.ring-primary') });
    const count = await selected.count();
    return count > 0 ? selected.first() : null;
  }

  /**
   * Get audiobook progress percentage
   */
  async getAudiobookProgress(index: number): Promise<string | null> {
    const card = this.audiobookCards.nth(index);
    const progressText = card.locator('text=/%/');
    if (await progressText.count() > 0) {
      return await progressText.first().textContent();
    }
    return null;
  }

  /**
   * Get file progress percentage
   */
  async getFileProgress(index: number): Promise<string | null> {
    const file = this.fileCards.nth(index);
    const progressText = file.locator('text=/%/');
    if (await progressText.count() > 0) {
      return await progressText.textContent();
    }
    return null;
  }

  /**
   * Verify audiobook is selected
   */
  async expectAudiobookSelected(index: number): Promise<void> {
    const card = this.audiobookCards.nth(index);
    await expect(card).toHaveClass(/ring-primary/);
  }

  /**
   * Verify file is selected
   */
  async expectFileSelected(index: number): Promise<void> {
    const file = this.fileCards.nth(index);
    await expect(file).toHaveClass(/ring-primary/);
  }

  /**
   * Get audiobook titles
   */
  async getAudiobookTitles(): Promise<string[]> {
    return await this.audiobookCards.locator('.card-title').allTextContents();
  }

  /**
   * Check if empty state is shown
   */
  async isEmptyStateShown(): Promise<boolean> {
    const noMatches = this.page.locator('text=No matches found');
    return await noMatches.isVisible();
  }

  /**
   * Bulk select audiobooks with Ctrl+Click
   */
  async bulkSelectAudiobooks(indices: number[]): Promise<void> {
    if (indices.length === 0) return;

    // Select first card normally
    await this.audiobookCards.nth(indices[0]).click();
    await this.page.waitForTimeout(200);

    // Select remaining cards with Ctrl modifier
    for (let i = 1; i < indices.length; i++) {
      await this.audiobookCards.nth(indices[i]).click({ modifiers: ['Control'] });
      await this.page.waitForTimeout(100);
    }
  }

  /**
   * Delete audiobook via context menu (right-click)
   */
  async deleteAudiobookViaContextMenu(index: number): Promise<void> {
    await this.audiobookCards.nth(index).click({ button: 'right' });
    await this.page.waitForTimeout(200);

    const deleteOption = this.page.locator('text=Delete');
    if (await deleteOption.isVisible()) {
      await deleteOption.click();
    }
  }

  /**
   * Get empty state message
   */
  async getEmptyStateMessage(): Promise<string> {
    const emptyState = this.page.locator('text=No audiobooks, text=empty');
    if (await emptyState.first().isVisible()) {
      return await emptyState.first().textContent() ?? '';
    }
    return '';
  }

  /**
   * Sort by last listened
   */
  async sortByLastListened(): Promise<void> {
    await this.sortBy('RecentlyPlayed');
  }

  /**
   * Get audiobook author by index
   */
  async getAudiobookAuthor(index: number): Promise<string> {
    const card = this.audiobookCards.nth(index);
    const authorElement = card.locator('.author, .card-subtitle');
    if (await authorElement.count() > 0) {
      return await authorElement.textContent() ?? '';
    }
    return '';
  }

  /**
   * Get all selected audiobook cards
   */
  async getSelectedAudiobookCards(): Promise<Locator> {
    return this.audiobookCards.filter({ has: this.page.locator('.ring-primary, .selected') });
  }

  /**
   * Get bulk action bar
   */
  getBulkActionBar(): Locator {
    return this.page.locator('.bulk-action-bar, .selection-bar');
  }

  /**
   * Check if bulk action bar is visible
   */
  async isBulkActionBarVisible(): Promise<boolean> {
    return await this.getBulkActionBar().isVisible();
  }
}
