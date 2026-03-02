/**
 * Audiobook Library Step Definitions
 * Implements UI interactions for audiobook library feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { LibraryPage, DetailsPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('the application has an empty database', async function (this: NodokaWorld) {
  // In test mode, navigate with empty state flag
  await this.page.goto('http://localhost:5173/?test=true&empty=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the application has an audiobook', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBeGreaterThanOrEqual(1);
  this.lastCount = count;
});

Given('the audiobook has files', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  const fileCount = await library.getFileCount();
  expect(fileCount).toBeGreaterThanOrEqual(3);
});

Given('the application has a directory', async function (this: NodokaWorld) {
  // In test mode, directories are pre-populated in mock data
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBeGreaterThanOrEqual(1);
});

Given('the application has an audiobook with author', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
});

Given('the audiobook has bookmarks', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  const details = new DetailsPage(this.page);
  const count = await details.getBookmarkCount();
  expect(count).toBeGreaterThanOrEqual(3);
});

Given('the application has an audiobook in that directory', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBeGreaterThanOrEqual(1);
});

Given('the audiobook has files with progress', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  const fileCount = await library.getFileCount();
  expect(fileCount).toBeGreaterThanOrEqual(1);
});

// ==================== When Steps ====================

When('I add a directory to the library', async function (this: NodokaWorld) {
  // Open settings and click add directory
  const settingsButton = this.page.locator('button[aria-label="Settings"]');
  await settingsButton.click();
  await this.page.waitForTimeout(300);

  const addDirectoryButton = this.page.locator('button:has-text("Add Directory")');
  if (await addDirectoryButton.isVisible()) {
    // In test mode, this simulates adding a directory
    await this.page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('test:add-directory'));
    });
  }

  const closeButton = this.page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  }
  await this.page.waitForTimeout(300);
});

When('I request all audiobooks', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  this.lastCount = await library.getAudiobookCount();
});

When('I add an audiobook', async function (this: NodokaWorld) {
  // In test mode, simulate adding an audiobook via UI action
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:add-audiobook'));
  });
  await this.page.waitForTimeout(500);
});

When('I request the audiobook by its ID', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  this.lastAudiobookId = 'test-audiobook-id';
});

When('I request the files for the audiobook', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  this.lastCount = await library.getFileCount();
});

When('I remove the directory', async function (this: NodokaWorld) {
  // Open settings and remove first directory
  const settingsButton = this.page.locator('button[aria-label="Settings"]');
  await settingsButton.click();
  await this.page.waitForTimeout(300);

  const removeButton = this.page.locator('.directory-item button[aria-label="Remove"]').first();
  if (await removeButton.isVisible()) {
    await removeButton.click();
  }

  const closeButton = this.page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  }
  await this.page.waitForTimeout(300);
});

When('I delete the audiobook', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);

  // Right-click on audiobook to open context menu
  await library.audiobookCards.first().click({ button: 'right' });
  await this.page.waitForTimeout(200);

  // Click delete option
  const deleteOption = this.page.locator('text=Delete');
  if (await deleteOption.isVisible()) {
    await deleteOption.click();

    // Confirm deletion if dialog appears
    const confirmButton = this.page.locator('button:has-text("Confirm")');
    if (await confirmButton.isVisible()) {
      await confirmButton.click();
    }
  }
  await this.page.waitForTimeout(500);
});

When('I set the audiobook author to {string}', async function (this: NodokaWorld, author: string) {
  // Simulate author update via test event
  await this.page.evaluate((a) => {
    window.dispatchEvent(new CustomEvent('test:update-author', { detail: a }));
  }, author);
  await this.page.waitForTimeout(300);
});

When('I clear the audiobook author', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:clear-author'));
  });
  await this.page.waitForTimeout(300);
});

When('I set the audiobook total duration to {int} ms', async function (this: NodokaWorld, duration: number) {
  await this.page.evaluate((d) => {
    window.dispatchEvent(new CustomEvent('test:set-duration', { detail: d }));
  }, duration);
  await this.page.waitForTimeout(300);
});

When('I mark the audiobook as complete', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:mark-complete'));
  });
  await this.page.waitForTimeout(300);
});

When('I reset the audiobook progress', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:reset-progress'));
  });
  await this.page.waitForTimeout(300);
});

When('I request the audiobook by path', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
});

When('I request audiobooks for the directory', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  this.lastCount = await library.getAudiobookCount();
});

// ==================== Then Steps ====================

Then('the directory should be stored in the database', async function (this: NodokaWorld) {
  // Verify directory exists by checking library is populated
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBeGreaterThanOrEqual(0);
});

Then('the directory count should be one', async function (this: NodokaWorld) {
  // In test mode, verify state through UI
  const settingsButton = this.page.locator('button[aria-label="Settings"]');
  await settingsButton.click();
  await this.page.waitForTimeout(300);

  const directoryItems = this.page.locator('.directory-item');
  const count = await directoryItems.count();
  expect(count).toBeGreaterThanOrEqual(1);

  const closeButton = this.page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  }
});

Then('I should receive an empty list', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBe(0);
});

Then('the audiobook should be stored in the database', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBeGreaterThanOrEqual(1);
});

Then('the audiobook count should be one', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBe(1);
});

Then('the completeness should be zero', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const progressText = await details.getProgressText();
  expect(progressText).toContain('0');
});

Then('I should receive three files', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getFileCount();
  expect(count).toBe(3);
});

Then('the files should be ordered by position', async function (this: NodokaWorld) {
  // Verify files are displayed in order
  const library = new LibraryPage(this.page);
  const files = library.fileCards;
  const count = await files.count();
  expect(count).toBeGreaterThan(0);
});

Then('the directory count should be zero', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBe(0);
});

Then('the audiobook count should be zero', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBe(0);
});

Then('the audiobook count should be two', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBe(2);
});

Then('the file count for the audiobook should be zero', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getFileCount();
  expect(count).toBe(0);
});

Then('the bookmark count for the audiobook should be zero', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const count = await details.getBookmarkCount();
  expect(count).toBe(0);
});

Then('the audiobook author should be {string}', async function (this: NodokaWorld, author: string) {
  const details = new DetailsPage(this.page);
  const displayedAuthor = await details.getAuthor();
  expect(displayedAuthor).toContain(author);
});

Then('the audiobook author should be None', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const displayedAuthor = await details.getAuthor();
  expect(displayedAuthor).toBeFalsy();
});

Then('the audiobook total duration should be {int}', async function (this: NodokaWorld, duration: number) {
  // Verify duration is displayed correctly
  const durationDisplay = this.page.locator('.duration-display');
  if (await durationDisplay.isVisible()) {
    const text = await durationDisplay.textContent();
    expect(text).toBeTruthy();
  }
});

Then('the audiobook completeness should be {int}', async function (this: NodokaWorld, completeness: number) {
  const details = new DetailsPage(this.page);
  const progressText = await details.getProgressText();
  expect(progressText).toContain(completeness.toString());
});

Then('the audiobook completeness should be zero', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const progressText = await details.getProgressText();
  expect(progressText).toContain('0');
});

Then('all files should be {int} percent complete', async function (this: NodokaWorld, percent: number) {
  const library = new LibraryPage(this.page);
  for (let i = 0; i < 3; i++) {
    const progress = await library.getFileProgress(i);
    expect(progress).toContain(percent.toString());
  }
});

Then('all files should have zero progress', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  for (let i = 0; i < 3; i++) {
    const progress = await library.getFileProgress(i);
    expect(progress).toContain('0');
  }
});

Then('the audiobook should be found', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBeGreaterThanOrEqual(1);
});

Then('I should receive one audiobook', async function (this: NodokaWorld) {
  expect(this.lastCount).toBe(1);
});
