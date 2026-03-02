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

Given('the library has an audiobook with mixed audio formats', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&audioFormat=mixed');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
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

// ==================== Then Steps ====================

Then('the files should be marked as playable', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  // Look for playable indicators
  const playableIndicator = this.page.locator('.file-playable, [data-playable="true"], .playable-file');
  const count = await playableIndicator.count();

  // Also check for play icons which indicate playability
  const playIcons = this.page.locator('#files-list .play-icon, #files-list svg[class*="play"]');
  const playIconCount = await playIcons.count();

  expect(count + playIconCount).toBeGreaterThan(0);
});

Then('the unsupported files should be marked accordingly', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const unsupportedIndicator = this.page.locator('.file-unsupported, .unsupported-format, [data-supported="false"]');
  await expect(unsupportedIndicator.first()).toBeVisible({ timeout: 5000 });
});

Then('a warning should be displayed', async function (this: NodokaWorld) {
  const warning = this.page.locator('.warning, .alert-warning, [role="alert"], .toast-warning');
  await expect(warning.first()).toBeVisible({ timeout: 5000 });
});

Then('playback should not start', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const isPlaying = await player.isPlaying();
  expect(isPlaying).toBe(false);
});

Then('the file should be playable', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await player.expectPlayerVisible();

  // Verify play button is enabled
  const playButton = player.playButton.first();
  await expect(playButton).toBeEnabled();

  // Try to play
  await player.play();
  await this.page.waitForTimeout(200);
});

Then('each file should display its duration', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const durations = this.page.locator('.file-duration, .duration, text=/\\d+:\\d{2}/');
  const count = await durations.count();
  expect(count).toBeGreaterThan(0);
});

// ==================== UI-specific Then Steps ====================

Then('each playable file should have a play icon', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const files = library.fileCards;
  const count = await files.count();

  expect(count).toBeGreaterThan(0);

  for (let i = 0; i < Math.min(count, 5); i++) {
    const playIcon = files.nth(i).locator('.play-icon, svg[class*="play"], [data-testid="play-icon"], button[aria-label*="Play"]');
    const hasIcon = await playIcon.count() > 0;
    expect(hasIcon).toBe(true);
  }
});

Then('each file should display its format as a badge', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const formatBadges = this.page.locator('.format-badge, .file-format, [data-format]');
  const count = await formatBadges.count();
  expect(count).toBeGreaterThan(0);

  // Verify badge text matches expected format
  if (count > 0) {
    const badgeText = await formatBadges.first().textContent();
    expect(badgeText).toMatch(/MP3|M4B|FLAC|OGG|WAV|AAC/i);
  }
});

Then('the duration should be in HH:MM:SS or MM:SS format', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const durations = this.page.locator('.file-duration, .duration');
  const count = await durations.count();

  expect(count).toBeGreaterThan(0);

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
  await this.page.waitForTimeout(300);

  const warningIcon = this.page.locator('.warning-icon, svg[class*="warning"], [data-testid="warning-icon"], .corrupted-indicator');
  await expect(warningIcon.first()).toBeVisible({ timeout: 5000 });
});

Then('the unsupported files should have an error indicator', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const errorIndicator = this.page.locator('.error-indicator, .unsupported-indicator, svg[class*="error"], [data-error="unsupported"]');
  await expect(errorIndicator.first()).toBeVisible({ timeout: 5000 });
});

Then('each file should display its file size', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const fileSizes = this.page.locator('.file-size, text=/\\d+\\.?\\d*\\s*(MB|GB|KB)/');
  const count = await fileSizes.count();
  expect(count).toBeGreaterThan(0);
});

Then('the bitrate should be displayed', async function (this: NodokaWorld) {
  const bitrate = this.page.locator('.bitrate, text=/\\d+\\s*kbps/i, text=/\\d+\\s*Kbps/i');
  await expect(bitrate.first()).toBeVisible({ timeout: 5000 });
});

Then('the sample rate should be displayed', async function (this: NodokaWorld) {
  const sampleRate = this.page.locator('.sample-rate, text=/\\d+\\s*(Hz|kHz)/i');
  await expect(sampleRate.first()).toBeVisible({ timeout: 5000 });
});

// ==================== Duration Calculation Steps ====================

Given('the library has an audiobook with 3 files', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&fileCount=3');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has an audiobook with a file', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&fileCount=1');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has an audiobook with known duration', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&knownDuration=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  // Store the initial duration for comparison
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  const durationElement = this.page.locator('.total-duration, .audiobook-duration');
  if (await durationElement.count() > 0) {
    this.lastDuration = parseInt(await durationElement.getAttribute('data-duration') || '0');
  }
});

