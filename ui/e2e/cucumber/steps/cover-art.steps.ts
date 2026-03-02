/**
 * Cover Art Step Definitions
 * Implements UI interactions for cover art feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { LibraryPage, DetailsPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('an audiobook directory without embedded cover art', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&noEmbeddedCover=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the directory contains folder.jpg', async function (this: NodokaWorld) {
  // Navigate with query param to indicate folder.jpg exists
  await this.page.goto('http://localhost:5173/?test=true&coverFile=folder.jpg');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the directory contains cover.jpg', async function (this: NodokaWorld) {
  // Navigate with query param to indicate cover.jpg exists
  await this.page.goto('http://localhost:5173/?test=true&coverFile=cover.jpg');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the directory contains no folder.jpg', async function (this: NodokaWorld) {
  // No folder.jpg in test data
});

Given('the directory contains no folder.jpg or cover.jpg', async function (this: NodokaWorld) {
  // No folder.jpg or cover.jpg in test data
});

Given('the directory contains cover.png', async function (this: NodokaWorld) {
  // Navigate with query param to indicate cover.png exists
  await this.page.goto('http://localhost:5173/?test=true&coverFile=cover.png');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('an audiobook directory without any cover art', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&noCover=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('an audiobook directory with corrupted folder.jpg', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&corruptedCover=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

Given('the directory contains valid cover.png', async function (this: NodokaWorld) {
  // Navigate with query param to indicate valid cover.png exists
  await this.page.goto('http://localhost:5173/?test=true&coverFile=cover.png&coverValid=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

// ==================== When Steps ====================

When('I select cover art', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);
});

// ==================== Then Steps ====================

Then('folder.jpg should be selected', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const coverSrc = await this.page.locator('.cover-art img').getAttribute('src');
  expect(coverSrc).toContain('folder.jpg');
});

Then('cover.jpg should be selected', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const coverSrc = await this.page.locator('.cover-art img').getAttribute('src');
  expect(coverSrc).toContain('cover.jpg');
});

Then('cover.png should be selected', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const coverSrc = await this.page.locator('.cover-art img').getAttribute('src');
  expect(coverSrc).toContain('cover.png');
});

Then('no cover art should be selected', async function (this: NodokaWorld) {
  const placeholder = this.page.locator('.cover-art .placeholder');
  await expect(placeholder).toBeVisible();
});
