/**
 * Type definitions for the Redux store - mirrors Rust backend types
 */

export interface Audiobook {
  id?: number;
  directory: string;
  name: string;
  author?: string;
  fullPath: string;
  completeness: number;
  defaultOrder: number;
  selectedFile?: string;
  totalDurationMs?: number;
  remainingDurationMs?: number;
  createdAt: string;
  lastListened?: string;
}

export interface AudiobookFile {
  audiobookId: number;
  name: string;
  fullPath: string;
  lengthOfFile?: number;
  seekPosition?: number;
  checksum?: string;
  position: number;
  completeness: number;
  fileExists: boolean;
  createdAt: string;
}

export interface Bookmark {
  id?: number;
  audiobookId: number;
  filePath: string;
  positionMs: number;
  label: string;
  note?: string;
  createdAt: string;
}

export interface Directory {
  fullPath: string;
  createdAt: string;
  lastScanned?: string;
}

export type PlaybackStatus = 'Playing' | 'Paused' | 'Stopped' | 'Ended';

export type ThemeMode = 'Light' | 'Dark' | 'System';

export type ActiveModal = 'None' | 'Settings' | 'BookmarkEditor' | 'KeyboardHelp' | 'SleepTimer';

export type AudiobookSortOrder = 'RecentlyPlayed' | 'Title' | 'Progress' | 'DateAdded';

export type AudiobookFilterStatus = 'All' | 'InProgress' | 'Complete' | 'NotStarted';

export type FocusMode = 'Enabled' | 'Disabled';

export type VolumeExpansion = 'Expanded' | 'Collapsed';

export type LoadState = 'Loading' | 'Ready';

export type NotificationKind = 'Success' | 'Info' | 'Warning' | 'Error';

export interface Notification {
  id: number;
  kind: NotificationKind;
  title?: string;
  message: string;
  actionLabel?: string;
  durationMs: number;
  createdAt: number;
}

export interface BookmarkEditor {
  id?: number;
  audiobookId: number;
  filePath: string;
  positionMs: number;
  label: string;
  note: string;
}
