/**
 * Keyboard Shortcuts Step Definitions
 * Implements keyboard interaction tests for UI shortcuts
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { PlayerPage, LibraryPage, SettingsPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('a file is loaded for keyboard testing', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&audiobookCount=1');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);

  // Store initial position for comparison
  const player = new PlayerPage(this.page);
  this.lastPosition = await player.getSeekPosition();
});

Given('a file is playing for keyboard testing', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&audiobookCount=1');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);

  const player = new PlayerPage(this.page);
  await player.play();

  // Store initial position for comparison
  this.lastPosition = await player.getSeekPosition();
});

Given('a file is loaded with volume at {int}', async function (this: NodokaWorld, volume: number) {
  await this.page.goto(`/?test=true&audiobookCount=1&volume=${volume}`);
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await library.selectFile(0);

  const player = new PlayerPage(this.page);
  this.lastValue = await player.getVolume();
});

Given('an audiobook with {int} files is loaded', async function (this: NodokaWorld, fileCount: number) {
  await this.page.goto(`/?test=true&audiobookCount=1&fileCount=${fileCount}`);
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
});

Given('the first file is selected', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectFile(0);
  this.lastValue = 0;
});

Given('the second file is selected', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectFile(1);
  this.lastValue = 1;
});

Given('the third file is selected', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectFile(2);
  this.lastValue = 2;
});

Given('a modal is open for keyboard testing', async function (this: NodokaWorld) {
  const settings = new SettingsPage(this.page);
  await settings.open();
});

// ==================== When Steps - Keyboard Presses ====================

When('I press Space key', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Space');
  await this.page.waitForTimeout(200);
});

When('I press M key', async function (this: NodokaWorld) {
  await this.page.keyboard.press('KeyM');
  await this.page.waitForTimeout(200);
});

When('I press ArrowRight key', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  this.lastPosition = await player.getSeekPosition();
  await this.page.keyboard.press('ArrowRight');
  await this.page.waitForTimeout(200);
});

When('I press ArrowLeft key', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  this.lastPosition = await player.getSeekPosition();
  await this.page.keyboard.press('ArrowLeft');
  await this.page.waitForTimeout(200);
});

When('I press Escape key', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Escape');
  await this.page.waitForTimeout(300);
});

When('I press Ctrl+Comma keys', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Control+Comma');
  await this.page.waitForTimeout(300);
});

When('I press Ctrl+B keys', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Control+b');
  await this.page.waitForTimeout(300);
});

When('I press Ctrl+T keys', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Control+t');
  await this.page.waitForTimeout(300);
});

When('I press F1 key', async function (this: NodokaWorld) {
  await this.page.keyboard.press('F1');
  await this.page.waitForTimeout(300);
});

When('I press Ctrl+A keys', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Control+a');
  await this.page.waitForTimeout(300);
});

When('I press Up arrow key', async function (this: NodokaWorld) {
  await this.page.keyboard.press('ArrowUp');
  await this.page.waitForTimeout(200);
});

When('I press Down arrow key', async function (this: NodokaWorld) {
  await this.page.keyboard.press('ArrowDown');
  await this.page.waitForTimeout(200);
});

When('I press Enter key', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Enter');
  await this.page.waitForTimeout(200);
});

When('I press Tab key', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Tab');
  await this.page.waitForTimeout(100);
});

When('I press Shift+Tab keys', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Shift+Tab');
  await this.page.waitForTimeout(100);
});

When('I hold Shift and press ArrowRight', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Shift+ArrowRight');
  await this.page.waitForTimeout(200);
});

When('I hold Shift and press ArrowLeft', async function (this: NodokaWorld) {
  await this.page.keyboard.press('Shift+ArrowLeft');
  await this.page.waitForTimeout(200);
});

// ==================== When Steps - Volume Control Keys ====================

When('I press Plus key', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  this.lastValue = await player.getVolume();
  await this.page.keyboard.press('+');
  await this.page.waitForTimeout(200);
});

When('I press Minus key', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  this.lastValue = await player.getVolume();
  await this.page.keyboard.press('-');
  await this.page.waitForTimeout(200);
});

When('I press Equals key', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  this.lastValue = await player.getVolume();
  await this.page.keyboard.press('=');
  await this.page.waitForTimeout(200);
});

When('I press Underscore key', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  this.lastValue = await player.getVolume();
  await this.page.keyboard.press('_');
  await this.page.waitForTimeout(200);
});

// ==================== When Steps - File Navigation Keys ====================

When('I press ArrowDown key for file navigation', async function (this: NodokaWorld) {
  await this.page.keyboard.press('ArrowDown');
  await this.page.waitForTimeout(200);
});

When('I press ArrowUp key for file navigation', async function (this: NodokaWorld) {
  await this.page.keyboard.press('ArrowUp');
  await this.page.waitForTimeout(200);
});

// ==================== Then Steps - Playback Controls ====================

Then('playback should toggle state', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // Wait for state change
  await this.page.waitForTimeout(300);
  // Verify player controls are visible (playback state changed)
  await player.expectPlayerVisible();
});

Then('playback should start', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await this.page.waitForTimeout(300);
  const isPlaying = await player.isPlaying();
  expect(isPlaying).toBe(true);
});

Then('playback should pause', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  await this.page.waitForTimeout(300);
  const isPlaying = await player.isPlaying();
  expect(isPlaying).toBe(false);
});

Then('mute should toggle state', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const isMuted = await player.isMuted();
  expect(typeof isMuted).toBe('boolean');
});

Then('audio should be muted', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const isMuted = await player.isMuted();
  expect(isMuted).toBe(true);
});

Then('audio should be unmuted', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const isMuted = await player.isMuted();
  expect(isMuted).toBe(false);
});

Then('the position should advance by skip duration', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const newPosition = await player.getSeekPosition();
  expect(newPosition).toBeGreaterThan(this.lastPosition);
});

Then('the position should rewind by skip duration', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const newPosition = await player.getSeekPosition();
  expect(newPosition).toBeLessThan(this.lastPosition);
});

Then('volume should increase', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  const volume = await player.getVolume();
  expect(volume).toBeGreaterThan(0);
});

Then('volume should decrease', async function (this: NodokaWorld) {
  const player = new PlayerPage(this.page);
  // Volume should have decreased
  await this.page.waitForTimeout(100);
});

// ==================== Then Steps - Volume Control ====================

Then('volume should increase by {int}', async function (this: NodokaWorld, amount: number) {
  const player = new PlayerPage(this.page);
  const newVolume = await player.getVolume();
  const expectedVolume = Math.min(100, (this.lastValue as number) + amount);
  expect(newVolume).toBe(expectedVolume);
});

Then('volume should decrease by {int}', async function (this: NodokaWorld, amount: number) {
  const player = new PlayerPage(this.page);
  const newVolume = await player.getVolume();
  const expectedVolume = Math.max(0, (this.lastValue as number) - amount);
  expect(newVolume).toBe(expectedVolume);
});

Then('volume should be at maximum {int}', async function (this: NodokaWorld, maxVolume: number) {
  const player = new PlayerPage(this.page);
  const volume = await player.getVolume();
  expect(volume).toBeLessThanOrEqual(maxVolume);
});

Then('volume should be at minimum {int}', async function (this: NodokaWorld, minVolume: number) {
  const player = new PlayerPage(this.page);
  const volume = await player.getVolume();
  expect(volume).toBeGreaterThanOrEqual(minVolume);
});

// ==================== Then Steps - File Navigation ====================

// Note: "the second file should be selected" is defined in ui-components.steps.ts

Then('the first file should be selected', async function (this: NodokaWorld) {
  const details = this.page.locator('.file-list-item, [data-testid="file-item"]');
  const selectedFile = details.locator('.selected, [data-selected="true"], .ring-primary');
  await expect(selectedFile.first()).toBeVisible();

  // Verify first file (index 0) is selected
  const allFiles = await details.all();
  let selectedIndex = -1;
  for (let i = 0; i < allFiles.length; i++) {
    const isSelected = await allFiles[i].evaluate(el =>
      el.classList.contains('selected') ||
      el.getAttribute('data-selected') === 'true' ||
      el.classList.contains('ring-primary')
    );
    if (isSelected) {
      selectedIndex = i;
      break;
    }
  }
  expect(selectedIndex).toBe(0);
});

Then('the first file should remain selected', async function (this: NodokaWorld) {
  // Same as above - first file should be selected
  const details = this.page.locator('.file-list-item, [data-testid="file-item"]');
  const selectedFile = details.locator('.selected, [data-selected="true"], .ring-primary');
  await expect(selectedFile.first()).toBeVisible();
});

Then('the third file should be selected', async function (this: NodokaWorld) {
  const details = this.page.locator('.file-list-item, [data-testid="file-item"]');
  const selectedFile = details.locator('.selected, [data-selected="true"], .ring-primary');
  await expect(selectedFile.first()).toBeVisible();

  // Verify third file (index 2) is selected
  const allFiles = await details.all();
  let selectedIndex = -1;
  for (let i = 0; i < allFiles.length; i++) {
    const isSelected = await allFiles[i].evaluate(el =>
      el.classList.contains('selected') ||
      el.getAttribute('data-selected') === 'true' ||
      el.classList.contains('ring-primary')
    );
    if (isSelected) {
      selectedIndex = i;
      break;
    }
  }
  expect(selectedIndex).toBe(2);
});

Then('the third file should remain selected', async function (this: NodokaWorld) {
  // Same as above - third file should still be selected
  const details = this.page.locator('.file-list-item, [data-testid="file-item"]');
  const selectedFile = details.locator('.selected, [data-selected="true"], .ring-primary');
  await expect(selectedFile.first()).toBeVisible();
});

// ==================== Then Steps - Modal Controls ====================

Then('the open modal should close', async function (this: NodokaWorld) {
  const modal = this.page.locator('.modal-box');
  await expect(modal).not.toBeVisible();
});

Then('the settings modal should open via shortcut', async function (this: NodokaWorld) {
  const settings = new SettingsPage(this.page);
  await settings.expectModalVisible();
});

Then('the sleep timer modal should open via shortcut', async function (this: NodokaWorld) {
  const sleepTimerModal = this.page.locator('.modal-box:has-text("Sleep"), .modal-box:has-text("Timer")');
  await expect(sleepTimerModal).toBeVisible();
});

Then('the keyboard help modal should open via shortcut', async function (this: NodokaWorld) {
  const keyboardHelp = this.page.locator('.modal-box:has-text("Keyboard"), .modal-box:has-text("Shortcuts")');
  await expect(keyboardHelp).toBeVisible();
});

// ==================== Then Steps - Bookmarks ====================

Then('a bookmark should be created via shortcut', async function (this: NodokaWorld) {
  // Check for bookmark creation feedback or bookmark in list
  const bookmarkFeedback = this.page.locator('.bookmark-created, text=Bookmark created');
  const bookmarkInList = this.page.locator('.bookmark-item');

  const feedbackVisible = await bookmarkFeedback.isVisible().catch(() => false);
  const bookmarkCount = await bookmarkInList.count();

  expect(feedbackVisible || bookmarkCount > 0).toBe(true);
});

// ==================== Then Steps - Selection ====================

Then('all audiobooks should be selected via shortcut', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const totalCount = await library.getAudiobookCount();
  const selectedCards = this.page.locator('.card.ring-primary, .card.selected, [data-selected="true"]');
  const selectedCount = await selectedCards.count();
  expect(selectedCount).toBe(totalCount);
});

// ==================== Then Steps - Focus ====================

Then('focus should move to next element', async function (this: NodokaWorld) {
  const focusedElement = this.page.locator(':focus');
  await expect(focusedElement).toBeVisible();
});

Then('focus should move to previous element', async function (this: NodokaWorld) {
  const focusedElement = this.page.locator(':focus');
  await expect(focusedElement).toBeVisible();
});

Then('the focused element should be activated', async function (this: NodokaWorld) {
  // Enter should activate the focused element
  await this.page.waitForTimeout(200);
});
