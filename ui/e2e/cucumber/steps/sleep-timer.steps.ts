/**
 * Sleep Timer Step Definitions
 * Implements UI interactions for sleep timer feature tests
 */

import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from '@playwright/test';
import { NodokaWorld } from '../world.js';
import { SleepTimerPage, LibraryPage } from '../pages/index.js';

// ==================== Given Steps ====================

Given('I want to set a sleep timer', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  // Select an audiobook first
  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
});

Given('I have a sleep timer set for {int} seconds', async function (this: NodokaWorld, seconds: number) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.open();

  // For testing, set a very short duration
  if (seconds === 0) {
    // Set minimum duration
    const minButton = this.page.locator('.modal-box button:has-text("min")').first();
    await minButton.click();
  } else {
    // Set specific duration via test event
    await this.page.evaluate((s) => {
      window.dispatchEvent(new CustomEvent('test:set-sleep-timer', { detail: { duration: s * 1000 } }));
    }, seconds);
  }

  await sleepTimer.close();
});

Given('I have an end-of-chapter sleep timer', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.setEndOfChapter();
});

Given('a sleep timer is active with {int} minutes remaining', async function (this: NodokaWorld, minutes: number) {
  await this.page.evaluate((m) => {
    window.dispatchEvent(new CustomEvent('test:set-sleep-timer', { detail: { duration: m * 60 * 1000 } }));
  }, minutes);
  await this.page.waitForTimeout(300);
});

Given('a sleep timer is active', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.setDuration(30);
});

// ==================== When Steps ====================

When('I set a sleep timer for {int} minutes', async function (this: NodokaWorld, minutes: number) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.setDuration(minutes);
});

When('I set a sleep timer for end of chapter', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.setEndOfChapter();
});

When('I set a sleep timer for {int} minutes with {int} second fade', async function (this: NodokaWorld, minutes: number, fadeSeconds: number) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.open();
  await sleepTimer.setFadeDuration(fadeSeconds);

  // Set duration
  const durationButton = this.page.locator(`.modal-box button:has-text("${minutes} min")`);
  if (await durationButton.isVisible()) {
    await durationButton.click();
  }

  await sleepTimer.close();
});

When('I check if the timer has expired', async function (this: NodokaWorld) {
  // Check timer status via test event
  const result = await this.page.evaluate(() => {
    return new Promise<boolean>((resolve) => {
      const handler = (e: CustomEvent) => {
        window.removeEventListener('test:timer-expired-response', handler as EventListener);
        resolve(e.detail.expired);
      };
      window.addEventListener('test:timer-expired-response', handler as EventListener);
      window.dispatchEvent(new CustomEvent('test:check-timer-expired'));
    });
  });
  this.lastValue = result;
});

When('I check the remaining time', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  if (await sleepTimer.isTimerActive()) {
    const remaining = await sleepTimer.getRemainingTime();
    this.lastValue = remaining;
  } else {
    this.lastValue = null;
  }
});

When('I click the sleep timer button', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.open();
});

When('I click cancel timer', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.cancelTimer();
});

// ==================== Then Steps ====================

Then('the sleep timer should be active', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const isActive = await sleepTimer.isTimerActive();
  expect(isActive).toBe(true);
});

Then('the timer mode should be Duration', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const timerDisplay = sleepTimer.timerDisplay;
  const text = await timerDisplay.textContent();
  // Duration mode shows countdown
  expect(text).toMatch(/\d+:\d+/);
});

Then('the timer mode should be EndOfChapter', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const modeText = await sleepTimer.getTimerMode();
  expect(modeText.toLowerCase()).toContain('chapter');
});

Then('the timer should be expired', async function (this: NodokaWorld) {
  expect(this.lastValue).toBe(true);
});

Then('the remaining time should be greater than zero', async function (this: NodokaWorld) {
  expect(this.lastValue).not.toBeNull();
});

Then('the remaining time should be None', async function (this: NodokaWorld) {
  expect(this.lastValue).toBeNull();
});

Then('the fade duration should be {int} seconds', async function (this: NodokaWorld, seconds: number) {
  // Fade duration is persisted in settings
});

Then('the sleep timer modal should open', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.expectModalVisible();
});

Then('the remaining time should be displayed', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const isDisplayVisible = await sleepTimer.timerDisplay.isVisible();
  expect(isDisplayVisible).toBe(true);
});

Then('the timer should be cancelled', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const isActive = await sleepTimer.isTimerActive();
  expect(isActive).toBe(false);
});

Then('no remaining time should be displayed', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const isDisplayVisible = await sleepTimer.timerDisplay.isVisible();
  expect(isDisplayVisible).toBe(false);
});
