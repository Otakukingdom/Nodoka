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
  expect(count).toBeGreaterThan(1);
});

// ==================== UI-specific Given Steps ====================

Given('the library has an audiobook with mixed audio formats', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&audioFormat=mixed');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

// ==================== UI-specific When Steps ====================

When('I view the file details', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  // Click on a file to view details
  const fileInfo = this.page.locator('.file-info, .file-details');
  if (await fileInfo.count() > 0) {
    await fileInfo.first().click();
  }
  await this.page.waitForTimeout(300);
});

// ==================== UI-specific Then Steps ====================

Then('each playable file should have a play icon', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const files = library.fileCards;
  const count = await files.count();

  for (let i = 0; i < Math.min(count, 5); i++) {
    const playIcon = files.nth(i).locator('.play-icon, svg[class*="play"], [data-testid="play-icon"]');
    const hasIcon = await playIcon.count() > 0;
    expect(hasIcon).toBe(true);
  }
});

Then('each file should display its format as a badge', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const formatBadges = this.page.locator('.format-badge, .file-format');
  const count = await formatBadges.count();
  expect(count).toBeGreaterThan(0);
});

Then('the duration should be in HH:MM:SS or MM:SS format', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const durations = this.page.locator('.file-duration');
  const count = await durations.count();

  for (let i = 0; i < Math.min(count, 3); i++) {
    const text = await durations.nth(i).textContent();
    // Match MM:SS or HH:MM:SS format
    const timeFormat = /^\d{1,2}:\d{2}(:\d{2})?$/;
    expect(text?.trim()).toMatch(timeFormat);
  }
});

Then('the corrupted file should have a warning icon', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const warningIcon = this.page.locator('.warning-icon, svg[class*="warning"], [data-testid="warning-icon"]');
  await expect(warningIcon.first()).toBeVisible();
});

Then('the unsupported files should have an error indicator', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const errorIndicator = this.page.locator('.error-indicator, .unsupported-indicator, svg[class*="error"]');
  await expect(errorIndicator.first()).toBeVisible();
});

Then('each file should display its file size', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const fileSizes = this.page.locator('.file-size, text=/\\d+\\.?\\d*\\s*(MB|GB|KB)/');
  const count = await fileSizes.count();
  expect(count).toBeGreaterThan(0);
});

Then('the bitrate should be displayed', async function (this: NodokaWorld) {
  const bitrate = this.page.locator('.bitrate, text=/\\d+\\s*kbps/, text=/\\d+\\s*Kbps/');
  await expect(bitrate.first()).toBeVisible();
});

Then('the sample rate should be displayed', async function (this: NodokaWorld) {
  const sampleRate = this.page.locator('.sample-rate, text=/\\d+\\s*(Hz|kHz)/');
  await expect(sampleRate.first()).toBeVisible();
});
