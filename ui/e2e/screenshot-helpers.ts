/**
 * State manipulation utilities for Playwright screenshot generation
 * Provides helper functions for precise UI state control
 */

import { Page } from '@playwright/test';

/**
 * Set player state via Redux dispatch
 */
export async function setPlayerState(
  page: Page,
  state: 'idle' | 'playing' | 'paused'
): Promise<void> {
  await page.evaluate((targetState) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      const playbackState = {
        idle: 'Stopped',
        playing: 'Playing',
        paused: 'Paused',
      }[targetState];
      store.dispatch({
        type: 'player/setPlaybackState',
        payload: playbackState,
      });
    }
  }, state);
  await page.waitForTimeout(100);
}

/**
 * Set player progress percentage
 */
export async function setPlayerProgress(page: Page, percent: number): Promise<void> {
  await page.evaluate((progress) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      const duration = 1800000; // 30 minutes
      const position = (duration * progress) / 100;
      store.dispatch({
        type: 'player/setCurrentPosition',
        payload: position,
      });
      store.dispatch({
        type: 'player/setDuration',
        payload: duration,
      });
    }
  }, percent);
  await page.waitForTimeout(100);
}

/**
 * Set volume level (0-100)
 */
export async function setVolume(page: Page, level: number, muted = false): Promise<void> {
  await page.evaluate(
    ({ volume, isMuted }) => {
      const store = (window as any).__REDUX_STORE__;
      if (store) {
        store.dispatch({
          type: 'player/setVolume',
          payload: volume,
        });
        store.dispatch({
          type: 'player/setMuted',
          payload: isMuted,
        });
      }
    },
    { volume: level, isMuted: muted }
  );
  await page.waitForTimeout(100);
}

/**
 * Set playback speed
 */
export async function setPlaybackSpeed(page: Page, speed: number): Promise<void> {
  await page.evaluate((playbackSpeed) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({
        type: 'player/setSpeed',
        payload: playbackSpeed,
      });
    }
  }, speed);
  await page.waitForTimeout(100);
}

/**
 * Expand volume slider for expanded state screenshot
 */
export async function expandVolumeSlider(page: Page): Promise<void> {
  const volumeGroup = page.locator('.group\\/volume, [data-testid="volume-control"]');
  await volumeGroup.hover();
  await page.waitForTimeout(300);
}

/**
 * Select an audiobook by ID
 */
export async function selectAudiobook(page: Page, id: number): Promise<void> {
  await page.evaluate((audiobookId) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({
        type: 'app/setSelectedAudiobookId',
        payload: audiobookId,
      });
    }
  }, id);
  await page.waitForTimeout(200);
}

/**
 * Open a modal by name
 */
export async function openModal(
  page: Page,
  name: 'settings' | 'keyboard-help' | 'sleep-timer' | 'bookmark-editor'
): Promise<void> {
  const actions: Record<string, () => Promise<void>> = {
    settings: async () => {
      await page.click('button[aria-label="Settings"]');
    },
    'keyboard-help': async () => {
      await page.keyboard.press('F1');
    },
    'sleep-timer': async () => {
      // Sleep timer is usually accessed via player controls
      const sleepButton = page.locator('button[aria-label="Sleep timer"]');
      if (await sleepButton.isVisible()) {
        await sleepButton.click();
      }
    },
    'bookmark-editor': async () => {
      // Open via keyboard shortcut or button
      await page.keyboard.press('b');
    },
  };

  await actions[name]();
  await page.waitForSelector('.modal-box', { timeout: 5000 });
  await page.waitForTimeout(300);
}

/**
 * Close any open modal
 */
export async function closeModal(page: Page): Promise<void> {
  const closeButton = page.locator('.modal-box button[aria-label="Close"]');
  if (await closeButton.isVisible()) {
    await closeButton.click();
  } else {
    const doneButton = page.locator('.modal-box button:has-text("Done")');
    if (await doneButton.isVisible()) {
      await doneButton.click();
    } else {
      await page.keyboard.press('Escape');
    }
  }
  await page.waitForTimeout(300);
}

/**
 * Set theme via Redux dispatch
 */
export async function setThemeViaRedux(
  page: Page,
  theme: 'Light' | 'Dark' | 'System'
): Promise<void> {
  await page.evaluate((themeValue) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({
        type: 'app/setTheme',
        payload: themeValue,
      });
    }
  }, theme);
  await page.waitForTimeout(300);
}

/**
 * Focus an element and wait for focus ring to render
 */
export async function focusElement(page: Page, selector: string): Promise<void> {
  await page.focus(selector);
  await page.waitForTimeout(200);
}

/**
 * Hover over an element and wait for hover state
 */
