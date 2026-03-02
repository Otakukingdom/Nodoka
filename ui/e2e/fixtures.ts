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
 * Mock directory data
 */
export const mockDirectories = [
  { fullPath: '/test/audiobooks', createdAt: new Date(Date.now() - 86400000 * 30).toISOString(), lastScanned: null },
  { fullPath: '/test/audiobooks2', createdAt: new Date(Date.now() - 86400000 * 15).toISOString(), lastScanned: new Date().toISOString() },
];

/**
 * Mock user preferences
 */
export const mockPreferences = {
  language: 'en',
  region: 'US',
};

/**
 * Mock sleep timer state
 */
export const mockSleepTimer = {
  active: false,
  mode: null as 'Duration' | 'EndOfChapter' | null,
  remainingSeconds: null as number | null,
  fadeDuration: 0,
};

/**
 * Mock progress tracking data
 */
export const mockProgressData = {
  lastListened: new Date(Date.now() - 3600000).toISOString(),
  selectedFile: '/test/audiobooks/The Great Adventure/Chapter 1.mp3',
};

/**
 * Inject mock Tauri API into the page
 */
async function injectMockTauri(page: Page) {
  await page.addInitScript(
    ({ audiobooks, files, bookmarks, settings, directories, preferences, sleepTimer, progressData }) => {
      // Player state mock
      let playbackState = 'Stopped';
      let currentTime = 0;
      let volume = settings.volume;
      let speed = settings.speed;
      let currentAudiobookId: number | null = null;
      let currentFile: string | null = null;

      // Local state for testing
      let localBookmarks = [...bookmarks];
      let localAudiobooks = [...audiobooks];
      let localFiles = [...files];
      let localPreferences = { ...preferences };
      let localSleepTimer = { ...sleepTimer };
      let nextBookmarkId = 100;

      (window as any).__TAURI__ = {
        invoke: async (cmd: string, args?: Record<string, unknown>) => {
          switch (cmd) {
            // Database commands - Audiobooks
            case 'get_all_audiobooks':
              return localAudiobooks;
            case 'get_audiobook_by_id':
              return localAudiobooks.find((a: any) => a.id === args?.['id']) ?? null;
            case 'get_audiobook_by_path':
              return localAudiobooks.find((a: any) => a.fullPath === args?.['path']) ?? null;
            case 'get_audiobooks_by_directory':
              return localAudiobooks.filter((a: any) => a.directory === args?.['directory']);
            case 'insert_audiobook':
              const newAudiobook = {
                id: Math.floor(Math.random() * 1000) + 100,
                ...args,
                completeness: 0,
                createdAt: new Date().toISOString(),
              };
              localAudiobooks.push(newAudiobook);
              return newAudiobook.id;
            case 'delete_audiobook':
              localAudiobooks = localAudiobooks.filter((a: any) => a.id !== args?.['id']);
              localFiles = localFiles.filter((f: any) => f.audiobookId !== args?.['id']);
              localBookmarks = localBookmarks.filter((b: any) => b.audiobookId !== args?.['id']);
              return undefined;
            case 'update_audiobook_completeness':
              const ab = localAudiobooks.find((a: any) => a.id === args?.['id']);
              if (ab) ab.completeness = args?.['completeness'] as number;
              return undefined;
            case 'update_audiobook_author':
              const abAuthor = localAudiobooks.find((a: any) => a.id === args?.['id']);
              if (abAuthor) abAuthor.author = args?.['author'] as string;
              return undefined;
            case 'update_audiobook_last_listened':
              const abLast = localAudiobooks.find((a: any) => a.id === args?.['id']);
              if (abLast) abLast.lastListened = new Date().toISOString();
              return undefined;

            // Database commands - Files
            case 'get_audiobook_files':
              return localFiles.filter((f: any) => f.audiobookId === args?.['audiobookId']);
            case 'update_file_progress':
              const file = localFiles.find((f: any) => f.fullPath === args?.['filePath']);
              if (file) {
                file.seekPosition = args?.['seekPosition'] as number;
                file.completeness = args?.['completeness'] as number;
              }
              return undefined;
            case 'mark_file_exists':
              const fileExists = localFiles.find((f: any) => f.fullPath === args?.['filePath']);
              if (fileExists) fileExists.fileExists = args?.['exists'] as boolean;
              return undefined;
            case 'mark_audiobook_files_missing':
              localFiles
                .filter((f: any) => f.audiobookId === args?.['audiobookId'])
                .forEach((f: any) => (f.fileExists = false));
              return undefined;

            // Database commands - Bookmarks
            case 'get_bookmarks':
              return localBookmarks.filter((b: any) => b.audiobookId === args?.['audiobookId']);
            case 'create_bookmark':
              const newBookmark = {
                id: nextBookmarkId++,
                audiobookId: args?.['audiobookId'] as number,
                filePath: args?.['filePath'] as string,
                positionMs: args?.['positionMs'] as number,
                label: args?.['label'] as string,
                note: args?.['note'] as string | undefined,
                createdAt: new Date().toISOString(),
              };
              localBookmarks.push(newBookmark);
              return newBookmark.id;
            case 'update_bookmark':
              const bm = localBookmarks.find((b: any) => b.id === args?.['id']);
              if (bm) Object.assign(bm, args);
              return undefined;
            case 'delete_bookmark':
              localBookmarks = localBookmarks.filter((b: any) => b.id !== args?.['id']);
              return undefined;
            case 'jump_to_bookmark':
              const bookmark = localBookmarks.find((b: any) => b.id === args?.['id']);
              if (bookmark) {
                currentTime = bookmark.positionMs;
                currentFile = bookmark.filePath;
              }
              return undefined;

            // Database commands - Directories
            case 'get_directories':
              return directories;
            case 'insert_directory':
              return undefined;
            case 'remove_directory':
              return undefined;
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
              currentTime = Math.min(args?.['positionMs'] as number, 1800000);
              return undefined;
            case 'set_volume':
              volume = Math.max(0, Math.min(200, args?.['volume'] as number));
              return undefined;
            case 'set_speed':
              speed = Math.max(0.5, Math.min(2.0, args?.['speed'] as number));
              return undefined;
            case 'load_file':
              playbackState = 'Paused';
              currentFile = args?.['path'] as string;
              return undefined;
            case 'get_current_time':
              return currentTime;
            case 'get_duration':
              return 1800000;
            case 'get_playback_state':
              return playbackState;

            // Settings commands
            case 'get_theme':
              return settings.theme;
            case 'set_theme':
              settings.theme = args?.['theme'] as string;
              return undefined;
            case 'get_panel_ratio':
              return settings.panelRatio;
            case 'set_panel_ratio':
              settings.panelRatio = args?.['ratio'] as number;
              return undefined;
            case 'get_focus_mode':
              return settings.focusMode;
            case 'set_focus_mode':
              settings.focusMode = args?.['enabled'] as boolean;
              return undefined;
            case 'get_volume':
              return volume;
            case 'set_volume_preference':
              settings.volume = args?.['volume'] as number;
              return undefined;
            case 'get_speed':
              return speed;
            case 'set_speed_preference':
              settings.speed = args?.['speed'] as number;
              return undefined;
            case 'get_prefers_reduced_motion':
              return settings.prefersReducedMotion;
            case 'set_prefers_reduced_motion':
              settings.prefersReducedMotion = args?.['enabled'] as boolean;
              return undefined;
            case 'get_window_position':
              return null;
            case 'set_window_position':
              return undefined;
            case 'get_window_size':
              return null;
            case 'set_window_size':
              return undefined;
            case 'get_skip_forward_duration':
              return 30;
            case 'set_skip_forward_duration':
              return undefined;
            case 'get_skip_backward_duration':
              return 15;
            case 'set_skip_backward_duration':
              return undefined;
            case 'get_auto_save_interval':
              return 5;
            case 'set_auto_save_interval':
              if ((args?.['interval'] as number) < 1 || (args?.['interval'] as number) > 60) {
                throw new Error('Invalid auto-save interval');
              }
              return undefined;
            case 'get_current_audiobook':
              return currentAudiobookId;
            case 'set_current_audiobook':
              currentAudiobookId = args?.['id'] as number;
              return undefined;
            case 'get_current_file':
              return currentFile;
            case 'set_current_file':
              currentFile = args?.['path'] as string;
              return undefined;
            case 'reset_settings_to_defaults':
              settings.volume = 100;
              settings.speed = 1.0;
              settings.theme = 'System';
              settings.focusMode = false;
              settings.prefersReducedMotion = false;
              return undefined;

            // User preferences
            case 'get_preference':
              return localPreferences[args?.['key'] as string] ?? null;
            case 'set_preference':
              if ((args?.['key'] as string).includes('\0') || (args?.['value'] as string)?.includes('\0')) {
                throw new Error('NUL bytes not allowed');
              }
              localPreferences[args?.['key'] as string] = args?.['value'];
              return undefined;
            case 'get_all_preferences':
              return Object.entries(localPreferences).map(([key, value]) => ({ key, value }));
            case 'delete_preference':
              delete localPreferences[args?.['key'] as string];
              return undefined;

            // Sleep timer commands
            case 'set_sleep_timer':
              localSleepTimer.active = true;
              localSleepTimer.mode = args?.['mode'] as 'Duration' | 'EndOfChapter';
              localSleepTimer.remainingSeconds = args?.['durationSeconds'] as number ?? null;
              localSleepTimer.fadeDuration = args?.['fadeDuration'] as number ?? 0;
              return undefined;
            case 'cancel_sleep_timer':
              localSleepTimer.active = false;
              localSleepTimer.mode = null;
              localSleepTimer.remainingSeconds = null;
              return undefined;
            case 'get_sleep_timer':
              return localSleepTimer.active ? localSleepTimer : null;

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
      directories: mockDirectories,
      preferences: mockPreferences,
      sleepTimer: mockSleepTimer,
      progressData: mockProgressData,
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

/**
 * Edge case mock data for testing empty states
 */
export const mockEmptyLibrary: typeof mockAudiobooks = [];

/**
 * Mock audiobooks without author
 */
export const mockAudiobooksWithoutAuthor = [
  { ...mockAudiobooks[2], author: null },
];

/**
 * Mock files with missing file indicator
 */
export const mockFilesWithMissing = [
  ...mockFiles,
  {
    audiobookId: 1,
    name: 'Chapter 4 - Missing File',
    fullPath: '/test/audiobooks/The Great Adventure/Chapter 4.mp3',
    lengthOfFile: 1800000,
    seekPosition: 0,
    position: 3,
    completeness: 0,
    fileExists: false,
    createdAt: new Date().toISOString(),
  },
];

/**
 * Mock audiobooks without cover art
 */
export const mockAudiobooksNoCover = [
  { ...mockAudiobooks[2], hasCoverArt: false },
];

/**
 * Helper to create a test with custom mock data scenario
 */
export function createTestWithScenario(
  scenario: 'default' | 'empty' | 'missing-files' | 'no-covers'
) {
  return base.extend({
    page: async ({ page }, use) => {
      const scenarioData = {
        default: {
          audiobooks: mockAudiobooks,
          files: mockFiles,
        },
        empty: {
          audiobooks: mockEmptyLibrary,
          files: [],
        },
        'missing-files': {
          audiobooks: mockAudiobooks,
          files: mockFilesWithMissing,
        },
        'no-covers': {
          audiobooks: mockAudiobooksNoCover,
          files: mockFiles,
        },
      };

      const data = scenarioData[scenario];

      await page.addInitScript(
        ({ audiobooks, files, bookmarks, settings, directories, preferences, sleepTimer, progressData }) => {
          // Player state mock
          let playbackState = 'Stopped';
          let currentTime = 0;
          let volume = settings.volume;
          let speed = settings.speed;
          let currentAudiobookId: number | null = null;
          let currentFile: string | null = null;

          // Local state
          let localBookmarks = [...bookmarks];
          let localAudiobooks = [...audiobooks];
          let localFiles = [...files];
          let localPreferences = { ...preferences };
          let localSleepTimer = { ...sleepTimer };
          let nextBookmarkId = 100;

          (window as any).__TAURI__ = {
            invoke: async (cmd: string, args?: Record<string, unknown>) => {
              switch (cmd) {
                case 'get_all_audiobooks':
                  return localAudiobooks;
                case 'get_audiobook_files':
                  return localFiles.filter((f: any) => f.audiobookId === args?.['audiobookId']);
                case 'get_bookmarks':
                  return localBookmarks.filter((b: any) => b.audiobookId === args?.['audiobookId']);
                case 'create_bookmark':
                  const newBookmark = {
                    id: nextBookmarkId++,
                    audiobookId: args?.['audiobookId'] as number,
                    filePath: args?.['filePath'] as string,
                    positionMs: args?.['positionMs'] as number,
                    label: args?.['label'] as string,
                    note: args?.['note'] as string | undefined,
                    createdAt: new Date().toISOString(),
                  };
                  localBookmarks.push(newBookmark);
                  return newBookmark.id;
                case 'get_theme':
                  return settings.theme;
                case 'set_theme':
                  settings.theme = args?.['theme'] as string;
                  return undefined;
                case 'play_pause':
                  playbackState = playbackState === 'Playing' ? 'Paused' : 'Playing';
                  return playbackState === 'Playing';
                case 'stop':
                  playbackState = 'Stopped';
                  currentTime = 0;
                  return undefined;
                case 'get_playback_state':
                  return playbackState;
                default:
                  return null;
              }
            },
          };
        },
        {
          audiobooks: data.audiobooks,
          files: data.files,
          bookmarks: mockBookmarks,
          settings: mockSettings,
          directories: mockDirectories,
          preferences: mockPreferences,
          sleepTimer: mockSleepTimer,
          progressData: mockProgressData,
        }
      );
      await use(page);
    },
  });
}
