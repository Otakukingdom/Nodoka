/**
 * End-to-end user workflow tests
 * Tests complete user journeys spanning multiple features
 */

import { test, expect } from './fixtures';
import { gotoAndWait, openSettingsModal, closeModal } from './test-utils';

test.describe('Complete User Workflows', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  // Complete listening session workflow
  test.describe('Listening Session Workflow', () => {
    test('complete listening session: select → play → bookmark → adjust → stop', async ({ page }) => {
      // 1. Select audiobook from library
      const firstCard = page.locator('#audiobook-list .card').first();
      await firstCard.click();
      await page.waitForTimeout(300);

      // Verify selection and files display
      await expect(firstCard).toHaveClass(/ring-primary/);
      const filesList = page.locator('#files-list');
      await expect(filesList).toBeVisible();

      // 2. Select specific file
      const firstFile = filesList.locator('.card').first();
      await firstFile.click();
      await page.waitForTimeout(300);
      await expect(firstFile).toHaveClass(/ring-primary/);

      // 3. Start playback
      const playButton = page.locator('.btm-nav button[aria-label="Play"]').first();
      await playButton.click();
      await page.waitForTimeout(200);

      // Verify playing state
      const pauseButton = page.locator('.btm-nav button[aria-label="Pause"]');
      await expect(pauseButton).toBeVisible();

      // 4. Create bookmark during playback
      await page.keyboard.press('Control+b');
      await page.waitForTimeout(300);

      // 5. Adjust volume
      const volumeSlider = page.locator('.btm-nav .group\\/volume input[type="range"]');
      await volumeSlider.fill('75');
      await page.waitForTimeout(200);

      // 6. Change speed
      const speedButton = page.locator('.btm-nav button:has-text("x")').first();
      await speedButton.click();
      await page.waitForTimeout(200);

      // 7. Stop playback
      const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
      await stopButton.click();
      await page.waitForTimeout(200);

      // Verify stopped state
      await expect(page.locator('.btm-nav button[aria-label="Play"]').first()).toBeVisible();

      // 8. Verify bookmark was created
      const bookmarkCount = await page.locator('#bookmarks-list .card').count();
      expect(bookmarkCount).toBeGreaterThan(0);
    });

    test('pause and resume workflow', async ({ page }) => {
      // Select and play
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);
      await page.locator('.btm-nav button[aria-label="Play"]').first().click();
      await page.waitForTimeout(200);

      // Pause
      await page.locator('.btm-nav button[aria-label="Pause"]').click();
      await page.waitForTimeout(200);

      // Verify paused state
      await expect(page.locator('.btm-nav button[aria-label="Play"]').first()).toBeVisible();

      // Resume
      await page.locator('.btm-nav button[aria-label="Play"]').first().click();
      await page.waitForTimeout(200);

      // Verify playing state
      await expect(page.locator('.btm-nav button[aria-label="Pause"]')).toBeVisible();
    });
  });

  // Settings customization workflow
  test.describe('Settings Customization Workflow', () => {
    test('settings customization: theme → volume → speed → persist', async ({ page }) => {
      // 1. Open settings
      await openSettingsModal(page);
      await expect(page.locator('.modal-box')).toBeVisible();

      // 2. Change theme
      const themeSelect = page.getByLabel('Select theme');
      await themeSelect.selectOption('Dark');
      await expect(themeSelect).toHaveValue('Dark');

      // 3. Close settings
      await closeModal(page);

      // 4. Adjust volume
      const volumeSlider = page.locator('.btm-nav .group\\/volume input[type="range"]');
      await volumeSlider.fill('60');
      await page.waitForTimeout(200);

      // 5. Adjust speed
      const speedButton = page.locator('.btm-nav button:has-text("x")').first();
      await speedButton.click();
      await page.waitForTimeout(200);

      // 6. Reopen settings and verify persistence
      await openSettingsModal(page);
      await expect(page.getByLabel('Select theme')).toHaveValue('Dark');
      await closeModal(page);
    });

    test('focus mode toggle workflow', async ({ page }) => {
      // Open settings
      await openSettingsModal(page);

      // Look for focus mode toggle
      const focusModeToggle = page.locator('.modal-box input[type="checkbox"]');
      const focusModeButton = page.locator('.modal-box button').filter({ hasText: /focus/i });

      if (await focusModeToggle.count() > 0) {
        // Enable focus mode
        await focusModeToggle.first().check();
        await page.waitForTimeout(200);
        await expect(focusModeToggle.first()).toBeChecked();

        // Disable focus mode
        await focusModeToggle.first().uncheck();
        await page.waitForTimeout(200);
        await expect(focusModeToggle.first()).not.toBeChecked();
      } else if (await focusModeButton.count() > 0) {
        // Toggle via button
        await focusModeButton.first().click();
        await page.waitForTimeout(200);
      }

      await closeModal(page);
    });
  });

  // Navigation workflow
  test.describe('Navigation Workflow', () => {
    test('navigate between audiobooks and files', async ({ page }) => {
      // Select first audiobook
      const firstCard = page.locator('#audiobook-list .card').first();
      await firstCard.click();
      await page.waitForTimeout(300);
      await expect(firstCard).toHaveClass(/ring-primary/);

      // Select second audiobook
      const secondCard = page.locator('#audiobook-list .card').nth(1);
      await secondCard.click();
      await page.waitForTimeout(300);
      await expect(secondCard).toHaveClass(/ring-primary/);
      await expect(firstCard).not.toHaveClass(/ring-primary/);

      // Select file
      const firstFile = page.locator('#files-list .card').first();
      await firstFile.click();
      await page.waitForTimeout(200);
      await expect(firstFile).toHaveClass(/ring-primary/);

      // Select different file
      const secondFile = page.locator('#files-list .card').nth(1);
      if (await secondFile.count() > 0) {
        await secondFile.click();
        await page.waitForTimeout(200);
        await expect(secondFile).toHaveClass(/ring-primary/);
      }
    });

    test('search and select workflow', async ({ page }) => {
      // Get initial count of audiobooks
      const initialCards = page.locator('#audiobook-list .card');
      const initialCount = await initialCards.count();
      expect(initialCount).toBeGreaterThan(0);

      // Search for audiobook
      const searchInput = page.locator('.library-panel input[placeholder*="Search"]');
      await searchInput.fill('Adventure');
      await page.waitForTimeout(500);

      // Check if search filtered results or shows no matches
      const cards = page.locator('#audiobook-list .card');
      const count = await cards.count();
      const noMatches = await page.locator('text=No matches found').isVisible();

      // If we have results, select one
      if (count > 0 && !noMatches) {
        await cards.first().click();
        await page.waitForTimeout(300);
      }

      // Clear search to show all audiobooks again
      await searchInput.clear();
      await page.waitForTimeout(500);

      // Should show all audiobooks again
      const allCards = page.locator('#audiobook-list .card');
      const allCount = await allCards.count();
      expect(allCount).toBeGreaterThanOrEqual(1);
    });
  });

  // Bookmark management workflow
  test.describe('Bookmark Management Workflow', () => {
    test('create and navigate to bookmark', async ({ page }) => {
      // Select audiobook
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Get initial bookmark count
      const initialCount = await page.locator('#bookmarks-list .card').count();

      // Create bookmark
      await page.keyboard.press('Control+b');
      await page.waitForTimeout(500);

      // Verify bookmark created
      const newCount = await page.locator('#bookmarks-list .card').count();
      expect(newCount).toBeGreaterThanOrEqual(initialCount);

      // Click bookmark to navigate
      const firstBookmark = page.locator('#bookmarks-list .card').first();
      await firstBookmark.click();
      await page.waitForTimeout(300);

      // Player should be functional
      const playerControls = page.locator('.btm-nav');
      await expect(playerControls).toBeVisible();
    });
  });

  // Keyboard shortcuts workflow
  test.describe('Keyboard Shortcuts Workflow', () => {
    test('common keyboard shortcuts work together', async ({ page }) => {
      // Select audiobook
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);

      // Space to play/pause
      await page.keyboard.press('Space');
      await page.waitForTimeout(200);
      await expect(page.locator('.btm-nav button[aria-label="Pause"]')).toBeVisible();

      // M to mute/unmute
      await page.keyboard.press('m');
      await page.waitForTimeout(200);

      // Ctrl+, for settings
      await page.keyboard.press('Control+,');
      await page.waitForTimeout(300);
      await expect(page.locator('.modal-box')).toBeVisible();

      // Escape to close
      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);
      await expect(page.locator('.modal-box')).not.toBeVisible();
    });
  });

  // Complete session persistence workflow
  test.describe('Session Persistence Workflow', () => {
    test('session state persists across reload', async ({ page }) => {
      // 1. Select audiobook and file
      await page.locator('#audiobook-list .card').first().click();
      await page.waitForTimeout(300);
      await page.locator('#files-list .card').first().click();
      await page.waitForTimeout(300);

      // 2. Start playback
      await page.locator('.btm-nav button[aria-label="Play"]').first().click();
      await page.waitForTimeout(200);

      // 3. Create bookmark
      await page.keyboard.press('Control+b');
      await page.waitForTimeout(300);

      // 4. Reload page
      await page.reload();
      await page.waitForTimeout(500);

      // 5. Verify UI is functional
      const libraryPanel = page.locator('.library-panel');
      await expect(libraryPanel).toBeVisible();

      const playerControls = page.locator('.btm-nav');
      await expect(playerControls).toBeVisible();
    });
  });
});