export async function hoverElement(page: Page, selector: string): Promise<void> {
  await page.hover(selector);
  await page.waitForTimeout(300);
}

/**
 * Set sleep timer state
 */
export async function setSleepTimerState(
  page: Page,
  state: 'inactive' | 'duration' | 'end-of-chapter' | 'active'
): Promise<void> {
  await page.evaluate((timerState) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      const payloads: Record<string, any> = {
        inactive: { active: false, mode: null, remainingSeconds: null },
        duration: { active: true, mode: 'Duration', remainingSeconds: 1800 },
        'end-of-chapter': { active: true, mode: 'EndOfChapter', remainingSeconds: null },
        active: { active: true, mode: 'Duration', remainingSeconds: 300 },
      };
      store.dispatch({
        type: 'sleepTimer/setState',
        payload: payloads[timerState],
      });
    }
  }, state);
  await page.waitForTimeout(200);
}

/**
 * Set focus mode state
 */
export async function setFocusMode(page: Page, enabled: boolean): Promise<void> {
  await page.evaluate((isEnabled) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({
        type: 'app/setFocusMode',
        payload: isEnabled ? 'Enabled' : 'Disabled',
      });
    }
  }, enabled);
  await page.waitForTimeout(200);
}

/**
 * Set reduced motion preference
 */
export async function setReducedMotion(page: Page, enabled: boolean): Promise<void> {
  await page.evaluate((isEnabled) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({
        type: 'app/setPrefersReducedMotion',
        payload: isEnabled,
      });
    }
  }, enabled);
  await page.waitForTimeout(200);
}

/**
 * Show drag and drop overlay
 */
export async function showDragDropOverlay(page: Page): Promise<void> {
  // Simulate drag enter on the document
  await page.evaluate(() => {
    const event = new DragEvent('dragenter', {
      bubbles: true,
      cancelable: true,
      dataTransfer: new DataTransfer(),
    });
    document.body.dispatchEvent(event);
  });
  await page.waitForTimeout(300);
}

/**
 * Set directory scan progress
 */
export async function setDirectoryScanProgress(
  page: Page,
  progress: { current: number; total: number; status: string }
): Promise<void> {
  await page.evaluate((scanProgress) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({
        type: 'app/setScanProgress',
        payload: scanProgress,
      });
    }
  }, progress);
  await page.waitForTimeout(200);
}

/**
 * Set search query in library
 */
export async function setSearchQuery(page: Page, query: string): Promise<void> {
  const searchInput = page.locator('.library-panel input[type="text"]');
  await searchInput.fill(query);
  await page.waitForTimeout(300);
}

/**
 * Set sort order in library
 */
export async function setSortOrder(
  page: Page,
  order: 'RecentlyPlayed' | 'Title' | 'Progress' | 'DateAdded'
): Promise<void> {
  await page.evaluate((sortOrder) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({
        type: 'app/setSortOrder',
        payload: sortOrder,
      });
    }
  }, order);
  await page.waitForTimeout(200);
}

/**
 * Trigger multiple notifications for stacked state
 */
export async function triggerStackedNotifications(page: Page): Promise<void> {
  const notifications = [
    { kind: 'Success', message: 'Audiobook added successfully' },
    { kind: 'Info', message: 'Sync completed' },
    { kind: 'Warning', message: 'Low disk space' },
  ];

  for (const notif of notifications) {
    await page.evaluate((n) => {
      const store = (window as any).__REDUX_STORE__;
      if (store) {
        store.dispatch({
          type: 'app/addNotification',
          payload: {
            id: Date.now() + Math.random(),
            kind: n.kind,
            message: n.message,
            durationMs: 10000,
            createdAt: Date.now(),
          },
        });
      }
    }, notif);
    await page.waitForTimeout(100);
  }
  await page.waitForTimeout(300);
}

/**
 * Force loading state
 */
export async function forceLoadingState(page: Page, isLoading: boolean): Promise<void> {
  await page.evaluate((loading) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({
        type: 'app/setLoadState',
        payload: loading ? 'Loading' : 'Ready',
      });
    }
  }, isLoading);
  await page.waitForTimeout(200);
}

/**
 * Screenshot configuration for consistent output
 */
export const screenshotConfig = {
  animations: 'disabled' as const,
  fullPage: false,
  maxDiffPixels: 200,
};

/**
 * Full page screenshot configuration
 */
export const fullPageScreenshotConfig = {
  ...screenshotConfig,
  fullPage: true as const,
};

/**
 * Screenshot path generator for organized output
 */
export function getScreenshotPath(
  outputDir: string,
  category: string,
  name: string,
  theme?: 'light' | 'dark'
): string {
  const themeSuffix = theme ? `-${theme}` : '';
  return `${outputDir}/${category}/${name}${themeSuffix}.png`;
}
