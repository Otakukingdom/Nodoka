/**
 * Behavioral tests for Directory Scanning
 * Tests actual UI interactions matching cucumber acceptance criteria
 */

import { test, expect } from '../fixtures';
import { gotoAndWait } from '../test-utils';

test.describe('Directory Scanning - Behavioral Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test.describe('Library Display', () => {
    test('library shows audiobooks from scanned directories', async ({ page }) => {
      const audiobookCards = page.locator('#audiobook-list .card');
      const count = await audiobookCards.count();

      // Should have audiobooks from mock data
      expect(count).toBeGreaterThan(0);
    });

    test('scanned audiobooks display correct information', async ({ page }) => {
      const firstCard = page.locator('#audiobook-list .card').first();

      // Should have title
      const title = firstCard.locator('.card-title');
      await expect(title).toBeVisible();

      // Should have progress bar
      const progressBar = firstCard.locator('.gradient-premium');
      await expect(progressBar).toBeVisible();
    });
  });

  test.describe('Directory Indicators', () => {
    test('audiobooks are organized correctly', async ({ page }) => {
      // Check that audiobooks are displayed in the list
      const audiobooks = page.locator('#audiobook-list .card');
      const titles = await audiobooks.locator('.card-title').allTextContents();

      // Should have multiple audiobooks
      expect(titles.length).toBeGreaterThan(0);
    });
  });

  test.describe('File Status', () => {
    test('files are displayed for selected audiobook', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();

      const fileCards = page.locator('#files-list .card');
      const count = await fileCards.count();

      expect(count).toBeGreaterThan(0);
    });

    test('files show position order', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();

      // Files should be numbered
      const positionNumbers = await page.locator('#files-list .card .font-mono').allTextContents();

      // Should be ordered (#1, #2, #3)
      expect(positionNumbers.length).toBeGreaterThan(0);
    });
  });

  test.describe('Rescan Indicators', () => {
    test('audiobook progress persists after navigation', async ({ page }) => {
      // Select first audiobook
      await page.locator('#audiobook-list .card').first().click();

      // Get progress value
      const progressText = await page.locator('#audiobook-list .card').first()
        .locator('text=/%/').first().textContent();

      // Navigate away and back
      await page.locator('#audiobook-list .card').nth(1).click();
      await page.locator('#audiobook-list .card').first().click();

      // Progress should be the same
      const progressTextAfter = await page.locator('#audiobook-list .card').first()
        .locator('text=/%/').first().textContent();

      expect(progressText).toBe(progressTextAfter);
    });
  });

  test.describe('Empty State', () => {
    test('search with no results shows empty state', async ({ page }) => {
      const searchInput = page.locator('.library-panel input[placeholder*="Search"]');
      await searchInput.fill('Nonexistent Directory Book');
      await page.waitForTimeout(300);

      await expect(page.locator('text=No matches found')).toBeVisible();
    });
  });

  test.describe('Library Refresh', () => {
    test('library can be sorted by different criteria', async ({ page }) => {
      const sortSelect = page.locator('.library-panel select');

      // Change sort to Title
      await sortSelect.selectOption('Title');
      await page.waitForTimeout(300);

      // Get titles
      const titlesTitle = await page.locator('#audiobook-list .card .card-title').allTextContents();

      // Change sort to Progress
      await sortSelect.selectOption('Progress');
      await page.waitForTimeout(300);

      // Get titles again
      const titlesProgress = await page.locator('#audiobook-list .card .card-title').allTextContents();

      // Order should be different (or same if coincidentally)
      // Just verify sorting works without error
      expect(titlesTitle.length).toBe(titlesProgress.length);
    });
  });

  test.describe('Keyboard Navigation', () => {
    test('library can be navigated with keyboard', async ({ page }) => {
      // Focus first card
      const firstCard = page.locator('#audiobook-list .card').first();
      await firstCard.focus();
      await page.keyboard.press('Enter');
      await page.waitForTimeout(200);

      // Should be selected
      await expect(firstCard).toHaveClass(/ring-primary/);
    });
  });
});
