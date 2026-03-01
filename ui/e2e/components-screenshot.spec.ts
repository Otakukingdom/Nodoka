/**
 * E2E screenshot tests for all UI components
 * Provides comprehensive visual regression coverage for TopBar, LibraryPanel, DetailsPanel, and PlayerControls
 */

import { test, expect, viewports } from './fixtures';
import { screenshotConfig, fullPageScreenshotConfig, gotoAndWait, setTheme } from './test-utils';

test.describe('TopBar Component Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('TopBar component screenshot', async ({ page }) => {
    const topBar = page.locator('.navbar');
    await expect(topBar).toHaveScreenshot('topbar-component.png', screenshotConfig);
  });

  test('TopBar logo screenshot', async ({ page }) => {
    const logo = page.locator('.navbar .gradient-premium');
    await expect(logo).toHaveScreenshot('topbar-logo.png', screenshotConfig);
  });

  test('TopBar logo hover state screenshot', async ({ page }) => {
    const logo = page.locator('.navbar .gradient-premium');
    await logo.hover();
    await page.waitForTimeout(300);
    await expect(logo).toHaveScreenshot('topbar-logo-hover.png', screenshotConfig);
  });

  test('TopBar settings button screenshot', async ({ page }) => {
    const settingsButton = page.locator('.navbar button[aria-label="Settings"]');
    await expect(settingsButton).toHaveScreenshot('topbar-settings-button.png', screenshotConfig);
  });

  test('TopBar settings button focus state screenshot', async ({ page }) => {
    const settingsButton = page.locator('.navbar button[aria-label="Settings"]');
    await settingsButton.focus();
    await page.waitForTimeout(200);
    await expect(settingsButton).toHaveScreenshot('topbar-settings-button-focus.png', screenshotConfig);
  });

  test('TopBar settings button hover state screenshot', async ({ page }) => {
    const settingsButton = page.locator('.navbar button[aria-label="Settings"]');
    await settingsButton.hover();
    await page.waitForTimeout(300);
    await expect(settingsButton).toHaveScreenshot('topbar-settings-button-hover.png', screenshotConfig);
  });

  test('TopBar dark theme screenshot', async ({ page }) => {
    await setTheme(page, 'Dark');
    const topBar = page.locator('.navbar');
    await expect(topBar).toHaveScreenshot('topbar-dark-theme.png', screenshotConfig);
  });
});

test.describe('LibraryPanel Component Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('LibraryPanel component screenshot', async ({ page }) => {
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toHaveScreenshot('library-panel-component.png', screenshotConfig);
  });

  test('LibraryPanel toolbar screenshot', async ({ page }) => {
    const toolbar = page.locator('.library-panel > div:first-child');
    await expect(toolbar).toHaveScreenshot('library-panel-toolbar.png', screenshotConfig);
  });

  test('LibraryPanel search input screenshot', async ({ page }) => {
    const searchInput = page.locator('.library-panel input[type="text"]');
    await expect(searchInput).toHaveScreenshot('library-panel-search-input.png', screenshotConfig);
  });

  test('LibraryPanel search input focus state screenshot', async ({ page }) => {
    const searchInput = page.locator('.library-panel input[type="text"]');
    await searchInput.focus();
    await page.waitForTimeout(200);
    await expect(searchInput).toHaveScreenshot('library-panel-search-input-focus.png', screenshotConfig);
  });

  test('LibraryPanel sort select screenshot', async ({ page }) => {
    const sortSelect = page.locator('.library-panel select');
    await expect(sortSelect).toHaveScreenshot('library-panel-sort-select.png', screenshotConfig);
  });

  test('LibraryPanel empty state screenshot', async ({ page }) => {
    const emptyState = page.locator('#audiobook-list');
    await expect(emptyState).toHaveScreenshot('library-panel-empty-state.png', screenshotConfig);
  });

  test('LibraryPanel dark theme screenshot', async ({ page }) => {
    await setTheme(page, 'Dark');
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toHaveScreenshot('library-panel-dark-theme.png', screenshotConfig);
  });
});

test.describe('DetailsPanel Component Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('DetailsPanel component screenshot', async ({ page }) => {
    const detailsPanel = page.locator('[aria-label="File and bookmark details"]');
    await expect(detailsPanel).toHaveScreenshot('details-panel-component.png', screenshotConfig);
  });

  test('DetailsPanel files section screenshot', async ({ page }) => {
    const filesSection = page.locator('#files-list');
    await expect(filesSection).toHaveScreenshot('details-panel-files-section.png', screenshotConfig);
  });

  test('DetailsPanel files header screenshot', async ({ page }) => {
    const filesHeader = page.locator('[aria-label="File and bookmark details"] > div:first-child > div:first-child');
    await expect(filesHeader).toHaveScreenshot('details-panel-files-header.png', screenshotConfig);
  });

  test('DetailsPanel bookmarks section screenshot', async ({ page }) => {
    const bookmarksSection = page.locator('#bookmarks-list');
    await expect(bookmarksSection).toHaveScreenshot('details-panel-bookmarks-section.png', screenshotConfig);
  });

  test('DetailsPanel bookmarks header screenshot', async ({ page }) => {
    const bookmarksHeader = page.locator('[aria-label="File and bookmark details"] > div:last-child > div:first-child');
    await expect(bookmarksHeader).toHaveScreenshot('details-panel-bookmarks-header.png', screenshotConfig);
  });

  test('DetailsPanel dark theme screenshot', async ({ page }) => {
    await setTheme(page, 'Dark');
    const detailsPanel = page.locator('[aria-label="File and bookmark details"]');
    await expect(detailsPanel).toHaveScreenshot('details-panel-dark-theme.png', screenshotConfig);
  });
});

