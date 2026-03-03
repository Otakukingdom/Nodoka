/**
 * Tauri command wrappers for IPC calls to Rust backend
 */

import { invoke } from '@tauri-apps/api/core';
import type {
  Audiobook,
  AudiobookFile,
  Bookmark,
  Directory,
} from '../store/types';

// Check if running in Playwright test mode (web-only without Tauri)
const isPlaywrightTest = import.meta.env.VITE_PLAYWRIGHT_TEST === 'true' ||
  (typeof window !== 'undefined' && !window.__TAURI_INTERNALS__);

// Mock data for Playwright tests
const mockAudiobooks: Audiobook[] = [
  { id: 1, directory: '/path/to', name: 'Sample Audiobook 1', author: 'Author One', completeness: 50, fullPath: '/path/to/book1', defaultOrder: 0, createdAt: '2024-01-01T00:00:00Z' },
  { id: 2, directory: '/path/to', name: 'Sample Audiobook 2', author: 'Author Two', completeness: 100, fullPath: '/path/to/book2', defaultOrder: 1, createdAt: '2024-01-02T00:00:00Z' },
  { id: 3, directory: '/path/to', name: 'Sample Audiobook 3', author: 'Author Three', completeness: 0, fullPath: '/path/to/book3', defaultOrder: 2, createdAt: '2024-01-03T00:00:00Z' },
];

const mockFiles: AudiobookFile[] = [
  { audiobookId: 1, name: 'Chapter 1.mp3', fullPath: '/path/to/book1/ch1.mp3', position: 0, completeness: 100, fileExists: true, createdAt: '2024-01-01T00:00:00Z' },
  { audiobookId: 1, name: 'Chapter 2.mp3', fullPath: '/path/to/book1/ch2.mp3', position: 1, completeness: 50, fileExists: true, createdAt: '2024-01-01T00:00:00Z' },
  { audiobookId: 1, name: 'Chapter 3.mp3', fullPath: '/path/to/book1/ch3.mp3', position: 2, completeness: 0, fileExists: true, createdAt: '2024-01-01T00:00:00Z' },
];

const mockBookmarks: Bookmark[] = [
  { id: 1, audiobookId: 1, filePath: '/path/to/book1/ch1.mp3', positionMs: 30000, label: 'Interesting part', createdAt: '2024-01-15T10:30:00Z' },
];

const mockDirectories: Directory[] = [
  { fullPath: '/Users/test/audiobooks', createdAt: '2024-01-01T00:00:00Z' },
];

// Database commands
export async function getAllAudiobooks(): Promise<Audiobook[]> {
  if (isPlaywrightTest) return mockAudiobooks;
  return await invoke('get_all_audiobooks');
}

export async function getAudiobookById(id: number): Promise<Audiobook | null> {
  if (isPlaywrightTest) return mockAudiobooks.find(b => b.id === id) ?? null;
  return await invoke('get_audiobook_by_id', { id });
}

export async function getAudiobookFiles(audiobookId: number): Promise<AudiobookFile[]> {
  if (isPlaywrightTest) return mockFiles;
  return await invoke('get_audiobook_files', { audiobookId });
}

export async function getBookmarks(audiobookId: number): Promise<Bookmark[]> {
  if (isPlaywrightTest) return mockBookmarks;
  return await invoke('get_bookmarks', { audiobookId });
}

export async function getDirectories(): Promise<Directory[]> {
  if (isPlaywrightTest) return mockDirectories;
  return await invoke('get_directories');
}

export async function insertDirectory(path: string): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('insert_directory', { path });
}

export async function removeDirectory(path: string): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('remove_directory', { path });
}

export async function scanDirectory(path: string): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('scan_directory', { path });
}

// Player commands
export async function playPause(): Promise<boolean> {
  if (isPlaywrightTest) return true;
  return await invoke('play_pause');
}

export async function stop(): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('stop');
}

export async function seek(positionMs: number): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('seek', { positionMs });
}

export async function setVolume(volume: number): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('set_volume', { volume });
}

export async function setSpeed(speed: number): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('set_speed', { speed });
}

export async function loadFile(filePath: string): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('load_file', { filePath });
}

export async function getCurrentTime(): Promise<number> {
  if (isPlaywrightTest) return 30000;
  return await invoke('get_current_time');
}

export async function getDuration(): Promise<number> {
  if (isPlaywrightTest) return 3600000;
  return await invoke('get_duration');
}

export async function getPlaybackState(): Promise<string> {
  if (isPlaywrightTest) return 'Paused';
  return await invoke('get_playback_state');
}

// Bookmark commands
export async function createBookmark(
  audiobookId: number,
  filePath: string,
  positionMs: number,
  label: string,
  note?: string
): Promise<number> {
  if (isPlaywrightTest) return Date.now();
  return await invoke('create_bookmark', {
    audiobookId,
    filePath,
    positionMs,
    label,
    note,
  });
}

export async function updateBookmark(bookmark: Bookmark): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('update_bookmark', { bookmark });
}

export async function deleteBookmark(id: number): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('delete_bookmark', { id });
}

export async function jumpToBookmark(id: number): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('jump_to_bookmark', { id });
}

// Settings commands
export async function getTheme(): Promise<string> {
  if (isPlaywrightTest) return 'Light';
  return await invoke('get_theme');
}

export async function setTheme(theme: string): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('set_theme', { theme });
}

export async function getPanelRatio(): Promise<number> {
  if (isPlaywrightTest) return 0.4;
  return await invoke('get_panel_ratio');
}

export async function setPanelRatio(ratio: number): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('set_panel_ratio', { ratio });
}

export async function getFocusMode(): Promise<boolean> {
  if (isPlaywrightTest) return false;
  return await invoke('get_focus_mode');
}

export async function setFocusMode(enabled: boolean): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('set_focus_mode', { enabled });
}

export async function getVolume(): Promise<number> {
  if (isPlaywrightTest) return 100;
  return await invoke('get_volume');
}

export async function setVolumePreference(volume: number): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('set_volume_preference', { volume });
}

export async function getSpeed(): Promise<number> {
  if (isPlaywrightTest) return 1.0;
  return await invoke('get_speed');
}

export async function setSpeedPreference(speed: number): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('set_speed_preference', { speed });
}

export async function getPrefersReducedMotion(): Promise<boolean> {
  if (isPlaywrightTest) return false;
  return await invoke('get_prefers_reduced_motion');
}

export async function setPrefersReducedMotion(prefers: boolean): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('set_prefers_reduced_motion', { prefers });
}

// Directory validation result
export interface DirectoryValidation {
  exists: boolean;
  readable: boolean;
  error: string | null;
}

// Validate directory accessibility
export async function validateDirectory(path: string): Promise<DirectoryValidation> {
  if (isPlaywrightTest) {
    return { exists: true, readable: true, error: null };
  }
  return await invoke('validate_directory', { path });
}

// Update directory path (for relocated folders)
export async function updateDirectoryPath(oldPath: string, newPath: string): Promise<void> {
  if (isPlaywrightTest) return;
  return await invoke('update_directory_path', { oldPath, newPath });
}
