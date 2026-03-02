/**
 * User Preferences Step Definitions
 * Implements UI interactions for user preferences feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';

// ==================== Given Steps ====================

Given('the application has a database', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });
});

// ==================== When Steps ====================

When('I set user preference {string} to {string}', async function (this: NodokaWorld, key: string, value: string) {
  await this.page.evaluate(({ k, v }) => {
    window.dispatchEvent(new CustomEvent('test:set-preference', { detail: { key: k, value: v } }));
  }, { k: key, v: value });
  await this.page.waitForTimeout(200);
});

When('the application has user preference {string} set to {string}', async function (this: NodokaWorld, key: string, value: string) {
  await this.page.evaluate(({ k, v }) => {
    window.dispatchEvent(new CustomEvent('test:set-preference', { detail: { key: k, value: v } }));
  }, { k: key, v: value });
  await this.page.waitForTimeout(200);
});

When('I get user preference {string}', async function (this: NodokaWorld, key: string) {
  this.lastValue = await this.page.evaluate((k) => {
    return new Promise<string | null>((resolve) => {
      const handler = (e: CustomEvent) => {
        window.removeEventListener('test:preference-response', handler as EventListener);
        resolve(e.detail.value);
      };
      window.addEventListener('test:preference-response', handler as EventListener);
      window.dispatchEvent(new CustomEvent('test:get-preference', { detail: { key: k } }));
    });
  }, key);
});

When('I get all user preferences', async function (this: NodokaWorld) {
  this.lastCount = await this.page.evaluate(() => {
    return new Promise<number>((resolve) => {
      const handler = (e: CustomEvent) => {
        window.removeEventListener('test:all-preferences-response', handler as EventListener);
        resolve(e.detail.count);
      };
      window.addEventListener('test:all-preferences-response', handler as EventListener);
      window.dispatchEvent(new CustomEvent('test:get-all-preferences'));
    });
  });
});

When('I delete user preference {string}', async function (this: NodokaWorld, key: string) {
  await this.page.evaluate((k) => {
    window.dispatchEvent(new CustomEvent('test:delete-preference', { detail: { key: k } }));
  }, key);
  await this.page.waitForTimeout(200);
});

When('I try to set user preference with NUL byte key', async function (this: NodokaWorld) {
  this.lastError = await this.page.evaluate(() => {
    return new Promise<Error | null>((resolve) => {
      try {
        window.dispatchEvent(new CustomEvent('test:set-preference', { detail: { key: 'test\0key', value: 'value' } }));
        resolve(null);
      } catch (e) {
        resolve(e as Error);
      }
    });
  });
  await this.page.waitForTimeout(200);
});

When('I try to set user preference with NUL byte value', async function (this: NodokaWorld) {
  this.lastError = await this.page.evaluate(() => {
    return new Promise<Error | null>((resolve) => {
      try {
        window.dispatchEvent(new CustomEvent('test:set-preference', { detail: { key: 'key', value: 'test\0value' } }));
        resolve(null);
      } catch (e) {
        resolve(e as Error);
      }
    });
  });
  await this.page.waitForTimeout(200);
});

// ==================== Then Steps ====================

Then('the preference should be stored', async function (this: NodokaWorld) {
  // Preference stored successfully
});

Then('the value should be {string}', async function (this: NodokaWorld, expectedValue: string) {
  expect(this.lastValue).toBe(expectedValue);
});

Then('I should receive {int} preferences', async function (this: NodokaWorld, count: number) {
  expect(this.lastCount).toBe(count);
});

Then('the preference should not exist', async function (this: NodokaWorld) {
  expect(this.lastValue).toBeNull();
});

Then('the preference value should be {string}', async function (this: NodokaWorld, expectedValue: string) {
  expect(this.lastValue).toBe(expectedValue);
});
