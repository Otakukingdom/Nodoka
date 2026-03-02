/**
 * Cucumber hooks for Playwright-based UI tests
 * Manages browser lifecycle and test state
 */

import {
  BeforeAll,
  AfterAll,
  Before,
  After,
  setWorldConstructor,
  AfterStep,
  BeforeStep,
} from '@cucumber/cucumber';
import { chromium, Browser, BrowserContext } from '@playwright/test';
import { NodokaWorld } from './world.js';

let browser: Browser;
let context: BrowserContext;

setWorldConstructor(NodokaWorld);

BeforeAll(async () => {
  browser = await chromium.launch({
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox'],
  });
});

AfterAll(async () => {
  await browser.close();
});

// Default before hook for all scenarios
Before(async function (this: NodokaWorld) {
  context = await browser.newContext({
    viewport: { width: 1280, height: 720 },
    ignoreHTTPSErrors: true,
  });
  this.page = await context.newPage();
  this.testStartTime = Date.now();

  // Set default timeout
  this.page.setDefaultTimeout(10000);

  // Navigate to the app with test mode enabled
  await this.page.goto('http://localhost:5173/?test=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

// Before hook for @accessibility tagged scenarios
Before({ tags: '@accessibility' }, async function (this: NodokaWorld) {
  // Ensure page is ready for accessibility testing
  await this.page.waitForLoadState('networkidle');
});

// Before hook for @keyboard tagged scenarios
Before({ tags: '@keyboard' }, async function (this: NodokaWorld) {
  // Reset any keyboard state
  await this.page.keyboard.press('Escape');
  await this.page.waitForTimeout(100);
});

// Before hook for @notifications tagged scenarios
Before({ tags: '@notifications' }, async function (this: NodokaWorld) {
  // Clear any existing notifications
  const notifications = this.page.locator('.toast, .notification');
  const count = await notifications.count();
  for (let i = 0; i < count; i++) {
    const dismissBtn = notifications.nth(i).locator('button');
    if (await dismissBtn.isVisible()) {
      await dismissBtn.click();
    }
  }
});

After(async function (this: NodokaWorld) {
  // Clean up any open modals
  const modals = this.page.locator('.modal-box');
  const modalCount = await modals.count();
  for (let i = 0; i < modalCount; i++) {
    const closeButton = modals.nth(i).locator('button[aria-label="Close"], button:has-text("Close")');
    if (await closeButton.isVisible()) {
      await closeButton.click();
      await this.page.waitForTimeout(100);
    }
  }

  // Close page and context
  await this.page.close();
  await context.close();
});

// After hook for @ui tagged scenarios - take screenshot on failure
After({ tags: '@ui' }, async function (this: NodokaWorld, testCase) {
  if (testCase.result?.status === 'FAILED') {
    const screenshot = await this.page.screenshot({
      fullPage: true,
      path: `e2e-screenshots/failure-${Date.now()}.png`,
    });
    // Attach screenshot to world for reporting
    this.attach(screenshot.toString('base64'), 'image/png');
  }
});