test.describe('PlayerControls Component Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('PlayerControls component screenshot', async ({ page }) => {
    const playerControls = page.locator('[aria-label="Audio player controls"]');
    await expect(playerControls).toHaveScreenshot('player-controls-component.png', screenshotConfig);
  });

  test('PlayerControls play button screenshot', async ({ page }) => {
    const playButton = page.locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]').first();
    await expect(playButton).toHaveScreenshot('player-controls-play-button.png', screenshotConfig);
  });

  test('PlayerControls play button focus state screenshot', async ({ page }) => {
    const playButton = page.locator('.btm-nav button[aria-label="Play"], .btm-nav button[aria-label="Pause"]').first();
    await playButton.focus();
    await page.waitForTimeout(200);
    await expect(playButton).toHaveScreenshot('player-controls-play-button-focus.png', screenshotConfig);
  });

  test('PlayerControls stop button screenshot', async ({ page }) => {
    const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
    await expect(stopButton).toHaveScreenshot('player-controls-stop-button.png', screenshotConfig);
  });

  test('PlayerControls stop button focus state screenshot', async ({ page }) => {
    const stopButton = page.locator('.btm-nav button[aria-label="Stop"]');
    await stopButton.focus();
    await page.waitForTimeout(200);
    await expect(stopButton).toHaveScreenshot('player-controls-stop-button-focus.png', screenshotConfig);
  });

  test('PlayerControls volume button screenshot', async ({ page }) => {
    const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]').first();
    await expect(volumeButton).toHaveScreenshot('player-controls-volume-button.png', screenshotConfig);
  });

  test('PlayerControls volume button focus state screenshot', async ({ page }) => {
    const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]').first();
    await volumeButton.focus();
    await page.waitForTimeout(200);
    await expect(volumeButton).toHaveScreenshot('player-controls-volume-button-focus.png', screenshotConfig);
  });

  test('PlayerControls speed button screenshot', async ({ page }) => {
    const speedButton = page.locator('.btm-nav button:has-text("x")').first();
    await expect(speedButton).toHaveScreenshot('player-controls-speed-button.png', screenshotConfig);
  });

  test('PlayerControls seek bar screenshot', async ({ page }) => {
    const seekBar = page.locator('.btm-nav input[aria-label="Seek"]');
    await expect(seekBar).toHaveScreenshot('player-controls-seek-bar.png', screenshotConfig);
  });

  test('PlayerControls seek bar container hover state screenshot', async ({ page }) => {
    const seekBarContainer = page.locator('.btm-nav .flex-1.relative.group');
    await seekBarContainer.hover();
    await page.waitForTimeout(200);
    await expect(seekBarContainer).toHaveScreenshot('player-controls-seek-bar-hover.png', screenshotConfig);
  });

  test('PlayerControls dark theme screenshot', async ({ page }) => {
    await setTheme(page, 'Dark');
    const playerControls = page.locator('[aria-label="Audio player controls"]');
    await expect(playerControls).toHaveScreenshot('player-controls-dark-theme.png', screenshotConfig);
  });
});

test.describe('Component Screenshot Tests - Light Theme', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
    await setTheme(page, 'Light');
  });

  test('All components light theme - TopBar', async ({ page }) => {
    const topBar = page.locator('.navbar');
    await expect(topBar).toHaveScreenshot('topbar-light-theme.png', screenshotConfig);
  });

  test('All components light theme - LibraryPanel', async ({ page }) => {
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toHaveScreenshot('library-panel-light-theme.png', screenshotConfig);
  });

  test('All components light theme - DetailsPanel', async ({ page }) => {
    const detailsPanel = page.locator('[aria-label="File and bookmark details"]');
    await expect(detailsPanel).toHaveScreenshot('details-panel-light-theme.png', screenshotConfig);
  });

  test('All components light theme - PlayerControls', async ({ page }) => {
    const playerControls = page.locator('[aria-label="Audio player controls"]');
    await expect(playerControls).toHaveScreenshot('player-controls-light-theme.png', screenshotConfig);
  });
});

