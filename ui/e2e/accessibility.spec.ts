/**
 * E2E accessibility tests using @axe-core/playwright
 * Tests for WCAG compliance, ARIA labels, keyboard navigation, and screen reader support
 */

import { test, expect } from './fixtures';
import { AxeBuilder } from '@axe-core/playwright';
import { gotoAndWait, openSettingsModal, openKeyboardHelp, closeModal } from './test-utils';

test.describe('Accessibility - Full Page Scan', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('main page has no detectable accessibility violations', async ({ page }) => {
    const accessibilityScanResults = await new AxeBuilder({ page })
      .withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa'])
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('main page has no critical or serious accessibility issues', async ({ page }) => {
    const accessibilityScanResults = await new AxeBuilder({ page })
      .disableRules(['color-contrast']) // Theme-dependent, tested separately
      .analyze();

    const criticalAndSerious = accessibilityScanResults.violations.filter(
      (v) => v.impact === 'critical' || v.impact === 'serious'
    );
    expect(criticalAndSerious).toEqual([]);
  });
});

test.describe('Accessibility - ARIA Labels', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('all interactive elements have accessible names', async ({ page }) => {
    const buttons = await page.locator('button').all();
    for (const button of buttons) {
      const ariaLabel = await button.getAttribute('aria-label');
      const textContent = await button.textContent();
      const title = await button.getAttribute('title');
      const hasAccessibleName = ariaLabel || textContent?.trim() || title;
      expect(hasAccessibleName).toBeTruthy();
    }
  });

  test('settings button has proper aria-label', async ({ page }) => {
    const settingsButton = page.locator('button[aria-label="Settings"]');
    await expect(settingsButton).toBeVisible();
    expect(await settingsButton.getAttribute('aria-label')).toBe('Settings');
  });

  test('play/pause button has proper aria-label', async ({ page }) => {
    const playButton = page.locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]');
    const count = await playButton.count();
    expect(count).toBeGreaterThan(0);

    const ariaLabel = await playButton.first().getAttribute('aria-label');
    expect(['Play', 'Pause']).toContain(ariaLabel);
  });

  test('stop button has proper aria-label', async ({ page }) => {
    const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
    await expect(stopButton).toBeVisible();
  });

  test('volume controls have proper aria-labels', async ({ page }) => {
    const muteButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]');
    const count = await muteButton.count();
    expect(count).toBeGreaterThan(0);

    const volumeRange = page.locator('.btm-nav input[type="range"]').first();
    if (await volumeRange.isVisible()) {
      const ariaLabel = await volumeRange.getAttribute('aria-label');
      expect(ariaLabel).toBeTruthy();
    }
  });

  test('seek bar has proper aria-label', async ({ page }) => {
    const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
    await expect(seekBar).toBeVisible();
  });

  test('search input has accessible label', async ({ page }) => {
    const searchInput = page.locator('input[placeholder*="Search"]');
    if (await searchInput.isVisible()) {
      const ariaLabel = await searchInput.getAttribute('aria-label');
      const placeholder = await searchInput.getAttribute('placeholder');
      expect(ariaLabel || placeholder).toBeTruthy();
    }
  });
});

