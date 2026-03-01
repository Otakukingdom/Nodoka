/**
 * E2E screenshot tests for loading states
 * Captures visual appearance of loading indicators and skeleton screens
 */

import { test, expect } from './fixtures';
import { screenshotConfig, gotoAndWait, setTheme } from './test-utils';

test.describe('Initial Loading Screenshots', () => {
  test('initial loading spinner', async ({ page }) => {
    await page.goto('/');
    // Try to catch the loading spinner before app is ready
    const spinner = page.locator('.gradient-premium.animate-spin, .loading-spinner, .loading').first();
    const isVisible = await spinner.isVisible({ timeout: 2000 }).catch(() => false);
    if (isVisible) {
      await expect(spinner).toHaveScreenshot(
        'initial-loading-spinner.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });

  test('page loading state before ready', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    // Capture the initial state before full app load
    await expect(page).toHaveScreenshot(
      'page-initial-loading.png',
      { ...screenshotConfig, fullPage: true, animations: 'allow' }
    );
  });
});

test.describe('Skeleton Loading Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('audiobook card skeleton light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    const skeleton = page.locator('.skeleton').first();
    if (await skeleton.isVisible()) {
      await expect(skeleton).toHaveScreenshot(
        'skeleton-card-light.png',
        screenshotConfig
      );
    }
  });

  test('audiobook card skeleton dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const skeleton = page.locator('.skeleton').first();
    if (await skeleton.isVisible()) {
      await expect(skeleton).toHaveScreenshot(
        'skeleton-card-dark.png',
        screenshotConfig
      );
    }
  });

  test('skeleton title element', async ({ page }) => {
    // Navigate and try to catch skeleton state
    await page.goto('/');
    const titleSkeleton = page.locator('.card-body .skeleton').first();
    const isVisible = await titleSkeleton.isVisible({ timeout: 3000 }).catch(() => false);
    if (isVisible) {
      await expect(titleSkeleton).toHaveScreenshot(
        'skeleton-title-element.png',
        screenshotConfig
      );
    }
  });

  test('skeleton progress bar', async ({ page }) => {
    await page.goto('/');
    const progressSkeleton = page.locator('.skeleton.rounded-full').first();
    const isVisible = await progressSkeleton.isVisible({ timeout: 3000 }).catch(() => false);
    if (isVisible) {
      await expect(progressSkeleton).toHaveScreenshot(
        'skeleton-progress-bar.png',
        screenshotConfig
      );
    }
  });

  test('full skeleton card', async ({ page }) => {
    await page.goto('/');
    const skeletonCard = page.locator('.card:has(.skeleton)').first();
    const isVisible = await skeletonCard.isVisible({ timeout: 3000 }).catch(() => false);
    if (isVisible) {
      await expect(skeletonCard).toHaveScreenshot(
        'skeleton-card-full.png',
        screenshotConfig
      );
    }
  });
});

test.describe('Skeleton Shimmer Effect', () => {
  test('skeleton with shimmer animation', async ({ page }) => {
    await page.goto('/');
    const shimmer = page.locator('.skeleton-shimmer, .skeleton').first();
    const isVisible = await shimmer.isVisible({ timeout: 3000 }).catch(() => false);
    if (isVisible) {
      await expect(shimmer).toHaveScreenshot(
        'skeleton-shimmer-effect.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });

  test('shimmer overlay animation', async ({ page }) => {
    await page.goto('/');
    const shimmerOverlay = page.locator('.shimmer').first();
    const isVisible = await shimmerOverlay.isVisible({ timeout: 3000 }).catch(() => false);
    if (isVisible) {
      await expect(shimmerOverlay).toHaveScreenshot(
        'shimmer-overlay-animation.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});

test.describe('Loading State in Library', () => {
  test('library loading skeleton state', async ({ page }) => {
    await page.goto('/');
    // Wait briefly to catch library loading
    await page.waitForTimeout(500);
    const libraryPanel = page.locator('.library-panel');
    // Look for loading indicator in library
    const loadingIndicator = libraryPanel.locator('.skeleton, .loading').first();
    const isVisible = await loadingIndicator.isVisible({ timeout: 2000 }).catch(() => false);
    if (isVisible) {
      await expect(libraryPanel).toHaveScreenshot(
        'library-loading-state.png',
        screenshotConfig
      );
    }
  });

  test('multiple skeleton cards', async ({ page }) => {
    await page.goto('/');
    await page.waitForTimeout(500);
    const skeletons = page.locator('.skeleton');
    const count = await skeletons.count();
    if (count > 1) {
      // Capture area with multiple skeletons
      const container = page.locator('.library-panel #audiobook-list');
      await expect(container).toHaveScreenshot(
        'multiple-skeleton-cards.png',
        screenshotConfig
      );
    }
  });
});

test.describe('Spinner Loading States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('loading spinner with gradient', async ({ page }) => {
    const spinner = page.locator('.gradient-premium.animate-spin').first();
    if (await spinner.isVisible()) {
      await expect(spinner).toHaveScreenshot(
        'gradient-loading-spinner.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });

  test('daisyui loading spinner', async ({ page }) => {
    const loading = page.locator('.loading').first();
    if (await loading.isVisible()) {
      await expect(loading).toHaveScreenshot(
        'daisyui-loading-spinner.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});

test.describe('Loading State Animations', () => {
  test('skeleton animation frames', async ({ page }) => {
    await page.goto('/');
    const skeleton = page.locator('.skeleton').first();
    const isVisible = await skeleton.isVisible({ timeout: 3000 }).catch(() => false);
    if (isVisible) {
      // Capture multiple animation frames
      await expect(skeleton).toHaveScreenshot(
        'skeleton-frame-1.png',
        { ...screenshotConfig, animations: 'allow' }
      );
      await page.waitForTimeout(500);
      await expect(skeleton).toHaveScreenshot(
        'skeleton-frame-2.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});
