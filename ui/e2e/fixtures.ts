/**
 * Custom Playwright fixtures for e2e testing
 * Provides mock Tauri API setup for testing without the Rust backend
 */

import { test as base, expect, Page } from '@playwright/test';

// Re-export expect for convenience
export { expect };

/**
 * Mock audiobook data matching Rust backend types
 */
export const mockAudiobooks = [
  {
    id: 1,
    directory: '/test/audiobooks',
    name: 'The Great Adventure',
    author: 'Jane Author',
    fullPath: '/test/audiobooks/The Great Adventure',
    completeness: 45,
    defaultOrder: 0,
    totalDurationMs: 3600000,
    remainingDurationMs: 1980000,
    createdAt: new Date(Date.now() - 86400000 * 7).toISOString(),
    lastListened: new Date(Date.now() - 3600000).toISOString(),
  },
  {
    id: 2,
    directory: '/test/audiobooks',
    name: 'Science Fiction Stories',
    author: 'John Writer',
    fullPath: '/test/audiobooks/Science Fiction Stories',
    completeness: 100,
    defaultOrder: 1,
    totalDurationMs: 7200000,
    remainingDurationMs: 0,
    createdAt: new Date(Date.now() - 86400000 * 30).toISOString(),
    lastListened: new Date(Date.now() - 86400000).toISOString(),
  },
  {
    id: 3,
    directory: '/test/audiobooks',
    name: 'Mystery Novel',
    fullPath: '/test/audiobooks/Mystery Novel',
    completeness: 0,
    defaultOrder: 2,
    createdAt: new Date(Date.now() - 86400000 * 2).toISOString(),
  },
];

/**
 * Mock file data for audiobook files
 */
export const mockFiles = [
  {
    audiobookId: 1,
    name: 'Chapter 1 - The Beginning',
    fullPath: '/test/audiobooks/The Great Adventure/Chapter 1.mp3',
    lengthOfFile: 1800000,
    seekPosition: 900000,
    position: 0,
    completeness: 100,
    fileExists: true,
    createdAt: new Date(Date.now() - 86400000 * 7).toISOString(),
  },
  {
    audiobookId: 1,
    name: 'Chapter 2 - The Journey',
    fullPath: '/test/audiobooks/The Great Adventure/Chapter 2.mp3',
    lengthOfFile: 1800000,
    seekPosition: 0,
    position: 1,
    completeness: 0,
    fileExists: true,
    createdAt: new Date(Date.now() - 86400000 * 7).toISOString(),
  },
  {
    audiobookId: 1,
    name: 'Chapter 3 - The Discovery',
    fullPath: '/test/audiobooks/The Great Adventure/Chapter 3.mp3',
    lengthOfFile: 1800000,
    seekPosition: 0,
    position: 2,
    completeness: 0,
    fileExists: true,
    createdAt: new Date(Date.now() - 86400000 * 7).toISOString(),
  },
];

/**
 * Mock bookmark data
 */
export const mockBookmarks = [
  {
    id: 1,
    audiobookId: 1,
    filePath: '/test/audiobooks/The Great Adventure/Chapter 1.mp3',
    positionMs: 300000,
    label: 'Important scene',
    note: 'This is where the protagonist meets the mentor',
    createdAt: new Date(Date.now() - 86400000).toISOString(),
  },
  {
    id: 2,
    audiobookId: 1,
    filePath: '/test/audiobooks/The Great Adventure/Chapter 1.mp3',
    positionMs: 600000,
    label: 'Plot twist',
    createdAt: new Date(Date.now() - 3600000).toISOString(),
  },
];

/**
 * Mock settings data
 */
export const mockSettings = {
  theme: 'Light',
  panelRatio: 0.4,
  focusMode: false,
  volume: 80,
  speed: 1.0,
  prefersReducedMotion: false,
};

/**
 * Inject mock Tauri API into the page
 */
async function injectMockTauri(page: Page) {
  await page.addInitScript(
    ({ audiobooks, files, bookmarks, settings }) => {
      // Player state mock
      let playbackState = 'Stopped';
      let currentTime = 0;
      let volume = settings.volume;
      let speed = settings.speed;

      (window as any).__TAURI__ = {
        invoke: async (cmd: string, args?: Record<string, unknown>) => {
          switch (cmd) {
            // Database commands
            case 'get_all_audiobooks':
              return audiobooks;
            case 'get_audiobook_by_id':
              return audiobooks.find((a: any) => a.id === args?.['id']) ?? null;
            case 'get_audiobook_files':
              return files.filter((f: any) => f.audiobookId === args?.['audiobookId']);
            case 'get_bookmarks':
              return bookmarks.filter((b: any) => b.audiobookId === args?.['audiobookId']);
            case 'get_directories':
              return [{ fullPath: '/test/audiobooks', createdAt: new Date().toISOString() }];
            case 'insert_directory':
            case 'remove_directory':
            case 'scan_directory':
              return undefined;

            // Player commands
            case 'play_pause':
              playbackState = playbackState === 'Playing' ? 'Paused' : 'Playing';
              return playbackState === 'Playing';
            case 'stop':
              playbackState = 'Stopped';
              currentTime = 0;
              return undefined;
            case 'seek':
              currentTime = args?.['positionMs'] as number;
              return undefined;
            case 'set_volume':
              volume = args?.['volume'] as number;
              return undefined;
            case 'set_speed':
              speed = args?.['speed'] as number;
              return undefined;
            case 'load_file':
              playbackState = 'Paused';
              return undefined;
            case 'get_current_time':
              return currentTime;
            case 'get_duration':
              return 1800000;
            case 'get_playback_state':
              return playbackState;

            // Bookmark commands
            case 'create_bookmark':
              return Math.floor(Math.random() * 1000) + 10;
            case 'update_bookmark':
            case 'delete_bookmark':
            case 'jump_to_bookmark':
              return undefined;

            // Settings commands
            case 'get_theme':
              return settings.theme;
            case 'set_theme':
              settings.theme = args?.['theme'] as string;
              return undefined;
            case 'get_panel_ratio':
              return settings.panelRatio;
            case 'set_panel_ratio':
              return undefined;
            case 'get_focus_mode':
              return settings.focusMode;
            case 'set_focus_mode':
              return undefined;
            case 'get_volume':
              return volume;
            case 'set_volume_preference':
              return undefined;
            case 'get_speed':
              return speed;
            case 'set_speed_preference':
              return undefined;
            case 'get_prefers_reduced_motion':
              return settings.prefersReducedMotion;
            case 'set_prefers_reduced_motion':
              return undefined;

            default:
              console.warn(`Unknown Tauri command: ${cmd}`);
              return null;
          }
        },
      };
    },
    {
      audiobooks: mockAudiobooks,
      files: mockFiles,
      bookmarks: mockBookmarks,
      settings: mockSettings,
    }
  );
}

/**
 * Extended test fixture with app-specific setup
 */
export const test = base.extend({
  page: async ({ page }, use) => {
    await injectMockTauri(page);
    await use(page);
  },
});

/**
 * Viewport configurations for responsive testing
 */
export const viewports = {
  desktop: { width: 1280, height: 720 },
  tablet: { width: 1024, height: 768 },
  small: { width: 800, height: 600 },
  mobile: { width: 375, height: 667 },
} as const;