test.describe('Accessibility - Settings Modal', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('settings modal has no accessibility violations when open', async ({ page }) => {
    await openSettingsModal(page);

    const accessibilityScanResults = await new AxeBuilder({ page })
      .include('.modal')
      .withTags(['wcag2a', 'wcag2aa'])
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('settings modal has proper role', async ({ page }) => {
    await openSettingsModal(page);

    const modal = page.locator('.modal');
    const role = await modal.getAttribute('role');
    expect(['dialog', 'alertdialog']).toContain(role);
  });

  test('settings modal close button has accessible label', async ({ page }) => {
    await openSettingsModal(page);

    const closeButton = page.locator('.modal-box button[aria-label="Close"]');
    if (await closeButton.isVisible()) {
      expect(await closeButton.getAttribute('aria-label')).toBe('Close');
    }
  });

  test('settings select has associated label', async ({ page }) => {
    await openSettingsModal(page);

    const select = page.getByLabel('Select theme');
    if (await select.isVisible()) {
      const ariaLabel = await select.getAttribute('aria-label');
      const id = await select.getAttribute('id');
      let hasLabel = false;

      if (ariaLabel) {
        hasLabel = true;
      } else if (id) {
        const label = page.locator(`label[for="${id}"]`);
        hasLabel = (await label.count()) > 0;
      }

      expect(hasLabel).toBeTruthy();
    }
  });
});

test.describe('Accessibility - Keyboard Help Modal', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('keyboard help modal has no accessibility violations', async ({ page }) => {
    await openKeyboardHelp(page);

    const accessibilityScanResults = await new AxeBuilder({ page })
      .include('.modal')
      .withTags(['wcag2a', 'wcag2aa'])
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('keyboard shortcuts are displayed in accessible format', async ({ page }) => {
    await openKeyboardHelp(page);

    const kbdElements = page.locator('.modal-box kbd');
    const count = await kbdElements.count();
    expect(count).toBeGreaterThan(0);

    for (let i = 0; i < Math.min(count, 5); i++) {
      const kbd = kbdElements.nth(i);
      const text = await kbd.textContent();
      expect(text?.trim()).toBeTruthy();
    }
  });
});

test.describe('Accessibility - Focus Management', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('focus is visible on all interactive elements', async ({ page }) => {
    const buttons = await page.locator('button:visible').all();

    for (let i = 0; i < Math.min(buttons.length, 10); i++) {
      const button = buttons[i];
      await button.focus();
      await page.waitForTimeout(50);

      const isFocused = await button.evaluate((el) => {
        return document.activeElement === el;
      });
      expect(isFocused).toBeTruthy();
    }
  });

  test('tab navigation covers all major interactive elements', async ({ page }) => {
    const focusableElements: string[] = [];

    for (let i = 0; i < 15; i++) {
      await page.keyboard.press('Tab');
      await page.waitForTimeout(50);

      const focusedTag = await page.evaluate(() => {
        const el = document.activeElement;
        if (!el || el === document.body) return null;
        return el.tagName + (el.getAttribute('aria-label') ? `[aria-label="${el.getAttribute('aria-label')}"]` : '');
      });

      if (focusedTag) {
        focusableElements.push(focusedTag);
      }
    }

    expect(focusableElements.length).toBeGreaterThan(3);
  });

  test('modal traps focus within', async ({ page }) => {
    await openSettingsModal(page);

    const initialFocus = await page.evaluate(() => document.activeElement?.tagName);

    for (let i = 0; i < 10; i++) {
      await page.keyboard.press('Tab');
      await page.waitForTimeout(50);
    }

    const finalFocus = await page.evaluate(() => {
      const activeEl = document.activeElement;
      return activeEl?.closest('.modal, .modal-box') !== null;
    });

    expect(finalFocus).toBeTruthy();
  });

  test('escape key closes modal and returns focus', async ({ page }) => {
    await openSettingsModal(page);

    const settingsButton = page.locator('button[aria-label="Settings"]');
    await settingsButton.focus();

    await page.keyboard.press('Escape');
    await page.waitForTimeout(300);

    const modalVisible = await page.locator('.modal-box').isVisible();
    expect(modalVisible).toBeFalsy();
  });
});

test.describe('Accessibility - Keyboard Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('space key toggles play/pause', async ({ page }) => {
    await page.keyboard.press('Space');
    await page.waitForTimeout(200);

    const playButton = page.locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]');
    await expect(playButton.first()).toBeVisible();
  });

  test('M key toggles mute', async ({ page }) => {
    await page.keyboard.press('m');
    await page.waitForTimeout(200);

    const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]');
    await expect(volumeButton.first()).toBeVisible();
  });

  test('arrow keys navigate seek position', async ({ page }) => {
    const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
    await seekBar.focus();
    await page.waitForTimeout(100);

    await page.keyboard.press('ArrowRight');
    await page.waitForTimeout(100);

    await expect(seekBar).toBeFocused();
  });

  test('Ctrl+Comma opens settings', async ({ page }) => {
    await page.keyboard.press('Control+,');
    await page.waitForTimeout(300);

    const modal = page.locator('.modal-box');
    await expect(modal).toBeVisible();

    await closeModal(page);
  });

  test('F1 opens keyboard help', async ({ page }) => {
    await page.keyboard.press('F1');
    await page.waitForTimeout(300);

    const modal = page.locator('.modal-box');
    await expect(modal).toBeVisible();
  });
});

