/**
 * Archive Handling Step Definitions
 * Implements UI interactions for archive handling feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { LibraryPage, PlayerPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('the library has a ZIP archive audiobook', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=zip');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a RAR archive audiobook', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=rar');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a 7z archive audiobook', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=7z');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a nested archive', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=nested');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a ZIP archive with multiple audiobooks', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=zip&multiBook=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a corrupted archive', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=corrupted');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the library has a password protected archive', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&archiveType=password');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

// ==================== When Steps ====================

When('I scan the archive', async function (this: NodokaWorld) {
  // Trigger a library scan which will process archives
  const settingsButton = this.page.locator('button[aria-label="Settings"]');
  await settingsButton.click();
  await this.page.waitForTimeout(300);

  // Click rescan button
  const rescanButton = this.page.locator('button:has-text("Rescan"), button:has-text("Scan"), button:has-text("Rescan All")').first();
  if (await rescanButton.isVisible()) {
    await rescanButton.click();
    await this.page.waitForTimeout(500);
  }

  // Close settings modal
  const closeButton = this.page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  }
  await this.page.waitForTimeout(500);
});

When('I try to select the audiobook', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  if (count > 0) {
    await library.selectAudiobook(0);
  }
});

When('I force a full rescan', async function (this: NodokaWorld) {
  // Open settings and trigger full rescan
  const settingsButton = this.page.locator('button[aria-label="Settings"]');
  await settingsButton.click();
  await this.page.waitForTimeout(300);

  const rescanButton = this.page.locator('button:has-text("Rescan All")');
  if (await rescanButton.isVisible()) {
    await rescanButton.click();
  }

  const closeButton = this.page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  }
  await this.page.waitForTimeout(500);
});

// ==================== Then Steps ====================

Then('the files should be listed from the archive', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const fileCount = await library.getFileCount();
  expect(fileCount).toBeGreaterThan(0);
});

Then('I can play files from the archive', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectFile(0);

  const player = new PlayerPage(this.page);
  await player.expectPlayerVisible();
});

Then('the innermost files should be accessible', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const fileCount = await library.getFileCount();
  expect(fileCount).toBeGreaterThan(0);
});

Then('each audiobook should be listed separately', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();
  expect(count).toBeGreaterThan(1);
});

Then('a password prompt should appear', async function (this: NodokaWorld) {
  const passwordPrompt = this.page.locator('.password-prompt, input[type="password"]');
  await expect(passwordPrompt).toBeVisible();
});