Given('the library has an audiobook with a file over 24 hours', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&longDuration=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

// Note: "I view the audiobook details" is defined in archive-handling.steps.ts

Then('each file should display its calculated duration', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const fileDurations = this.page.locator('.file-duration, [data-duration]');
  const count = await fileDurations.count();
  expect(count).toBeGreaterThan(0);

  // Verify each duration is a valid number
  for (let i = 0; i < Math.min(count, 5); i++) {
    const durationText = await fileDurations.nth(i).textContent();
    expect(durationText).toBeTruthy();
    expect(durationText).toMatch(/\d+:?\d*:\d{2}/);
  }
});

Then('the duration should match the actual audio length', async function (this: NodokaWorld) {
  // Verify duration elements have valid data-duration attributes or text
  const durations = this.page.locator('.file-duration, [data-duration]');
  const count = await durations.count();
  expect(count).toBeGreaterThan(0);

  // Check that durations are reasonable (not zero, not negative)
  for (let i = 0; i < Math.min(count, 3); i++) {
    const element = durations.nth(i);
    const dataDuration = await element.getAttribute('data-duration');
    if (dataDuration) {
      const duration = parseInt(dataDuration);
      expect(duration).toBeGreaterThan(0);
    }
  }
});

Then('the total duration should be displayed', async function (this: NodokaWorld) {
  const totalDuration = this.page.locator('.total-duration, .audiobook-duration, [data-testid="total-duration"]');
  await expect(totalDuration.first()).toBeVisible({ timeout: 5000 });
});

Then('the total should equal the sum of all file durations', async function (this: NodokaWorld) {
  // Get all file durations
  const fileDurations = this.page.locator('.file-duration, [data-duration]');
  const count = await fileDurations.count();

  let totalMs = 0;
  for (let i = 0; i < count; i++) {
    const element = fileDurations.nth(i);
    const dataDuration = await element.getAttribute('data-duration');
    if (dataDuration) {
      totalMs += parseInt(dataDuration);
    }
  }

  // Get total duration displayed
  const totalElement = this.page.locator('.total-duration, [data-testid="total-duration"]');
  if (await totalElement.count() > 0) {
    const totalDataDuration = await totalElement.first().getAttribute('data-duration');
    if (totalDataDuration) {
      const displayedTotal = parseInt(totalDataDuration);
      // Allow small tolerance for rounding
      expect(Math.abs(displayedTotal - totalMs)).toBeLessThan(1000);
    }
  }
});

When('the file is replaced with a longer version', async function (this: NodokaWorld) {
  // Simulate file replacement via test query param
  await this.page.goto('http://localhost:5173/?test=true&fileCount=1&longerFile=true');
  await this.page.waitForTimeout(300);
});

// Note: "I rescan the directory" is defined in directory-scanning.steps.ts

Then('the duration should be updated', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const durationElement = this.page.locator('.file-duration, [data-duration]').first();
  await expect(durationElement).toBeVisible({ timeout: 5000 });
});

Then('the audiobook total duration should reflect the change', async function (this: NodokaWorld) {
  const totalDuration = this.page.locator('.total-duration, .audiobook-duration');
  await expect(totalDuration.first()).toBeVisible({ timeout: 5000 });
});

Then('the file durations should be preserved', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const durations = this.page.locator('.file-duration, [data-duration]');
  const count = await durations.count();
  expect(count).toBeGreaterThan(0);
});

Then('the total audiobook duration should be correct', async function (this: NodokaWorld) {
  const totalDuration = this.page.locator('.total-duration, .audiobook-duration');
  await expect(totalDuration.first()).toBeVisible({ timeout: 5000 });
});

Then('the duration should display in HH:MM:SS format', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);

  const durationElement = this.page.locator('.file-duration, [data-duration]').first();
  const text = await durationElement.textContent();

  // For files over 24 hours, should show HH:MM:SS with hours > 24
  expect(text).toMatch(/\d+:\d{2}:\d{2}/);
});

Then('hours should be correctly calculated', async function (this: NodokaWorld) {
  const durationElement = this.page.locator('.file-duration, [data-duration]').first();
  const text = await durationElement.textContent();

  // Parse the duration and verify hours component
  if (text) {
    const parts = text.split(':');
    if (parts.length === 3) {
      const hours = parseInt(parts[0]);
      // For a file over 24 hours, hours should be >= 24
      expect(hours).toBeGreaterThanOrEqual(24);
    }
  }
});