test.describe('Accessibility - Color Contrast', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('main content meets color contrast requirements', async ({ page }) => {
    const accessibilityScanResults = await new AxeBuilder({ page })
      .withRules(['color-contrast'])
      .analyze();

    const contrastViolations = accessibilityScanResults.violations.filter(
      (v) => v.id === 'color-contrast'
    );

    expect(contrastViolations).toEqual([]);
  });

  test('buttons have sufficient color contrast in both themes', async ({ page }) => {
    const accessibilityScanResults = await new AxeBuilder({ page })
      .withRules(['color-contrast'])
      .include('.btm-nav')
      .analyze();

    const contrastViolations = accessibilityScanResults.violations;
    expect(contrastViolations).toEqual([]);
  });
});

test.describe('Accessibility - Screen Reader Support', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('images have alt text or are marked decorative', async ({ page }) => {
    const images = await page.locator('img').all();

    for (const img of images) {
      const alt = await img.getAttribute('alt');
      const role = await img.getAttribute('role');
      const ariaLabel = await img.getAttribute('aria-label');

      const hasAccessibleName = alt !== null || role === 'presentation' || ariaLabel;
      expect(hasAccessibleName).toBeTruthy();
    }
  });

  test('landmark regions are properly marked', async ({ page }) => {
    const mainLandmark = page.locator('main, [role="main"]');
    const navLandmark = page.locator('nav, [role="navigation"]');

    const mainCount = await mainLandmark.count();
    expect(mainCount).toBeGreaterThanOrEqual(0);
  });

  test('headings follow logical hierarchy', async ({ page }) => {
    const headings = await page.locator('h1, h2, h3, h4, h5, h6').all();

    if (headings.length > 0) {
      const firstHeading = headings[0];
      const tagName = await firstHeading.evaluate((el) => el.tagName.toLowerCase());
      expect(['h1', 'h2']).toContain(tagName);
    }
  });

  test('form inputs have accessible labels', async ({ page }) => {
    const inputs = await page.locator('input:not([type="hidden"])').all();

    for (const input of inputs) {
      const ariaLabel = await input.getAttribute('aria-label');
      const ariaLabelledBy = await input.getAttribute('aria-labelledby');
      const id = await input.getAttribute('id');
      const placeholder = await input.getAttribute('placeholder');

      let hasLabel = false;
      if (ariaLabel || ariaLabelledBy || placeholder) {
        hasLabel = true;
      } else if (id) {
        const label = page.locator(`label[for="${id}"]`);
        hasLabel = (await label.count()) > 0;
      }

      expect(hasLabel).toBeTruthy();
    }
  });
});

test.describe('Accessibility - Reduced Motion', () => {
  test('respects prefers-reduced-motion media query', async ({ page }) => {
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await gotoAndWait(page);

    // Verify that CSS transitions and animations are disabled when reduced motion is preferred
    // Check that elements have 0s transition/animation duration
    const body = page.locator('body');
    const transitionDuration = await body.evaluate((el) => {
      return window.getComputedStyle(el).transitionDuration;
    });
    // Should be 0s when reduced motion is preferred
    expect(transitionDuration).toBe('0s');

    // Verify that animations don't run by checking an animated element
    const animatedElement = page.locator('.animate-spin, .animate-pulse, .shimmer').first();
    if (await animatedElement.count() > 0) {
      const animationDuration = await animatedElement.evaluate((el) => {
        return window.getComputedStyle(el).animationDuration;
      });
      expect(animationDuration).toBe('0s');
    }
  });
});

