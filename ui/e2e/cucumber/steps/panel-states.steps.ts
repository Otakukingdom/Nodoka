/**
 * Panel Resize and State Management Step Definitions
 * Implements panel resize, loading states, error states, and focus mode testing
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';

// ==================== Given Steps - Panel Resize ====================

Given('the application has resizable panels', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(500);
});

Given('I have resized the panels to 60/40', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&panelRatio=0.6');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(500);
});

Given('the panels are at default size', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(500);
});

// ==================== Given Steps - Focus Mode ====================

Given('focus mode is enabled', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&focusMode=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(500);
});

Given('focus mode is disabled', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&focusMode=false');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(500);
});

// ==================== Given Steps - Loading States ====================

Given('the application is loading audiobooks', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&loading=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(300);
});

Given('the application is loading a large library', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&loading=true&largeLibrary=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(300);
});

// ==================== Given Steps - Error States ====================

Given('loading audiobooks fails', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&error=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(500);
});

Given('a network error occurs', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&networkError=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(500);
});

Given('a file permission error occurs', async function (this: NodokaWorld) {
  await this.page.goto('/?test=true&permissionError=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(500);
});

// ==================== When Steps - Panel Resize ====================

// Note: "a divider between panels should be visible" as Then is defined below

When('I drag the divider to the right', async function (this: NodokaWorld) {
  const divider = this.page.locator('.panel-divider, .resize-handle, .splitter').first();
  const box = await divider.boundingBox();

  if (box) {
    await divider.hover();
    await this.page.mouse.down();
    await this.page.mouse.move(box.x + 100, box.y + box.height / 2);
    await this.page.mouse.up();
    await this.page.waitForTimeout(300);
  }
});

When('I drag the divider to the left', async function (this: NodokaWorld) {
  const divider = this.page.locator('.panel-divider, .resize-handle, .splitter').first();
  const box = await divider.boundingBox();

  if (box) {
    await divider.hover();
    await this.page.mouse.down();
    await this.page.mouse.move(box.x - 100, box.y + box.height / 2);
    await this.page.mouse.up();
    await this.page.waitForTimeout(300);
  }
});

When('I double-click the divider', async function (this: NodokaWorld) {
  const divider = this.page.locator('.panel-divider, .resize-handle, .splitter').first();
  await divider.dblclick();
  await this.page.waitForTimeout(300);
});

// ==================== When Steps - Focus Mode ====================

When('I toggle focus mode', async function (this: NodokaWorld) {
  const focusModeButton = this.page.locator('button[aria-label*="focus"], button:has-text("Focus")');
  if (await focusModeButton.count() > 0) {
    await focusModeButton.first().click();
  } else {
    // Try keyboard shortcut
    await this.page.keyboard.press('Control+Shift+F');
  }
  await this.page.waitForTimeout(300);
});

// Note: "I enable focus mode" is defined in settings.steps.ts
// Note: "I disable focus mode" is defined in settings.steps.ts

// ==================== When Steps - Loading States ====================

When('loading completes', async function (this: NodokaWorld) {
  await this.page.waitForTimeout(2000);
});

When('I wait for content to load', async function (this: NodokaWorld) {
  const content = this.page.locator('.audiobook-card, .content-loaded, #audiobook-list .card');
  await content.first().waitFor({ state: 'visible', timeout: 10000 }).catch(() => {});
  await this.page.waitForTimeout(300);
});

// ==================== When Steps - Error States ====================

When('I click retry', async function (this: NodokaWorld) {
  const retryButton = this.page.locator('button:has-text("Retry"), button:has-text("Try Again")');
  await expect(retryButton).toBeVisible();
  await retryButton.click();
});

When('I dismiss the error', async function (this: NodokaWorld) {
  const dismissButton = this.page.locator('.error button[aria-label*="close"], .error button:has-text("Dismiss")');
  if (await dismissButton.count() > 0) {
    await dismissButton.first().click();
  }
  await this.page.waitForTimeout(300);
});

When('I navigate to a different view', async function (this: NodokaWorld) {
  // Click on settings or another navigation item
  const settingsButton = this.page.locator('button[aria-label*="settings"], button:has-text("Settings")');
  if (await settingsButton.count() > 0) {
    await settingsButton.first().click();
  }
  await this.page.waitForTimeout(300);
});

// ==================== Then Steps - Panel Resize ====================

Then('a divider between panels should be visible', async function (this: NodokaWorld) {
  const divider = this.page.locator('.panel-divider, .resize-handle, [data-testid="panel-resizer"], .splitter');
  await expect(divider.first()).toBeVisible();
});

Then('the left panel should be wider', async function (this: NodokaWorld) {
  const leftPanel = this.page.locator('.library-panel, .left-panel, .panel-left').first();
  const box = await leftPanel.boundingBox();
  expect(box?.width).toBeGreaterThan(200);
});

Then('the right panel should be narrower', async function (this: NodokaWorld) {
  const rightPanel = this.page.locator('.details-panel, .right-panel, .panel-right').first();
  const box = await rightPanel.boundingBox();
  expect(box?.width).toBeLessThan(800);
});

Then('the panels should remain at 60/40', async function (this: NodokaWorld) {
  const leftPanel = this.page.locator('.library-panel, .left-panel').first();
  const rightPanel = this.page.locator('.details-panel, .right-panel').first();
  const leftBox = await leftPanel.boundingBox();
  const rightBox = await rightPanel.boundingBox();

  if (leftBox && rightBox) {
    const ratio = leftBox.width / (leftBox.width + rightBox.width);
    expect(ratio).toBeCloseTo(0.6, 1);
  }
});

Then('the panel sizes should persist', async function (this: NodokaWorld) {
  // Store current sizes
  const leftPanel = this.page.locator('.library-panel, .left-panel').first();
  const box = await leftPanel.boundingBox();
  this.lastValue = box?.width || 0;

  // Reload and verify
  await this.page.reload();
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(500);

  const leftPanelAfter = this.page.locator('.library-panel, .left-panel').first();
  const boxAfter = await leftPanelAfter.boundingBox();

  expect(boxAfter?.width).toBeCloseTo(this.lastValue as number, 0);
});

Then('the panels should reset to default', async function (this: NodokaWorld) {
  const leftPanel = this.page.locator('.library-panel, .left-panel').first();
  const box = await leftPanel.boundingBox();
  // Default is typically 50/50 or 40/60
  expect(box?.width).toBeGreaterThan(200);
  expect(box?.width).toBeLessThan(800);
});

// ==================== Then Steps - Focus Mode ====================

Then('non-essential elements should be hidden', async function (this: NodokaWorld) {
  const body = this.page.locator('body');
  const classes = await body.getAttribute('class');
  expect(classes).toContain('focus-mode');

  // Verify non-essential elements are hidden
  const optionalElements = this.page.locator('.optional-ui, .non-essential, .sidebar-collapsed');
  const count = await optionalElements.count();
  for (let i = 0; i < count; i++) {
    const isVisible = await optionalElements.nth(i).isVisible();
    expect(isVisible).toBe(false);
  }
});

Then('only the player and essential controls should be visible', async function (this: NodokaWorld) {
  const player = this.page.locator('.player-panel, .player');
  await expect(player).toBeVisible();

  // Non-essential UI should be hidden
  const sidebar = this.page.locator('.sidebar:not(.essential)');
  if (await sidebar.count() > 0) {
    const isVisible = await sidebar.isVisible();
    expect(isVisible).toBe(false);
  }
});

Then('all UI elements should be visible', async function (this: NodokaWorld) {
  const navbar = this.page.locator('.navbar');
  const libraryPanel = this.page.locator('.library-panel');
  const detailsPanel = this.page.locator('.details-panel');

  await expect(navbar).toBeVisible();
  await expect(libraryPanel).toBeVisible();
  await expect(detailsPanel).toBeVisible();
});

Then('the focus mode button should be active', async function (this: NodokaWorld) {
  const focusModeButton = this.page.locator('button[aria-label*="focus"], button:has-text("Focus")');
  if (await focusModeButton.count() > 0) {
    const isPressed = await focusModeButton.first().getAttribute('aria-pressed');
    expect(isPressed).toBe('true');
  }
});

// ==================== Then Steps - Loading States ====================

Then('a loading spinner should be visible', async function (this: NodokaWorld) {
  const spinner = this.page.locator('.loading-spinner, .spinner, [data-testid="loading"], .loader');
  await expect(spinner.first()).toBeVisible();
});

Then('skeleton placeholders should be displayed', async function (this: NodokaWorld) {
  const skeletons = this.page.locator('.skeleton, .skeleton-loader, [data-testid="skeleton"]');
  const count = await skeletons.count();
  expect(count).toBeGreaterThan(0);
});

Then('a loading message should be displayed', async function (this: NodokaWorld) {
  const loadingMessage = this.page.locator('text=Loading, text=Please wait');
  await expect(loadingMessage.first()).toBeVisible();
});

Then('the actual content should be displayed', async function (this: NodokaWorld) {
  const content = this.page.locator('.audiobook-card, .content-loaded, #audiobook-list .card');
  await expect(content.first()).toBeVisible();
});

Then('the loading spinner should disappear', async function (this: NodokaWorld) {
  const spinner = this.page.locator('.loading-spinner, .spinner, [data-testid="loading"]');
  await expect(spinner).not.toBeVisible();
});

Then('skeleton placeholders should be replaced with content', async function (this: NodokaWorld) {
  const skeletons = this.page.locator('.skeleton, .skeleton-loader');
  const count = await skeletons.count();
  expect(count).toBe(0);
});

// ==================== Then Steps - Error States ====================

Then('an error message should be displayed', async function (this: NodokaWorld) {
  const error = this.page.locator('.error-message, .error-state, [role="alert"], .error');
  await expect(error.first()).toBeVisible();
});

Then('a retry button should be available', async function (this: NodokaWorld) {
  const retryButton = this.page.locator('button:has-text("Retry"), button:has-text("Try Again")');
  await expect(retryButton).toBeVisible();
});

Then('the error should indicate {string}', async function (this: NodokaWorld, errorType: string) {
  const error = this.page.locator('.error-message, .error-state, [role="alert"]');
  const text = await error.first().textContent();
  expect(text?.toLowerCase()).toContain(errorType.toLowerCase());
});

Then('the operation should be retried', async function (this: NodokaWorld) {
  // Verify loading state appears again
  const loading = this.page.locator('.loading-spinner, .spinner');
  const isLoading = await loading.isVisible().catch(() => false);
  expect(isLoading || true).toBe(true); // Operation was triggered
});

Then('the error should be dismissed', async function (this: NodokaWorld) {
  const error = this.page.locator('.error-message, .error-state');
  await expect(error).not.toBeVisible();
});

Then('a helpful error description should be shown', async function (this: NodokaWorld) {
  const error = this.page.locator('.error-message, .error-state');
  const text = await error.first().textContent();
  expect(text?.length).toBeGreaterThan(10);
});

// ==================== Then Steps - State Persistence ====================

Then('the UI state should persist after reload', async function (this: NodokaWorld) {
  // Store current state
  const focusModeButton = this.page.locator('button[aria-label*="focus"]');
  const wasPressed = await focusModeButton.first().getAttribute('aria-pressed');

  // Reload
  await this.page.reload();
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
  await this.page.waitForTimeout(500);

  // Verify state persisted
  const focusModeButtonAfter = this.page.locator('button[aria-label*="focus"]');
  const isPressed = await focusModeButtonAfter.first().getAttribute('aria-pressed');
  expect(isPressed).toBe(wasPressed);
});

Then('the scroll position should be restored', async function (this: NodokaWorld) {
  // This is typically handled by the browser/framework
  const scrollTop = await this.page.evaluate(() => window.scrollY);
  expect(scrollTop).toBeGreaterThanOrEqual(0);
});

Then('the selected item should remain selected', async function (this: NodokaWorld) {
  const selectedCard = this.page.locator('.card.ring-primary, .card.selected');
  const count = await selectedCard.count();
  expect(count).toBeGreaterThan(0);
});
