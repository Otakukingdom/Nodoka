/**
 * Event bus for handling UI events (replaces iced Message system)
 * Uses vanilla TypeScript with a simple pub/sub pattern
 */

import type { AudiobookSortOrder, AudiobookFilterStatus, ThemeMode } from '../state/types';

export type EventType =
  // Player events
  | 'PlayPause'
  | 'Stop'
  | 'SeekTo'
  | 'VolumeChanged'
  | 'VolumeToggleExpanded'
  | 'SpeedChanged'
  | 'PlayerTimeUpdated'
  // Navigation events
  | 'AudiobookSelected'
  | 'FileSelected'
  | 'NextFile'
  | 'PreviousFile'
  | 'SeekForward'
  | 'SeekBackward'
  // Bookmark events
  | 'CreateBookmark'
  | 'BookmarkEdit'
  | 'BookmarkDelete'
  | 'BookmarkJump'
  | 'BookmarkSave'
  | 'BookmarkCancel'
  // Library events
  | 'DirectoryAdd'
  | 'DirectoryRemove'
  | 'DirectoryRescan'
  | 'SearchQueryChanged'
  | 'SortOrderChanged'
  | 'FilterStatusChanged'
  // UI events
  | 'OpenSettings'
  | 'CloseSettings'
  | 'ThemeChanged'
  | 'PanelRatioChanged'
  | 'ToggleFocusMode'
  | 'ShowKeyboardHelp'
  | 'HideKeyboardHelp'
  // Modal events
  | 'CloseModal';

export interface EventPayload {
  PlayPause: never;
  Stop: never;
  SeekTo: { positionMs: number };
  VolumeChanged: { volume: number };
  VolumeToggleExpanded: never;
  SpeedChanged: { speed: number };
  PlayerTimeUpdated: { timeMs: number };
  AudiobookSelected: { id: number };
  FileSelected: { path: string };
  NextFile: never;
  PreviousFile: never;
  SeekForward: { seconds: number };
  SeekBackward: { seconds: number };
  CreateBookmark: never;
  BookmarkEdit: { id: number };
  BookmarkDelete: { id: number };
  BookmarkJump: { id: number };
  BookmarkSave: never;
  BookmarkCancel: never;
  DirectoryAdd: never;
  DirectoryRemove: { path: string };
  DirectoryRescan: { path: string };
  SearchQueryChanged: { query: string };
  SortOrderChanged: { order: AudiobookSortOrder };
  FilterStatusChanged: { status: AudiobookFilterStatus };
  OpenSettings: never;
  CloseSettings: never;
  ThemeChanged: { theme: ThemeMode };
  PanelRatioChanged: { ratio: number };
  ToggleFocusMode: never;
  ShowKeyboardHelp: never;
  HideKeyboardHelp: never;
  CloseModal: never;
}

export type Event = {
  [K in EventType]: { type: K; payload: EventPayload[K] };
}[EventType];

type EventHandler<E extends EventType> = (payload: EventPayload[E]) => void | Promise<void>;

export class EventBus {
  private listeners: Map<EventType, Set<EventHandler<any>>> = new Map();

  on<E extends EventType>(event: E, handler: EventHandler<E>): () => void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)!.add(handler);

    // Return unsubscribe function
    return () => {
      this.listeners.get(event)?.delete(handler);
    };
  }

  async emit<E extends EventType>(event: E, payload?: EventPayload[E]): Promise<void> {
    const handlers = this.listeners.get(event);
    if (handlers) {
      for (const handler of handlers) {
        await handler(payload);
      }
    }
  }
}

export const eventBus = new EventBus();
