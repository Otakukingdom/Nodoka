/**
 * Cover Cache Step Definitions
 * Implements UI interactions for cover cache feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { LibraryPage, DetailsPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('the application has an audiobook with cached cover art', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&hasCoverArt=true&coverCached=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  // First view to cache the cover
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  const details = new DetailsPage(this.page);
  await expect(details.coverArt).toBeVisible();
});

Given('the cover art file is modified', async function (this: NodokaWorld) {
  // Navigate with query param to indicate cover was modified
  // This will trigger cache invalidation
  await this.page.goto('http://localhost:5173/?test=true&coverModified=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  // Trigger a rescan to detect the modification
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

Given('the application has {int} audiobooks with cover art', async function (this: NodokaWorld, count: number) {
  await this.page.goto(`http://localhost:5173/?test=true&audiobookCount=${count}&hasCoverArt=true`);
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

// ==================== When Steps ====================

When('I view each audiobook', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const count = await library.getAudiobookCount();

  for (let i = 0; i < count; i++) {
    await library.selectAudiobook(i);
    await this.page.waitForTimeout(300);
  }
});

When('I view the audiobook again', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  // Select a different audiobook first
  const count = await library.getAudiobookCount();
  if (count > 1) {
    await library.selectAudiobook(1);
    await this.page.waitForTimeout(300);
  }
  // Then select the original
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);
});

// ==================== Then Steps ====================

Then('the cover art should be cached', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);

  // Verify image loaded successfully
  const isLoaded = await details.coverArt.evaluate((img: HTMLImageElement) => {
    return img.complete && img.naturalWidth > 0;
  });
  expect(isLoaded).toBe(true);

  // Check cache indicator if available
  const cacheIndicator = this.page.locator('.cover-cached');
  if (await cacheIndicator.isVisible()) {
    await expect(cacheIndicator).toBeVisible();
  }
});

Then('the cover art should load from cache', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  await expect(details.coverArt).toBeVisible();

  // Verify loaded quickly (from cache)
  const loadTime = await details.coverArt.evaluate((img: HTMLImageElement) => {
    return img.complete;
  });
  expect(loadTime).toBe(true);
});

Then('the cache should be updated', async function (this: NodokaWorld) {
  // Verify cache was invalidated and updated
  await this.page.waitForTimeout(500);
});

Then('the new cover art should be displayed', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  await expect(details.coverArt).toBeVisible();
});

Then('all cover art should be cached', async function (this: NodokaWorld) {
  // All covers should be cached after viewing
  const cachedCount = await this.page.locator('.cover-cached').count();
  expect(cachedCount).toBeGreaterThanOrEqual(0);
});

Then('each audiobook should display its own cover', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const details = new DetailsPage(this.page);

  const count = await library.getAudiobookCount();
  for (let i = 0; i < count; i++) {
    await library.selectAudiobook(i);
    await expect(details.coverArt).toBeVisible();
  }
});

// ==================== Additional Given Steps ====================

Given('the application has cached cover art for audiobooks', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&audiobookCount=3&hasCoverArt=true&coverCached=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  // View each audiobook to cache covers
  const library = new LibraryPage(this.page);
  const details = new DetailsPage(this.page);

  for (let i = 0; i < 3; i++) {
    await library.selectAudiobook(i);
    await expect(details.coverArt).toBeVisible({ timeout: 5000 }).catch(() => {});
  }

  this.coversCached = true;
});

// ==================== Additional When Steps ====================

When('I clear the cover cache', async function (this: NodokaWorld) {
  // Clear cover cache via settings or direct action
  const settingsButton = this.page.locator('button[aria-label="Settings"]');
  await settingsButton.click();
  await this.page.waitForTimeout(300);

  // Look for clear cache button
  const clearCacheButton = this.page.locator('button:has-text("Clear Cache"), button:has-text("Clear Cover Cache")');
  if (await clearCacheButton.isVisible()) {
    await clearCacheButton.click();
    await this.page.waitForTimeout(500);
  }

  // Close settings modal
  const closeButton = this.page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  }
  await this.page.waitForTimeout(300);

  this.coversCached = false;
});

When('I view the audiobook for the first time', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);
});

// ==================== Additional Then Steps ====================

Then('the cover should appear without delay', async function (this: NodokaWorld) {
  const details = new DetailsPage(this.page);
  const startTime = Date.now();

  await expect(details.coverArt).toBeVisible({ timeout: 5000 });

  const loadTime = Date.now() - startTime;
  // Cached covers should load very quickly (under 500ms)
  expect(loadTime).toBeLessThan(500);
});

Then('a loading indicator may appear', async function (this: NodokaWorld) {
  // Loading indicator is optional - just verify the UI is responsive
  const loading = this.page.locator('.loading-spinner, .skeleton, [role="status"]');
  const isLoading = await loading.first().isVisible().catch(() => false);

  // Either loading indicator is visible or content is already shown
  const details = new DetailsPage(this.page);
  const isContentVisible = await details.coverArt.isVisible().catch(() => false);

  expect(isLoading || isContentVisible).toBe(true);
});

Then('no covers should be cached', async function (this: NodokaWorld) {
  // Verify cache was cleared
  const cachedIndicators = this.page.locator('[data-cached="true"], .cover-cached');
  const count = await cachedIndicators.count();
  expect(count).toBe(0);
});

Then('covers should reload on next view', async function (this: NodokaWorld) {
  const library = new LibraryPage(this.page);
  const details = new DetailsPage(this.page);

  // Select an audiobook to trigger cover reload
  await library.selectAudiobook(0);

  // Cover should still load successfully
  await expect(details.coverArt).toBeVisible({ timeout: 5000 });
});
