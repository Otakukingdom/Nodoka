/**
 * Tauri API mock for standalone Playwright testing
 *
 * This mock intercepts all Tauri invoke calls and returns realistic test data,
 * allowing Playwright tests to run in CI without the Rust backend.
 */

import { vi } from 'vitest';
import type { Audiobook, AudiobookFile, Bookmark, Directory } from '../store/types';

// Mock data matching the Rust backend types
const mockAudiobooks: Audiobook[] = [
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

const mockFiles: AudiobookFile[] = [
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

const mockBookmarks: Bookmark[] = [
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

const mockDirectories: Directory[] = [
  {
    fullPath: '/test/audiobooks',
    createdAt: new Date(Date.now() - 86400000 * 30).toISOString(),
    lastScanned: new Date(Date.now() - 3600000).toISOString(),
  },
];

// Player state mock
let mockPlaybackState = 'Stopped';
let mockCurrentTime = 0;
let mockVolume = 80;
let mockSpeed = 1.0;

// Create mock invoke function
export function createMockInvoke() {
  return vi.fn((cmd: string, args?: Record<string, unknown>) => {
    switch (cmd) {
      // Database commands
      case 'get_all_audiobooks':
        return mockAudiobooks;
      case 'get_audiobook_by_id':
        return mockAudiobooks.find((a) => a.id === args?.['id']) ?? null;
      case 'get_audiobook_files':
        return mockFiles.filter((f) => f.audiobookId === args?.['audiobookId']);
      case 'get_bookmarks':
        return mockBookmarks.filter((b) => b.audiobookId === args?.['audiobookId']);
      case 'get_directories':
        return mockDirectories;
      case 'insert_directory':
        return undefined;
      case 'remove_directory':
        return undefined;
      case 'scan_directory':
        return undefined;

      // Player commands
      case 'play_pause':
        mockPlaybackState = mockPlaybackState === 'Playing' ? 'Paused' : 'Playing';
        return mockPlaybackState === 'Playing';
      case 'stop':
        mockPlaybackState = 'Stopped';
        mockCurrentTime = 0;
        return undefined;
      case 'seek':
        mockCurrentTime = args?.['positionMs'] as number;
        return undefined;
      case 'set_volume':
        mockVolume = args?.['volume'] as number;
        return undefined;
      case 'set_speed':
        mockSpeed = args?.['speed'] as number;
        return undefined;
      case 'load_file':
        mockPlaybackState = 'Paused';
        return undefined;
      case 'get_current_time':
        return mockCurrentTime;
      case 'get_duration':
        return 1800000;
      case 'get_playback_state':
        return mockPlaybackState;

      // Bookmark commands
      case 'create_bookmark':
        return Math.floor(Math.random() * 1000) + 10;
      case 'update_bookmark':
        return undefined;
      case 'delete_bookmark':
        return undefined;
      case 'jump_to_bookmark':
        return undefined;

      // Settings commands
      case 'get_theme':
        return 'Light';
      case 'set_theme':
        return undefined;
      case 'get_panel_ratio':
        return 0.4;
      case 'set_panel_ratio':
        return undefined;
      case 'get_focus_mode':
        return false;
      case 'set_focus_mode':
        return undefined;
      case 'get_volume':
        return mockVolume;
      case 'set_volume_preference':
        return undefined;
      case 'get_speed':
        return mockSpeed;
      case 'set_speed_preference':
        return undefined;
      case 'get_prefers_reduced_motion':
        return false;
      case 'set_prefers_reduced_motion':
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

      default:
        console.warn(`Unknown Tauri command: ${cmd}`);
        return null;
    }
  });
}

// Setup function to mock the Tauri API
export function setupTauriMock() {
  const mockInvoke = createMockInvoke();

  // Mock @tauri-apps/api/core
  vi.mock('@tauri-apps/api/core', () => ({
    invoke: mockInvoke,
  }));

  return mockInvoke;
}

// Export mock data for test customization
export const mockData = {
  audiobooks: mockAudiobooks,
  files: mockFiles,
  bookmarks: mockBookmarks,
  directories: mockDirectories,
};

// Reset function to restore initial state
export function resetMockState() {
  mockPlaybackState = 'Stopped';
  mockCurrentTime = 0;
  mockVolume = 80;
  mockSpeed = 1.0;
}
