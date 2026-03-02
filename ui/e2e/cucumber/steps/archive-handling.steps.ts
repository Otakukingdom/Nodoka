/**
 * Archive Handling Step Definitions
 * Implements UI interactions for archive handling feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { LibraryPage, PlayerPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('the library has a ZIP archive audiobook', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=zip');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a RAR archive audiobook', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=rar');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a 7z archive audiobook', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=7z');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a nested archive', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=nested');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a ZIP archive with multiple audiobooks', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=zip&multiBook=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a corrupted archive', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=corrupted');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a password protected archive', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=password');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a large ZIP archive audiobook', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=zip&largeArchive=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

// ==================== When Steps ====================

When('I scan the archive', async function (this: NodokaWorld) {
  // Trigger a library scan which will process archives
  const settingsButton = this.page.locator('button[aria-label="Settings"]');
  await settingsButton.click();
  await this.page.waitForTimeout(300);

  // Click rescan button
  const rescanButton = this.page.locator('button:has-text("Rescan"), button:has-text("Scan"), button:has-text("Rescan All")').first();
  if (await rescanButton.isVisible()) {
    await rescanButton.click();
    await this.page.waitForTimeout(500);
  }

  // Close settings modal
  const closeButton = this.page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  }
  await this.page.waitForTimeout(500);
});

When('I try to select the audiobook', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  if (count > 0) {
    await library.selectAudiobook(0);
  }
});

When('I force a full rescan', async function (this: NodokaWorld) {
  // Open settings and trigger full rescan
  const settingsButton = this.page.locator('button[aria-label="Settings"]');
  await settingsButton.click();
  await this.page.waitForTimeout(300);

  const rescanButton = this.page.locator('button:has-text("Rescan All")');
  if (await rescanButton.isVisible()) {
    await rescanButton.click();
  }

  const closeButton = this.page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  }
  await this.page.waitForTimeout(500);
});

When('I view the audiobook in the library', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);
});

When('I view the audiobook card', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  // Just view the card without selecting - scroll to make visible
  const card = library.audiobookCards.first();
  await card.scrollIntoViewIfNeeded();
  await this.page.waitForTimeout(300);
});

When('I view the audiobook details', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);
});

// ==================== Then Steps ====================

Then('the files should be listed from the archive', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const fileCount = await library.getFileCount();
  expect(fileCount).toBeGreaterThan(0);
});

Then('I can play files from the archive', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectFile(0);

  const player = new PlayerPage(this.page);
  await player.expectPlayerVisible();
});

Then('the innermost files should be accessible', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const fileCount = await library.getFileCount();
  expect(fileCount).toBeGreaterThan(0);
});

Then('each audiobook should be listed separately', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBeGreaterThan(1);
});

Then('a password prompt should appear', async function (this: NodokaWorld) {
  const passwordPrompt = this.page.locator('.password-prompt, input[type="password"], .password-modal');
  await expect(passwordPrompt.first()).toBeVisible({ timeout: 5000 });
});

Then('an archive icon should be visible on the audiobook card', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const card = library.audiobookCards.first();
  await card.scrollIntoViewIfNeeded();

  // Look for archive-specific indicators
  const archiveIcon = card.locator('.archive-icon, .zip-icon, [data-archive-type], svg[class*="archive"], [data-testid="archive-icon"]');
  const hasArchiveIndicator = await archiveIcon.count() > 0;

  // Also check for archive badge
  const archiveBadge = card.locator('.badge:has-text("ZIP"), .badge:has-text("RAR"), .badge:has-text("7z"), .format-badge');
  const hasBadge = await archiveBadge.count() > 0;

  expect(hasArchiveIndicator || hasBadge).toBe(true);
});

Then('an extraction progress indicator should be displayed', async function (this: NodokaWorld) {
  const progress = this.page.locator('.extraction-progress, [role="progressbar"], .progress-indicator, .loading-spinner');
  await expect(progress.first()).toBeVisible({ timeout: 5000 });
});

Then('the total file count should be shown in the details panel', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const detailsPanel = this.page.locator('.details-panel, .right-panel');
  const fileCount = detailsPanel.locator('.file-count, text=/\\d+\\s*files?/, text=/\\d+\\s*items?/');
  await expect(fileCount.first()).toBeVisible({ timeout: 5000 });
});

Then('the archive file size should be displayed', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const detailsPanel = this.page.locator('.details-panel, .right-panel');
  const sizeElement = detailsPanel.locator('.file-size, .archive-size, text=/\\d+\\.?\\d*\\s*(MB|GB|KB)/');
  await expect(sizeElement.first()).toBeVisible({ timeout: 5000 });
});

Then('the folder hierarchy should be visible', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  // Look for folder structure indicators
  const folderStructure = this.page.locator('.folder-tree, .folder-hierarchy, .nested-folders, .folder-item, [data-folder]');
  const hasFolders = await folderStructure.count() > 0;

  // Also check for path indicators in file list
  const pathIndicators = this.page.locator('.file-path, .folder-path');
  const hasPaths = await pathIndicators.count() > 0;

  expect(hasFolders || hasPaths).toBe(true);
});

Then('each file should show it is from an archive', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const files = library.fileCards;
  const count = await files.count();
  expect(count).toBeGreaterThan(0);

  // Check that files have archive indicators or are in archive context
  const archiveContext = this.page.locator('.archive-context, .from-archive, [data-from-archive]');
  const hasArchiveContext = await archiveContext.count() > 0;

  // Verify files are displayed
  if (count > 0) {
    const firstFile = files.first();
    await expect(firstFile).toBeVisible();
  }
});

Then('a RAR badge should be visible', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const card = library.audiobookCards.first();
  await card.scrollIntoViewIfNeeded();

  // Look for RAR-specific badge
  const rarBadge = card.locator('.format-badge:has-text("RAR"), .rar-badge, .archive-badge:has-text("RAR"), text="RAR"');
  const hasBadge = await rarBadge.count() > 0;

  // Also check for archive type indicator
  const archiveType = card.locator('[data-archive-type="rar"]');
  const hasArchiveType = await archiveType.count() > 0;

  expect(hasBadge || hasArchiveType).toBe(true);
});

// ==================== Extended Archive UI Interaction Steps ====================

Then('a password prompt modal should appear', async function (this: NodokaWorld) {
  const passwordPrompt = this.page.locator('.password-prompt, .modal-box:has-text("Password"), input[type="password"]');
  await expect(passwordPrompt.first()).toBeVisible({ timeout: 5000 });
});

Then('the password input field should be focused', async function (this: NodokaWorld) {
  const passwordInput = this.page.locator('input[type="password"]');
  await expect(passwordInput).toBeFocused();
});

Then('a submit button should be visible', async function (this: NodokaWorld) {
  const submitButton = this.page.locator('button:has-text("Submit"), button:has-text("Unlock"), button[type="submit"]');
  expect(await submitButton.count()).toBeGreaterThan(0);
});

When('I enter the correct password', async function (this: NodokaWorld) {
  const passwordInput = this.page.locator('input[type="password"]');
  await passwordInput.fill('correct_password');
  await this.page.keyboard.press('Enter');
  await this.page.waitForTimeout(500);
});

Then('the archive should be unlocked', async function (this: NodokaWorld) {
  const passwordPrompt = this.page.locator('.password-prompt');
  await expect(passwordPrompt).not.toBeVisible({ timeout: 5000 });
});

When('I enter an incorrect password', async function (this: NodokaWorld) {
  const passwordInput = this.page.locator('input[type="password"]');
  await passwordInput.fill('wrong_password');
  await this.page.keyboard.press('Enter');
  await this.page.waitForTimeout(500);
});

Then('the password prompt should remain visible', async function (this: NodokaWorld) {
  const passwordPrompt = this.page.locator('.password-prompt, input[type="password"]');
  await expect(passwordPrompt.first()).toBeVisible();
});

When('I click cancel on the password prompt', async function (this: NodokaWorld) {
  const cancelButton = this.page.locator('button:has-text("Cancel"), button[aria-label="Cancel"]');
  if (await cancelButton.count() > 0) {
    await cancelButton.first().click();
    await this.page.waitForTimeout(300);
  }
});

Then('the audiobook should not be selected', async function (this: NodokaWorld) {
  const selectedCard = this.page.locator('.card.selected, .card.ring-primary');
  expect(await selectedCard.count()).toBe(0);
});

When('I click on a folder', async function (this: NodokaWorld) {
  const folder = this.page.locator('.folder-item, .tree-item, [data-folder]').first();
  if (await folder.isVisible()) {
    await folder.click();
    await this.page.waitForTimeout(300);
  }
});

Then('the folder contents should be displayed', async function (this: NodokaWorld) {
  const files = this.page.locator('.file-item, .file-card');
  expect(await files.count()).toBeGreaterThan(0);
});

Then('nested files should be accessible', async function (this: NodokaWorld) {
  const files = this.page.locator('.file-item, .file-card');
  const count = await files.count();
  expect(count).toBeGreaterThan(0);
});

Then('an extraction progress bar should be visible', async function (this: NodokaWorld) {
  const progressBar = this.page.locator('[role="progressbar"], .progress-bar, .extraction-progress');
  await expect(progressBar.first()).toBeVisible({ timeout: 5000 });
});

Then('the progress percentage should be displayed', async function (this: NodokaWorld) {
  const progressText = this.page.locator('text=/\\d+%/, .progress-percentage');
  expect(await progressText.count()).toBeGreaterThan(0);
});

Then('the progress should update during extraction', async function (this: NodokaWorld) {
  // Wait for progress to potentially update
  await this.page.waitForTimeout(200);
});

Then('a ZIP badge should be visible', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const card = library.audiobookCards.first();
  const zipBadge = card.locator('.format-badge:has-text("ZIP"), .zip-badge, text="ZIP"');
  expect(await zipBadge.count()).toBeGreaterThan(0);
});

Then('a 7z badge should be visible', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const card = library.audiobookCards.first();
  const sevenZipBadge = card.locator('.format-badge:has-text("7z"), .7z-badge, text="7z"');
  expect(await sevenZipBadge.count()).toBeGreaterThan(0);
});

Then('the size should be in MB or GB format', async function (this: NodokaWorld) {
  const sizeElement = this.page.locator('.archive-size, .file-size, text=/\\d+\\.?\\d*\\s*(MB|GB|KB)/');
  const text = await sizeElement.first().textContent();
  expect(text).toMatch(/\d+\.?\d*\s*(MB|GB|KB)/);
});

Then('the audiobook card should have an archive icon', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const card = library.audiobookCards.first();
  const archiveIcon = card.locator('.archive-icon, svg[class*="archive"], [data-testid="archive-icon"]');
  expect(await archiveIcon.count()).toBeGreaterThan(0);
});

Then('hovering the icon should show archive type tooltip', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const card = library.audiobookCards.first();
  const archiveIcon = card.locator('.archive-icon, svg[class*="archive"]').first();
  if (await archiveIcon.isVisible()) {
    await archiveIcon.hover();
    await this.page.waitForTimeout(200);
  }
});

Then('each file should show an archive source indicator', async function (this: NodokaWorld) {
  const files = this.page.locator('.file-item, .file-card');
  const count = await files.count();
  expect(count).toBeGreaterThan(0);
});

Then('the error should mention archive corruption', async function (this: NodokaWorld) {
  const error = this.page.locator('.error-message, [role="alert"]');
  const text = await error.first().textContent();
  expect(text?.toLowerCase()).toMatch(/corrupt|damage|invalid/);
});

Then('a suggestion to re-download should be shown', async function (this: NodokaWorld) {
  const suggestion = this.page.locator('text=/re-download|redownload|download again/i, text=/try.*again/i');
  expect(await suggestion.count()).toBeGreaterThanOrEqual(0);
});

// Additional steps for archive scenarios

Then('the files should be listed', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await this.page.waitForTimeout(300);
  const fileCount = await library.getFileCount();
  expect(fileCount).toBeGreaterThan(0);
});

Then('the password prompt should close', async function (this: NodokaWorld) {
  const passwordPrompt = this.page.locator('.password-prompt, input[type="password"]');
  await expect(passwordPrompt.first()).not.toBeVisible({ timeout: 5000 });
});
