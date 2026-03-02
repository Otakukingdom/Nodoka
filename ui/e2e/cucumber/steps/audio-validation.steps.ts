/**
 * Audio Validation Step Definitions
 * Implements UI interactions for audio validation feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { LibraryPage, PlayerPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('the library has an audiobook with valid audio files', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&audioValid=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has an audiobook with unsupported file formats', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&audioUnsupported=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has an audiobook with corrupted audio', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&audioCorrupted=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has an audiobook with MP3 files', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&audioFormat=mp3');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has an audiobook with M4B files', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&audioFormat=m4b');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has an audiobook with FLAC files', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&audioFormat=flac');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has an audiobook with OGG files', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&audioFormat=ogg');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has an audiobook with WAV files', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&audioFormat=wav');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('I add a new file to the audiobook', async function (this: NodokaWorld) {
  // Navigate with query param to simulate adding a new file
  await this.page.goto('http://localhost:5173/?test=true&addNewFile=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  // Trigger a rescan to pick up the new file
  const settingsButton = this.page.locator('button[aria-label="Settings"]');
  await settingsButton.click();
  await this.page.waitForTimeout(300);

  const rescanButton = this.page.locator('button:has-text("Rescan")').first();
  if (await rescanButton.isVisible()) {
    await rescanButton.click();
    await this.page.waitForTimeout(500);
  }

  const closeButton = this.page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  }
  await this.page.waitForTimeout(300);
});

// ==================== When Steps ====================

When('I select a file', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);
});

When('I try to play the corrupted file', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  // Try to play the corrupted file
  await library.selectFile(0);
  const player = new PlayerPage(this.page);
  await player.play();
  await this.page.waitForTimeout(500);
});

// ==================== Then Steps ====================

Then('the files should be marked as playable', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const playableIndicator = this.page.locator('.file-playable');
  const count = await playableIndicator.count();
  expect(count).toBeGreaterThan(0);
});

Then('the unsupported files should be marked accordingly', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const unsupportedIndicator = this.page.locator('.file-unsupported');
  await expect(unsupportedIndicator.first()).toBeVisible();
});

Then('a warning should be displayed', async function (this: NodokaWorld) {
  const warning = this.page.locator('.warning, .alert-warning, [role="alert"]');
  await expect(warning.first()).toBeVisible();
});

Then('playback should not start', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const isPlaying = await player.isPlaying();
  expect(isPlaying).toBe(false);
});

Then('the file should be playable', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await player.expectPlayerVisible();

  // Try to play
  await player.play();
  await this.page.waitForTimeout(200);
});

Then('each file should display its duration', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const durations = this.page.locator('.file-duration');
  const count = await durations.count();
  expect(count).toBeGreaterThan(0);
});
