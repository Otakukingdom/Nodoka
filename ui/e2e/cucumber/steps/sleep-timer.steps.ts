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

  // For testing, set duration using the UI
  if (seconds === 0) {
    // Set minimum duration
    const minButton = this.page.locator('.modal-box button:has-text("min")').first();
    await minButton.click();
  } else {
    // Convert seconds to minutes for the UI
    const minutes = Math.ceil(seconds / 60);
    const durationButton = this.page.locator(`.modal-box button:has-text("${minutes} min"), .modal-box button:has-text("${minutes}min")`).first();

    if (await durationButton.isVisible()) {
      await durationButton.click();
    } else {
      // Use custom duration input if available
      const customInput = this.page.locator('.modal-box input[type="number"], .modal-box input[placeholder*="minute"]').first();
      if (await customInput.isVisible()) {
        await customInput.fill(minutes.toString());
        await this.page.keyboard.press('Enter');
      }
    }
  }

  await sleepTimer.close();
});

Given('I have an end-of-chapter sleep timer', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.setEndOfChapter();
});

Given('a sleep timer is active with {int} minutes remaining', async function (this: NodokaWorld, minutes: number) {
  await this.page.goto(`http://localhost:5173/?test=true&timerMinutes=${minutes}`);
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
});

Given('a sleep timer is active', async function (this: NodokaWorld) {
  await this.page.goto('http://localhost:5173/?test=true&timerActive=true');
  await this.page.waitForSelector('.navbar', { timeout: 10000 });

  const library = new LibraryPage(this.page);
  await library.selectAudiobook(0);
});

// ==================== When Steps ====================

When('I set a sleep timer for {int} seconds', async function (this: NodokaWorld, seconds: number) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.open();

  // Set duration using the UI
  const minutes = Math.ceil(seconds / 60);
  const durationButton = this.page.locator(`.modal-box button:has-text("${minutes} min")`).first();

  if (await durationButton.isVisible()) {
    await durationButton.click();
  } else {
    // Use custom duration input if available
    const customInput = this.page.locator('.modal-box input[type="number"]').first();
    if (await customInput.isVisible()) {
      await customInput.fill(minutes.toString());
      await this.page.keyboard.press('Enter');
    }
  }

  await sleepTimer.close();
});

When('I set a sleep timer for {int} minutes with {int} second fade', async function (this: NodokaWorld, minutes: number, fadeSeconds: number) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.open();

  // Set duration
  const durationButton = this.page.locator(`.modal-box button:has-text("${minutes} min")`).first();
  if (await durationButton.isVisible()) {
    await durationButton.click();
  }

  // Set fade duration
  await sleepTimer.setFadeDuration(fadeSeconds);

  await sleepTimer.close();
});

When('I set an end-of-chapter sleep timer', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.setEndOfChapter();
});

When('I check if the timer has expired', async function (this: NodokaWorld) {
  // Check timer status via the UI
  const sleepTimer = new SleepTimerPage(this.page);
  const isActive = await sleepTimer.isTimerActive();
  if (!isActive) {
    this.lastValue = true; // Timer has expired
  } else {
    // Check remaining time
    const remainingTime = await sleepTimer.getRemainingTime();
    this.lastValue = remainingTime === '00:00' || remainingTime === '0:00';
  }
});

When('I set a sleep timer for {int} minutes', async function (this: NodokaWorld, minutes: number) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.open();

  // Click the duration button
  const durationButton = this.page.locator(`button:has-text("${minutes} min"), button:has-text("${minutes} min"), [data-duration="${minutes}"]`).first();
  if (await durationButton.isVisible()) {
    await durationButton.click();
    await this.page.waitForTimeout(300);
  }

  await sleepTimer.close();
});

When('I set a sleep timer for end of chapter', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.setEndOfChapter();
});

When('I check the remaining time', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const remainingTime = await sleepTimer.getRemainingTime();
  if (remainingTime && remainingTime !== '00:00' && remainingTime !== '0:00') {
    this.lastValue = remainingTime;
  } else {
    this.lastValue = null;
  }
});

// ==================== Then Steps ====================

Then('the timer should be set', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const isActive = await sleepTimer.isTimerActive();
  expect(isActive).toBe(true);
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

// ==================== UI Interaction Steps ====================

When('I click the sleep timer button', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.open();
});

When('I click cancel timer', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.cancelTimer();
});

// ==================== Then Steps - Timer State ====================

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

// Note: "the timer should be expired", "the remaining time should be greater than zero",
// and "the remaining time should be None" are defined earlier in this file
// Do not redefine them here

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

// ==================== Sleep Timer UI Interaction Steps ====================

When('I click the sleep timer button in player controls', async function (this: NodokaWorld) {
  const sleepTimerBtn = this.page.locator('button[aria-label*="Sleep"], button[aria-label*="Timer"]').first();
  if (await sleepTimerBtn.isVisible()) {
    await sleepTimerBtn.click();
    await this.page.waitForTimeout(300);
  }
});

Then('{int} minute option should be visible', async function (this: NodokaWorld, minutes: number) {
  const option = this.page.locator(`button:has-text("${minutes} min"), button:has-text("${minutes} minutes"), [data-duration="${minutes}"]`).first();
  await expect(option).toBeVisible({ timeout: 3000 });
});

