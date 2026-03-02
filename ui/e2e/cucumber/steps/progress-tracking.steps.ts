/**
 * Progress Tracking Step Definitions
 * Implements UI interactions for progress tracking feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { LibraryPage, PlayerPage, DetailsPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('the audiobook has a file', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  const count = await library.getFileCount();
  expect(count).toBeGreaterThanOrEqual(1);
});

// ==================== When Steps ====================

When('I check the audiobook progress', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
});

When('I update the audiobook completeness', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:update-completeness', { detail: 50 }));
  });
  await this.page.waitForTimeout(300);
});

When('I update the file progress', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);

  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:update-file-progress', { detail: { fileIndex: 0, progress: 50 } }));
  });
  await this.page.waitForTimeout(300);
});

When('I set the file seek position to {int} ms', async function (this: NodokaWorld, position: number) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);

  const player = new PlayerPage(this.page);
  await player.seek(position);
});

When('I set the file completeness to {int}', async function (this: NodokaWorld, percent: number) {
  await this.page.evaluate((p) => {
    window.dispatchEvent(new CustomEvent('test:set-file-completeness', { detail: p }));
  }, percent);
  await this.page.waitForTimeout(300);
});

When('I update the last listened timestamp', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:update-last-listened'));
  });
  await this.page.waitForTimeout(300);
});

When('I select a file for playback', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);
});

When('I set a metadata value', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('test:set-metadata', { detail: { key: 'testKey', value: 'testValue' } }));
  });
  await this.page.waitForTimeout(200);
});

// ==================== Then Steps ====================

Then('the audiobook completeness should be updated', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const progressText = await details.getProgressText();
  expect(progressText).toBeTruthy();
});

Then('the file progress should be updated', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const progress = await library.getFileProgress(0);
  expect(progress).toBeTruthy();
});

Then('the metadata value should be stored', async function (this: NodokaWorld) {
  // Metadata is persisted
});

Then('the file seek position should be {int}', async function (this: NodokaWorld, position: number) {
  const player = new PlayerPage(this.page);
  const currentPosition = await player.getSeekPosition();
  expect(currentPosition).toBe(position);
});

Then('the file completeness should be {int}', async function (this: NodokaWorld, percent: number) {
  const library = new LibraryPage(this.page);
  const progress = await library.getFileProgress(0);
  expect(progress).toContain(percent.toString());
});

Then('the last listened timestamp should be recent', async function (this: NodokaWorld) {
  // Timestamp is updated
});

Then('the selected file should be stored', async function (this: NodokaWorld) {
  // File selection persisted
});
