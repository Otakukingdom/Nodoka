/**
 * E2E screenshot tests for bookmark UI components
 * Captures visual appearance of bookmarks panel and bookmark cards
 */

import { test, expect } from './fixtures';
import { screenshotConfig, gotoAndWait, setTheme } from './test-utils';

test.describe('Bookmarks Panel Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('bookmarks panel empty state light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    const bookmarksList = page.locator('#bookmarks-list');
    await expect(bookmarksList).toHaveScreenshot(
      'bookmarks-empty-light.png',
      screenshotConfig
    );
  });

  test('bookmarks panel empty state dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const bookmarksList = page.locator('#bookmarks-list');
    await expect(bookmarksList).toHaveScreenshot(
      'bookmarks-empty-dark.png',
      screenshotConfig
    );
  });

  test('bookmarks section header', async ({ page }) => {
    const bookmarksHeader = page.locator('#bookmarks-list').locator('..').locator('h2').first();
    if (await bookmarksHeader.isVisible()) {
      await expect(bookmarksHeader).toHaveScreenshot(
        'bookmarks-header.png',
        screenshotConfig
      );
    }
  });
});

test.describe('Bookmark Card Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('bookmark card light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    const bookmarkCard = page.locator('#bookmarks-list .card').first();
    if (await bookmarkCard.isVisible()) {
      await expect(bookmarkCard).toHaveScreenshot(
        'bookmark-card-light.png',
        screenshotConfig
      );
    }
  });

  test('bookmark card dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const bookmarkCard = page.locator('#bookmarks-list .card').first();
    if (await bookmarkCard.isVisible()) {
      await expect(bookmarkCard).toHaveScreenshot(
        'bookmark-card-dark.png',
        screenshotConfig
      );
    }
  });

  test('bookmark card hover state', async ({ page }) => {
    const bookmarkCard = page.locator('#bookmarks-list .card').first();
    if (await bookmarkCard.isVisible()) {
      await bookmarkCard.hover();
      await page.waitForTimeout(200);
      await expect(bookmarkCard).toHaveScreenshot(
        'bookmark-card-hover.png',
        screenshotConfig
      );
    }
  });

  test('bookmark card with note', async ({ page }) => {
    // Find bookmark card that has a note (border-l-2 styling)
    const bookmarkWithNote = page.locator('#bookmarks-list .card:has(.border-l-2)').first();
    if (await bookmarkWithNote.isVisible()) {
      await expect(bookmarkWithNote).toHaveScreenshot(
        'bookmark-card-with-note.png',
        screenshotConfig
      );
    }
  });

  test('bookmark time badge', async ({ page }) => {
    const badge = page.locator('#bookmarks-list .badge').first();
    if (await badge.isVisible()) {
      await expect(badge).toHaveScreenshot(
        'bookmark-time-badge.png',
        screenshotConfig
      );
    }
  });
});

test.describe('Bookmarks Panel Layout', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('bookmarks panel full section', async ({ page }) => {
    await setTheme(page, 'Dark');
    const bookmarksSection = page.locator('#bookmarks-list').locator('..').locator('div').first();
    await expect(bookmarksSection).toHaveScreenshot(
      'bookmarks-section-full.png',
      screenshotConfig
    );
  });

  test('bookmarks icon in header', async ({ page }) => {
    const icon = page.locator('#bookmarks-list').locator('..').locator('.bg-primary\\/10').first();
    if (await icon.isVisible()) {
      await expect(icon).toHaveScreenshot(
        'bookmarks-icon.png',
        screenshotConfig
      );
    }
  });
});

test.describe('Bookmarks Empty State Content', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('empty bookmarks message', async ({ page }) => {
    const emptyMessage = page.locator('#bookmarks-list').getByText('No bookmarks yet');
    if (await emptyMessage.isVisible()) {
      await expect(emptyMessage).toHaveScreenshot(
        'bookmarks-empty-message.png',
        screenshotConfig
    );
    }
  });

  test('empty bookmarks shortcut hint', async ({ page }) => {
    const hint = page.locator('#bookmarks-list').getByText('Ctrl+B');
    if (await hint.isVisible()) {
      await expect(hint).toHaveScreenshot(
        'bookmarks-shortcut-hint.png',
        screenshotConfig
      );
    }
  });

  test('empty bookmarks icon circle', async ({ page }) => {
    const iconCircle = page.locator('#bookmarks-list .rounded-full').first();
    if (await iconCircle.isVisible()) {
      await expect(iconCircle).toHaveScreenshot(
        'bookmarks-empty-icon-circle.png',
        screenshotConfig
      );
    }
  });
});

test.describe('Bookmarks Accessibility', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('bookmark card keyboard focus', async ({ page }) => {
    const bookmarkCard = page.locator('#bookmarks-list .card').first();
    if (await bookmarkCard.isVisible()) {
      await bookmarkCard.focus();
      await page.waitForTimeout(200);
      await expect(bookmarkCard).toHaveScreenshot(
        'bookmark-card-focus.png',
        screenshotConfig
      );
    }
  });
});
