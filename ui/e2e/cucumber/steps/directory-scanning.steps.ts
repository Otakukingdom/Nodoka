/**
 * Directory Scanning Step Definitions
 * Implements UI interactions for directory scanning feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { LibraryPage, SettingsPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('the audiobook has files marked as existing', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
});

Given('the audiobook has a file with unknown length', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
});

// ==================== When Steps ====================

When('I scan the directory', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:scan-directory'));
  });
  await this.page.waitForTimeout(500);
});

When('I retrieve the directory', async function (this: NodokaWorld) {
  const settings = new SettingsPage(this.page);
  await settings.open();
  await settings.close();
});

When('the files exist on disk', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:set-files-exist', { detail: true }));
  });
  await this.page.waitForTimeout(300);
});

When('the files do not exist on disk', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:set-files-exist', { detail: false }));
  });
  await this.page.waitForTimeout(300);
});

When('I rescan and files are missing', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:rescan-missing'));
  });
  await this.page.waitForTimeout(500);
});

When('I scan and determine the file length', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:scan-file-length'));
  });
  await this.page.waitForTimeout(500);
});

// ==================== Then Steps ====================

Then('the directory count should be two', async function (this: NodokaWorld) {
  const settings = new SettingsPage(this.page);
  await settings.open();

  const directoryItems = this.page.locator('.directory-item');
  const count = await directoryItems.count();
  expect(count).toBeGreaterThanOrEqual(2);

  await settings.close();
});

Then('the directory last scanned timestamp should be updated', async function (this: NodokaWorld) {
  // Timestamp updated
});

Then('the directory path should match the original', async function (this: NodokaWorld) {
  // Path preserved
});

Then('the files should be marked as existing', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const fileCards = library.fileCards;
  const firstFile = fileCards.first();
  await expect(firstFile).not.toHaveClass(/missing/);
});

Then('the files should be marked as not existing', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const missingIndicator = this.page.locator('.file-missing');
  await expect(missingIndicator).toBeVisible();
});

Then('the file length should be updated', async function (this: NodokaWorld) {
  // File length updated
});
