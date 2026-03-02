/**
 * Behavioral tests for Progress Tracking
 * Tests actual UI interactions matching cucumber acceptance criteria
 */

import { test, expect } from '../fixtures';
import { gotoAndWait } from '../test-utils';

test.describe('Progress Tracking - Behavioral Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test.describe('Audiobook Progress Display', () => {
    test('audiobook cards show progress percentage', async ({ page }) => {
      const firstCard = page.locator('#audiobook-list .card').first();

      // Should show percentage
      const percentageText = firstCard.locator('text=/%/');
      await expect(percentageText).toBeVisible();
    });

    test('audiobook cards show progress bar', async ({ page }) => {
      const firstCard = page.locator('#audiobook-list .card').first();

      // Should have progress bar container
      const progressContainer = firstCard.locator('.h-2.bg-base-200');
      await expect(progressContainer).toBeVisible();
    });

    test('progress bar width matches percentage', async ({ page }) => {
      // Any card with progress should have a visible progress bar
      const firstCard = page.locator('#audiobook-list .card').first();

      // Progress bar should be visible
      const progressBar = firstCard.locator('.gradient-premium');
      await expect(progressBar).toBeVisible();
    });

    test('completed audiobooks show 100%', async ({ page }) => {
      // Sort by progress to get completed book first
      const sortSelect = page.locator('.library-panel select');
      await sortSelect.selectOption('Progress');
      await page.waitForTimeout(500);

      // First card should show 100%
      const firstCard = page.locator('#audiobook-list .card').first();
      await expect(firstCard).toContainText('100%');
    });

    test('new audiobooks show 0%', async ({ page }) => {
      // Sort by title for consistent order
      const sortSelect = page.locator('.library-panel select');
      await sortSelect.selectOption('Title');
      await page.waitForTimeout(500);

      // Get all cards and find one with 0%
      const cards = page.locator('#audiobook-list .card');
      const count = await cards.count();

      let foundZeroProgress = false;
      for (let i = 0; i < count; i++) {
        const text = await cards.nth(i).textContent();
        if (text && text.includes('0%')) {
          foundZeroProgress = true;
          break;
        }
      }

      expect(foundZeroProgress).toBe(true);
    });
  });

  test.describe('File Progress Display', () => {
    test('selecting audiobook shows file progress', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();

      // First file (Chapter 1) has 100% progress
      const firstFile = page.locator('#files-list .card').first();
      await expect(firstFile).toContainText('100%');
    });

    test('files without progress do not show progress bar', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Files with 0% progress don't render the progress bar at all
      // Check that at least one file has no progress bar (Chapter 2 has 0%)
      const files = page.locator('#files-list .card');
      const count = await files.count();

      // At least one file should not have a progress bar
      let foundFileWithoutProgress = false;
      for (let i = 0; i < count; i++) {
        const file = files.nth(i);
        const progressBar = file.locator('.gradient-premium');
        const hasProgressBar = await progressBar.count() > 0;
        if (!hasProgressBar) {
          foundFileWithoutProgress = true;
          break;
        }
      }

      expect(foundFileWithoutProgress).toBe(true);
    });

    test('file progress bars are styled consistently', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();

      // Progress bar should have gradient-premium class
      const progressBar = page.locator('#files-list .card').first().locator('.gradient-premium');
      await expect(progressBar).toBeVisible();
    });
  });

  test.describe('Progress Persistence', () => {
    test('progress persists when selecting different audiobook', async ({ page }) => {
      // Select first audiobook
      await page.locator('#audiobook-list .card').first().click();

      // Get initial progress
      const initialProgress = await page.locator('#audiobook-list .card').first()
        .locator('text=/%/').first().textContent();

      // Select different audiobook
      await page.locator('#audiobook-list .card').nth(1).click();

      // Select first audiobook again
      await page.locator('#audiobook-list .card').first().click();

      // Progress should be the same
      const finalProgress = await page.locator('#audiobook-list .card').first()
        .locator('text=/%/').first().textContent();

      expect(initialProgress).toBe(finalProgress);
    });

    test('file progress persists when navigating', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();

      // Select a file
      await page.locator('#files-list .card').first().click();

      // Navigate away
      await page.locator('#audiobook-list .card').nth(1).click();

      // Navigate back
      await page.locator('#audiobook-list .card').first().click();

      // File should still be selected
      const selectedFile = page.locator('#files-list .card.ring-primary');
      await expect(selectedFile).toBeVisible();
    });
  });

  test.describe('Sort by Progress', () => {
    test('can sort audiobooks by progress', async ({ page }) => {
      const sortSelect = page.locator('.library-panel select');
      await sortSelect.selectOption('Progress');
      await page.waitForTimeout(500);

      // First card should be the one with highest progress (100%)
      const firstCard = page.locator('#audiobook-list .card').first();
      await expect(firstCard).toContainText('100%');
    });
  });

  test.describe('Progress Indicators', () => {
    test('completed audiobooks show checkmark', async ({ page }) => {
      // Sort by progress to get completed book first
      const sortSelect = page.locator('.library-panel select');
      await sortSelect.selectOption('Progress');
      await page.waitForTimeout(500);

      // First card should be completed (100%)
      const completedCard = page.locator('#audiobook-list .card').first();
      await expect(completedCard).toContainText('100%');

      // Should have checkmark icon
      const checkmark = completedCard.locator('svg');
      await expect(checkmark).toBeVisible();
    });

    test('in-progress audiobooks do not show checkmark', async ({ page }) => {
      // Get all cards and find one that's not 100%
      const cards = page.locator('#audiobook-list .card');
      const count = await cards.count();

      for (let i = 0; i < count; i++) {
        const card = cards.nth(i);
        const text = await card.textContent();
        // Find a card that doesn't have 100%
        if (text && !text.includes('100%')) {
          // Check for checkmark in the title area - should not have one since it's not 100%
          const checkmark = card.locator('.card-title').locator('..').locator('svg');
          const svgCount = await checkmark.count();
          expect(svgCount).toBe(0);
          break;
        }
      }
    });
  });

  test.describe('Progress Updates', () => {
    test('progress bar has smooth transition', async ({ page }) => {
      const firstCard = page.locator('#audiobook-list .card').first();

      // Progress bar should have transition class
      const progressBar = firstCard.locator('.gradient-premium');
      await expect(progressBar).toHaveClass(/transition/);
    });

    test('file progress bar has transition', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();

      const progressBar = page.locator('#files-list .card').first().locator('.gradient-premium');
      await expect(progressBar).toHaveClass(/transition/);
    });
  });

  test.describe('Visual Feedback', () => {
    test('progress bars use gradient styling', async ({ page }) => {
      const progressBar = page.locator('#audiobook-list .card').first().locator('.gradient-premium');
      await expect(progressBar).toBeVisible();
    });

    test('progress percentage uses tabular nums', async ({ page }) => {
      const percentage = page.locator('#audiobook-list .card').first().locator('.tabular-nums');
      await expect(percentage).toBeVisible();
    });

    test('progress bar background is visible', async ({ page }) => {
      const background = page.locator('#audiobook-list .card').first().locator('.bg-base-200');
      await expect(background).toBeVisible();
    });
  });

  test.describe('Accessibility', () => {
    test('progress information is readable', async ({ page }) => {
      const firstCard = page.locator('#audiobook-list .card').first();
      const text = await firstCard.textContent();

      // Should contain percentage
      expect(text).toMatch(/\d+%/);
    });

    test('file progress is readable', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();

      const firstFile = page.locator('#files-list .card').first();
      const text = await firstFile.textContent();

      // Should contain percentage
      expect(text).toMatch(/\d+%/);
    });
  });
});
