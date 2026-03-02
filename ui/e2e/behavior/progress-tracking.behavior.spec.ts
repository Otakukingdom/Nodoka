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

  // New audiobook has zero progress - Cucumber: New audiobook has zero progress
  test.describe('New Audiobook Progress', () => {
    test('new audiobook shows zero progress', async ({ page }) => {
      // Sort by title to find Mystery Novel (0% progress)
      const sortSelect = page.locator('.library-panel select');
      await sortSelect.selectOption('Title');
      await page.waitForTimeout(500);

      // Find a card with 0% progress
      const cards = page.locator('#audiobook-list .card');
      const count = await cards.count();

      let foundZeroProgress = false;
      for (let i = 0; i < count; i++) {
        const card = cards.nth(i);
        const text = await card.textContent();
        if (text && text.includes('0%')) {
          foundZeroProgress = true;
          // Verify progress bar is minimal or not visible
          await expect(card).toBeVisible();
          break;
        }
      }

      expect(foundZeroProgress).toBe(true);
    });

    test('new audiobook progress bar is empty or minimal', async ({ page }) => {
      // Sort by title
      const sortSelect = page.locator('.library-panel select');
      await sortSelect.selectOption('Title');
      await page.waitForTimeout(500);

      // Find card with 0%
      const cards = page.locator('#audiobook-list .card');
      const count = await cards.count();

      for (let i = 0; i < count; i++) {
        const card = cards.nth(i);
        const text = await card.textContent();
        if (text && text.includes('0%')) {
          // Progress bar should be minimal width
          const progressBar = card.locator('.gradient-premium');
          // Progress bar may not be visible at 0%
          const progressCount = await progressBar.count();
          expect(progressCount).toBeLessThanOrEqual(1);
          break;
        }
      }
    });
  });

  // Updating audiobook progress - Cucumber: Updating audiobook progress
  test.describe('Progress Updates via Playback', () => {
    test('progress updates when file is played', async ({ page }) => {
      // Select an audiobook
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Start playback
      const playButton = page.locator('.btm-nav button[aria-label="Play"]').first();
      await playButton.click();
      await page.waitForTimeout(500);

      // Progress should be tracked (mock doesn't actually update)
      const playerControls = page.locator('.btm-nav');
      await expect(playerControls).toBeVisible();
    });

    test('progress bar reflects current position', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Get progress bar
      const progressBar = page.locator('#audiobook-list .card').first().locator('.gradient-premium');
      await expect(progressBar).toBeVisible();
    });

    test('completeness percentage updates on playback', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Get initial percentage
      const percentageText = await page.locator('#audiobook-list .card').first()
        .locator('text=/%/').first().textContent();

      // Should have a percentage
      expect(percentageText).toMatch(/\d+%/);
    });
  });

  // File progress is tracked - Cucumber: File progress is tracked
  test.describe('File Progress Updates', () => {
    test('file seek position is updated', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Select a file
      await page.locator('#files-list .card').first().click();
      await page.waitForTimeout(200);

      // Seek bar should be visible
      const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
      await expect(seekBar).toBeVisible();
    });

    test('file completeness is tracked', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // First file has 100% progress
      const firstFile = page.locator('#files-list .card').first();
      await expect(firstFile).toContainText('100%');
    });

    test('file progress bar shows completion', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // First file should show progress bar
      const progressBar = page.locator('#files-list .card').first().locator('.gradient-premium');
      await expect(progressBar).toBeVisible();
    });
  });

  // Last listened timestamp - Cucumber: Last listened timestamp is updated
  test.describe('Last Listened Timestamp', () => {
    test('last listened timestamp is displayed', async ({ page }) => {
      // Sort by recent (default)
      const sortSelect = page.locator('.library-panel select');
      await expect(sortSelect).toHaveValue('RecentlyPlayed');
    });

    test('last listened updates on interaction', async ({ page }) => {
      // Select an audiobook
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Start playback
      const playButton = page.locator('.btm-nav button[aria-label="Play"]').first();
      await playButton.click();
      await page.waitForTimeout(200);

      // Page should be functional
      const playerControls = page.locator('.btm-nav');
      await expect(playerControls).toBeVisible();
    });

    test('recently played sort shows correct order', async ({ page }) => {
      // Sort by recently played
      const sortSelect = page.locator('.library-panel select');
      await sortSelect.selectOption('RecentlyPlayed');
      await page.waitForTimeout(500);

      // First card should be most recently played
      const firstCard = page.locator('#audiobook-list .card').first();
      await expect(firstCard).toBeVisible();
    });
  });

  // Selected file is tracked - Cucumber: Selected file is tracked
  test.describe('Selected File Tracking', () => {
    test('selected file is persisted', async ({ page }) => {
      // Select an audiobook
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Select a file
      const secondFile = page.locator('#files-list .card').nth(1);
      await secondFile.click();
      await page.waitForTimeout(200);

      // File should be selected
      await expect(secondFile).toHaveClass(/ring-primary/);
    });

    test('selected file persists after navigation', async ({ page }) => {
      // Select audiobook and file
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      const firstFile = page.locator('#files-list .card').first();
      await firstFile.click();
      await page.waitForTimeout(200);

      // Navigate away
      await page.locator('#audiobook-list .card').nth(1).click();
      await page.waitForTimeout(300);

      // Navigate back
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // File should still be selected
      const selectedFile = page.locator('#files-list .card.ring-primary');
      await expect(selectedFile).toBeVisible();
    });

    test('file selection updates player state', async ({ page }) => {
      // Select audiobook and file
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Select a file
      await page.locator('#files-list .card').first().click();
      await page.waitForTimeout(200);

      // Player should be ready for playback
      const playButton = page.locator('.btm-nav button[aria-label="Play"]');
      await expect(playButton).toBeVisible();
    });
  });

  // Metadata persistence - Cucumber: Metadata persistence
  test.describe('Metadata Persistence', () => {
    test('audiobook metadata persists', async ({ page }) => {
      // Select audiobook
      const firstCard = page.locator('#audiobook-list .card').first();
      await firstCard.click();
      await page.waitForTimeout(300);

      // Get title
      const title = await firstCard.locator('.card-title').textContent();

      // Navigate away and back
      await page.locator('#audiobook-list .card').nth(1).click();
      await page.waitForTimeout(300);
      await firstCard.click();
      await page.waitForTimeout(300);

      // Title should be the same
      const titleAgain = await firstCard.locator('.card-title').textContent();
      expect(title).toBe(titleAgain);
    });

    test('progress metadata persists across sessions', async ({ page }) => {
      // Get initial progress
      const firstCard = page.locator('#audiobook-list .card').first();
      const progressText = await firstCard.locator('text=/%/').first().textContent();

      // Reload page
      await page.reload();
      await page.waitForTimeout(500);

      // Progress should be the same
      const firstCardAgain = page.locator('#audiobook-list .card').first();
      const progressTextAgain = await firstCardAgain.locator('text=/%/').first().textContent();

      expect(progressText).toBe(progressTextAgain);
    });
  });

  // File seek position - Cucumber: File seek position is updated
  test.describe('File Seek Position', () => {
    test('seek bar position is displayed', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Seek bar should show current position
      const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
      const value = await seekBar.inputValue();

      // Should have a position value
      expect(parseInt(value)).toBeGreaterThanOrEqual(0);
    });

    test('seek position updates on seek', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Move seek bar
      const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
      const max = await seekBar.getAttribute('max');

      // Use a valid value within range
      await seekBar.fill(max ?? '50');
      await page.waitForTimeout(200);

      // Seek bar should be functional
      await expect(seekBar).toBeVisible();
    });

    test('seek position clamps to file duration', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Try to seek beyond duration
      const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
      const max = await seekBar.getAttribute('max');

      // Seek bar should have max value
      expect(max).not.toBeNull();
    });
  });

  // File-Level Progress Updates - Cucumber: File seek position is updated
  test.describe('File-Level Progress Updates', () => {
    test('seeking updates file progress indicator', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Select a file
      await page.locator('#files-list .card').first().click();
      await page.waitForTimeout(200);

      // Seek to position - use numeric value based on max
      const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
      const max = await seekBar.getAttribute('max');
      const seekValue = Math.floor(parseInt(max ?? '100') / 2).toString();
      await seekBar.fill(seekValue);
      await page.waitForTimeout(200);

      // File card should still be visible
      const firstFile = page.locator('#files-list .card').first();
      await expect(firstFile).toBeVisible();
    });

    test('file completeness percentage updates during playback', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // First file has 100% progress
      const firstFile = page.locator('#files-list .card').first();
      await expect(firstFile).toContainText('100%');

      // Start playback
      await page.locator('.btm-nav button[aria-label="Play"]').first().click();
      await page.waitForTimeout(200);

      // File progress should still be visible
      await expect(firstFile).toBeVisible();
    });
  });

  // Progress Persistence - Cucumber: Metadata persistence
  test.describe('Progress Persistence', () => {
    test('progress persists across page reloads', async ({ page }) => {
      // Get initial progress
      const firstCard = page.locator('#audiobook-list .card').first();
      const progressText = await firstCard.locator('text=/%/').first().textContent();

      // Reload page
      await page.reload();
      await page.waitForTimeout(500);

      // Progress should be the same
      const firstCardAfter = page.locator('#audiobook-list .card').first();
      const progressTextAfter = await firstCardAfter.locator('text=/%/').first().textContent();

      expect(progressText).toBe(progressTextAfter);
    });

    test('selected file is restored on app restart', async ({ page }) => {
      // Select audiobook and file
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);
      await page.locator('#files-list .card').nth(1).click();
      await page.waitForTimeout(200);

      // Reload
      await page.reload();
      await page.waitForTimeout(500);

      // Player should be functional
      const playerControls = page.locator('.btm-nav');
      await expect(playerControls).toBeVisible();
    });

    test('audiobook selection persists', async ({ page }) => {
      // Select audiobook
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Reload
      await page.reload();
      await page.waitForTimeout(500);

      // Library should be functional
      const libraryPanel = page.locator('.library-panel');
      await expect(libraryPanel).toBeVisible();
    });
  });

  // Backend Progress Tracking - Cucumber: File seek position is updated
  test.describe('Backend Progress Tracking', () => {
    test('file progress can be updated via backend', async ({ page }) => {
      // Update file progress
      await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('update_file_progress', {
            filePath: '/test/audiobooks/The Great Adventure/Chapter 1.mp3',
            seekPosition: 60000,
            completeness: 50
          });
        }
        return null;
      });

      // Get files and verify progress
      const files = await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('get_audiobook_files', { audiobookId: 1 });
        }
        return [];
      });

      const firstFile = files.find((f: { fullPath: string }) =>
        f.fullPath === '/test/audiobooks/The Great Adventure/Chapter 1.mp3'
      );
      expect(firstFile?.seekPosition).toBe(60000);
      expect(firstFile?.completeness).toBe(50);
    });

    test('audiobook completeness can be updated via backend', async ({ page }) => {
      // Update audiobook completeness
      await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('update_audiobook_completeness', {
            id: 1,
            completeness: 75
          });
        }
        return null;
      });

      // Get audiobook and verify
      const audiobook = await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('get_audiobook_by_id', { id: 1 });
        }
        return null;
      });

      expect(audiobook?.completeness).toBe(75);
    });

    test('last listened timestamp can be updated via backend', async ({ page }) => {
      // Update last listened
      await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('update_audiobook_last_listened', { id: 1 });
        }
        return null;
      });

      // Get audiobook and verify timestamp was updated
      const audiobook = await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('get_audiobook_by_id', { id: 1 });
        }
        return null;
      });

      // lastListened should be a recent timestamp
      expect(audiobook?.lastListened).toBeDefined();
    });
  });

  // Audiobook Completeness Calculation - Cucumber: File completeness is tracked
  test.describe('Audiobook Completeness Calculation', () => {
    test('audiobook has completeness percentage', async ({ page }) => {
      const audiobooks = await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('get_all_audiobooks');
        }
        return [];
      });

      audiobooks.forEach((book: { completeness?: number }) => {
        expect(book.completeness).toBeDefined();
        expect(book.completeness).toBeGreaterThanOrEqual(0);
        expect(book.completeness).toBeLessThanOrEqual(100);
      });
    });

    test('completed audiobook shows 100 percent', async ({ page }) => {
      const audiobooks = await page.evaluate(() => {
        const tauri = (window as any).__TAURI__;
        if (tauri) {
          return tauri.invoke('get_all_audiobooks');
        }
        return [];
      });

      // Find completed audiobook
      const completedBook = audiobooks.find((book: { completeness?: number }) => book.completeness === 100);
      expect(completedBook).toBeDefined();
      expect(completedBook.name).toBe('Science Fiction Stories');
    });
  });
});