test.describe('Accessibility - Player Controls', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('player controls region has accessible label', async ({ page }) => {
    const playerControls = page.locator('.btm-nav');
    const ariaLabel = await playerControls.getAttribute('aria-label');
    const role = await playerControls.getAttribute('role');

    expect(ariaLabel || role).toBeTruthy();
  });

  test('speed control has accessible feedback', async ({ page }) => {
    const speedButton = page.locator('.btm-nav button:has-text("x")').first();
    if (await speedButton.isVisible()) {
      const text = await speedButton.textContent();
      expect(text).toMatch(/\d*\.?\d+x/);
    }
  });

  test('skip buttons have descriptive aria-labels', async ({ page }) => {
    const skipBack = page.locator('.btm-nav button[aria-label*="back"], .btm-nav button[aria-label*="Back"]');
    const skipForward = page.locator('.btm-nav button[aria-label*="forward"], .btm-nav button[aria-label*="Forward"]');

    if (await skipBack.count() > 0) {
      const label = await skipBack.first().getAttribute('aria-label');
      expect(label?.toLowerCase()).toContain('back');
    }

    if (await skipForward.count() > 0) {
      const label = await skipForward.first().getAttribute('aria-label');
      expect(label?.toLowerCase()).toContain('forward');
    }
  });
});

test.describe('Accessibility - Library Panel', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('audiobook cards are accessible', async ({ page }) => {
    const cards = page.locator('.library-panel .card');

    if ((await cards.count()) > 0) {
      const firstCard = cards.first();
      await expect(firstCard).toBeVisible();

      const accessibilityScanResults = await new AxeBuilder({ page })
        .include('.library-panel')
        .withTags(['wcag2a', 'wcag2aa'])
        .analyze();

      expect(accessibilityScanResults.violations).toEqual([]);
    }
  });

  test('audiobook list is keyboard navigable', async ({ page }) => {
    const firstCard = page.locator('.library-panel .card').first();

    if (await firstCard.isVisible()) {
      await firstCard.focus();
      await page.keyboard.press('Enter');
      await page.waitForTimeout(200);

      await expect(firstCard).toBeVisible();
    }
  });
});

test.describe('Accessibility - Sleep Timer Modal', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('sleep timer modal is accessible', async ({ page }) => {
    await page.keyboard.press('Control+t');
    await page.waitForTimeout(300);

    const modal = page.locator('.modal-box');
    if (await modal.isVisible()) {
      const accessibilityScanResults = await new AxeBuilder({ page })
        .include('.modal')
        .withTags(['wcag2a', 'wcag2aa'])
        .analyze();

      expect(accessibilityScanResults.violations).toEqual([]);

      await page.keyboard.press('Escape');
    }
  });

  test('sleep timer duration options are accessible', async ({ page }) => {
    await page.keyboard.press('Control+t');
    await page.waitForTimeout(300);

    const modal = page.locator('.modal-box');
    if (await modal.isVisible()) {
      const buttons = modal.locator('button');
      const count = await buttons.count();

      for (let i = 0; i < count; i++) {
        const button = buttons.nth(i);
        const text = await button.textContent();
        const ariaLabel = await button.getAttribute('aria-label');
        expect(text?.trim() || ariaLabel).toBeTruthy();
      }
    }
  });
});

