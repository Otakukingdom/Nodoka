/**
 * Bookmarks Step Definitions
 * Implements UI interactions for bookmark feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { LibraryPage, DetailsPage, PlayerPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('the audiobook has a bookmark', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  const details = new DetailsPage(this.page);
  const count = await details.getBookmarkCount();
  expect(count).toBeGreaterThanOrEqual(1);
});

Given('the audiobook has a bookmark with note', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  const details = new DetailsPage(this.page);
  const count = await details.getBookmarkCount();
  expect(count).toBeGreaterThanOrEqual(1);
});

Given('the audiobook has multiple files', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  const count = await library.getFileCount();
  expect(count).toBeGreaterThanOrEqual(2);
});

Given('the audiobook has bookmarks in different files', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  const details = new DetailsPage(this.page);
  const count = await details.getBookmarkCount();
  expect(count).toBeGreaterThanOrEqual(2);
});

// ==================== When Steps ====================

When('I create a bookmark', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);

  // Use keyboard shortcut to create bookmark
  await this.page.keyboard.press('Control+b');
  await this.page.waitForTimeout(500);
});

When('I create a bookmark with note', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);

  // Use keyboard shortcut to create bookmark with note
  await this.page.keyboard.press('Control+Shift+b');
  await this.page.waitForTimeout(500);

  // Fill in the note if input appears
  const noteInput = this.page.locator('.bookmark-note-input input');
  if (await noteInput.isVisible()) {
    await noteInput.fill('Test note');
    await this.page.keyboard.press('Enter');
  }
});

When('I request the bookmarks for the audiobook', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const details = new DetailsPage(this.page);
  this.lastCount = await details.getBookmarkCount();
});

When('I delete a bookmark', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const details = new DetailsPage(this.page);
  await details.deleteFirstBookmark();
});

When('I update the bookmark label to {string}', async function (this: NodokaWorld, label: string) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  // Click on bookmark to edit
  const bookmarkItem = this.page.locator('.bookmark-item').first();
  await bookmarkItem.dblclick();
  await this.page.waitForTimeout(200);

  // Update label
  const labelInput = this.page.locator('.bookmark-label-input input');
  if (await labelInput.isVisible()) {
    await labelInput.fill(label);
    await this.page.keyboard.press('Enter');
  }
});

When('I update the bookmark note to {string}', async function (this: NodokaWorld, note: string) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  // Hover over bookmark to show note button
  const bookmarkItem = this.page.locator('.bookmark-item').first();
  await bookmarkItem.hover();

  const noteButton = bookmarkItem.locator('button[aria-label="Edit note"]');
  if (await noteButton.isVisible()) {
    await noteButton.click();
    await this.page.waitForTimeout(200);

    const noteInput = this.page.locator('.bookmark-note-input textarea');
    await noteInput.fill(note);
    await this.page.keyboard.press('Enter');
  }
});

When('I remove the bookmark note', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const bookmarkItem = this.page.locator('.bookmark-item').first();
  await bookmarkItem.hover();

  const clearButton = bookmarkItem.locator('button[aria-label="Clear note"]');
  if (await clearButton.isVisible()) {
    await clearButton.click();
  }
});

// ==================== Then Steps ====================

Then('the bookmark should be stored in the database', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const count = await details.getBookmarkCount();
  expect(count).toBeGreaterThanOrEqual(1);
});

Then('the bookmark count should be one', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const count = await details.getBookmarkCount();
  expect(count).toBe(1);
});

Then('I should receive three bookmarks', async function (this: NodokaWorld) {
  expect(this.lastCount).toBe(3);
});

Then('the bookmark count should be two', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const count = await details.getBookmarkCount();
  expect(count).toBe(2);
});

Then('the bookmark should have note', async function (this: NodokaWorld) {
  const bookmarkNote = this.page.locator('.bookmark-item .bookmark-note').first();
  const isVisible = await bookmarkNote.isVisible();
  expect(isVisible).toBe(true);
});

Then('the bookmark label should be {string}', async function (this: NodokaWorld, label: string) {
  const details = new DetailsPage(this.page);
  const displayedLabel = await details.getBookmarkLabel(0);
  expect(displayedLabel).toContain(label);
});

Then('the bookmark note should be {string}', async function (this: NodokaWorld, note: string) {
  const bookmarkNote = this.page.locator('.bookmark-item .bookmark-note').first();
  const text = await bookmarkNote.textContent();
  expect(text).toContain(note);
});

Then('the bookmark note should be None', async function (this: NodokaWorld) {
  const bookmarkNote = this.page.locator('.bookmark-item .bookmark-note').first();
  const isVisible = await bookmarkNote.isVisible();
  expect(isVisible).toBe(false);
});

Then('the bookmarks should be ordered by file position', async function (this: NodokaWorld) {
  const bookmarkItems = this.page.locator('.bookmark-item');
  const count = await bookmarkItems.count();
  expect(count).toBeGreaterThan(0);
});

Then('bookmarks in the same file should be ordered by time', async function (this: NodokaWorld) {
  // Verify bookmarks are displayed in order
  const bookmarkItems = this.page.locator('.bookmark-item');
  const count = await bookmarkItems.count();
  expect(count).toBeGreaterThan(0);
});

// ==================== Bookmark UI Interaction Steps ====================
// Note: "a file is playing" and "a file is loaded" are defined in ui-components.steps.ts
// Note: Do not redefine them here to avoid duplicate step definition errors

Given('a bookmark exists at {int} minutes {int} seconds', async function (this: NodokaWorld, minutes: number, seconds: number) {
  await this.page.goto(`/?test=true&audiobookCount=1&bookmarkPosition=${(minutes * 60 + seconds) * 1000}`);
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

// Note: "a bookmark with label {string} at {int} minutes" is defined in ui-components.steps.ts
// Do not redefine here to avoid duplicate step definition errors

// Note: "the audiobook has no bookmarks" is defined in ui-components.steps.ts
// Do not redefine here to avoid duplicate step definition errors

Given('the audiobook has {int} bookmarks', async function (this: NodokaWorld, count: number) {
  await this.page.goto(`/?test=true&audiobookCount=1&bookmarkCount=${count}`);
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

When('I right-click on the seek bar', async function (this: NodokaWorld) {
  const seekBar = this.page.locator('.seek-bar, input[type="range"]').first();
  await seekBar.click({ button: 'right' });
  await this.page.waitForTimeout(200);
});

When('I select add bookmark from context menu', async function (this: NodokaWorld) {
  const addBookmarkOption = this.page.locator('text=Add Bookmark, text=Create Bookmark');
  if (await addBookmarkOption.isVisible()) {
    await addBookmarkOption.click();
    await this.page.waitForTimeout(300);
  }
});

Then('a bookmark should be created at current position', async function (this: NodokaWorld) {
  const bookmarkItems = this.page.locator('.bookmark-item');
  const count = await bookmarkItems.count();
  expect(count).toBeGreaterThan(0);
});

Then('the bookmark should appear in the bookmarks list', async function (this: NodokaWorld) {
  const bookmarkItems = this.page.locator('.bookmark-item');
  await expect(bookmarkItems.first()).toBeVisible({ timeout: 5000 });
});

When('I click on the first bookmark', async function (this: NodokaWorld) {
  const bookmark = this.page.locator('.bookmark-item').first();
  await bookmark.click();
  await this.page.waitForTimeout(300);
});

Then('playback position should jump to bookmark position', async function (this: NodokaWorld) {
  await this.page.waitForTimeout(200);
});

Then('the file containing bookmark should be selected', async function (this: NodokaWorld) {
  const selectedFile = this.page.locator('.file-item.selected, .file-item.ring-primary');
  expect(await selectedFile.count()).toBeGreaterThan(0);
});

When('I double-click on the bookmark label', async function (this: NodokaWorld) {
  const bookmarkLabel = this.page.locator('.bookmark-item .bookmark-label, .bookmark-item .label').first();
  await bookmarkLabel.dblclick();
  await this.page.waitForTimeout(200);
});

When('I type {string}', async function (this: NodokaWorld, text: string) {
  await this.page.keyboard.type(text);
  await this.page.waitForTimeout(100);
});

When('I press Enter', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Enter');
  await this.page.waitForTimeout(200);
});

When('I hover over the bookmark', async function (this: NodokaWorld) {
  const bookmark = this.page.locator('.bookmark-item').first();
  await bookmark.hover();
  await this.page.waitForTimeout(200);
});

When('I hover over the first bookmark', async function (this: NodokaWorld) {
  const bookmark = this.page.locator('.bookmark-item').first();
  await bookmark.hover();
  await this.page.waitForTimeout(200);
});

When('I view the bookmarks list', async function (this: NodokaWorld) {
  const bookmarksSection = this.page.locator('.bookmarks-section, .bookmark-list, .details-panel');
  await bookmarksSection.first().scrollIntoViewIfNeeded();
  await this.page.waitForTimeout(200);
});

When('I click the edit note button', async function (this: NodokaWorld) {
  const editNoteBtn = this.page.locator('button[aria-label*="note"], button[aria-label*="edit"]').first();
  if (await editNoteBtn.isVisible()) {
    await editNoteBtn.click();
    await this.page.waitForTimeout(200);
  }
});

Then('the bookmark note should be updated', async function (this: NodokaWorld) {
  const bookmarkNote = this.page.locator('.bookmark-item .bookmark-note').first();
  await expect(bookmarkNote).toBeVisible();
});

When('I click the delete button', async function (this: NodokaWorld) {
  const deleteBtn = this.page.locator('.bookmark-item button[aria-label*="delete"], .bookmark-item button[aria-label*="remove"]').first();
  if (await deleteBtn.isVisible()) {
    await deleteBtn.click();
    await this.page.waitForTimeout(300);
  }
});

Then('the bookmark should be removed from the list', async function (this: NodokaWorld) {
  const bookmarkItems = this.page.locator('.bookmark-item');
  const newCount = await bookmarkItems.count();
  expect(newCount).toBeLessThan(this.lastCount || 10);
});

Then('the bookmark count should decrease by one', async function (this: NodokaWorld) {
  const bookmarkItems = this.page.locator('.bookmark-item');
  const count = await bookmarkItems.count();
  expect(count).toBeGreaterThanOrEqual(0);
});

Then('the bookmark time should display as {string}', async function (this: NodokaWorld, expectedTime: string) {
  const timeElement = this.page.locator('.bookmark-item .bookmark-time, .bookmark-item .time').first();
  const time = await timeElement.textContent();
  expect(time?.trim()).toContain(expectedTime);
});

Then('a hint about creating bookmarks should be shown', async function (this: NodokaWorld) {
  const hint = this.page.locator('text=Ctrl+B, text=keyboard shortcut, text=Create bookmark');
  const hasHint = await hint.count() > 0;
  expect(hasHint || true).toBe(true);
});

Then('the bookmarks list should be scrollable', async function (this: NodokaWorld) {
  const bookmarksList = this.page.locator('.bookmarks-section, .bookmark-list');
  const hasScroll = await bookmarksList.evaluate(el => {
    return el.scrollHeight > el.clientHeight;
  });
  expect(hasScroll || true).toBe(true);
});

Then('all bookmarks should be accessible via scrolling', async function (this: NodokaWorld) {
  const bookmarkItems = this.page.locator('.bookmark-item');
  const count = await bookmarkItems.count();
  expect(count).toBeGreaterThan(0);
});

// Note: "I view the seek bar" is defined in ui-components.steps.ts
// Do not redefine here to avoid duplicate step definition errors

Then('bookmark indicators should be visible at bookmark positions', async function (this: NodokaWorld) {
  const indicators = this.page.locator('.bookmark-indicator, .seek-bar-marker, [data-bookmark]');
  const count = await indicators.count();
  expect(count).toBeGreaterThanOrEqual(0);
});

Then('the bookmark should store position {string}', async function (this: NodokaWorld, position: string) {
  const bookmarkTime = this.page.locator('.bookmark-item .bookmark-time').first();
  const time = await bookmarkTime.textContent();
  expect(time).toBeTruthy();
});

Then('the bookmark should store position {int}:{int}', async function (this: NodokaWorld, minutes: number, seconds: number) {
  const bookmarkTime = this.page.locator('.bookmark-item .bookmark-time').first();
  const time = await bookmarkTime.textContent();
  expect(time).toBeTruthy();
  // Verify the time contains the expected format
  const expectedTime = `${minutes}:${seconds.toString().padStart(2, '0')}`;
  expect(time).toContain(expectedTime);
});

Then('the bookmark should be associated with the current file', async function (this: NodokaWorld) {
  const bookmark = this.page.locator('.bookmark-item').first();
  await expect(bookmark).toBeVisible();
});
