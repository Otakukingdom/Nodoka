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
} from '@cucumber/cucumber';
import { chromium, Browser, BrowserContext } from '@playwright/test';
import { NodokaWorld } from './world.js';

let browser: Browser;
let context: BrowserContext;

setWorldConstructor(NodokaWorld);

BeforeAll(async () => {
  browser = await chromium.launch({ headless: true });
});

AfterAll(async () => {
  await browser.close();
});

Before(async function (this: NodokaWorld) {
  context = await browser.newContext();
  this.page = await context.newPage();
  this.testStartTime = Date.now();

  // Navigate to the app with test mode enabled
  // Use base URL for dev server (localhost:5173)
  await this.page.goto('http://localhost:5173/?test=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

After(async function (this: NodokaWorld) {
  await this.page.close();
  await context.close();
});