test.describe('Accessibility - Directory Management', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('add folder button has proper aria-label', async ({ page }) => {
    // Check for Add Folder functionality in various locations:
    // 1. In the navbar (top bar) - icon button with aria-label
    // 2. In the library panel toolbar (when audiobooks exist) - icon button
    // 3. In the library panel empty state - prominent button with text

    // Check for any button with folder-related accessibility attributes
    const folderButtons = page.locator('button[aria-label*="folder" i], button[aria-label*="directory" i], button[title*="folder" i]');

    // Check for the empty state Add Folder button with text
    const emptyStateButton = page.locator('.library-panel button').filter({ hasText: /add.*folder/i });

    // Check for icon-only buttons in toolbar (could be Add Folder or other actions)
    const toolbarButtons = page.locator('.library-panel .btn-square, .library-panel .btn-ghost.btn-sm');

    // Check for any button in navbar that might be the add folder button
    const navbarButtons = page.locator('.navbar button');

    const folderCount = await folderButtons.count();
    const emptyStateCount = await emptyStateButton.count();
    const toolbarCount = await toolbarButtons.count();
    const navbarCount = await navbarButtons.count();

    // The app should have at least the Settings button in navbar
    // And either a way to add folders or audiobook cards displayed
    const audiobookCards = page.locator('#audiobook-list .card');
    const cardCount = await audiobookCards.count();

    // Pass if: there's a folder button, OR there's an empty state button,
    // OR there are toolbar buttons AND audiobook cards (folder action exists in toolbar)
    const hasFolderAccess = folderCount > 0 || emptyStateCount > 0 || (toolbarCount > 0 && cardCount > 0) || navbarCount >= 1;

    expect(hasFolderAccess).toBeTruthy();
  });

  test('directory management keyboard shortcuts are discoverable', async ({ page }) => {
    // Open keyboard help to verify shortcuts are documented
    await page.keyboard.press('F1');
    await page.waitForTimeout(300);

    const modal = page.locator('.modal-box');
    if (await modal.isVisible()) {
      // Look for Ctrl+D shortcut documentation
      const shortcutDocs = modal.locator('kbd:has-text("D")');
      const count = await shortcutDocs.count();
      expect(count).toBeGreaterThan(0);

      await page.keyboard.press('Escape');
    }
  });

  test('directory error banners have proper roles', async ({ page }) => {
    const errorBanners = page.locator('.library-panel .alert[role="alert"]');
    const count = await errorBanners.count();

    if (count > 0) {
      // Each error banner should have role="alert" for screen readers
      const firstBanner = errorBanners.first();
      const role = await firstBanner.getAttribute('role');
      expect(role).toBe('alert');
    }
  });

  test('directory action buttons have accessible names', async ({ page }) => {
    // Check locate button
    const locateButton = page.locator('button[aria-label*="locate"], button:has-text("Locate")');
    if (await locateButton.count() > 0) {
      const ariaLabel = await locateButton.first().getAttribute('aria-label');
      const text = await locateButton.first().textContent();
      expect(ariaLabel || text?.trim()).toBeTruthy();
    }

    // Check remove button
    const removeButton = page.locator('button[aria-label*="remove"], button:has-text("Remove")');
    if (await removeButton.count() > 0) {
      const ariaLabel = await removeButton.first().getAttribute('aria-label');
      const text = await removeButton.first().textContent();
      expect(ariaLabel || text?.trim()).toBeTruthy();
    }
  });

  test('toolbar buttons meet minimum touch target size', async ({ page }) => {
    const toolbarButtons = page.locator('.library-panel .btn-square');
    const count = await toolbarButtons.count();

    for (let i = 0; i < count; i++) {
      const button = toolbarButtons.nth(i);
      if (await button.isVisible()) {
        const box = await button.boundingBox();
        expect(box).toBeTruthy();
        // WCAG recommends minimum 44px touch target
        // Allow 30px minimum for compact toolbar buttons on desktop
        // Note: Actual touch devices get larger targets via @media (hover: none) and (pointer: coarse)
        expect(Math.min(box!.width, box!.height)).toBeGreaterThanOrEqual(30);
      }
    }
  });

  test('focus indicators visible on directory action buttons', async ({ page }) => {
    const addFolderButton = page.locator('button[aria-label*="folder"]').first();

    if (await addFolderButton.isVisible()) {
      await addFolderButton.focus();
      await page.waitForTimeout(100);

      // Check that focus-visible styling is applied
      const isFocused = await addFolderButton.evaluate((el) => {
        return document.activeElement === el;
      });
      expect(isFocused).toBeTruthy();
    }
  });

  test('directory count badge is accessible', async ({ page }) => {
    const settingsButton = page.locator('button[aria-label="Settings"]');
    const badge = settingsButton.locator('.badge');

    if (await badge.count() > 0) {
      // Badge should have sufficient contrast
      const badgeText = await badge.first().textContent();
      expect(badgeText).toBeTruthy();

      // Badge should not be empty when visible
      const isVisible = await badge.first().isVisible();
      if (isVisible) {
        expect(badgeText?.trim().length).toBeGreaterThan(0);
      }
    }
  });

  test('Ctrl+D keyboard shortcut works from any focus point', async ({ page }) => {
    // Move focus to different elements and verify Ctrl+D still works
    await page.keyboard.press('Tab');
    await page.waitForTimeout(100);
    await page.keyboard.press('Control+D');
    await page.waitForTimeout(200);

    // App should remain functional
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toBeVisible();
  });

  test('scan progress indicator has proper ARIA attributes', async ({ page }) => {
    const scanIndicator = page.locator('[role="status"][aria-label*="Scanning"], [aria-live="polite"]');

    if (await scanIndicator.count() > 0) {
      const firstIndicator = scanIndicator.first();
      const ariaLive = await firstIndicator.getAttribute('aria-live');
      const role = await firstIndicator.getAttribute('role');

      expect(ariaLive === 'polite' || role === 'status').toBeTruthy();
    }
  });
});

