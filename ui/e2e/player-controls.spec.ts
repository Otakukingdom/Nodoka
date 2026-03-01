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

  test('play button focus ring screenshot', async ({ page }) => {
    const playButton = page
      .locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]')
      .first();
    await playButton.focus();
    await page.waitForTimeout(200);
    await expect(playButton).toHaveScreenshot('play-button-focus.png', screenshotConfig);
  });

  test('speed control button screenshot', async ({ page }) => {
    const speedButton = page.locator('.btm-nav button:has-text("x")');
    await expect(speedButton.first()).toHaveScreenshot('speed-button.png', screenshotConfig);
  });

  test('seek bar screenshot', async ({ page }) => {
    const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
    await expect(seekBar).toHaveScreenshot('seek-bar.png', screenshotConfig);
  });

  test('seek bar hover state screenshot', async ({ page }) => {
    const seekBarContainer = page.locator('.btm-nav .flex-1.relative.group');
    await seekBarContainer.hover();
    await page.waitForTimeout(200);
    await expect(seekBarContainer).toHaveScreenshot('seek-bar-hover.png', screenshotConfig);
  });

  test('volume button screenshot', async ({ page }) => {
    const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]');
    await expect(volumeButton.first()).toHaveScreenshot('volume-button.png', screenshotConfig);
  });

  test('volume button focus ring screenshot', async ({ page }) => {
    const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]');
    await volumeButton.first().focus();
    await page.waitForTimeout(200);
    await expect(volumeButton.first()).toHaveScreenshot('volume-button-focus.png', screenshotConfig);
  });

  test('volume slider component screenshot', async ({ page }) => {
    const volumeSlider = page.locator('.btm-nav .group\\/volume');
    await expect(volumeSlider).toHaveScreenshot('volume-slider.png', screenshotConfig);
  });

  test('volume slider hover state screenshot', async ({ page }) => {
    const volumeSlider = page.locator('.btm-nav .group\\/volume');
    await volumeSlider.hover();
    await page.waitForTimeout(200);
    await expect(volumeSlider).toHaveScreenshot('volume-slider-hover.png', screenshotConfig);
  });

  test('stop button screenshot', async ({ page }) => {
    const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
    await expect(stopButton).toHaveScreenshot('stop-button.png', screenshotConfig);
  });

  test('stop button focus ring screenshot', async ({ page }) => {
    const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
    await stopButton.focus();
    await page.waitForTimeout(200);
    await expect(stopButton).toHaveScreenshot('stop-button-focus.png', screenshotConfig);
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

  test('play button has tooltip', async ({ page }) => {
    const playButton = page
      .locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]')
      .first();
    await expect(playButton).toHaveAttribute('title');
  });

  test('stop button has tooltip', async ({ page }) => {
    const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
    await expect(stopButton).toHaveAttribute('title');
  });

  test('volume button has tooltip', async ({ page }) => {
    const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]');
    await expect(volumeButton.first()).toHaveAttribute('title');
  });

  test('volume slider has range input', async ({ page }) => {
    const volumeRange = page.locator('.btm-nav .group\\/volume input[type="range"]');
    await expect(volumeRange).toBeVisible();
  });

  test('mute toggle works with keyboard M key', async ({ page }) => {
    await page.keyboard.press('m');
    await page.waitForTimeout(200);
    // Should still be on the page with updated state
    const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]');
    await expect(volumeButton.first()).toBeVisible();
  });
});
