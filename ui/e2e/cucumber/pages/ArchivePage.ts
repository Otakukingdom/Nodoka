/**
 * Archive Page Object Model
 * Encapsulates archive handling interactions for testing
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from '../../pom/BasePage.js';

export class ArchivePage extends BasePage {
  readonly archiveCard: Locator;
  readonly extractProgress: Locator;
  readonly fileList: Locator;
  readonly archiveIcon: Locator;
  readonly archiveBadge: Locator;
  readonly passwordPrompt: Locator;
  readonly passwordInput: Locator;
  readonly passwordSubmit: Locator;
  readonly errorMessage: Locator;
  readonly folderStructure: Locator;

  constructor(page: Page) {
    super(page);
    this.archiveCard = page.locator('.card:has(.archive-icon), .card:has(.zip-icon)');
    this.extractProgress = page.locator('.extraction-progress, [role="progressbar"]');
    this.fileList = page.locator('.file-list-item, #files-list .card');
    this.archiveIcon = page.locator('.archive-icon, .zip-icon, svg[class*="archive"]');
    this.archiveBadge = page.locator('.format-badge:has-text("RAR"), .rar-badge, .archive-badge');
    this.passwordPrompt = page.locator('.password-prompt, .modal-box:has-text("Password")');
    this.passwordInput = page.locator('input[type="password"]');
    this.passwordSubmit = page.locator('button:has-text("Submit"), button:has-text("Unlock")');
    this.errorMessage = page.locator('.error-message, .alert-error, [role="alert"]');
    this.folderStructure = page.locator('.folder-tree, .folder-hierarchy, .nested-folders');
  }

  /**
   * Select archive audiobook by index
   */
  async selectArchive(index: number): Promise<void> {
    await this.archiveCard.nth(index).click();
    await this.page.waitForTimeout(300);
  }

  /**
   * Wait for extraction to complete
   */
  async waitForExtraction(timeout: number = 30000): Promise<void> {
    await this.extractProgress.waitFor({ state: 'hidden', timeout });
    await this.page.waitForTimeout(500);
  }

  /**
   * Get file count from archive
   */
  async getFileCount(): Promise<number> {
    return await this.fileList.count();
  }

  /**
   * Check if archive icon is visible
   */
  async hasArchiveIcon(): Promise<boolean> {
    return await this.archiveIcon.first().isVisible();
  }

  /**
   * Check if archive badge is visible
   */
  async hasArchiveBadge(): Promise<boolean> {
    return await this.archiveBadge.first().isVisible();
  }

  /**
   * Check if password prompt is visible
   */
  async isPasswordPromptVisible(): Promise<boolean> {
    return await this.passwordPrompt.isVisible();
  }

  /**
   * Enter password for protected archive
   */
  async enterPassword(password: string): Promise<void> {
    await this.passwordInput.fill(password);
    await this.passwordSubmit.click();
    await this.page.waitForTimeout(500);
  }

  /**
   * Check if error message is displayed
   */
  async hasError(): Promise<boolean> {
    return await this.errorMessage.first().isVisible();
  }

  /**
   * Get error message text
   */
  async getErrorMessage(): Promise<string> {
    return (await this.errorMessage.first().textContent()) ?? '';
  }

  /**
   * Check if folder structure is visible
   */
  async hasFolderStructure(): Promise<boolean> {
    return await this.folderStructure.count() > 0;
  }

  /**
   * Get archive type badge text
   */
  async getArchiveTypeBadge(): Promise<string> {
    const badge = this.archiveBadge.first();
    if (await badge.isVisible()) {
      return (await badge.textContent()) ?? '';
    }
    return '';
  }

  /**
   * Verify archive card is visible
   */
  async expectArchiveCardVisible(): Promise<void> {
    await expect(this.archiveCard.first()).toBeVisible();
  }

  /**
   * Verify extraction progress is shown
   */
  async expectExtractionProgress(): Promise<void> {
    await expect(this.extractProgress.first()).toBeVisible();
  }

  /**
   * Verify password prompt is shown
   */
  async expectPasswordPrompt(): Promise<void> {
    await expect(this.passwordPrompt).toBeVisible();
  }

  /**
   * Verify error is displayed
   */
  async expectError(): Promise<void> {
    await expect(this.errorMessage.first()).toBeVisible();
  }
}