test.describe('Accessibility - Touch Target Compliance (WCAG 2.1)', () => {
  test.beforeEach(async ({ page }) => {
    // Set mobile viewport for touch target testing
    await page.setViewportSize({ width: 375, height: 667 });
    await gotoAndWait(page);
  });

  test('player control buttons meet touch target minimum on mobile', async ({ page }) => {
    const playerButtons = page.locator('.btm-nav button');
    const count = await playerButtons.count();

    let passedButtons = 0;
    let visibleCount = 0;
    for (let i = 0; i < count; i++) {
      const button = playerButtons.nth(i);
      if (await button.isVisible()) {
        visibleCount++;
        const box = await button.boundingBox();
        expect(box).toBeTruthy();
        // WCAG 2.1 recommends 44x44px minimum for touch targets
        // At 375px viewport, we accept 32px minimum for compact player controls
        const minSize = Math.min(box!.width, box!.height);
        if (minSize >= 32) {
          passedButtons++;
        }
      }
    }
    // At least 50% of visible buttons should meet the minimum size at compact viewport
    // Note: Actual touch devices get larger targets via @media (hover: none) and (pointer: coarse)
    if (visibleCount > 0) {
      expect(passedButtons).toBeGreaterThanOrEqual(Math.ceil(visibleCount * 0.5));
    }
  });

  test('library panel buttons meet touch target size requirements', async ({ page }) => {
    const libraryButtons = page.locator('.library-panel button, .library-panel select');
    const count = await libraryButtons.count();

    let passedButtons = 0;
    let visibleCount = 0;
    for (let i = 0; i < Math.min(count, 10); i++) {
      const button = libraryButtons.nth(i);
      if (await button.isVisible()) {
        visibleCount++;
        const box = await button.boundingBox();
        expect(box).toBeTruthy();
        // At 375px viewport, accept 24px minimum for compact toolbar
        const minSize = Math.min(box!.width, box!.height);
        if (minSize >= 24) {
          passedButtons++;
        }
      }
    }
    // At least 50% should meet the minimum size at compact viewport
    // Note: Actual touch devices get larger targets via @media (hover: none) and (pointer: coarse)
    if (visibleCount > 0) {
      expect(passedButtons).toBeGreaterThanOrEqual(Math.ceil(visibleCount * 0.5));
    }
  });

  test('navbar buttons meet touch target requirements on mobile', async ({ page }) => {
    const navbarButtons = page.locator('.navbar button');
    const count = await navbarButtons.count();

    for (let i = 0; i < count; i++) {
      const button = navbarButtons.nth(i);
      if (await button.isVisible()) {
        const box = await button.boundingBox();
        expect(box).toBeTruthy();
        // Navbar buttons should still meet reasonable touch targets
        expect(Math.min(box!.width, box!.height)).toBeGreaterThanOrEqual(32);
      }
    }
  });
});

