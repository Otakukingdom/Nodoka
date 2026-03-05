/**
 * E2E screenshot tests for animation states
 * Captures visual appearance of CSS animations in the UI
 */

import { test, expect } from './fixtures';
import { screenshotConfig, gotoAndWait, setTheme } from './test-utils';

test.describe('Logo Animation Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('logo hover pulse animation', async ({ page }) => {
    const logo = page.locator('.navbar .gradient-premium');
    await logo.hover();
    await page.waitForTimeout(300);
    // Capture with animations allowed to see pulse effect
    await expect(logo).toHaveScreenshot(
      'logo-pulse-animation.png',
      { ...screenshotConfig, animations: 'allow' }
    );
  });

  test('logo static state', async ({ page }) => {
    const logo = page.locator('.navbar .gradient-premium');
    // Capture without hover to see static state
    await expect(logo).toHaveScreenshot(
      'logo-static.png',
      screenshotConfig
    );
  });
});

test.describe('Playing Indicator Animation', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('play button pulse-playing animation', async ({ page }) => {
    // Look for play button with pulse animation
    const playButton = page.locator('.btm-nav .pulse-playing').first();
    if (await playButton.isVisible()) {
      await expect(playButton).toHaveScreenshot(
        'play-button-pulse-animation.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});

test.describe('Card Hover Animation', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('card lift hover effect light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    const card = page.locator('.library-panel .card').first();
    if (await card.isVisible()) {
      await card.hover();
      await page.waitForTimeout(300);
      await expect(card).toHaveScreenshot(
        'card-hover-lift-light.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });

  test('card lift hover effect dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const card = page.locator('.library-panel .card').first();
    if (await card.isVisible()) {
      await card.hover();
      await page.waitForTimeout(300);
      await expect(card).toHaveScreenshot(
        'card-hover-lift-dark.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});

test.describe('Modal Animation', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('modal scale-in animation', async ({ page }) => {
    // Open modal and capture during animation
    await page.click('button[aria-label="Settings"]');
    await page.waitForTimeout(50); // Short wait to catch animation
    const modal = page.locator('.modal-box');
    if (await modal.isVisible()) {
      await expect(modal).toHaveScreenshot(
        'modal-scale-in-animation.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });

  test('modal fade-in backdrop', async ({ page }) => {
    await page.click('button[aria-label="Settings"]');
    await page.waitForSelector('.modal-box');
    await page.waitForTimeout(300);
    const backdrop = page.locator('.modal');
    if (await backdrop.isVisible()) {
      await expect(backdrop).toHaveScreenshot(
        'modal-backdrop-fade-animation.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});

test.describe('Shimmer Loading Animation', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('shimmer effect on skeleton', async ({ page }) => {
    const shimmer = page.locator('.shimmer, .skeleton-shimmer').first();
    if (await shimmer.isVisible()) {
      await expect(shimmer).toHaveScreenshot(
        'shimmer-animation.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });

  test('skeleton loading animation', async ({ page }) => {
    const skeleton = page.locator('.skeleton').first();
    if (await skeleton.isVisible()) {
      await expect(skeleton).toHaveScreenshot(
        'skeleton-animation.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});

test.describe('Button Interaction Animation', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('settings button rotate animation', async ({ page }) => {
    const settingsButton = page.locator('button[aria-label="Settings"]');
    await settingsButton.hover();
    await page.waitForTimeout(300);
    // The gear icon should rotate on hover
    await expect(settingsButton).toHaveScreenshot(
      'settings-rotate-animation.png',
      { ...screenshotConfig, animations: 'allow' }
    );
  });

  test('button scale on hover', async ({ page }) => {
    const playButton = page
      .locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]')
      .first();
    await playButton.hover();
    await page.waitForTimeout(200);
    await expect(playButton).toHaveScreenshot(
      'button-scale-hover.png',
      { ...screenshotConfig, animations: 'allow' }
    );
  });
});

test.describe('Theme Transition Animation', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('theme transition light to dark', async ({ page }) => {
    await setTheme(page, 'Light');
    await page.waitForTimeout(300);
    // Capture page during theme transition
    await setTheme(page, 'Dark');
    await page.waitForTimeout(200); // Mid-transition - increased for stability
    await expect(page).toHaveScreenshot(
      'theme-transition-mid.png',
      { ...screenshotConfig, animations: 'allow', fullPage: true, maxDiffPixelRatio: 0.05 }
    );
  });
});

test.describe('Notification Animation', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('notification slide-in animation', async ({ page }) => {
    const notification = page.locator('.toast-slide-in, .alert').first();
    if (await notification.isVisible()) {
      await expect(notification).toHaveScreenshot(
        'notification-slide-animation.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});

test.describe('Card Entrance Animation', () => {
  test('card entrance animation', async ({ page }) => {
    await page.goto('/');
    // Try to catch card entrance animation
    const card = page.locator('.card-animate-in').first();
    if (await card.isVisible()) {
      await expect(card).toHaveScreenshot(
        'card-entrance-animation.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});
