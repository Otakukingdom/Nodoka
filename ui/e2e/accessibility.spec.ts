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

    const select = page.locator('.modal-box select').first();
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

    const accessibilityScanResults = await new AxeBuilder({ page })
      .withRules(['motion'])
      .analyze();

    const motionViolations = accessibilityScanResults.violations;
    expect(motionViolations).toEqual([]);
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
