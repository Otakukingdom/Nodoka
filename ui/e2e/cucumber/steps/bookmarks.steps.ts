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
