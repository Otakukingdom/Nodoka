/**
 * E2E screenshot tests for player controls
 */

import { test, expect } from './fixtures';
import { screenshotConfig, gotoAndWait } from './test-utils';

test.describe('Player Controls Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('player controls screenshot', async ({ page }) => {
    const playerControls = page.locator('.btm-nav');
    await expect(playerControls).toHaveScreenshot('player-controls.png', screenshotConfig);
  });

  test('play button screenshot', async ({ page }) => {
    const playButton = page
      .locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]')
      .first();
    await expect(playButton).toHaveScreenshot('play-button.png', screenshotConfig);
  });

  test('speed control button screenshot', async ({ page }) => {
    const speedButton = page.locator('.btm-nav button:has-text("x")');
    await expect(speedButton.first()).toHaveScreenshot('speed-button.png', screenshotConfig);
  });

  test('seek bar screenshot', async ({ page }) => {
    const seekBar = page.locator('.btm-nav input[type="range"]');
    await expect(seekBar).toHaveScreenshot('seek-bar.png', screenshotConfig);
  });

  test('seek bar hover state screenshot', async ({ page }) => {
    const seekBarContainer = page.locator('.btm-nav .flex-1.relative.group');
    await seekBarContainer.hover();
    await page.waitForTimeout(200);
    await expect(seekBarContainer).toHaveScreenshot('seek-bar-hover.png', screenshotConfig);
  });

  test('volume button screenshot', async ({ page }) => {
    const volumeButton = page.locator('.btm-nav button[aria-label="Volume"]');
    await expect(volumeButton).toHaveScreenshot('volume-button.png', screenshotConfig);
  });

  test('stop button screenshot', async ({ page }) => {
    const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
    await expect(stopButton).toHaveScreenshot('stop-button.png', screenshotConfig);
  });
});

test.describe('Player Controls Component Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('player controls has play button', async ({ page }) => {
    const playButton = page.locator(
      '.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]'
    );
    await expect(playButton).toBeVisible();
  });
});