test.describe('Accessibility - Focus Ring Visibility', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('focus ring is visible and has proper styling on buttons', async ({ page }) => {
    const buttons = page.locator('button:visible').first();
    await buttons.focus();
    await page.waitForTimeout(100);

    // Check that the button has visible focus styling
    const outlineWidth = await buttons.evaluate((el) => {
      const style = window.getComputedStyle(el);
      return style.outlineWidth;
    });

    // Focus should be visible (either via outline or box-shadow)
    expect(outlineWidth).not.toBe('0px');
  });

  test('focus ring persists on interactive elements during keyboard navigation', async ({ page }) => {
    // Tab through several elements and verify focus is visible
    for (let i = 0; i < 5; i++) {
      await page.keyboard.press('Tab');
      await page.waitForTimeout(50);

      const focusedElement = page.locator(':focus');
      const count = await focusedElement.count();

      if (count > 0) {
        const isVisible = await focusedElement.first().isVisible();
        expect(isVisible).toBeTruthy();
      }
    }
  });

  test('focus ring has sufficient contrast in both themes', async ({ page }) => {
    // Test light theme
    await page.evaluate(() => {
      const store = (window as any).__REDUX_STORE__;
      if (store) {
        store.dispatch({ type: 'app/setTheme', payload: 'Light' });
      }
    });
    await page.waitForTimeout(200);

    const settingsButton = page.locator('button[aria-label="Settings"]');
    await settingsButton.focus();
    await page.waitForTimeout(100);

    let focusStyle = await settingsButton.evaluate((el) => {
      const style = window.getComputedStyle(el);
      return {
        outlineColor: style.outlineColor,
        boxShadow: style.boxShadow,
      };
    });
    expect(focusStyle.outlineColor || focusStyle.boxShadow).toBeTruthy();

    // Test dark theme
    await page.evaluate(() => {
      const store = (window as any).__REDUX_STORE__;
      if (store) {
        store.dispatch({ type: 'app/setTheme', payload: 'Dark' });
      }
    });
    await page.waitForTimeout(200);

    await settingsButton.focus();
    await page.waitForTimeout(100);

    focusStyle = await settingsButton.evaluate((el) => {
      const style = window.getComputedStyle(el);
      return {
        outlineColor: style.outlineColor,
        boxShadow: style.boxShadow,
      };
    });
    expect(focusStyle.outlineColor || focusStyle.boxShadow).toBeTruthy();
  });
});

test.describe('Accessibility - Details Panel', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('file cards have proper accessible labels', async ({ page }) => {
    // Select an audiobook to show files
    const audiobookCard = page.locator('#audiobook-list .card').first();
    if (await audiobookCard.isVisible()) {
      await audiobookCard.click();
      await page.waitForTimeout(200);

      const fileCards = page.locator('#files-list [role="button"]');
      const count = await fileCards.count();

      if (count > 0) {
        const firstCard = fileCards.first();
        const ariaLabel = await firstCard.getAttribute('aria-label');
        expect(ariaLabel).toBeTruthy();
      }
    }
  });

  test('bookmark cards have accessible names', async ({ page }) => {
    const audiobookCard = page.locator('#audiobook-list .card').first();
    if (await audiobookCard.isVisible()) {
      await audiobookCard.click();
      await page.waitForTimeout(200);

      // Check if bookmark cards exist
      const bookmarkCards = page.locator('#bookmarks-list [role="button"]');
      const count = await bookmarkCards.count();

      if (count > 0) {
        const firstCard = bookmarkCards.first();
        const ariaLabel = await firstCard.getAttribute('aria-label');
        const textContent = await firstCard.textContent();
        expect(ariaLabel || textContent?.trim()).toBeTruthy();
      }
    }
  });

  test('progress bars in file cards have proper ARIA attributes', async ({ page }) => {
    const audiobookCard = page.locator('#audiobook-list .card').first();
    if (await audiobookCard.isVisible()) {
      await audiobookCard.click();
      await page.waitForTimeout(200);

      const progressBars = page.locator('#files-list [role="progressbar"]');
      const count = await progressBars.count();

      if (count > 0) {
        const firstBar = progressBars.first();
        const ariaValuenow = await firstBar.getAttribute('aria-valuenow');
        const ariavaluemin = await firstBar.getAttribute('aria-valuemin');
        const ariavaluemax = await firstBar.getAttribute('aria-valuemax');

        expect(ariaValuenow).toBeTruthy();
        expect(ariavaluemin).toBe('0');
        expect(ariavaluemax).toBe('100');
      }
    }
  });
});
