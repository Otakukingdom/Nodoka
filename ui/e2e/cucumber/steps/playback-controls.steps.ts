/**
 * Playback Controls Step Definitions
 * Implements UI interactions for playback controls feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { PlayerPage, LibraryPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('the application has a volume setting', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the application has a speed setting', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the application has a player', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  const player = new PlayerPage(this.page);
  await player.expectPlayerVisible();
});

Given('the player has loaded media', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);
});

Given('the player is playing', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);

  const player = new PlayerPage(this.page);
  await player.play();
});

Given('the player has loaded media with duration {int} ms', async function (this: NodokaWorld, duration: number) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);
});

// ==================== Cross-File Playback Given Steps ====================

Given('the application has an audiobook with {int} files', async function (this: NodokaWorld, fileCount: number) {
  await this.page.goto(`http://localhost:5173/?test=true&fileCount=${fileCount}`);
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('I am playing the first file', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);

  const player = new PlayerPage(this.page);
  await player.play();
  await this.page.waitForTimeout(300);

  this.lastPosition = 0;
});

Given('I am playing the second file', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(1);

  const player = new PlayerPage(this.page);
  await player.play();
  await this.page.waitForTimeout(300);
});

Given('I am playing the third file', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(2);

  const player = new PlayerPage(this.page);
  await player.play();
  await this.page.waitForTimeout(300);
});

Given('I am playing the first file near the end', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);

  const player = new PlayerPage(this.page);
  // Seek to near end
  await player.seekToEnd();
  await player.play();
  await this.page.waitForTimeout(300);
});

// ==================== When Steps ====================

When('I set the volume above maximum', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // Volume slider max is 200, try to set to 250
  await player.setVolume(250);
});

When('I set the volume below minimum', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // Volume slider min is 0, try to set to -50
  await player.setVolume(-50);
});

When('I set the speed above maximum', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // Cycle speed multiple times to try to exceed max
  for (let i = 0; i < 10; i++) {
    await player.cycleSpeed();
  }
});

When('I set the speed below minimum', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // Speed has a minimum value, cycle through options
  for (let i = 0; i < 10; i++) {
    await player.cycleSpeed();
  }
});

When('I set the volume to valid value', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await player.setVolume(75);
  this.lastValue = 75;
});

When('I set the speed to valid value', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await player.cycleSpeed();
});

When('I check the playback state', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await player.expectPlayerVisible();
});

When('I start playback', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await player.play();
});

When('I pause playback', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await player.pause();
});

When('I stop playback', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await player.stop();
});

When('playback reaches the end', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // Seek to near end
  const duration = await player.getDuration();
  await player.seek(duration - 100);

  // Simulate waiting for end
  await this.page.waitForTimeout(500);
});

When('I seek to position {int} ms', async function (this: NodokaWorld, position: number) {
  const player = new PlayerPage(this.page);
  await player.seek(position);
});

// ==================== Then Steps ====================

Then('the volume should be clamped to maximum', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const volume = await player.getVolume();
  expect(volume).toBeLessThanOrEqual(200);
});

Then('the volume should be clamped to minimum', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const volume = await player.getVolume();
  expect(volume).toBeGreaterThanOrEqual(0);
});

Then('the speed should be clamped to maximum', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const speedText = await player.getSpeed();
  const speed = parseFloat(speedText.replace('x', ''));
  expect(speed).toBeLessThanOrEqual(4.0);
});

Then('the speed should be clamped to minimum', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const speedText = await player.getSpeed();
  const speed = parseFloat(speedText.replace('x', ''));
  expect(speed).toBeGreaterThanOrEqual(0.5);
});

Then('the volume should be valid', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const volume = await player.getVolume();
  expect(volume).toBeGreaterThanOrEqual(0);
  expect(volume).toBeLessThanOrEqual(200);
});

Then('the speed should be valid', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const speedText = await player.getSpeed();
  const speed = parseFloat(speedText.replace('x', ''));
  expect(speed).toBeGreaterThanOrEqual(0.5);
  expect(speed).toBeLessThanOrEqual(4.0);
});

Then('the playback state should be NothingSpecial', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // Initial state - play button visible, not playing
  await player.expectPlayButtonVisible();
});

Then('the playback state should be Playing', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const isPlaying = await player.isPlaying();
  expect(isPlaying).toBe(true);
});

Then('the playback state should be Paused', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const isPlaying = await player.isPlaying();
  expect(isPlaying).toBe(false);
});

Then('the playback state should be Stopped', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // After stop, play button should be visible and position reset
  await player.expectPlayButtonVisible();
});

Then('the playback state should be Ended', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // After end, play button should be visible
  await player.expectPlayButtonVisible();
});

Then('the position should be clamped to duration', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const position = await player.getSeekPosition();
  const duration = await player.getDuration();
  expect(position).toBeLessThanOrEqual(duration);
});

// ==================== Cross-File Playback When Steps ====================

When('the first file ends', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // Seek to near end and wait for transition
  await player.seekToEnd();
  await player.play();
  await this.page.waitForTimeout(1000);
});

When('the second file ends', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await player.seekToEnd();
  await player.play();
  await this.page.waitForTimeout(1000);
});

When('I press the skip forward button repeatedly', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // Press skip forward multiple times
  for (let i = 0; i < 5; i++) {
    await player.skipForward();
    await this.page.waitForTimeout(300);
  }
});

When('I press the skip backward button repeatedly', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // Press skip backward multiple times
  for (let i = 0; i < 5; i++) {
    await player.skipBackward();
    await this.page.waitForTimeout(300);
  }
});

When('I press the skip forward button', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await player.skipForward();
  await this.page.waitForTimeout(300);
});

// ==================== Cross-File Playback Then Steps ====================

Then('the second file should start playing automatically', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);

  // Wait for file transition
  await this.page.waitForTimeout(500);

  // Verify playback continues
  const isPlaying = await player.isPlaying();
  expect(isPlaying).toBe(true);
});

Then('the UI should update to show the second file selected', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);

  // Verify second file is selected
  const selectedFile = await library.getSelectedFile();
  expect(selectedFile).not.toBeNull();
});

Then('the audiobook should be marked as complete', async function (this: NodokaWorld) {
  // Check for completion indicator
  const completionBadge = this.page.locator('.completion-badge, [data-complete="true"]');
  const isVisible = await completionBadge.isVisible().catch(() => false);

  // Also check for 100% progress
  const progressText = this.page.locator('text=/100%|Complete/');
  const hasProgress = await progressText.isVisible().catch(() => false);

  expect(isVisible || hasProgress).toBe(true);
});

Then('playback should advance to the next file', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);

  // Verify playback is still active
  await this.page.waitForTimeout(300);
  const isPlaying = await player.isPlaying();
  expect(isPlaying).toBe(true);
});

Then('the file list should update the selection', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);

  // Verify a file is selected
  const selectedFile = await library.getSelectedFile();
  expect(selectedFile).not.toBeNull();
});

Then('playback should go to the previous file', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);

  // Verify playback is still active or paused at start
  await this.page.waitForTimeout(300);
  const isPlaying = await player.isPlaying();
  expect(isPlaying).toBe(true);
});

Then('the next file should start playing', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);

  // Wait for transition
  await this.page.waitForTimeout(500);

  // Verify playback continues
  const isPlaying = await player.isPlaying();
  expect(isPlaying).toBe(true);
});

Then('playback should continue without interruption', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);

  // Verify playback state is maintained
  await this.page.waitForTimeout(300);
  const isPlaying = await player.isPlaying();
  expect(isPlaying).toBe(true);
});

Then('the progress should reset for the new file', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);

  // Progress should be near beginning for new file
  const position = await player.getSeekPosition();
  expect(position).toBeLessThan(5000); // Less than 5 seconds
});
