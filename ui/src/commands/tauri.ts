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


// Database commands
export async function getAllAudiobooks(): Promise<Audiobook[]> {
  return await invoke('get_all_audiobooks');
}

export async function getAudiobookById(id: number): Promise<Audiobook | null> {
  return await invoke('get_audiobook_by_id', { id });
}

export async function getAudiobookFiles(audiobookId: number): Promise<AudiobookFile[]> {
  return await invoke('get_audiobook_files', { audiobookId });
}

export async function getBookmarks(audiobookId: number): Promise<Bookmark[]> {
  return await invoke('get_bookmarks', { audiobookId });
}

export async function getDirectories(): Promise<Directory[]> {
  return await invoke('get_directories');
}

export async function insertDirectory(path: string): Promise<void> {
  return await invoke('insert_directory', { path });
}

export async function removeDirectory(path: string): Promise<void> {
  return await invoke('remove_directory', { path });
}

export async function scanDirectory(path: string): Promise<void> {
  return await invoke('scan_directory', { path });
}

// Player commands
export async function playPause(): Promise<boolean> {
  return await invoke('play_pause');
}

export async function stop(): Promise<void> {
  return await invoke('stop');
}

export async function seek(positionMs: number): Promise<void> {
  return await invoke('seek', { positionMs });
}

export async function setVolume(volume: number): Promise<void> {
  return await invoke('set_volume', { volume });
}

export async function setSpeed(speed: number): Promise<void> {
  return await invoke('set_speed', { speed });
}

export async function loadFile(filePath: string): Promise<void> {
  return await invoke('load_file', { filePath });
}

export async function getCurrentTime(): Promise<number> {
  return await invoke('get_current_time');
}

export async function getDuration(): Promise<number> {
  return await invoke('get_duration');
}

export async function getPlaybackState(): Promise<string> {
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
  return await invoke('create_bookmark', {
    audiobookId,
    filePath,
    positionMs,
    label,
    note,
  });
}

export async function updateBookmark(bookmark: Bookmark): Promise<void> {
  return await invoke('update_bookmark', { bookmark });
}

export async function deleteBookmark(id: number): Promise<void> {
  return await invoke('delete_bookmark', { id });
}

export async function jumpToBookmark(id: number): Promise<void> {
  return await invoke('jump_to_bookmark', { id });
}

// Settings commands
export async function getTheme(): Promise<string> {
  return await invoke('get_theme');
}

export async function setTheme(theme: string): Promise<void> {
  return await invoke('set_theme', { theme });
}

export async function getPanelRatio(): Promise<number> {
  return await invoke('get_panel_ratio');
}

export async function setPanelRatio(ratio: number): Promise<void> {
  return await invoke('set_panel_ratio', { ratio });
}

export async function getFocusMode(): Promise<boolean> {
  return await invoke('get_focus_mode');
}

export async function setFocusMode(enabled: boolean): Promise<void> {
  return await invoke('set_focus_mode', { enabled });
}

export async function getVolume(): Promise<number> {
  return await invoke('get_volume');
}

export async function setVolumePreference(volume: number): Promise<void> {
  return await invoke('set_volume_preference', { volume });
}

export async function getSpeed(): Promise<number> {
  return await invoke('get_speed');
}

export async function setSpeedPreference(speed: number): Promise<void> {
  return await invoke('set_speed_preference', { speed });
}

export async function getPrefersReducedMotion(): Promise<boolean> {
  return await invoke('get_prefers_reduced_motion');
}

export async function setPrefersReducedMotion(prefers: boolean): Promise<void> {
  return await invoke('set_prefers_reduced_motion', { prefers });
}

export async function getSkipForwardDuration(): Promise<number> {
  return await invoke('get_skip_forward_duration');
}

export async function setSkipForwardDuration(seconds: number): Promise<void> {
  return await invoke('set_skip_forward_duration', { seconds });
}

export async function getSkipBackwardDuration(): Promise<number> {
  return await invoke('get_skip_backward_duration');
}

export async function setSkipBackwardDuration(seconds: number): Promise<void> {
  return await invoke('set_skip_backward_duration', { seconds });
}

export async function getAutoSaveInterval(): Promise<number> {
  return await invoke('get_auto_save_interval');
}

export async function setAutoSaveInterval(interval: number): Promise<void> {
  return await invoke('set_auto_save_interval', { interval });
}

// Directory validation result
export interface DirectoryValidation {
  exists: boolean;
  readable: boolean;
  error: string | null;
}

// Validate directory accessibility
export async function validateDirectory(path: string): Promise<DirectoryValidation> {
  return await invoke('validate_directory', { path });
}

// Update directory path (for relocated folders)
export async function updateDirectoryPath(oldPath: string, newPath: string): Promise<void> {
  return await invoke('update_directory_path', { oldPath, newPath });
}
