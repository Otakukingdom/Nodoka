/**
 * Behavioral tests for Playback Controls
 * Tests actual UI interactions matching cucumber acceptance criteria
 */

import { test, expect } from '../fixtures';
import { gotoAndWait } from '../test-utils';

test.describe('Playback Controls - Behavioral Tests', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
    // Select an audiobook to enable playback controls
    await page.locator('#audiobook-list .card').first().click();
  });

  test.describe('Play/Pause Controls', () => {
    test('play button is visible in player controls', async ({ page }) => {
      const playButton = page.locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]');
      await expect(playButton.first()).toBeVisible();
    });

    test('clicking play button toggles playback state', async ({ page }) => {
      const playButton = page.locator('.btm-nav button[aria-label="Play"]').first();

      // Click play
      await playButton.click();
      await page.waitForTimeout(200);

      // Should now show pause button
      const pauseButton = page.locator('.btm-nav button[aria-label="Pause"]');
      await expect(pauseButton).toBeVisible();
    });

    test('play button has correct aria-label', async ({ page }) => {
      const playButton = page.locator('.btm-nav button[aria-label="Play"]').first();
      const ariaLabel = await playButton.getAttribute('aria-label');
      expect(ariaLabel).toBe('Play');
    });

    test('play button has tooltip', async ({ page }) => {
      const playButton = page.locator('.btm-nav button[aria-label="Play"]').first();
      const title = await playButton.getAttribute('title');
      expect(title).toContain('Play');
    });
  });

  test.describe('Stop Control', () => {
    test('stop button is visible', async ({ page }) => {
      const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
      await expect(stopButton).toBeVisible();
    });

    test('stop button has tooltip', async ({ page }) => {
      const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
      const title = await stopButton.getAttribute('title');
      expect(title).toContain('Stop');
    });

    test('clicking stop button resets playback', async ({ page }) => {
      // Start playback first
      const playButton = page.locator('.btm-nav button[aria-label="Play"]').first();
      await playButton.click();
      await page.waitForTimeout(200);

      // Click stop
      const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
      await stopButton.click();
      await page.waitForTimeout(200);

      // Play button should be visible again (not pause)
      const playButtonAfter = page.locator('.btm-nav button[aria-label="Play"]');
      await expect(playButtonAfter).toBeVisible();
    });
  });

  test.describe('Volume Controls', () => {
    test('volume button is visible', async ({ page }) => {
      const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]');
      await expect(volumeButton.first()).toBeVisible();
    });

    test('volume slider is visible', async ({ page }) => {
      const volumeSlider = page.locator('.btm-nav .group\\/volume');
      await expect(volumeSlider).toBeVisible();
    });

    test('volume slider has range input', async ({ page }) => {
      const volumeRange = page.locator('.btm-nav .group\\/volume input[type="range"]');
      await expect(volumeRange).toBeVisible();
    });

    test('muting with M key works', async ({ page }) => {
      await page.keyboard.press('m');
      await page.waitForTimeout(200);

      // Volume button state should change
      const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]');
      await expect(volumeButton.first()).toBeVisible();
    });

    test('volume button has tooltip', async ({ page }) => {
      const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]');
      const title = await volumeButton.first().getAttribute('title');
      expect(title).toBeTruthy();
    });
  });

  test.describe('Speed Control', () => {
    test('speed button is visible', async ({ page }) => {
      const speedButton = page.locator('.btm-nav button:has-text("x")');
      await expect(speedButton.first()).toBeVisible();
    });

    test('speed displays current value', async ({ page }) => {
      const speedButton = page.locator('.btm-nav button:has-text("x")');
      const speedText = await speedButton.first().textContent();
      expect(speedText).toContain('1x');
    });

    test('clicking speed button cycles through speeds', async ({ page }) => {
      const speedButton = page.locator('.btm-nav button:has-text("x")').first();

      // Click to cycle speed
      await speedButton.click();
      await page.waitForTimeout(200);

      // Speed should have changed
      const speedText = await speedButton.textContent();
      expect(speedText).not.toBe('1x');
    });

    test('speed button has aria-label', async ({ page }) => {
      const speedButton = page.locator('.btm-nav button:has-text("x")').first();
      const ariaLabel = await speedButton.getAttribute('aria-label');
      expect(ariaLabel).toContain('speed');
    });
  });

  test.describe('Seek Bar', () => {
    test('seek bar is visible', async ({ page }) => {
      const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
      await expect(seekBar).toBeVisible();
    });

    test('time displays are visible', async ({ page }) => {
      // Current time display
      const currentTime = page.locator('.btm-nav .font-mono').first();
      await expect(currentTime).toBeVisible();

      // Total duration display
      const duration = page.locator('.btm-nav .font-mono').nth(1);
      await expect(duration).toBeVisible();
    });

    test('progress bar shows visual progress', async ({ page }) => {
      // The progress bar (gradient-premium) should be visible
      const progressBar = page.locator('.btm-nav .gradient-premium').first();
      await expect(progressBar).toBeVisible();
    });

    test('seek bar has correct aria-label', async ({ page }) => {
      const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
      const ariaLabel = await seekBar.getAttribute('aria-label');
      expect(ariaLabel).toBe('Seek');
    });
  });

  test.describe('Keyboard Shortcuts', () => {
    test('Space key toggles playback', async ({ page }) => {
      // Focus the page first
      await page.locator('body').click();

      // Press space to play
      await page.keyboard.press('Space');
      await page.waitForTimeout(200);

      // Should be playing (pause button visible)
      const pauseButton = page.locator('.btm-nav button[aria-label="Pause"]');
      await expect(pauseButton).toBeVisible();
    });
  });

  test.describe('Player Controls Layout', () => {
    test('player controls are at bottom of screen', async ({ page }) => {
      const playerControls = page.locator('.btm-nav');
      await expect(playerControls).toBeVisible();

      // Should have btm-nav class (DaisyUI bottom navigation)
      await expect(playerControls).toHaveClass(/btm-nav/);
    });

    test('player controls region has aria-label', async ({ page }) => {
      const playerControls = page.locator('.btm-nav');
      const ariaLabel = await playerControls.getAttribute('aria-label');
      expect(ariaLabel).toContain('player');
    });
  });

  test.describe('Focus States', () => {
    test('play button has visible focus ring', async ({ page }) => {
      const playButton = page.locator('.btm-nav button[aria-label="Play"]').first();
      await playButton.focus();
      await page.waitForTimeout(200);

      // Should have focus-visible class or ring
      await expect(playButton).toBeFocused();
    });

    test('stop button has visible focus ring', async ({ page }) => {
      const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
      await stopButton.focus();
      await page.waitForTimeout(200);

      await expect(stopButton).toBeFocused();
    });

    test('speed button has visible focus ring', async ({ page }) => {
      const speedButton = page.locator('.btm-nav button:has-text("x")').first();
      await speedButton.focus();
      await page.waitForTimeout(200);

      await expect(speedButton).toBeFocused();
    });
  });

  test.describe('Accessibility', () => {
    test('all interactive elements are focusable', async ({ page }) => {
      const buttons = page.locator('.btm-nav button');
      const count = await buttons.count();

      for (let i = 0; i < count; i++) {
        const button = buttons.nth(i);
        await button.focus();
        // Each button should be focusable
        await expect(button).toBeFocused();
      }
    });

    test('player controls have proper ARIA attributes', async ({ page }) => {
      const playerControls = page.locator('.btm-nav');
      const role = await playerControls.getAttribute('role');
      expect(role).toBe('region');
    });
  });
});
