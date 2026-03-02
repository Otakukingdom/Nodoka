/**
 * Player Page Object Model
 * Encapsulates player control interactions (play, pause, seek, volume, speed)
 */

import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from './BasePage';

export class PlayerPage extends BasePage {
  // Locators
  readonly playerControls: Locator;
  readonly playButton: Locator;
  readonly pauseButton: Locator;
  readonly stopButton: Locator;
  readonly seekBar: Locator;
  readonly volumeSlider: Locator;
  readonly volumeButton: Locator;
  readonly speedButton: Locator;
  readonly skipForwardButton: Locator;
  readonly skipBackwardButton: Locator;
  readonly currentTimeDisplay: Locator;
  readonly durationDisplay: Locator;

  constructor(page: Page) {
    super(page);
    this.playerControls = page.locator('.btm-nav');
    this.playButton = page.locator('.btm-nav button[aria-label="Play"]');
    this.pauseButton = page.locator('.btm-nav button[aria-label="Pause"]');
    this.stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
    this.seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
    this.volumeSlider = page.locator('.btm-nav .group\\/volume input[type="range"]');
    this.volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]');
    this.speedButton = page.locator('.btm-nav button:has-text("x")');
    this.skipForwardButton = page.locator('.btm-nav button[aria-label="Skip forward"]');
    this.skipBackwardButton = page.locator('.btm-nav button[aria-label="Skip backward"]');
    this.currentTimeDisplay = page.locator('.btm-nav .font-mono').first();
    this.durationDisplay = page.locator('.btm-nav .font-mono').nth(1);
  }

  /**
   * Start playback
   */
  async play(): Promise<void> {
    await this.playButton.first().click();
    await this.page.waitForTimeout(200);
  }

  /**
   * Pause playback
   */
  async pause(): Promise<void> {
    await this.pauseButton.click();
    await this.page.waitForTimeout(200);
  }

  /**
   * Stop playback
   */
  async stop(): Promise<void> {
    await this.stopButton.click();
    await this.page.waitForTimeout(200);
  }

  /**
   * Toggle play/pause
   */
  async togglePlayPause(): Promise<void> {
    const playVisible = await this.playButton.first().isVisible();
    if (playVisible) {
      await this.play();
    } else {
      await this.pause();
    }
  }

  /**
   * Check if playing
   */
  async isPlaying(): Promise<boolean> {
    return await this.pauseButton.isVisible();
  }

  /**
   * Set seek position (in milliseconds)
   */
  async seek(positionMs: number): Promise<void> {
    await this.seekBar.fill(positionMs.toString());
    await this.page.waitForTimeout(200);
  }

  /**
   * Get current seek position
   */
  async getSeekPosition(): Promise<number> {
    const value = await this.seekBar.inputValue();
    return parseInt(value);
  }

  /**
   * Get duration
   */
  async getDuration(): Promise<number> {
    const max = await this.seekBar.getAttribute('max');
    return max ? parseInt(max) : 0;
  }

  /**
   * Set volume (0-200)
   */
  async setVolume(volume: number): Promise<void> {
    await this.volumeSlider.fill(volume.toString());
    await this.page.waitForTimeout(200);
  }

  /**
   * Get current volume
   */
  async getVolume(): Promise<number> {
    const value = await this.volumeSlider.inputValue();
    return parseInt(value);
  }

  /**
   * Toggle mute
   */
  async toggleMute(): Promise<void> {
    await this.volumeButton.first().click();
    await this.page.waitForTimeout(200);
  }

  /**
   * Check if muted
   */
  async isMuted(): Promise<boolean> {
    const label = await this.volumeButton.first().getAttribute('aria-label');
    return label === 'Unmute';
  }

  /**
   * Cycle speed
   */
  async cycleSpeed(): Promise<void> {
    await this.speedButton.first().click();
    await this.page.waitForTimeout(200);
  }

  /**
   * Get current speed
   */
  async getSpeed(): Promise<string> {
    return (await this.speedButton.first().textContent()) ?? '1x';
  }

  /**
   * Skip forward
   */
  async skipForward(): Promise<void> {
    await this.skipForwardButton.click();
    await this.page.waitForTimeout(200);
  }

  /**
   * Skip backward
   */
  async skipBackward(): Promise<void> {
    await this.skipBackwardButton.click();
    await this.page.waitForTimeout(200);
  }

  /**
   * Get current time display
   */
  async getCurrentTimeDisplay(): Promise<string> {
    return (await this.currentTimeDisplay.textContent()) ?? '0:00';
  }

  /**
   * Get duration display
   */
  async getDurationDisplay(): Promise<string> {
    return (await this.durationDisplay.textContent()) ?? '0:00';
  }

  /**
   * Verify player is visible
   */
  async expectPlayerVisible(): Promise<void> {
    await expect(this.playerControls).toBeVisible();
  }

  /**
   * Verify play button is visible
   */
  async expectPlayButtonVisible(): Promise<void> {
    await expect(this.playButton.first()).toBeVisible();
  }

  /**
   * Verify pause button is visible
   */
  async expectPauseButtonVisible(): Promise<void> {
    await expect(this.pauseButton).toBeVisible();
  }

  /**
   * Press keyboard shortcut for play/pause
   */
  async pressPlayPauseShortcut(): Promise<void> {
    await this.page.keyboard.press('Space');
    await this.page.waitForTimeout(200);
  }

  /**
   * Press keyboard shortcut for mute
   */
  async pressMuteShortcut(): Promise<void> {
    await this.page.keyboard.press('m');
    await this.page.waitForTimeout(200);
  }

  /**
   * Drag seek bar to a percentage position
   */
  async dragSeekBar(percent: number): Promise<void> {
    const box = await this.seekBar.boundingBox();
    if (box) {
      const targetX = box.x + (box.width * percent / 100);
      await this.seekBar.hover();
      await this.page.mouse.down();
      await this.page.mouse.move(targetX, box.y + box.height / 2);
      await this.page.mouse.up();
      await this.page.waitForTimeout(200);
    }
  }

  /**
   * Get configured skip forward duration from settings
   */
  async getSkipForwardDuration(): Promise<number> {
    // This is typically a setting, try to read from UI or return default
    const skipButton = this.skipForwardButton;
    const ariaLabel = await skipButton.getAttribute('aria-label');
    // Parse from aria-label like "Skip forward 30 seconds"
    const match = ariaLabel?.match(/(\d+)/);
    return match ? parseInt(match[1]) : 30;
  }

  /**
   * Wait for playback to end
   */
  async waitForPlaybackEnd(timeout: number = 60000): Promise<void> {
    await this.page.waitForFunction(
      () => {
        const pauseButton = document.querySelector('button[aria-label="Pause"]');
        return !pauseButton || pauseButton.getAttribute('hidden') !== null;
      },
      { timeout }
    );
  }

  /**
   * Check if play button is visible (not playing)
   */
  async isPlayButtonVisible(): Promise<boolean> {
    return await this.playButton.first().isVisible();
  }

  /**
   * Check if pause button is visible (playing)
   */
  async isPauseButtonVisible(): Promise<boolean> {
    return await this.pauseButton.isVisible();
  }

  /**
   * Get skip backward duration from settings
   */
  async getSkipBackwardDuration(): Promise<number> {
    const skipButton = this.skipBackwardButton;
    const ariaLabel = await skipButton.getAttribute('aria-label');
    const match = ariaLabel?.match(/(\d+)/);
    return match ? parseInt(match[1]) : 15;
  }
}
