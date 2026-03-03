/**
 * Screenshot generation test suite for Nodoka UI
 * Captures comprehensive UI states for documentation and visual QA
 *
 * Run via: cargo xtask screenshot
 * Or: npm run screenshot
 */

import { test, expect } from '@playwright/test';
import {
  setPlayerState,
  setPlayerProgress,
  setVolume,
  setPlaybackSpeed,
  expandVolumeSlider,
  selectAudiobook,
  openModal,
  closeModal,
  setThemeViaRedux,
  focusElement,
  hoverElement,
  setSleepTimerState,
  setFocusMode,
  setReducedMotion,
  showDragDropOverlay,
  setDirectoryScanProgress,
  setSearchQuery,
  setSortOrder,
  triggerStackedNotifications,
  forceLoadingState,
  screenshotConfig,
} from './screenshot-helpers';
import {
  screenshotAudiobooks,
  audiobookWithLongTitle,
  audiobookWithoutAuthor,
  audiobookWithManyChapters,
  standardFiles,
  filesWithMissing,
  manyChapterFiles,
  standardBookmarks,
  manyBookmarks,
  bookmarkWithLongLabel,
  screenshotDirectories,
  screenshotSettings,
  sleepTimerStates,
  playerStates,
  volumeStates,
  speedVariations,
  progressLevels,
  getScenarioData,
} from './fixtures/screenshot-scenarios';

/**
 * Screenshot output directory from environment or default
 */
const outputDir = process.env.SCREENSHOT_OUTPUT_DIR || 'screenshots';

/**
 * Helper to take screenshot with consistent naming
 */
async function takeScreenshot(
  // eslint-disable-next-line no-unused-vars
  page: import('@playwright/test').Page,
  category: string,
  name: string,
  theme?: 'light' | 'dark'
): Promise<void> {
  const themeSuffix = theme ? `-${theme}` : '';
  const path = `${outputDir}/${category}/${name}${themeSuffix}.png`;
  await page.screenshot({
    path,
    animations: 'disabled',
    fullPage: false,
  });
}

/**
 * Helper to inject scenario data into page
 */
async function injectScenarioData(
  page: import('@playwright/test').Page,
  scenario: 'default' | 'empty' | 'edge-cases' | 'many-chapters'
): Promise<void> {
  const data = getScenarioData(scenario);
  await page.evaluate(
    ({ audiobooks, files, bookmarks, directories, settings }) => {
      const store = (window as any).__REDUX_STORE__;
      if (store) {
        // Set library data
        store.dispatch({ type: 'library/setAudiobooks', payload: audiobooks });
        store.dispatch({ type: 'library/setFiles', payload: files });
        store.dispatch({ type: 'library/setBookmarks', payload: bookmarks });
        store.dispatch({ type: 'library/setDirectories', payload: directories });

        // Set settings
        store.dispatch({ type: 'app/setTheme', payload: settings.theme });
        store.dispatch({ type: 'app/setPanelRatio', payload: settings.panelRatio });
        store.dispatch({ type: 'app/setFocusMode', payload: settings.focusMode ? 'Enabled' : 'Disabled' });
        store.dispatch({ type: 'player/setVolume', payload: settings.volume });
        store.dispatch({ type: 'player/setSpeed', payload: settings.speed });
        store.dispatch({ type: 'app/setPrefersReducedMotion', payload: settings.prefersReducedMotion });
      }
    },
    data
  );
  await page.waitForTimeout(300);
}

/**
 * Helper to inject custom audiobooks
 */
async function injectAudiobooks(
  page: import('@playwright/test').Page,
  audiobooks: typeof screenshotAudiobooks
): Promise<void> {
  await page.evaluate((books) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({ type: 'library/setAudiobooks', payload: books });
    }
  }, audiobooks);
  await page.waitForTimeout(300);
}

/**
 * Helper to inject custom files
 */
async function injectFiles(
  page: import('@playwright/test').Page,
  files: typeof standardFiles
): Promise<void> {
  await page.evaluate((f) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({ type: 'library/setFiles', payload: f });
    }
  }, files);
  await page.waitForTimeout(300);
}

/**
 * Helper to inject custom bookmarks
 */
