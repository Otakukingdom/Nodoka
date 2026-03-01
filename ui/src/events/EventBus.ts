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

type AnyEventHandler = EventHandler<EventType>;

export class EventBus {
  private listeners: Map<EventType, Set<AnyEventHandler>> = new Map<EventType, Set<AnyEventHandler>>();

  on<E extends EventType>(event: E, handler: EventHandler<E>): () => void {
    const existingHandlers = this.listeners.get(event);
    if (existingHandlers === undefined) {
      this.listeners.set(event, new Set<AnyEventHandler>([handler as AnyEventHandler]));
    } else {
      existingHandlers.add(handler as AnyEventHandler);
    }

    // Return unsubscribe function
    return () => {
      this.listeners.get(event)?.delete(handler as AnyEventHandler);
    };
  }

  async emit<E extends EventType>(event: E, payload?: EventPayload[E]): Promise<void> {
    const handlers = this.listeners.get(event);
    if (handlers) {
      for (const handler of handlers) {
        // Cast payload to the expected type - events with 'never' payload get undefined
        await handler(payload as EventPayload[E]);
      }
    }
  }
}

export const eventBus = new EventBus();