test.describe('Component Screenshot Tests - Responsive', () => {
  test.use({ viewport: viewports.tablet });

  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('Tablet - TopBar component screenshot', async ({ page }) => {
    const topBar = page.locator('.navbar');
    await expect(topBar).toHaveScreenshot('topbar-tablet.png', screenshotConfig);
  });

  test('Tablet - LibraryPanel component screenshot', async ({ page }) => {
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toHaveScreenshot('library-panel-tablet.png', screenshotConfig);
  });

  test('Tablet - DetailsPanel component screenshot', async ({ page }) => {
    const detailsPanel = page.locator('[aria-label="File and bookmark details"]');
    await expect(detailsPanel).toHaveScreenshot('details-panel-tablet.png', screenshotConfig);
  });

  test('Tablet - PlayerControls component screenshot', async ({ page }) => {
    const playerControls = page.locator('[aria-label="Audio player controls"]');
    await expect(playerControls).toHaveScreenshot('player-controls-tablet.png', screenshotConfig);
  });
});

test.describe('Component Screenshot Tests - Mobile', () => {
  test.use({ viewport: viewports.mobile });

  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('Mobile - TopBar component screenshot', async ({ page }) => {
    const topBar = page.locator('.navbar');
    await expect(topBar).toHaveScreenshot('topbar-mobile.png', screenshotConfig);
  });

  test('Mobile - LibraryPanel component screenshot', async ({ page }) => {
    const libraryPanel = page.locator('.library-panel');
    await expect(libraryPanel).toHaveScreenshot('library-panel-mobile.png', screenshotConfig);
  });

  test('Mobile - DetailsPanel component screenshot', async ({ page }) => {
    const detailsPanel = page.locator('[aria-label="File and bookmark details"]');
    await expect(detailsPanel).toHaveScreenshot('details-panel-mobile.png', screenshotConfig);
  });

  test('Mobile - PlayerControls component screenshot', async ({ page }) => {
    const playerControls = page.locator('[aria-label="Audio player controls"]');
    await expect(playerControls).toHaveScreenshot('player-controls-mobile.png', screenshotConfig);
  });
});

test.describe('VolumeSlider Component Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('VolumeSlider component screenshot', async ({ page }) => {
    const volumeSlider = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]').first();
    await expect(volumeSlider).toHaveScreenshot('volume-slider-component.png', screenshotConfig);
  });

  test('VolumeSlider muted state screenshot', async ({ page }) => {
    // Find the mute/unmute button
    const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]').first();
    // Get initial state
    const initialLabel = await volumeButton.getAttribute('aria-label');
    // If currently unmuted, click to mute
    if (initialLabel === 'Mute') {
      await volumeButton.click();
      await page.waitForTimeout(200);
    }
    // Now capture the muted state
    const mutedButton = page.locator('.btm-nav button[aria-label="Unmute"]').first();
    await expect(mutedButton).toHaveScreenshot('volume-slider-muted.png', screenshotConfig);
  });

  test('VolumeSlider container hover state screenshot', async ({ page }) => {
    const volumeContainer = page.locator('.btm-nav .group\\/volume').first();
    await volumeContainer.hover();
    await page.waitForTimeout(300);
    await expect(volumeContainer).toHaveScreenshot('volume-slider-hover.png', screenshotConfig);
  });

  test('VolumeSlider button focus state screenshot', async ({ page }) => {
    const volumeButton = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]').first();
    await volumeButton.focus();
    await page.waitForTimeout(200);
    await expect(volumeButton).toHaveScreenshot('volume-slider-focus.png', screenshotConfig);
  });

  test('VolumeSlider dark theme screenshot', async ({ page }) => {
    await setTheme(page, 'Dark');
    const volumeSlider = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]').first();
    await expect(volumeSlider).toHaveScreenshot('volume-slider-dark-theme.png', screenshotConfig);
  });

  test('VolumeSlider light theme screenshot', async ({ page }) => {
    await setTheme(page, 'Light');
    const volumeSlider = page.locator('.btm-nav button[aria-label="Mute"], .btm-nav button[aria-label="Unmute"]').first();
    await expect(volumeSlider).toHaveScreenshot('volume-slider-light-theme.png', screenshotConfig);
  });
});

test.describe('Full Layout Component Screenshots', () => {
  test.beforeEach(async ({ page }) => {
    await gotoAndWait(page);
  });

  test('Full layout with all components screenshot', async ({ page }) => {
    await expect(page).toHaveScreenshot('full-layout-components.png', fullPageScreenshotConfig);
  });

  test('Full layout dark theme with all components screenshot', async ({ page }) => {
    await setTheme(page, 'Dark');
    await expect(page).toHaveScreenshot('full-layout-components-dark.png', fullPageScreenshotConfig);
  });

  test('Full layout light theme with all components screenshot', async ({ page }) => {
    await setTheme(page, 'Light');
    await expect(page).toHaveScreenshot('full-layout-components-light.png', fullPageScreenshotConfig);
  });
});
