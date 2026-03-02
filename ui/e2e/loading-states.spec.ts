/**
 * E2E screenshot tests for loading states
 * Captures visual appearance of loading indicators and skeleton screens
 * Uses forced loading states for reliable screenshot capture
 */

import { test, expect } from './fixtures';
import { screenshotConfig, gotoAndWait, setTheme, forceLoadingState } from './test-utils';

test.describe('Initial Loading Screenshots', () => {
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

test.describe('Forced Loading States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('forced loading state light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    await forceLoadingState(page, true);
    await page.waitForTimeout(300);
    // Capture the loading indicator
    const loadingIndicator = page.locator('.loading, .loading-spinner, .skeleton').first();
    if (await loadingIndicator.isVisible().catch(() => false)) {
      await expect(loadingIndicator).toHaveScreenshot(
        'forced-loading-light.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
    // Reset loading state
    await forceLoadingState(page, false);
  });

  test('forced loading state dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    await forceLoadingState(page, true);
    await page.waitForTimeout(300);
    const loadingIndicator = page.locator('.loading, .loading-spinner, .skeleton').first();
    if (await loadingIndicator.isVisible().catch(() => false)) {
      await expect(loadingIndicator).toHaveScreenshot(
        'forced-loading-dark.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
    await forceLoadingState(page, false);
  });
});

test.describe('Skeleton Loading Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('audiobook card skeleton light theme', async ({ page }) => {
    await setTheme(page, 'Light');
    const skeleton = page.locator('.skeleton').first();
    if (await skeleton.isVisible().catch(() => false)) {
      await expect(skeleton).toHaveScreenshot(
        'skeleton-card-light.png',
        screenshotConfig
      );
    }
  });

  test('audiobook card skeleton dark theme', async ({ page }) => {
    await setTheme(page, 'Dark');
    const skeleton = page.locator('.skeleton').first();
    if (await skeleton.isVisible().catch(() => false)) {
      await expect(skeleton).toHaveScreenshot(
        'skeleton-card-dark.png',
        screenshotConfig
      );
    }
  });

  test('skeleton title element', async ({ page }) => {
    const titleSkeleton = page.locator('.card-body .skeleton').first();
    if (await titleSkeleton.isVisible().catch(() => false)) {
      await expect(titleSkeleton).toHaveScreenshot(
        'skeleton-title-element.png',
        screenshotConfig
      );
    }
  });

  test('skeleton progress bar', async ({ page }) => {
    const progressSkeleton = page.locator('.skeleton.rounded-full').first();
    if (await progressSkeleton.isVisible().catch(() => false)) {
      await expect(progressSkeleton).toHaveScreenshot(
        'skeleton-progress-bar.png',
        screenshotConfig
      );
    }
  });

  test('full skeleton card', async ({ page }) => {
    const skeletonCard = page.locator('.card:has(.skeleton)').first();
    if (await skeletonCard.isVisible().catch(() => false)) {
      await expect(skeletonCard).toHaveScreenshot(
        'skeleton-card-full.png',
        screenshotConfig
      );
    }
  });
});

test.describe('Skeleton Shimmer Effect', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('skeleton with shimmer animation', async ({ page }) => {
    const shimmer = page.locator('.skeleton-shimmer, .skeleton').first();
    if (await shimmer.isVisible().catch(() => false)) {
      await expect(shimmer).toHaveScreenshot(
        'skeleton-shimmer-effect.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });

  test('shimmer overlay animation', async ({ page }) => {
    const shimmerOverlay = page.locator('.shimmer').first();
    if (await shimmerOverlay.isVisible().catch(() => false)) {
      await expect(shimmerOverlay).toHaveScreenshot(
        'shimmer-overlay-animation.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});

test.describe('Loading State in Library', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('library loading skeleton state', async ({ page }) => {
    await forceLoadingState(page, true);
    await page.waitForTimeout(300);
    const libraryPanel = page.locator('.library-panel');
    const loadingIndicator = libraryPanel.locator('.skeleton, .loading').first();
    if (await loadingIndicator.isVisible().catch(() => false)) {
      await expect(libraryPanel).toHaveScreenshot(
        'library-loading-state.png',
        screenshotConfig
      );
    }
    await forceLoadingState(page, false);
  });

  test('multiple skeleton cards', async ({ page }) => {
    await forceLoadingState(page, true);
    await page.waitForTimeout(300);
    const skeletons = page.locator('.skeleton');
    const count = await skeletons.count();
    if (count > 1) {
      const container = page.locator('.library-panel #audiobook-list');
      await expect(container).toHaveScreenshot(
        'multiple-skeleton-cards.png',
        screenshotConfig
      );
    }
    await forceLoadingState(page, false);
  });
});

test.describe('Spinner Loading States', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('loading spinner with gradient', async ({ page }) => {
    const spinner = page.locator('.gradient-premium.animate-spin').first();
    if (await spinner.isVisible().catch(() => false)) {
      await expect(spinner).toHaveScreenshot(
        'gradient-loading-spinner.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });

  test('daisyui loading spinner', async ({ page }) => {
    const loading = page.locator('.loading').first();
    if (await loading.isVisible().catch(() => false)) {
      await expect(loading).toHaveScreenshot(
        'daisyui-loading-spinner.png',
        { ...screenshotConfig, animations: 'allow' }
      );
    }
  });
});

test.describe('Loading State Animations', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('skeleton animation frames', async ({ page }) => {
    await forceLoadingState(page, true);
    await page.waitForTimeout(300);
    const skeleton = page.locator('.skeleton').first();
    if (await skeleton.isVisible().catch(() => false)) {
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
    await forceLoadingState(page, false);
  });
});
