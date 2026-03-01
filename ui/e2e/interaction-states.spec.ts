/**
 * E2E screenshot tests for interaction states
 * Captures hover, focus, active, and pressed states for interactive elements
 */

import { test, expect } from './fixtures';
import { screenshotConfig, gotoAndWait, setTheme } from './test-utils';

test.describe('Button Interaction States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('play button hover state', async ({ page }) => {
    const playButton = page
      .locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]')
      .first();
    await playButton.hover();
    await page.waitForTimeout(200);
    await expect(playButton).toHaveScreenshot('play-button-hover.png', screenshotConfig);
  });

  test('play button active state', async ({ page }) => {
    const playButton = page
      .locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]')
      .first();
    await playButton.hover();
    await page.mouse.down();
    await page.waitForTimeout(100);
    await expect(playButton).toHaveScreenshot('play-button-active.png', screenshotConfig);
    await page.mouse.up();
  });

  test('play button pressed state after click', async ({ page }) => {
    const playButton = page
      .locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]')
      .first();
    const initialAria = await playButton.getAttribute('aria-label');
    await playButton.click();
    await page.waitForTimeout(300);
    await expect(playButton).toHaveScreenshot('play-button-pressed.png', screenshotConfig);
  });

  test('stop button hover state', async ({ page }) => {
    const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
    await stopButton.hover();
    await page.waitForTimeout(200);
    await expect(stopButton).toHaveScreenshot('stop-button-hover.png', screenshotConfig);
  });

  test('stop button active state', async ({ page }) => {
    const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
    await stopButton.hover();
    await page.mouse.down();
    await page.waitForTimeout(100);
    await expect(stopButton).toHaveScreenshot('stop-button-active.png', screenshotConfig);
    await page.mouse.up();
  });

  test('volume button hover state', async ({ page }) => {
    const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]').first();
    await volumeButton.hover();
    await page.waitForTimeout(200);
    await expect(volumeButton).toHaveScreenshot('volume-button-hover.png', screenshotConfig);
  });

  test('volume button active state', async ({ page }) => {
    const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]').first();
    await volumeButton.hover();
    await page.mouse.down();
    await page.waitForTimeout(100);
    await expect(volumeButton).toHaveScreenshot('volume-button-active.png', screenshotConfig);
    await page.mouse.up();
  });
});

test.describe('Top Bar Button Interaction States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('settings button hover state', async ({ page }) => {
    const settingsButton = page.locator('button[aria-label="Settings"]');
    await settingsButton.hover();
    await page.waitForTimeout(200);
    await expect(settingsButton).toHaveScreenshot('settings-button-hover.png', screenshotConfig);
  });

  test('settings button active state', async ({ page }) => {
    const settingsButton = page.locator('button[aria-label="Settings"]');
    await settingsButton.hover();
    await page.mouse.down();
    await page.waitForTimeout(100);
    await expect(settingsButton).toHaveScreenshot('settings-button-active.png', screenshotConfig);
    await page.mouse.up();
  });

  test('theme toggle button hover state', async ({ page }) => {
    const themeButton = page.locator('button[aria-label*="theme"], button[aria-label*="Theme"]').first();
    if (await themeButton.isVisible()) {
      await themeButton.hover();
      await page.waitForTimeout(200);
      await expect(themeButton).toHaveScreenshot('theme-button-hover.png', screenshotConfig);
    }
  });
});

test.describe('Input Interaction States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('search input filled state', async ({ page }) => {
    const searchInput = page.locator('input[placeholder*="Search"]');
    await searchInput.fill('test search query');
    await page.waitForTimeout(200);
    await expect(searchInput).toHaveScreenshot('search-input-filled.png', screenshotConfig);
  });

  test('search input with text and focus', async ({ page }) => {
    const searchInput = page.locator('input[placeholder*="Search"]');
    await searchInput.fill('audiobook');
    await searchInput.focus();
    await page.waitForTimeout(200);
    await expect(searchInput).toHaveScreenshot('search-input-filled-focus.png', screenshotConfig);
  });

  test('search input hover state', async ({ page }) => {
    const searchInput = page.locator('input[placeholder*="Search"]');
    await searchInput.hover();
    await page.waitForTimeout(200);
    await expect(searchInput).toHaveScreenshot('search-input-hover.png', screenshotConfig);
  });
});

test.describe('Seek Bar Interaction States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('seek bar active dragging state', async ({ page }) => {
    const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
    const box = await seekBar.boundingBox();
    if (box) {
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await page.mouse.down();
      await page.waitForTimeout(100);
      await expect(seekBar).toHaveScreenshot('seek-bar-dragging.png', screenshotConfig);
      await page.mouse.up();
    }
  });
});

test.describe('Card Interaction States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('audiobook card hover state light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    const firstCard = page.locator('.library-panel .card').first();
    if (await firstCard.isVisible()) {
      await firstCard.hover();
      await page.waitForTimeout(200);
      await expect(firstCard).toHaveScreenshot('card-hover-light.png', screenshotConfig);
    }
  });

  test('audiobook card hover state dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const firstCard = page.locator('.library-panel .card').first();
    if (await firstCard.isVisible()) {
      await firstCard.hover();
      await page.waitForTimeout(200);
      await expect(firstCard).toHaveScreenshot('card-hover-dark.png', screenshotConfig);
    }
  });

  test('audiobook card active/pressed state', async ({ page }) => {
    const firstCard = page.locator('.library-panel .card').first();
    if (await firstCard.isVisible()) {
      await firstCard.hover();
      await page.mouse.down();
      await page.waitForTimeout(100);
      await expect(firstCard).toHaveScreenshot('card-active.png', screenshotConfig);
      await page.mouse.up();
    }
  });
});

test.describe('Modal Button Interaction States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('modal close button hover state', async ({ page }) => {
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');
    await page.waitForTimeout(300);
    const closeButton = page.locator('.modal-box button[aria-label="Close"]');
    if (await closeButton.isVisible()) {
      await closeButton.hover();
      await page.waitForTimeout(200);
      await expect(closeButton).toHaveScreenshot('modal-close-hover.png', screenshotConfig);
    }
  });

  test('modal done button hover state', async ({ page }) => {
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');
    await page.waitForTimeout(300);
    const doneButton = page.locator('.modal-box button:has-text("Done")');
    if (await doneButton.isVisible()) {
      await doneButton.hover();
      await page.waitForTimeout(200);
      await expect(doneButton).toHaveScreenshot('modal-done-hover.png', screenshotConfig);
    }
  });

  test('modal done button active state', async ({ page }) => {
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');
    await page.waitForTimeout(300);
    const doneButton = page.locator('.modal-box button:has-text("Done")');
    if (await doneButton.isVisible()) {
      await doneButton.hover();
      await page.mouse.down();
      await page.waitForTimeout(100);
      await expect(doneButton).toHaveScreenshot('modal-done-active.png', screenshotConfig);
      await page.mouse.up();
    }
  });
});

test.describe('Speed Control Interaction States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('speed button hover state', async ({ page }) => {
    const speedButton = page.locator('.btm-nav button:has-text("x")').first();
    await speedButton.hover();
    await page.waitForTimeout(200);
    await expect(speedButton).toHaveScreenshot('speed-button-hover.png', screenshotConfig);
  });

  test('speed button active state', async ({ page }) => {
    const speedButton = page.locator('.btm-nav button:has-text("x")').first();
    await speedButton.hover();
    await page.mouse.down();
    await page.waitForTimeout(100);
    await expect(speedButton).toHaveScreenshot('speed-button-active.png', screenshotConfig);
    await page.mouse.up();
  });
});