async function injectBookmarks(
  page: import('@playwright/test').Page,
  bookmarks: typeof standardBookmarks
): Promise<void> {
  await page.evaluate((b) => {
    const store = (window as any).__REDUX_STORE__;
    if (store) {
      store.dispatch({ type: 'library/setBookmarks', payload: b });
    }
  }, bookmarks);
  await page.waitForTimeout(300);
}

// Configure test for parallel execution
test.describe.parallel('Screenshot Generation', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to app
    await page.goto('/');

    // Wait for app to be ready
    await page.waitForSelector('.app-container, [data-testid="app-container"]', {
      timeout: 10000,
    });

    // Wait for initial render to complete
    await page.waitForTimeout(500);
  });

  // ========================================
  // APP SHELL STATES
  // ========================================
  test.describe('app-shell', () => {
    test('loading-skeleton', async ({ page }) => {
      await forceLoadingState(page, true);
      await takeScreenshot(page, 'app-shell', 'loading-skeleton');
    });

    test('empty-library-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await injectScenarioData(page, 'empty');
      await takeScreenshot(page, 'app-shell', 'empty-library', 'light');
    });

    test('empty-library-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await injectScenarioData(page, 'empty');
      await takeScreenshot(page, 'app-shell', 'empty-library', 'dark');
    });

    test('populated-library-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await injectScenarioData(page, 'default');
      await takeScreenshot(page, 'app-shell', 'populated-library', 'light');
    });

    test('populated-library-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await injectScenarioData(page, 'default');
      await takeScreenshot(page, 'app-shell', 'populated-library', 'dark');
    });
  });

  // ========================================
  // LIBRARY PANEL STATES
  // ========================================
  test.describe('library-panel', () => {
    test.beforeEach(async ({ page }) => {
      await injectScenarioData(page, 'default');
    });

    test('audiobook-selected-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await selectAudiobook(page, 1);
      await takeScreenshot(page, 'library-panel', 'audiobook-selected', 'light');
    });

    test('audiobook-selected-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await selectAudiobook(page, 1);
      await takeScreenshot(page, 'library-panel', 'audiobook-selected', 'dark');
    });

    test('audiobook-hover', async ({ page }) => {
      // Audiobook cards are .card elements within the audiobook list
      const audiobookCard = page.locator('#audiobook-list .card').first();
      await audiobookCard.hover();
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'library-panel', 'audiobook-hover');
    });

    test('search-empty', async ({ page }) => {
      const searchInput = page.locator('.library-panel input[type="text"], [data-testid="search-input"]');
      await searchInput.click();
      await takeScreenshot(page, 'library-panel', 'search-empty');
    });

    test('search-with-query', async ({ page }) => {
      await setSearchQuery(page, 'Adventure');
      await takeScreenshot(page, 'library-panel', 'search-with-query');
    });

    test('search-no-results', async ({ page }) => {
      await setSearchQuery(page, 'NonexistentBook');
      await takeScreenshot(page, 'library-panel', 'search-no-results');
    });

    test('sort-dropdown-open', async ({ page }) => {
      // Sort dropdown is a select element in the toolbar
      const sortSelect = page.locator('.library-panel select').first();
      await sortSelect.click();
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'library-panel', 'sort-dropdown-open');
    });
  });

  // ========================================
  // DETAILS PANEL STATES
  // ========================================
  test.describe('details-panel', () => {
    test('no-selection-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await injectScenarioData(page, 'default');
      await takeScreenshot(page, 'details-panel', 'no-selection', 'light');
    });

    test('no-selection-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await injectScenarioData(page, 'default');
      await takeScreenshot(page, 'details-panel', 'no-selection', 'dark');
    });

    test('audiobook-details-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await injectScenarioData(page, 'default');
      await selectAudiobook(page, 1);
      await takeScreenshot(page, 'details-panel', 'audiobook-details', 'light');
    });

    test('audiobook-details-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await injectScenarioData(page, 'default');
      await selectAudiobook(page, 1);
      await takeScreenshot(page, 'details-panel', 'audiobook-details', 'dark');
    });

    test('files-list', async ({ page }) => {
      await injectScenarioData(page, 'default');
      await selectAudiobook(page, 1);
      // Navigate to files tab if needed
      const filesTab = page.locator('[data-testid="files-tab"], button:has-text("Files")');
      if (await filesTab.isVisible()) {
        await filesTab.click();
        await page.waitForTimeout(200);
      }
      await takeScreenshot(page, 'details-panel', 'files-list');
    });

    test('empty-files-state', async ({ page }) => {
      await injectAudiobooks(page, screenshotAudiobooks);
      await injectFiles(page, []);
      await selectAudiobook(page, 1);
      await takeScreenshot(page, 'details-panel', 'empty-files-state');
    });

    test('bookmarks-list', async ({ page }) => {
      await injectScenarioData(page, 'default');
      await selectAudiobook(page, 1);
      // Navigate to bookmarks tab if needed
      const bookmarksTab = page.locator('[data-testid="bookmarks-tab"], button:has-text("Bookmarks")');
      if (await bookmarksTab.isVisible()) {
        await bookmarksTab.click();
        await page.waitForTimeout(200);
      }
      await takeScreenshot(page, 'details-panel', 'bookmarks-list');
    });

    test('empty-bookmarks-state', async ({ page }) => {
      await injectAudiobooks(page, screenshotAudiobooks);
      await injectFiles(page, standardFiles);
      await injectBookmarks(page, []);
      await selectAudiobook(page, 1);
      await takeScreenshot(page, 'details-panel', 'empty-bookmarks-state');
    });

    test('missing-file-indicator', async ({ page }) => {
      await injectAudiobooks(page, screenshotAudiobooks);
      await injectFiles(page, filesWithMissing);
      await selectAudiobook(page, 1);
      // Navigate to files tab if needed
      const filesTab = page.locator('[data-testid="files-tab"], button:has-text("Files")');
      if (await filesTab.isVisible()) {
        await filesTab.click();
        await page.waitForTimeout(200);
      }
      await takeScreenshot(page, 'details-panel', 'missing-file-indicator');
    });
  });

  // ========================================
  // PLAYER CONTROLS STATES
  // ========================================
  test.describe('player-controls', () => {
    test.beforeEach(async ({ page }) => {
      await injectScenarioData(page, 'default');
      await selectAudiobook(page, 1);
    });

    test('idle-stopped', async ({ page }) => {
      await setPlayerState(page, 'idle');
      await takeScreenshot(page, 'player-controls', 'idle-stopped');
    });

    test('paused', async ({ page }) => {
      await setPlayerState(page, 'paused');
      await setPlayerProgress(page, 50);
      await takeScreenshot(page, 'player-controls', 'paused');
    });

    test('playing', async ({ page }) => {
      await setPlayerState(page, 'playing');
      await setPlayerProgress(page, 50);
      await takeScreenshot(page, 'player-controls', 'playing');
    });

    test('progress-0-percent', async ({ page }) => {
      await setPlayerState(page, 'paused');
      await setPlayerProgress(page, 0);
      await takeScreenshot(page, 'player-controls', 'progress-0-percent');
    });

    test('progress-50-percent', async ({ page }) => {
      await setPlayerState(page, 'paused');
      await setPlayerProgress(page, 50);
      await takeScreenshot(page, 'player-controls', 'progress-50-percent');
    });

    test('progress-100-percent', async ({ page }) => {
      await setPlayerState(page, 'paused');
      await setPlayerProgress(page, 100);
      await takeScreenshot(page, 'player-controls', 'progress-100-percent');
    });

    test('volume-slider-expanded', async ({ page }) => {
      await setPlayerState(page, 'paused');
      await expandVolumeSlider(page);
      await takeScreenshot(page, 'player-controls', 'volume-slider-expanded');
    });

    test('volume-muted', async ({ page }) => {
      await setPlayerState(page, 'paused');
      await setVolume(page, 0, true);
      await takeScreenshot(page, 'player-controls', 'volume-muted');
    });

    test('volume-max', async ({ page }) => {
      await setPlayerState(page, 'paused');
      await setVolume(page, 100, false);
      await takeScreenshot(page, 'player-controls', 'volume-max');
    });

    test('speed-0.75x', async ({ page }) => {
      await setPlayerState(page, 'paused');
      await setPlaybackSpeed(page, 0.75);
      await takeScreenshot(page, 'player-controls', 'speed-0.75x');
    });

    test('speed-1.0x', async ({ page }) => {
      await setPlayerState(page, 'paused');
      await setPlaybackSpeed(page, 1.0);
      await takeScreenshot(page, 'player-controls', 'speed-1.0x');
    });

    test('speed-1.5x', async ({ page }) => {
      await setPlayerState(page, 'paused');
      await setPlaybackSpeed(page, 1.5);
      await takeScreenshot(page, 'player-controls', 'speed-1.5x');
    });

    test('speed-2.0x', async ({ page }) => {
      await setPlayerState(page, 'paused');
      await setPlaybackSpeed(page, 2.0);
      await takeScreenshot(page, 'player-controls', 'speed-2.0x');
    });
  });

  // ========================================
  // MODALS - SETTINGS
  // ========================================
  test.describe('modals/settings', () => {
    test.beforeEach(async ({ page }) => {
      await injectScenarioData(page, 'default');
    });

    test('default-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await openModal(page, 'settings');
      await takeScreenshot(page, 'modals/settings', 'default', 'light');
    });

    test('default-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await openModal(page, 'settings');
      await takeScreenshot(page, 'modals/settings', 'default', 'dark');
    });

    test('theme-dropdown-open', async ({ page }) => {
      await openModal(page, 'settings');
      // Theme dropdown is the first select in the settings modal
      const themeDropdown = page.locator('.modal-box .form-control select').first();
      await themeDropdown.click();
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'modals/settings', 'theme-dropdown-open');
    });

    test('theme-option-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await openModal(page, 'settings');
      await takeScreenshot(page, 'modals/settings', 'theme-option-light');
    });

    test('theme-option-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await openModal(page, 'settings');
      await takeScreenshot(page, 'modals/settings', 'theme-option-dark');
    });

    test('theme-option-system', async ({ page }) => {
      await setThemeViaRedux(page, 'System');
      await openModal(page, 'settings');
      await takeScreenshot(page, 'modals/settings', 'theme-option-system');
    });
  });

  // ========================================
  // MODALS - KEYBOARD HELP
  // ========================================
  test.describe('modals/keyboard-help', () => {
    test('keyboard-help-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await injectScenarioData(page, 'default');
      await openModal(page, 'keyboard-help');
      await takeScreenshot(page, 'modals/keyboard-help', 'keyboard-help', 'light');
    });

    test('keyboard-help-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await injectScenarioData(page, 'default');
      await openModal(page, 'keyboard-help');
      await takeScreenshot(page, 'modals/keyboard-help', 'keyboard-help', 'dark');
    });
  });

  // ========================================
  // MODALS - SLEEP TIMER
  // ========================================
  test.describe('modals/sleep-timer', () => {
    test.beforeEach(async ({ page }) => {
      await injectScenarioData(page, 'default');
      await selectAudiobook(page, 1);
    });

    test('inactive-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await setSleepTimerState(page, 'inactive');
      await openModal(page, 'sleep-timer');
      await takeScreenshot(page, 'modals/sleep-timer', 'inactive', 'light');
    });

    test('inactive-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await setSleepTimerState(page, 'inactive');
      await openModal(page, 'sleep-timer');
      await takeScreenshot(page, 'modals/sleep-timer', 'inactive', 'dark');
    });

    test('duration-mode', async ({ page }) => {
      await setSleepTimerState(page, 'duration');
      await openModal(page, 'sleep-timer');
      await takeScreenshot(page, 'modals/sleep-timer', 'duration-mode');
    });

    test('end-of-chapter-mode', async ({ page }) => {
      await setSleepTimerState(page, 'end-of-chapter');
      await openModal(page, 'sleep-timer');
      await takeScreenshot(page, 'modals/sleep-timer', 'end-of-chapter-mode');
    });

    test('active-countdown', async ({ page }) => {
      await setSleepTimerState(page, 'active');
      await openModal(page, 'sleep-timer');
      await takeScreenshot(page, 'modals/sleep-timer', 'active-countdown');
    });
  });

  // ========================================
  // MODALS - BOOKMARK EDITOR
  // ========================================
  // NOTE: BookmarkEditorModal component does not exist yet.
  // These tests are skipped until the modal is implemented.
  test.describe.skip('modals/bookmark-editor', () => {
    test.beforeEach(async ({ page }) => {
      await injectScenarioData(page, 'default');
      await selectAudiobook(page, 1);
    });

    test('new-bookmark-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await openModal(page, 'bookmark-editor');
      await takeScreenshot(page, 'modals/bookmark-editor', 'new-bookmark', 'light');
    });

    test('new-bookmark-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await openModal(page, 'bookmark-editor');
      await takeScreenshot(page, 'modals/bookmark-editor', 'new-bookmark', 'dark');
    });

    test('edit-existing', async ({ page }) => {
      await injectBookmarks(page, standardBookmarks);
      // Click on an existing bookmark to edit
      const bookmarkItem = page.locator('[data-testid="bookmark-item"], .bookmark-item').first();
      if (await bookmarkItem.isVisible()) {
        await bookmarkItem.dblclick();
        await page.waitForTimeout(300);
      } else {
        await openModal(page, 'bookmark-editor');
      }
      await takeScreenshot(page, 'modals/bookmark-editor', 'edit-existing');
    });
  });

  // ========================================
  // MODALS - CONFIRMATION
  // ========================================
  test.describe('modals/confirmation', () => {
    test('confirmation-dialog-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await injectScenarioData(page, 'default');
      // Trigger a confirmation dialog - typically via delete action
      await selectAudiobook(page, 1);
      const deleteButton = page.locator('[data-testid="delete-audiobook"], button[aria-label="Delete"]');
      if (await deleteButton.isVisible()) {
        await deleteButton.click();
        await page.waitForTimeout(300);
      } else {
        // Manually dispatch confirmation state
        await page.evaluate(() => {
          const store = (window as any).__REDUX_STORE__;
          if (store) {
            store.dispatch({
              type: 'app/showConfirmation',
              payload: {
                title: 'Delete Audiobook',
                message: 'Are you sure you want to delete this audiobook?',
                confirmLabel: 'Delete',
                cancelLabel: 'Cancel',
              },
            });
          }
        });
        await page.waitForTimeout(300);
      }
      await takeScreenshot(page, 'modals/confirmation', 'confirmation-dialog', 'light');
    });

    test('confirmation-dialog-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await injectScenarioData(page, 'default');
      // Trigger confirmation via Redux
      await page.evaluate(() => {
        const store = (window as any).__REDUX_STORE__;
        if (store) {
          store.dispatch({
            type: 'app/showConfirmation',
            payload: {
              title: 'Delete Audiobook',
              message: 'Are you sure you want to delete this audiobook?',
              confirmLabel: 'Delete',
              cancelLabel: 'Cancel',
            },
          });
        }
      });
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'modals/confirmation', 'confirmation-dialog', 'dark');
    });
  });

  // ========================================
  // NOTIFICATIONS
  // ========================================
  test.describe('notifications', () => {
    test.beforeEach(async ({ page }) => {
      await injectScenarioData(page, 'default');
    });

    test('success-toast', async ({ page }) => {
      await page.evaluate(() => {
        const store = (window as any).__REDUX_STORE__;
        if (store) {
          store.dispatch({
            type: 'app/addNotification',
            payload: {
              id: Date.now(),
              kind: 'Success',
              message: 'Audiobook added successfully',
              durationMs: 10000,
              createdAt: Date.now(),
            },
          });
        }
      });
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'notifications', 'success-toast');
    });

    test('info-toast', async ({ page }) => {
      await page.evaluate(() => {
        const store = (window as any).__REDUX_STORE__;
        if (store) {
          store.dispatch({
            type: 'app/addNotification',
            payload: {
              id: Date.now(),
              kind: 'Info',
              message: 'Sync completed successfully',
              durationMs: 10000,
              createdAt: Date.now(),
            },
          });
        }
      });
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'notifications', 'info-toast');
    });

    test('warning-toast', async ({ page }) => {
      await page.evaluate(() => {
        const store = (window as any).__REDUX_STORE__;
        if (store) {
          store.dispatch({
            type: 'app/addNotification',
            payload: {
              id: Date.now(),
              kind: 'Warning',
              message: 'Low disk space available',
              durationMs: 10000,
              createdAt: Date.now(),
            },
          });
        }
      });
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'notifications', 'warning-toast');
    });

    test('error-toast', async ({ page }) => {
      await page.evaluate(() => {
        const store = (window as any).__REDUX_STORE__;
        if (store) {
          store.dispatch({
            type: 'app/addNotification',
            payload: {
              id: Date.now(),
              kind: 'Error',
              message: 'Failed to load audiobook',
              durationMs: 10000,
              createdAt: Date.now(),
            },
          });
        }
      });
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'notifications', 'error-toast');
    });

    test('multiple-stacked', async ({ page }) => {
      await triggerStackedNotifications(page);
      await takeScreenshot(page, 'notifications', 'multiple-stacked');
    });
  });

  // ========================================
  // SPECIAL STATES
  // ========================================
  test.describe('special-states', () => {
    test.beforeEach(async ({ page }) => {
      await injectScenarioData(page, 'default');
    });

    test('drag-drop-overlay-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await showDragDropOverlay(page);
      await takeScreenshot(page, 'special-states', 'drag-drop-overlay', 'light');
    });

    test('drag-drop-overlay-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await showDragDropOverlay(page);
      await takeScreenshot(page, 'special-states', 'drag-drop-overlay', 'dark');
    });

    test('directory-scan-progress', async ({ page }) => {
      await setDirectoryScanProgress(page, {
        current: 3,
        total: 10,
        status: 'Scanning audiobooks...',
      });
      await takeScreenshot(page, 'special-states', 'directory-scan-progress');
    });

    test('focus-mode-enabled-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await setFocusMode(page, true);
      await takeScreenshot(page, 'special-states', 'focus-mode-enabled', 'light');
    });

    test('focus-mode-enabled-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await setFocusMode(page, true);
      await takeScreenshot(page, 'special-states', 'focus-mode-enabled', 'dark');
    });

    test('reduced-motion-preference', async ({ page }) => {
      await setReducedMotion(page, true);
      await takeScreenshot(page, 'special-states', 'reduced-motion-preference');
    });
  });

  // ========================================
  // RESPONSIVE LAYOUTS
  // ========================================
  test.describe('responsive', () => {
    test.beforeEach(async ({ page }) => {
      await injectScenarioData(page, 'default');
    });

    test('desktop-1280x720-light', async ({ page }) => {
      await page.setViewportSize({ width: 1280, height: 720 });
      await setThemeViaRedux(page, 'Light');
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'responsive', 'desktop-1280x720', 'light');
    });

    test('desktop-1280x720-dark', async ({ page }) => {
      await page.setViewportSize({ width: 1280, height: 720 });
      await setThemeViaRedux(page, 'Dark');
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'responsive', 'desktop-1280x720', 'dark');
    });

    test('tablet-1024x768', async ({ page }) => {
      await page.setViewportSize({ width: 1024, height: 768 });
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'responsive', 'tablet-1024x768');
    });

    test('small-desktop-800x600', async ({ page }) => {
      await page.setViewportSize({ width: 800, height: 600 });
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'responsive', 'small-desktop-800x600');
    });

    test('compact-640x480', async ({ page }) => {
      await page.setViewportSize({ width: 640, height: 480 });
      await page.waitForTimeout(300);
      await takeScreenshot(page, 'responsive', 'compact-640x480');
    });
  });

  // ========================================
  // ACCESSIBILITY
  // ========================================
  test.describe('accessibility', () => {
    test.beforeEach(async ({ page }) => {
      await injectScenarioData(page, 'default');
    });

    test('focus-ring-settings-button-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await focusElement(page, 'button[aria-label="Settings"]');
      await takeScreenshot(page, 'accessibility', 'focus-ring-settings-button', 'light');
    });

    test('focus-ring-settings-button-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await focusElement(page, 'button[aria-label="Settings"]');
      await takeScreenshot(page, 'accessibility', 'focus-ring-settings-button', 'dark');
    });

    test('focus-ring-search-input', async ({ page }) => {
      await focusElement(page, '.library-panel input[type="text"], [data-testid="search-input"]');
      await takeScreenshot(page, 'accessibility', 'focus-ring-search-input');
    });

    test('focus-ring-audiobook-item', async ({ page }) => {
      // Audiobook cards are .card elements within the audiobook list
      const audiobookItem = page.locator('#audiobook-list .card').first();
      await audiobookItem.focus();
      await page.waitForTimeout(200);
      await takeScreenshot(page, 'accessibility', 'focus-ring-audiobook-item');
    });

    test('focus-ring-modal-button', async ({ page }) => {
      await openModal(page, 'settings');
      // Use .first() to avoid strict mode violation when multiple buttons match
      const closeButton = page.locator('.modal-box button[aria-label="Close"], .modal-box button:has-text("Done")').first();
      await closeButton.focus();
      await page.waitForTimeout(200);
      await takeScreenshot(page, 'accessibility', 'focus-ring-modal-button');
    });
  });

  // ========================================
  // EDGE CASES
  // ========================================
  test.describe('edge-cases', () => {
    test('long-audiobook-title-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await injectAudiobooks(page, [audiobookWithLongTitle]);
      await selectAudiobook(page, audiobookWithLongTitle.id);
      await takeScreenshot(page, 'edge-cases', 'long-audiobook-title', 'light');
    });

    test('long-audiobook-title-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await injectAudiobooks(page, [audiobookWithLongTitle]);
      await selectAudiobook(page, audiobookWithLongTitle.id);
      await takeScreenshot(page, 'edge-cases', 'long-audiobook-title', 'dark');
    });

    test('long-author-name', async ({ page }) => {
      await injectAudiobooks(page, [audiobookWithLongTitle]);
      await selectAudiobook(page, audiobookWithLongTitle.id);
      await takeScreenshot(page, 'edge-cases', 'long-author-name');
    });

    test('many-chapters-scrolling', async ({ page }) => {
      await injectAudiobooks(page, [audiobookWithManyChapters]);
      await injectFiles(page, manyChapterFiles);
      await selectAudiobook(page, audiobookWithManyChapters.id);
      // Navigate to files tab
      const filesTab = page.locator('[data-testid="files-tab"], button:has-text("Files")');
      if (await filesTab.isVisible()) {
        await filesTab.click();
        await page.waitForTimeout(200);
      }
      await takeScreenshot(page, 'edge-cases', 'many-chapters-scrolling');
    });

    test('many-bookmarks', async ({ page }) => {
      await injectAudiobooks(page, screenshotAudiobooks);
      await injectFiles(page, standardFiles);
      await injectBookmarks(page, manyBookmarks);
      await selectAudiobook(page, 1);
      // Navigate to bookmarks tab
      const bookmarksTab = page.locator('[data-testid="bookmarks-tab"], button:has-text("Bookmarks")');
      if (await bookmarksTab.isVisible()) {
        await bookmarksTab.click();
        await page.waitForTimeout(200);
      }
      await takeScreenshot(page, 'edge-cases', 'many-bookmarks');
    });

    test('long-bookmark-label', async ({ page }) => {
      await injectAudiobooks(page, screenshotAudiobooks);
      await injectFiles(page, standardFiles);
      await injectBookmarks(page, [bookmarkWithLongLabel]);
      await selectAudiobook(page, 1);
      // Navigate to bookmarks tab
      const bookmarksTab = page.locator('[data-testid="bookmarks-tab"], button:has-text("Bookmarks")');
      if (await bookmarksTab.isVisible()) {
        await bookmarksTab.click();
        await page.waitForTimeout(200);
      }
      await takeScreenshot(page, 'edge-cases', 'long-bookmark-label');
    });

    test('no-author-metadata-light', async ({ page }) => {
      await setThemeViaRedux(page, 'Light');
      await injectAudiobooks(page, [audiobookWithoutAuthor]);
      await selectAudiobook(page, audiobookWithoutAuthor.id);
      await takeScreenshot(page, 'edge-cases', 'no-author-metadata', 'light');
    });

    test('no-author-metadata-dark', async ({ page }) => {
      await setThemeViaRedux(page, 'Dark');
      await injectAudiobooks(page, [audiobookWithoutAuthor]);
      await selectAudiobook(page, audiobookWithoutAuthor.id);
      await takeScreenshot(page, 'edge-cases', 'no-author-metadata', 'dark');
    });
  });
});
