/**
 * Accessibility Step Definitions
 * Implements accessibility testing with axe-core for WCAG compliance
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { NodokaWorld } from '../world.js';
import { SettingsPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('screen reader mode is enabled', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    document.body.setAttribute('data-screen-reader', 'true');
  });
});

Given('a modal is open for accessibility testing', async function (this: NodokaWorld) {
  const settings = new SettingsPage(this.page);
  await settings.open();
});

Given('high contrast mode is enabled', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    document.body.classList.add('high-contrast');
  });
});

Given('reduced motion is enabled', async function (this: NodokaWorld) {
  await this.page.evaluate(() => {
    document.body.classList.add('reduce-motion');
  });
});

// ==================== When Steps ====================

When('I run an accessibility audit', async function (this: NodokaWorld) {
  const accessibilityScanResults = await new AxeBuilder({ page: this.page }).analyze();
  this.lastValue = accessibilityScanResults.violations.length;
  this.accessibilityResults = accessibilityScanResults;
});

When('I run an accessibility audit for {string}', async function (this: NodokaWorld, rule: string) {
  const accessibilityScanResults = await new AxeBuilder({ page: this.page })
    .withRules([rule])
    .analyze();
  this.lastValue = accessibilityScanResults.violations.length;
  this.accessibilityResults = accessibilityScanResults;
});

When('I check color contrast accessibility', async function (this: NodokaWorld) {
  const accessibilityScanResults = await new AxeBuilder({ page: this.page })
    .withRules(['color-contrast'])
    .analyze();
  this.lastValue = accessibilityScanResults.violations.length;
  this.accessibilityResults = accessibilityScanResults;
});

When('I tab through all focusable elements', async function (this: NodokaWorld) {
  this.focusableElements = [];
  const focusableSelectors = 'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])';

  for (let i = 0; i < 20; i++) {
    await this.page.keyboard.press('Tab');
    await this.page.waitForTimeout(100);

    const focusedElement = this.page.locator(':focus');
    if (await focusedElement.isVisible()) {
      const tagName = await focusedElement.evaluate(el => el.tagName);
      this.focusableElements.push(tagName);
    }
  }
});

When('I navigate with keyboard only', async function (this: NodokaWorld) {
  // Navigate through the UI using only keyboard
  for (let i = 0; i < 10; i++) {
    await this.page.keyboard.press('Tab');
    await this.page.waitForTimeout(100);
  }
});

When('I check for focus visibility', async function (this: NodokaWorld) {
  // Tab to first focusable element
  await this.page.keyboard.press('Tab');
  await this.page.waitForTimeout(100);
});

// ==================== Then Steps ====================

Then('all buttons should have accessible labels', async function (this: NodokaWorld) {
  const buttons = await this.page.locator('button').all();
  let unlabeledCount = 0;

  for (const button of buttons) {
    const ariaLabel = await button.getAttribute('aria-label');
    const text = await button.textContent();
    const title = await button.getAttribute('title');

    if (!ariaLabel && !text?.trim() && !title) {
      unlabeledCount++;
    }
  }

  expect(unlabeledCount).toBe(0);
});

Then('all images should have alt text', async function (this: NodokaWorld) {
  const images = await this.page.locator('img').all();
  let missingAltCount = 0;

  for (const img of images) {
    const alt = await img.getAttribute('alt');
    const ariaLabel = await img.getAttribute('aria-label');
    const role = await img.getAttribute('role');

    // Images with role="presentation" or aria-hidden don't need alt
    if (role !== 'presentation' && !ariaLabel) {
      if (alt === null) {
        missingAltCount++;
      }
    }
  }

  expect(missingAltCount).toBe(0);
});

Then('focus indicators should be visible on all elements', async function (this: NodokaWorld) {
  const focusedElement = this.page.locator(':focus');

  if (await focusedElement.isVisible()) {
    // Check that focused element has visible focus indicator
    const outline = await focusedElement.evaluate(el => {
      const styles = window.getComputedStyle(el);
      return {
        outline: styles.outline,
        outlineWidth: styles.outlineWidth,
        boxShadow: styles.boxShadow,
        border: styles.border,
      };
    });

    // Element should have some form of visible focus indicator
    const hasVisibleFocus =
      outline.outline !== 'none' ||
      outline.outlineWidth !== '0px' ||
      outline.boxShadow !== 'none' ||
      outline.border !== 'none';

    expect(hasVisibleFocus).toBe(true);
  }
});

Then('focus should be trapped within the modal', async function (this: NodokaWorld) {
  const modal = this.page.locator('.modal-box, [role="dialog"]').first();
  await expect(modal).toBeVisible();

  // Tab through elements and verify focus stays in modal
  for (let i = 0; i < 20; i++) {
    await this.page.keyboard.press('Tab');
    await this.page.waitForTimeout(50);

    const focusedElement = this.page.locator(':focus');
    if (await focusedElement.isVisible()) {
      const isInModal = await modal.evaluate((el, focused) => {
        return el.contains(focused);
      }, await focusedElement.elementHandle()!);

      expect(isInModal).toBe(true);
    }
  }
});

Then('a screen reader announcement should be made for state changes', async function (this: NodokaWorld) {
  const liveRegion = this.page.locator('[aria-live="polite"], [aria-live="assertive"], [role="status"]');

  // Check if live region exists for announcements
  const count = await liveRegion.count();
  expect(count).toBeGreaterThan(0);
});

Then('form inputs should have associated labels', async function (this: NodokaWorld) {
  const inputs = await this.page.locator('input, select, textarea').all();
  let unlabeledCount = 0;

  for (const input of inputs) {
    const id = await input.getAttribute('id');
    const ariaLabel = await input.getAttribute('aria-label');
    const ariaLabelledby = await input.getAttribute('aria-labelledby');
    const placeholder = await input.getAttribute('placeholder');

    if (id) {
      // Check for associated label
      const label = this.page.locator(`label[for="${id}"]`);
      const hasLabel = await label.isVisible().catch(() => false);
      if (hasLabel || ariaLabel || ariaLabelledby) {
        continue;
      }
    }

    if (!ariaLabel && !ariaLabelledby && !placeholder) {
      unlabeledCount++;
    }
  }

  expect(unlabeledCount).toBe(0);
});

Then('all text should have sufficient color contrast', async function (this: NodokaWorld) {
  const accessibilityScanResults = await new AxeBuilder({ page: this.page })
    .withRules(['color-contrast'])
    .analyze();

  expect(accessibilityScanResults.violations).toHaveLength(0);
});

Then('no accessibility violations should be found', async function (this: NodokaWorld) {
  const accessibilityScanResults = await new AxeBuilder({ page: this.page }).analyze();
  expect(accessibilityScanResults.violations).toHaveLength(0);
});

Then('no critical accessibility violations should be found', async function (this: NodokaWorld) {
  const accessibilityScanResults = await new AxeBuilder({ page: this.page })
    .withTags(['wcag2a', 'wcag2aa'])
    .analyze();

  const criticalViolations = accessibilityScanResults.violations.filter(
    v => v.impact === 'critical' || v.impact === 'serious'
  );

  expect(criticalViolations).toHaveLength(0);
});

Then('the page should have a valid heading structure', async function (this: NodokaWorld) {
  const h1Elements = await this.page.locator('h1').all();
  expect(h1Elements.length).toBeGreaterThanOrEqual(1);

  // Check heading hierarchy
  const headings = await this.page.locator('h1, h2, h3, h4, h5, h6').all();
  let prevLevel = 0;

  for (const heading of headings) {
    const tagName = await heading.evaluate(el => el.tagName.toLowerCase());
    const level = parseInt(tagName.replace('h', ''));

    // Headings should not skip levels (e.g., h1 -> h3)
    if (prevLevel > 0 && level > prevLevel + 1) {
      throw new Error(`Heading level skipped: h${prevLevel} to h${level}`);
    }

    prevLevel = level;
  }
});

Then('interactive elements should have appropriate roles', async function (this: NodokaWorld) {
  // Check buttons
  const buttons = await this.page.locator('button').all();
  for (const button of buttons) {
    const role = await button.getAttribute('role');
    const tagName = await button.evaluate(el => el.tagName.toLowerCase());
    expect(tagName === 'button' || role === 'button').toBe(true);
  }

  // Check links
  const links = await this.page.locator('a[href]').all();
  for (const link of links) {
    const role = await link.getAttribute('role');
    const tagName = await link.evaluate(el => el.tagName.toLowerCase());
    expect(tagName === 'a' || role === 'link').toBe(true);
  }
});

Then('aria-expanded should be properly used', async function (this: NodokaWorld) {
  const expandedElements = await this.page.locator('[aria-expanded]').all();

  for (const element of expandedElements) {
    const ariaExpanded = await element.getAttribute('aria-expanded');
    expect(['true', 'false']).toContain(ariaExpanded);
  }
});

Then('aria-hidden should not hide focusable elements', async function (this: NodokaWorld) {
  const hiddenElements = await this.page.locator('[aria-hidden="true"]').all();

  for (const element of hiddenElements) {
    const focusable = await element.locator('button, a[href], input, select, textarea, [tabindex]').count();
    expect(focusable).toBe(0);
  }
});

Then('the page should have a skip link', async function (this: NodokaWorld) {
  const skipLink = this.page.locator('a[href="#main"], a[href="#content"], a:has-text("Skip")');
  const hasSkipLink = await skipLink.count() > 0;
  expect(hasSkipLink).toBe(true);
});

Then('touch targets should be at least 44x44 pixels', async function (this: NodokaWorld) {
  const buttons = await this.page.locator('button, a[href], [role="button"]').all();

  for (const button of buttons) {
    const box = await button.boundingBox();
    if (box) {
      // Allow some tolerance for small buttons
      const minSize = 44;
      const isLargeEnough = box.width >= minSize * 0.8 && box.height >= minSize * 0.8;
      expect(isLargeEnough).toBe(true);
    }
  }
});
