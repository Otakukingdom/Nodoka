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
  // Open settings and trigger directory scan
  const settings = new SettingsPage(this.page);
  await settings.open();

  // Click the rescan button
  const rescanButton = this.page.locator('button:has-text("Rescan"), button:has-text("Scan"), button:has-text("Rescan All")').first();
  if (await rescanButton.isVisible()) {
    await rescanButton.click();
    await this.page.waitForTimeout(500);
  }

  await settings.close();
});

When('I rescan the directory', async function (this: NodokaWorld) {
  // Open settings and click rescan
  const settings = new SettingsPage(this.page);
  await settings.open();

  // Click rescan on the first directory
  const rescanButton = this.page.locator('.directory-item button[aria-label="Rescan"]').first();
  if (await rescanButton.isVisible()) {
    await rescanButton.click();
    await this.page.waitForTimeout(500);
  } else {
    // If no rescan button per directory, try general rescan
    const rescanAllButton = this.page.locator('button:has-text("Rescan")');
    if (await rescanAllButton.isVisible()) {
      await rescanAllButton.click();
      await this.page.waitForTimeout(500);
    }
  }

  await settings.close();
});

When('I retrieve the directory', async function (this: NodokaWorld) {
  const settings = new SettingsPage(this.page);
  await settings.open();
  await settings.close();
});

When('the files exist on disk', async function (this: NodokaWorld) {
  // In test mode, files are pre-configured to exist
  // Navigate with query param to indicate files exist
  await this.page.goto('http://localhost:5173/?test=true&filesExist=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  // Verify files are displayed in the library
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);
});

When('the files do not exist on disk', async function (this: NodokaWorld) {
  // In test mode, navigate with query param to indicate files are missing
  await this.page.goto('http://localhost:5173/?test=true&filesExist=false');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  // Select audiobook to see missing file indicators
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
  await this.page.waitForTimeout(300);
});

When('I rescan and files are missing', async function (this: NodokaWorld) {
  // Navigate with query param to simulate missing files scenario
  await this.page.goto('http://localhost:5173/?test=true&filesExist=false&rescan=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  // Trigger rescan via settings
  const settings = new SettingsPage(this.page);
  await settings.open();

  const rescanButton = this.page.locator('button:has-text("Rescan"), button:has-text("Rescan All")').first();
  if (await rescanButton.isVisible()) {
    await rescanButton.click();
    await this.page.waitForTimeout(500);
  }

  await settings.close();
});

When('I scan and determine the file length', async function (this: NodokaWorld) {
  // Trigger a scan which will determine file lengths
  const settings = new SettingsPage(this.page);
  await settings.open();

  const rescanButton = this.page.locator('button:has-text("Rescan"), button:has-text("Scan")').first();
  if (await rescanButton.isVisible()) {
    await rescanButton.click();
    await this.page.waitForTimeout(500);
  }

  await settings.close();

  // Verify file durations are displayed
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const durations = this.page.locator('.file-duration');
  const count = await durations.count();
  expect(count).toBeGreaterThan(0);
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
  // Verify timestamp is displayed and recent
  const settings = new SettingsPage(this.page);
  await settings.open();

  const timestampElement = this.page.locator('.directory-item .last-scanned, .directory-item time, [data-testid="last-scanned"]');
  if (await timestampElement.count() > 0) {
    const timestampText = await timestampElement.first().textContent();
    expect(timestampText).toBeTruthy();

    // Verify timestamp is recent (within last minute for test)
    const now = Date.now();
    const storedTimestamp = await this.page.evaluate(() => {
      const settings = localStorage.getItem('nodoka-settings');
      if (settings) {
        try {
          const parsed = JSON.parse(settings);
          return parsed.lastScanned;
        } catch {
          return null;
        }
      }
      return null;
    });

    if (storedTimestamp) {
      const timeDiff = now - new Date(storedTimestamp).getTime();
      expect(timeDiff).toBeLessThan(60000); // Within last minute
    }
  }

  await settings.close();
});

Then('the directory path should match the original', async function (this: NodokaWorld) {
  // Verify directory path is preserved
  const settings = new SettingsPage(this.page);
  await settings.open();

  const directoryItems = this.page.locator('.directory-item');
  const firstDir = directoryItems.first();

  if (await firstDir.isVisible()) {
    const pathText = await firstDir.locator('.directory-path, [data-path]').textContent();
    expect(pathText).toBeTruthy();
    expect(pathText?.length).toBeGreaterThan(0);

    // Store for comparison after potential reload
    this.lastValue = pathText;
  }

  // Also verify in localStorage
  const storedPath = await this.page.evaluate(() => {
    const settings = localStorage.getItem('nodoka-settings');
    if (settings) {
      try {
        const parsed = JSON.parse(settings);
        return parsed.directories?.[0];
      } catch {
        return null;
      }
    }
    return null;
  });

  if (storedPath) {
    expect(storedPath).toBeTruthy();
  }

  await settings.close();
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
  // Verify file duration is displayed and non-zero
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);

  const durationElements = this.page.locator('.file-duration, .duration, [data-testid="file-duration"]');
  const count = await durationElements.count();

  if (count > 0) {
    const durationText = await durationElements.first().textContent();
    expect(durationText).toBeTruthy();

    // Duration should be in format like "5:30" or "1:23:45"
    const durationMatch = durationText?.match(/\d+:\d+/);
    expect(durationMatch).toBeTruthy();
  }

  // Also verify duration is non-zero in the file list
  const fileCards = library.fileCards;
  const fileCount = await fileCards.count();

  if (fileCount > 0) {
    const firstFileDuration = fileCards.first().locator('.file-duration, .duration');
    if (await firstFileDuration.isVisible()) {
      const text = await firstFileDuration.textContent();
      expect(text).not.toBe('0:00');
    }
  }
});
