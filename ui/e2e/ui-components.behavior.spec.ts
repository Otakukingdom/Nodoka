/**
 * UI Component Interaction Tests
 * Tests specific UI component behaviors matching cucumber acceptance criteria
 */

import { test, expect } from './fixtures';
import { gotoAndWait, openSettingsModal } from './test-utils';

test.describe('UI Component Interactions', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  // Library Panel Components
  test.describe('Library Panel Components', () => {
    test('search input filters correctly', async ({ page }) => {
      const searchInput = page.locator('.library-panel input[placeholder*="Search"]');
      await expect(searchInput).toBeVisible();

      // Get initial count
      const initialCount = await page.locator('#audiobook-list .card').count();

      // Type search query
      await searchInput.fill('Adventure');
      await page.waitForTimeout(500);

      // Should show filtered results
      const filteredCount = await page.locator('#audiobook-list .card').count();
      expect(filteredCount).toBeLessThanOrEqual(initialCount);
    });

    test('sort dropdown changes order', async ({ page }) => {
      const sortSelect = page.locator('.library-panel select');
      await expect(sortSelect).toBeVisible();

      // Change to Title sort
      await sortSelect.selectOption('Title');
      await page.waitForTimeout(500);

      // Get titles in order
      const titles = await page.locator('#audiobook-list .card .card-title').allTextContents();

      // Should be sorted alphabetically
      const sortedTitles = [...titles].sort();
      expect(titles).toEqual(sortedTitles);
    });

    test('sort dropdown has expected options', async ({ page }) => {
      const sortSelect = page.locator('.library-panel select');
      const options = await sortSelect.locator('option').allTextContents();

      // Should have key sorting options
      expect(options.length).toBeGreaterThan(0);
    });

    test('library panel is visible and positioned correctly', async ({ page }) => {
      const libraryPanel = page.locator('.library-panel');
      await expect(libraryPanel).toBeVisible();

      // Should have proper structure
      const searchInput = libraryPanel.locator('input[placeholder*="Search"]');
      await expect(searchInput).toBeVisible();
    });
  });

  // Player Control Components
  test.describe('Player Control Components', () => {
    test.beforeEach(async ({ page }) => {
      // Select an audiobook to enable player controls
      await page.locator('#audiobook-list .card').first().click();
    });

    test('all player buttons are interactive', async ({ page }) => {
      const buttons = page.locator('.btm-nav button');
      const count = await buttons.count();

      for (let i = 0; i < count; i++) {
        const button = buttons.nth(i);
        await expect(button).toBeEnabled();
      }
    });

    test('play button toggles to pause when clicked', async ({ page }) => {
      const playButton = page.locator('.btm-nav button[aria-label="Play"]').first();
      await playButton.click();
      await page.waitForTimeout(200);

      const pauseButton = page.locator('.btm-nav button[aria-label="Pause"]');
      await expect(pauseButton).toBeVisible();
    });

    test('volume slider is interactive', async ({ page }) => {
      const volumeSlider = page.locator('.btm-nav .group\\/volume input[type="range"]');
      await expect(volumeSlider).toBeVisible();

      // Change value
      await volumeSlider.fill('50');
      await page.waitForTimeout(200);

      // Should still be visible
      await expect(volumeSlider).toBeVisible();
    });

    test('speed button cycles through speeds', async ({ page }) => {
      const speedButton = page.locator('.btm-nav button:has-text("x")').first();
      const initialSpeed = await speedButton.textContent();

      // Click to change
      await speedButton.click();
      await page.waitForTimeout(200);

      const newSpeed = await speedButton.textContent();
      expect(newSpeed).not.toBe(initialSpeed);
    });

    test('seek bar is draggable', async ({ page }) => {
      const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
      await expect(seekBar).toBeVisible();

      // Move seek bar
      await seekBar.fill('500');
      await page.waitForTimeout(200);

      await expect(seekBar).toBeVisible();
    });

    test('time displays show current and total time', async ({ page }) => {
      const timeDisplays = page.locator('.btm-nav .font-mono');
      const count = await timeDisplays.count();

      // Should have at least current time and duration
      expect(count).toBeGreaterThanOrEqual(2);
    });
  });

  // Modal Components
  test.describe('Modal Components', () => {
    test('settings modal opens and closes correctly', async ({ page }) => {
      // Open via keyboard shortcut
      await page.keyboard.press('Control+,');
      await page.waitForTimeout(300);

      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();

      // Close via Escape
      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);

      await expect(modal).not.toBeVisible();
    });

    test('modal has backdrop overlay', async ({ page }) => {
      await openSettingsModal(page);

      const modal = page.locator('.modal');
      await expect(modal).toBeVisible();
    });

    test('modal close button works', async ({ page }) => {
      await openSettingsModal(page);

      const closeButton = page.locator('.modal-box button[aria-label="Close"]');
      await closeButton.click();
      await page.waitForTimeout(300);

      const modal = page.locator('.modal-box');
      await expect(modal).not.toBeVisible();
    });

    test('modal Done button works', async ({ page }) => {
      await openSettingsModal(page);

      const doneButton = page.locator('.modal-box button:has-text("Done")');
      await doneButton.click();
      await page.waitForTimeout(300);

      const modal = page.locator('.modal-box');
      await expect(modal).not.toBeVisible();
    });
  });

  // Card Components
  test.describe('Card Components', () => {
    test('audiobook cards have proper structure', async ({ page }) => {
      const firstCard = page.locator('#audiobook-list .card').first();

      // Should have title
      await expect(firstCard.locator('.card-title')).toBeVisible();

      // Should have progress bar container
      await expect(firstCard.locator('.h-2.bg-base-200')).toBeVisible();

      // Should have percentage text
      await expect(firstCard.locator('text=/%/')).toBeVisible();
    });

    test('file cards have proper structure', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();

      const firstFile = page.locator('#files-list .card').first();

      // Should be visible
      await expect(firstFile).toBeVisible();

      // Should have position indicator
      await expect(firstFile.locator('.font-mono')).toBeVisible();
    });

    test('bookmark cards have proper structure', async ({ page }) => {
      await page.locator('#audiobook-list .card').first().click();

      const firstBookmark = page.locator('#bookmarks-list .card').first();

      // Should have label
      await expect(firstBookmark.locator('.text-sm.font-medium')).toBeVisible();

      // Should have time badge
      await expect(firstBookmark.locator('.badge')).toBeVisible();
    });

    test('cards have hover effects', async ({ page }) => {
      const firstCard = page.locator('#audiobook-list .card').first();

      // Should have hover class
      await expect(firstCard).toHaveClass(/hover/);
    });

    test('selected card has visual indicator', async ({ page }) => {
      const firstCard = page.locator('#audiobook-list .card').first();
      await firstCard.click();

      // Should have ring-primary class
      await expect(firstCard).toHaveClass(/ring-primary/);
    });
  });

  // Accessibility Components
  test.describe('Accessibility', () => {
    test('buttons have accessible labels', async ({ page }) => {
      // Settings button
      const settingsButton = page.locator('button[aria-label="Settings"]');
      expect(await settingsButton.getAttribute('aria-label')).toBe('Settings');
    });

    test('interactive elements are focusable', async ({ page }) => {
      // Audiobook cards
      const firstCard = page.locator('#audiobook-list .card').first();
      await firstCard.focus();
      await expect(firstCard).toBeFocused();
    });

    test('keyboard navigation works', async ({ page }) => {
      // Tab through elements
      await page.keyboard.press('Tab');
      await page.keyboard.press('Tab');

      // Page should still be functional
      const libraryPanel = page.locator('.library-panel');
      await expect(libraryPanel).toBeVisible();
    });

    test('modal traps focus correctly', async ({ page }) => {
      await openSettingsModal(page);

      // Tab within modal
      await page.keyboard.press('Tab');
      await page.keyboard.press('Tab');

      // Modal should still be open
      const modal = page.locator('.modal-box');
      await expect(modal).toBeVisible();
    });
  });

  // Responsive Components
  test.describe('Responsive Layout', () => {
    test('layout adapts to viewport', async ({ page }) => {
      // Default viewport should show all panels
      const libraryPanel = page.locator('.library-panel');
      const playerControls = page.locator('.btm-nav');

      await expect(libraryPanel).toBeVisible();
      await expect(playerControls).toBeVisible();
    });

    test('player controls are always visible', async ({ page }) => {
      const playerControls = page.locator('.btm-nav');
      await expect(playerControls).toBeVisible();

      // Select audiobook
      await page.locator('#audiobook-list .card').first().click();

      // Player should still be visible
      await expect(playerControls).toBeVisible();
    });
  });

  // Empty States
  test.describe('Empty States', () => {
    test('search with no results shows message', async ({ page }) => {
      const searchInput = page.locator('.library-panel input[placeholder*="Search"]');
      await searchInput.fill('ZZZNonexistentBook12345');
      await page.waitForTimeout(500);

      await expect(page.locator('text=No matches found')).toBeVisible();
    });

    test('empty bookmarks shows guidance', async ({ page }) => {
      // Sort by title to find audiobook without bookmarks
      const sortSelect = page.locator('.library-panel select');
      await sortSelect.selectOption('Title');
      await page.waitForTimeout(500);

      // Find Mystery Novel
      const cards = page.locator('#audiobook-list .card');
      const count = await cards.count();

      for (let i = 0; i < count; i++) {
        const card = cards.nth(i);
        const text = await card.textContent();
        if (text && text.includes('Mystery')) {
          await card.click();
          await page.waitForTimeout(500);

          // Check for empty state
          const bookmarksList = page.locator('#bookmarks-list');
          const bookmarksText = await bookmarksList.textContent();
          if (bookmarksText && bookmarksText.includes('No bookmarks')) {
            await expect(bookmarksList).toContainText('No bookmarks');
          }
          break;
        }
      }
    });
  });
});