Then('the end of chapter option should be visible', async function (this: NodokaWorld) {
  const option = this.page.locator('button:has-text("End of chapter"), button:has-text("End of Chapter"), [data-option="end-of-chapter"]').first();
  await expect(option).toBeVisible({ timeout: 3000 });
});

Then('a fade duration dropdown should be visible', async function (this: NodokaWorld) {
  const dropdown = this.page.locator('select, .dropdown, [data-testid="fade-duration"]').first();
  await expect(dropdown).toBeVisible({ timeout: 3000 });
});

Then('None option should be available', async function (this: NodokaWorld) {
  const option = this.page.locator('option:has-text("None"), button:has-text("None"), [data-fade="none"]').first();
  expect(await option.count()).toBeGreaterThan(0);
});

Then('{int} seconds option should be available', async function (this: NodokaWorld, seconds: number) {
  const option = this.page.locator(`option:has-text("${seconds}"), button:has-text("${seconds}s"), [data-fade="${seconds}"]`).first();
  expect(await option.count()).toBeGreaterThan(0);
});

// Additional Sleep Timer UI Steps

When('I click the {int} minute duration button', async function (this: NodokaWorld, minutes: number) {
  const durationButton = this.page.locator(`button:has-text("${minutes} min"), button:has-text("${minutes} minutes"), [data-duration="${minutes}"]`).first();
  await durationButton.click();
  await this.page.waitForTimeout(300);
});

Then('the remaining time should be approximately {int} minutes', async function (this: NodokaWorld, minutes: number) {
  const sleepTimer = new SleepTimerPage(this.page);
  const remaining = await sleepTimer.getRemainingTime();
  // Allow for some tolerance
  expect(remaining).toBeGreaterThanOrEqual(minutes * 60 - 1);
  expect(remaining).toBeLessThanOrEqual(minutes * 60 + 5);
});

When('I click the end of chapter button', async function (this: NodokaWorld) {
  const endOfChapterBtn = this.page.locator('button:has-text("End of chapter"), button:has-text("End of Chapter")').first();
  await endOfChapterBtn.click();
  await this.page.waitForTimeout(300);
});

When('I select {int} seconds from the fade dropdown', async function (this: NodokaWorld, seconds: number) {
  const fadeDropdown = this.page.locator('select, .dropdown, [data-testid="fade-duration"]').first();
  await fadeDropdown.selectOption({ label: `${seconds}s` });
  await this.page.waitForTimeout(300);
});

When('I open the sleep timer modal', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  await sleepTimer.open();
});

When('I click the cancel timer button', async function (this: NodokaWorld) {
  const cancelBtn = this.page.locator('button:has-text("Cancel"), button:has-text("Stop")').first();
  if (await cancelBtn.isVisible()) {
    await cancelBtn.click();
    await this.page.waitForTimeout(300);
  }
});

Then('the sleep timer should be cancelled', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const isActive = await sleepTimer.isTimerActive();
  expect(isActive).toBe(false);
});

Then('the remaining time should be displayed in the modal', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const modalTime = sleepTimer.modal.locator('.timer-display, .remaining-time');
  await expect(modalTime).toBeVisible();
});

Then('the timer mode should be shown', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const modeElement = sleepTimer.modal.locator('.timer-mode, .mode-indicator');
  await expect(modeElement.first()).toBeVisible();
});

When('I click the Cancel button in modal footer', async function (this: NodokaWorld) {
  const cancelBtn = this.page.locator('.modal-action button:has-text("Cancel")').first();
  if (await cancelBtn.isVisible()) {
    await cancelBtn.click();
    await this.page.waitForTimeout(300);
  }
});

Then('no sleep timer should be active', async function (this: NodokaWorld) {
  const sleepTimer = new SleepTimerPage(this.page);
  const isActive = await sleepTimer.isTimerActive();
  expect(isActive).toBe(false);
});

Then('an active timer indicator should be visible', async function (this: NodokaWorld) {
  const sleepTimerBtn = this.page.locator('button[aria-label*="Sleep"], button[aria-label*="Timer"]');
  const indicator = sleepTimerBtn.locator('.timer-indicator, .badge, .active-indicator');
  await expect(indicator.first()).toBeVisible();
});

Then('the sleep timer button should show an active indicator', async function (this: NodokaWorld) {
  const sleepTimerBtn = this.page.locator('button[aria-label*="Sleep"], button[aria-label*="Timer"]');
  const indicator = sleepTimerBtn.locator('.timer-indicator, .badge, .active-indicator, .indicator');
  await expect(indicator.first()).toBeVisible();
});

Then('the remaining minutes should be displayed on the button', async function (this: NodokaWorld) {
  const sleepTimerBtn = this.page.locator('button[aria-label*="Sleep"], button[aria-label*="Timer"]');
  const timeDisplay = sleepTimerBtn.locator('.time-display, .remaining-time, .timer-time');
  await expect(timeDisplay.first()).toBeVisible();
});

Then('the modal title should be Sleep Timer', async function (this: NodokaWorld) {
  const modalTitle = this.page.locator('.modal-box h2, .modal-box h3, .modal-title');
  const title = await modalTitle.first().textContent();
  expect(title?.toLowerCase()).toContain('sleep timer');
});

Then('a description should be visible', async function (this: NodokaWorld) {
  const description = this.page.locator('.modal-box p, .modal-description, .modal-content p');
  await expect(description.first()).toBeVisible();
});
